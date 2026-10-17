//! Python adapter using docstrings as explicit documentation.

use std::path::Path;

use tree_sitter::{Language, Node, Tree};

use crate::model::{DocFile, DocSymbol, SymbolKind};

use super::{LanguageAdapter, diagnostics, node_text, source_span};

pub struct PythonAdapter;

impl LanguageAdapter for PythonAdapter {
    fn id(&self) -> &'static str {
        "python"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["py", "pyi"]
    }
    fn language(&self) -> Language {
        tree_sitter_python::LANGUAGE.into()
    }
    fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile {
        extract_python(path, source, tree)
    }
}

fn extract_python(path: &Path, source: &str, tree: &Tree) -> DocFile {
    let display_path = path.display().to_string();
    let root = tree.root_node();
    let documentation = suite_docstring(root.child_by_field_name("body").unwrap_or(root), source);
    let mut symbols = Vec::new();
    collect_symbols(root, false, true, source, &display_path, &mut symbols);
    DocFile {
        path: display_path.clone(),
        documentation,
        symbols,
        diagnostics: diagnostics(root, &display_path, "python"),
    }
}

fn collect_symbols(
    parent: Node<'_>,
    in_class: bool,
    at_module_scope: bool,
    source: &str,
    path: &str,
    symbols: &mut Vec<DocSymbol>,
) {
    let mut cursor = parent.walk();
    let children: Vec<_> = parent.named_children(&mut cursor).collect();
    for node in children {
        let kind = match node.kind() {
            "class_definition" => Some(SymbolKind::Struct),
            "function_definition" => Some(if in_class {
                SymbolKind::Method
            } else {
                SymbolKind::Function
            }),
            "assignment" if at_module_scope => constant_assignment(node, source),
            _ => None,
        };
        if let Some(kind) = kind {
            let name_node = if kind == SymbolKind::Constant {
                node.child_by_field_name("left")
            } else {
                node.child_by_field_name("name")
            };
            if let Some(name_node) = name_node
                && let Some(name) = node_text(name_node, source)
            {
                let body = node.child_by_field_name("body");
                let signature_end = body.map_or(node.end_byte(), |body| body.start_byte());
                let signature = source
                    .get(node.start_byte()..signature_end)
                    .map(str::trim)
                    .filter(|signature| !signature.is_empty())
                    .map(str::to_owned);
                symbols.push(DocSymbol {
                    kind,
                    name: name.to_owned(),
                    signature,
                    documentation: body.and_then(|body| suite_docstring(body, source)),
                    span: source_span(node, path),
                });
            }
        }
        let child_in_class = match node.kind() {
            "class_definition" => true,
            "function_definition" => false,
            _ => in_class,
        };
        let child_at_module_scope =
            at_module_scope && !matches!(node.kind(), "class_definition" | "function_definition");
        collect_symbols(
            node,
            child_in_class,
            child_at_module_scope,
            source,
            path,
            symbols,
        );
    }
}

fn constant_assignment(node: Node<'_>, source: &str) -> Option<SymbolKind> {
    let left = node.child_by_field_name("left")?;
    let name = node_text(left, source)?;
    (!name.is_empty()
        && name.chars().all(|ch| !ch.is_ascii_lowercase())
        && name.chars().any(|ch| ch.is_ascii_uppercase()))
    .then_some(SymbolKind::Constant)
}

fn suite_docstring(suite: Node<'_>, source: &str) -> Option<String> {
    let mut cursor = suite.walk();
    let first = suite.named_children(&mut cursor).next()?;
    if first.kind() != "expression_statement" {
        return None;
    }
    let mut cursor = first.walk();
    let literal = first.named_children(&mut cursor).next()?;
    if !matches!(literal.kind(), "string" | "concatenated_string") {
        return None;
    }
    let raw = node_text(literal, source)?.trim();
    Some(dedent_docstring(&unquote_docstring(raw)))
}

fn unquote_docstring(raw: &str) -> String {
    let quote_start = raw.find(['\'', '"']).unwrap_or(0);
    let content = &raw[quote_start..];
    let delimiter = if content.starts_with("'''") || content.starts_with("\"\"\"") {
        &content[..3]
    } else if content.starts_with('\'') || content.starts_with('"') {
        &content[..1]
    } else {
        return raw.to_owned();
    };
    content
        .strip_prefix(delimiter)
        .and_then(|body| body.strip_suffix(delimiter))
        .unwrap_or(content)
        .trim()
        .to_owned()
}

fn dedent_docstring(docstring: &str) -> String {
    let lines: Vec<_> = docstring.lines().collect();
    let indentation = lines
        .iter()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            if index == 0 || line.trim().is_empty() {
                line.trim_end()
            } else {
                &line[line.len().min(indentation)..]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned()
}
