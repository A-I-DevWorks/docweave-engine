//! C and C++ declaration extraction from their related Tree-sitter grammars.

use std::path::Path;

use tree_sitter::{Language, Node, Tree};

use crate::model::{DocFile, DocSymbol, SymbolKind};

use super::{LanguageAdapter, diagnostics, node_text, source_span};

pub struct CAdapter;
pub struct CppAdapter;

impl LanguageAdapter for CAdapter {
    fn id(&self) -> &'static str {
        "c"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["c", "h"]
    }
    fn language(&self) -> Language {
        tree_sitter_c::LANGUAGE.into()
    }
    fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile {
        extract_c_family(path, source, tree, "c")
    }
}

impl LanguageAdapter for CppAdapter {
    fn id(&self) -> &'static str {
        "cpp"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["cc", "cpp", "cxx", "hh", "hpp", "hxx"]
    }
    fn language(&self) -> Language {
        tree_sitter_cpp::LANGUAGE.into()
    }
    fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile {
        extract_c_family(path, source, tree, "cpp")
    }
}

fn extract_c_family(path: &Path, source: &str, tree: &Tree, language: &str) -> DocFile {
    let display_path = path.display().to_string();
    let root = tree.root_node();
    let children = named_children(root);
    let documentation = module_docs(&children, source);
    let mut symbols = Vec::new();
    collect_symbols(
        root,
        None,
        documentation.as_deref(),
        source,
        &display_path,
        &mut symbols,
    );
    DocFile {
        path: display_path.clone(),
        documentation,
        symbols,
        diagnostics: diagnostics(root, &display_path, language),
    }
}

fn collect_symbols(
    parent: Node<'_>,
    parent_scope: Option<SymbolKind>,
    file_documentation: Option<&str>,
    source: &str,
    path: &str,
    symbols: &mut Vec<DocSymbol>,
) {
    let children = named_children(parent);
    for (index, node) in children.iter().copied().enumerate() {
        if let Some(kind) = declaration_kind(node.kind()) {
            if let Some(name_node) = node.child_by_field_name("name")
                && let Some(name) = node_text(name_node, source)
            {
                let mut docs = leading_docs(&children, index, source);
                if kind == SymbolKind::Module && docs.as_deref() == file_documentation {
                    docs = None;
                }
                symbols.push(DocSymbol {
                    kind,
                    name: name.to_owned(),
                    signature: signature(node, source),
                    documentation: docs,
                    span: source_span(node, path),
                });
            }
            let child_scope = match kind {
                SymbolKind::Struct | SymbolKind::Enum => Some(kind),
                _ => parent_scope,
            };
            collect_symbols(node, child_scope, file_documentation, source, path, symbols);
            continue;
        }

        if matches!(
            node.kind(),
            "function_definition" | "declaration" | "field_declaration"
        ) && !contains_type_specifier(node)
            && let Some(function_node) = find_function_declarator(node)
            && let Some(name_node) = function_name(function_node)
            && let Some(name) = node_text(name_node, source)
        {
            symbols.push(DocSymbol {
                kind: if parent_scope == Some(SymbolKind::Struct) {
                    SymbolKind::Method
                } else {
                    SymbolKind::Function
                },
                name: name.to_owned(),
                signature: signature(node, source),
                documentation: leading_docs(&children, index, source),
                span: source_span(node, path),
            });
        }

        collect_symbols(
            node,
            parent_scope,
            file_documentation,
            source,
            path,
            symbols,
        );
    }
}

fn declaration_kind(node_kind: &str) -> Option<SymbolKind> {
    match node_kind {
        "struct_specifier" | "class_specifier" => Some(SymbolKind::Struct),
        "enum_specifier" => Some(SymbolKind::Enum),
        "namespace_definition" => Some(SymbolKind::Module),
        _ => None,
    }
}

fn named_children(node: Node<'_>) -> Vec<Node<'_>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).collect()
}

fn module_docs(children: &[Node<'_>], source: &str) -> Option<String> {
    let mut docs = Vec::new();
    let mut previous_end = 0;
    for node in children {
        if node.kind() != "comment"
            || source
                .get(previous_end..node.start_byte())
                .unwrap_or_default()
                .matches('\n')
                .count()
                > 1
        {
            break;
        }
        let Some(raw) = node_text(*node, source) else {
            break;
        };
        let Some(doc) = clean_doc_comment(raw) else {
            break;
        };
        docs.push(doc);
        previous_end = node.end_byte();
    }
    (!docs.is_empty()).then(|| docs.join("\n"))
}

fn contains_type_specifier(node: Node<'_>) -> bool {
    if matches!(
        node.kind(),
        "struct_specifier" | "class_specifier" | "enum_specifier"
    ) {
        return true;
    }
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .any(contains_type_specifier)
}

fn find_function_declarator(node: Node<'_>) -> Option<Node<'_>> {
    if node.kind() == "function_declarator" {
        return Some(node);
    }
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find_map(find_function_declarator)
}

fn function_name(function: Node<'_>) -> Option<Node<'_>> {
    let declarator = function.child_by_field_name("declarator")?;
    deepest_name(declarator)
}

fn deepest_name(node: Node<'_>) -> Option<Node<'_>> {
    if matches!(
        node.kind(),
        "identifier" | "field_identifier" | "qualified_identifier" | "operator_name"
    ) {
        return Some(node);
    }
    let mut cursor = node.walk();
    node.named_children(&mut cursor).find_map(deepest_name)
}

fn signature(node: Node<'_>, source: &str) -> Option<String> {
    let end = node
        .child_by_field_name("body")
        .map_or(node.end_byte(), |body| body.start_byte());
    source
        .get(node.start_byte()..end)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn leading_docs(children: &[Node<'_>], item_index: usize, source: &str) -> Option<String> {
    let mut docs = Vec::new();
    let mut next_start = children
        .get(item_index)
        .map_or(source.len(), |node| node.start_byte());
    for node in children[..item_index].iter().rev() {
        if node.kind() != "comment" {
            break;
        }
        let gap = source.get(node.end_byte()..next_start).unwrap_or_default();
        if gap.matches('\n').count() > 1 {
            break;
        }
        let Some(raw) = node_text(*node, source) else {
            break;
        };
        let Some(doc) = clean_doc_comment(raw) else {
            break;
        };
        docs.push(doc);
        next_start = node.start_byte();
    }
    docs.reverse();
    (!docs.is_empty()).then(|| docs.join("\n"))
}

fn clean_doc_comment(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if let Some(body) = trimmed
        .strip_prefix("///")
        .or_else(|| trimmed.strip_prefix("//!"))
    {
        return Some(body.strip_prefix(' ').unwrap_or(body).trim_end().to_owned());
    }
    if (trimmed.starts_with("/**") && !trimmed.starts_with("/***")) || trimmed.starts_with("/*!") {
        let body = trimmed
            .strip_prefix("/**")
            .or_else(|| trimmed.strip_prefix("/*!"))?
            .strip_suffix("*/")
            .unwrap_or(trimmed);
        let lines: Vec<_> = body
            .lines()
            .map(|line| {
                let line = line.trim_start();
                line.strip_prefix('*')
                    .map(|rest| rest.strip_prefix(' ').unwrap_or(rest))
                    .unwrap_or(line)
                    .trim_end()
            })
            .collect();
        return Some(lines.join("\n").trim().to_owned());
    }
    None
}
