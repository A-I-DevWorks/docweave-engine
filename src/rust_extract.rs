//! Rust Tree-sitter adapter that maps syntax trees to the shared documentation model.

use std::path::Path;

use tree_sitter::{Node, Tree};

use crate::model::{Diagnostic, DiagnosticSeverity, DocFile, DocSymbol, SourceSpan, SymbolKind};

/// Extract Rust declarations, explicit doc comments, and syntax diagnostics.
pub fn extract_rust(path: &Path, source: &str, tree: &Tree) -> DocFile {
    let display_path = path.display().to_string();
    let root = tree.root_node();
    let documentation = inner_docs(root, source);
    let mut symbols = Vec::new();
    collect_items(root, None, source, &display_path, &mut symbols);
    let mut diagnostics = Vec::new();
    collect_diagnostics(root, &display_path, &mut diagnostics);

    DocFile {
        path: display_path,
        documentation,
        symbols,
        diagnostics,
    }
}

fn collect_items(
    parent: Node<'_>,
    parent_kind: Option<&str>,
    source: &str,
    path: &str,
    symbols: &mut Vec<DocSymbol>,
) {
    let mut cursor = parent.walk();
    let children: Vec<_> = parent.named_children(&mut cursor).collect();
    for (index, node) in children.iter().copied().enumerate() {
        let (Some(kind), Some(name_node)) = (
            symbol_kind(node.kind(), parent_kind),
            node.child_by_field_name("name"),
        ) else {
            if matches!(node.kind(), "source_file" | "declaration_list") {
                collect_items(node, parent_kind, source, path, symbols);
            } else if matches!(node.kind(), "mod_item" | "impl_item" | "trait_item") {
                collect_items(node, Some(node.kind()), source, path, symbols);
            }
            continue;
        };
        if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
            let mut doc = leading_docs(&children, index, source);
            if kind == SymbolKind::Module
                && let Some(body) = node.child_by_field_name("body")
                && let Some(inner) = inner_docs(body, source)
            {
                doc = Some(match doc {
                    Some(outer) => format!("{outer}\n{inner}"),
                    None => inner,
                });
            }
            symbols.push(DocSymbol {
                kind,
                name: name.to_owned(),
                signature: signature(node, source),
                documentation: doc,
                span: span(node, path),
            });
        }

        if matches!(node.kind(), "source_file" | "declaration_list") {
            collect_items(node, parent_kind, source, path, symbols);
        } else if matches!(node.kind(), "mod_item" | "impl_item" | "trait_item") {
            collect_items(node, Some(node.kind()), source, path, symbols);
        }
    }
}

fn symbol_kind(kind: &str, parent_kind: Option<&str>) -> Option<SymbolKind> {
    match kind {
        "mod_item" => Some(SymbolKind::Module),
        "struct_item" => Some(SymbolKind::Struct),
        "enum_item" => Some(SymbolKind::Enum),
        "trait_item" => Some(SymbolKind::Trait),
        "function_item" if matches!(parent_kind, Some("impl_item" | "trait_item")) => {
            Some(SymbolKind::Method)
        }
        "function_signature_item" => Some(SymbolKind::Method),
        "function_item" => Some(SymbolKind::Function),
        "const_item" | "static_item" => Some(SymbolKind::Constant),
        _ => None,
    }
}

