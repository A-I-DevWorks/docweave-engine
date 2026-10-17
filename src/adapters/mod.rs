//! Source-language adapters that normalize syntax trees into the shared model.

mod c_like;
mod lua;
mod python;
mod rust;

use std::path::Path;

use tree_sitter::{Language, Node, Parser, Tree};

use crate::model::{Diagnostic, DiagnosticSeverity, DocFile, SourceSpan};

/// Contract for a language implementation registered with the source scanner.
///
/// An adapter owns file extensions, Tree-sitter grammar selection, and the
/// mapping from that grammar's nodes and comments into the shared model.
pub trait LanguageAdapter: Sync {
    /// Stable identifier used in diagnostics and the adapter registry.
    fn id(&self) -> &'static str;

    /// File extensions handled by this adapter, without a leading dot.
    fn extensions(&self) -> &'static [&'static str];

    /// Return the Tree-sitter grammar for this language.
    fn language(&self) -> Language;

    /// Extract one parsed file into the language-neutral model.
    fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile;
}

static RUST: rust::RustAdapter = rust::RustAdapter;
static C: c_like::CAdapter = c_like::CAdapter;
static CPP: c_like::CppAdapter = c_like::CppAdapter;
static PYTHON: python::PythonAdapter = python::PythonAdapter;
static LUA: lua::LuaAdapter = lua::LuaAdapter;

/// Built-in adapters in deterministic extension-dispatch order.
pub static ADAPTERS: [&dyn LanguageAdapter; 5] = [&RUST, &C, &CPP, &PYTHON, &LUA];

/// Find the registered adapter responsible for a source path.
pub fn adapter_for_path(path: &Path) -> Option<&'static dyn LanguageAdapter> {
    let extension = path.extension()?.to_str()?;
    ADAPTERS.iter().copied().find(|adapter| {
        adapter
            .extensions()
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(extension))
    })
}

/// Find a registered adapter by its stable identifier.
pub fn adapter_by_id(id: &str) -> Option<&'static dyn LanguageAdapter> {
    ADAPTERS.iter().copied().find(|adapter| adapter.id() == id)
}

/// Configure a parser for the adapter selected by a source file extension.
pub fn parser_for(path: &Path) -> anyhow::Result<Option<(Parser, &'static dyn LanguageAdapter)>> {
    let Some(adapter) = adapter_for_path(path) else {
        return Ok(None);
    };
    let mut parser = Parser::new();
    parser
        .set_language(&adapter.language())
        .map_err(anyhow::Error::msg)
        .map_err(|error| error.context(format!("failed to initialize {} grammar", adapter.id())))?;
    Ok(Some((parser, adapter)))
}

/// Produce stable parse diagnostics shared by the built-in adapters.
pub fn diagnostics(root: Node<'_>, path: &str, language: &str) -> Vec<Diagnostic> {
    fn visit(node: Node<'_>, path: &str, language: &str, out: &mut Vec<Diagnostic>) {
        if node.is_error() || node.is_missing() {
            let start = node.start_position();
            let end = node.end_position();
            out.push(Diagnostic {
                code: if node.is_error() {
                    format!("{language}.syntax_error")
                } else {
                    format!("{language}.missing_syntax")
                },
                severity: DiagnosticSeverity::Error,
                message: if node.is_error() {
                    format!("Tree-sitter found invalid {language} syntax.")
                } else {
                    format!("Tree-sitter expected a {language} syntax element that is missing.")
                },
                span: Some(SourceSpan {
                    path: path.to_owned(),
                    start_line: start.row + 1,
                    start_column: start.column + 1,
                    end_line: end.row + 1,
                    end_column: end.column + 1,
                }),
            });
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            visit(child, path, language, out);
        }
    }

    let mut out = Vec::new();
    visit(root, path, language, &mut out);
    out
}

/// Extract a one-based source range from a syntax node.
pub fn source_span(node: Node<'_>, path: &str) -> SourceSpan {
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

/// Read a node's source text without panicking on invalid UTF-8 boundaries.
pub fn node_text<'a>(node: Node<'_>, source: &'a str) -> Option<&'a str> {
    node.utf8_text(source.as_bytes()).ok()
}
