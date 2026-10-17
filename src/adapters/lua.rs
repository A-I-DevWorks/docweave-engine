//! Lua adapter for documented named functions and method-style declarations.

use std::path::Path;

use tree_sitter::{Language, Node, Tree};

use crate::model::{DocFile, DocSymbol, SymbolKind};

use super::{LanguageAdapter, diagnostics, node_text, source_span};

pub struct LuaAdapter;

impl LanguageAdapter for LuaAdapter {
    fn id(&self) -> &'static str {
        "lua"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["lua"]
    }
    fn language(&self) -> Language {
        tree_sitter_lua::LANGUAGE.into()
    }
    fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile {
        extract_lua(path, source, tree)
    }
}

fn extract_lua(path: &Path, source: &str, tree: &Tree) -> DocFile {
    let display_path = path.display().to_string();
    let root = tree.root_node();
    let children = named_children(root);
    let documentation = module_docs(&children, source);
    let mut symbols = Vec::new();
    collect_functions(root, source, &display_path, &mut symbols);
    DocFile {
        path: display_path.clone(),
        documentation,
        symbols,
        diagnostics: diagnostics(root, &display_path, "lua"),
    }
}

fn collect_functions(parent: Node<'_>, source: &str, path: &str, symbols: &mut Vec<DocSymbol>) {
    let children = named_children(parent);
    for (index, node) in children.iter().copied().enumerate() {
        if node.kind() == "function_declaration"
            && let Some(name_node) = node.child_by_field_name("name")
            && let Some(name) = node_text(name_node, source)
        {
            let body = node.child_by_field_name("body").or_else(|| {
                let mut cursor = node.walk();
                node.named_children(&mut cursor)
                    .find(|child| child.kind() == "block")
            });
            let end = body.map_or(node.end_byte(), |body| body.start_byte());
            let signature = source
                .get(node.start_byte()..end)
                .map(str::trim)
                .filter(|signature| !signature.is_empty())
                .map(str::to_owned);
            symbols.push(DocSymbol {
                kind: if name.contains(':') {
                    SymbolKind::Method
                } else {
                    SymbolKind::Function
                },
                name: name.to_owned(),
                signature,
                documentation: leading_docs(&children, index, source),
                span: source_span(node, path),
            });
        }
        collect_functions(node, source, path, symbols);
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
    let raw = raw.trim();
    let doc = raw.strip_prefix("---")?;
    Some(doc.strip_prefix(' ').unwrap_or(doc).trim_end().to_owned())
}