fn leading_docs(children: &[Node<'_>], item_index: usize, source: &str) -> Option<String> {
    let mut docs = Vec::new();
    for node in children[..item_index].iter().rev() {
        if node.kind() == "attribute_item" {
            continue;
        }
        let Ok(comment) = node.utf8_text(source.as_bytes()) else {
            break;
        };
        let comment = comment.trim();
        if let Some(text) = comment.strip_prefix("///") {
            docs.push(text.strip_prefix(' ').unwrap_or(text).to_owned());
        } else {
            break;
        }
    }
    docs.reverse();
    (!docs.is_empty()).then(|| docs.join("\n"))
}

fn inner_docs(node: Node<'_>, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    let mut docs = Vec::new();
    for child in node.named_children(&mut cursor) {
        let Ok(text) = child.utf8_text(source.as_bytes()) else {
            break;
        };
        let text = text.trim();
        if let Some(doc) = text.strip_prefix("//!") {
            docs.push(doc.strip_prefix(' ').unwrap_or(doc).to_owned());
        } else if !matches!(child.kind(), "attribute_item") {
            break;
        }
    }
    (!docs.is_empty()).then(|| docs.join("\n"))
}

fn signature(node: Node<'_>, source: &str) -> Option<String> {
    let end = (0..node.child_count())
        .filter_map(|index| node.child(index))
        .find(|child| {
            matches!(
                child.kind(),
                "block" | "field_declaration_list" | "enum_variant_list" | "declaration_list"
            )
        })
        .map_or(node.end_byte(), |body| body.start_byte());
    source
        .get(node.start_byte()..end)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn span(node: Node<'_>, path: &str) -> SourceSpan {
    let start = node.start_position();
    let end = node.end_position();
    SourceSpan {
        path: path.to_owned(),
        start_line: start.row + 1,
        start_column: start.column + 1,
        end_line: end.row + 1,
        end_column: end.column + 1,
    }
}

fn collect_diagnostics(node: Node<'_>, path: &str, diagnostics: &mut Vec<Diagnostic>) {
    if node.is_error() {
        diagnostics.push(Diagnostic {
            code: "rust.syntax_error".to_owned(),
            severity: DiagnosticSeverity::Error,
            message: "Tree-sitter found invalid Rust syntax.".to_owned(),
            span: Some(span(node, path)),
        });
    } else if node.is_missing() {
        diagnostics.push(Diagnostic {
            code: "rust.missing_syntax".to_owned(),
            severity: DiagnosticSeverity::Error,
            message: "Tree-sitter expected a syntax element that is missing.".to_owned(),
            span: Some(span(node, path)),
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_diagnostics(child, path, diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn parse(source: &str) -> Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Rust grammar");
        parser.parse(source, None).expect("parse source")
    }

    #[test]
    fn extracts_nested_items_generics_attributes_and_docs() {
        let source = r#"//! Crate guide.
/// Outer module.
#[allow(dead_code)]
mod outer {
    //! Inner module guide.
    /// A generic record.
    #[derive(Debug)]
    pub struct Record<T> { value: T }
    /// A generic trait.
    pub trait Convert<T> { fn convert(&self) -> T; }
    impl<T> Record<T> {
        /// Returns the stored value.
        pub fn value(&self) -> &T { &self.value }
    }
    /// An enum.
    pub enum Choice<T> { One(T), None }
    /// A constant.
    pub const ANSWER: usize = 42;
    /// A function.
    pub fn make<T>(value: T) -> Record<T> { Record { value } }
}"#;
        let tree = parse(source);
        let file = extract_rust(Path::new("lib.rs"), source, &tree);

        assert_eq!(file.documentation.as_deref(), Some("Crate guide."));
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "outer" && s.kind == SymbolKind::Module)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "Record" && s.kind == SymbolKind::Struct)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "Convert" && s.kind == SymbolKind::Trait)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "convert" && s.kind == SymbolKind::Method)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "value" && s.kind == SymbolKind::Method)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "Choice" && s.kind == SymbolKind::Enum)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "ANSWER" && s.kind == SymbolKind::Constant)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "make" && s.kind == SymbolKind::Function)
        );
        assert!(
            file.symbols
                .iter()
                .any(|s| s.name == "Record"
                    && s.documentation.as_deref() == Some("A generic record."))
        );
    }

    #[test]
    fn malformed_syntax_becomes_a_diagnostic() {
        let source = include_str!("../tests/fixtures/malformed.rs");
        let tree = parse(source);
        let file = extract_rust(Path::new("bad.rs"), source, &tree);

        assert!(!file.diagnostics.is_empty());
        assert!(file.diagnostics.iter().all(|d| d.span.is_some()));
    }

    #[test]
    fn fixture_covers_nested_generics_attributes_and_doc_comments() {
        let source = include_str!("../tests/fixtures/nested_generics.rs");
        let tree = parse(source);
        let file = extract_rust(Path::new("nested_generics.rs"), source, &tree);

        assert_eq!(
            file.documentation.as_deref(),
            Some("Fixture for file-level documentation.")
        );
        let module = file.symbols.iter().find(|s| s.name == "outer").unwrap();
        assert_eq!(
            module.documentation.as_deref(),
            Some("Outer module docs.\nInline module documentation.")
        );
        assert!(file.symbols.iter().any(|s| {
            s.name == "Record"
                && s.signature
                    .as_deref()
                    .is_some_and(|sig| sig.contains("Record<T>"))
        }));
        assert!(
            file.symbols.iter().any(|s| s.name == "make"
                && s.documentation.as_deref() == Some("Generic function docs."))
        );
    }
}
