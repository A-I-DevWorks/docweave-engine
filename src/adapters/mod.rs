//! Source-language adapters that normalize syntax trees into the shared model.

mod c_like;
mod lua;
mod python;
mod rust;

use std::path::Path;

use tree_sitter::{Language, Node, Parser, Tree};

use crate::model::{Diagnostic, DiagnosticSeverity, DocFile, SourceSpan};

/// Stable extension point for a source language supported by DocWeave.
///
/// Adapters own grammar-specific syntax and documentation conventions. They
/// must return source-relative paths, one-based source spans with exclusive
/// ends, and only facts supported by the syntax tree. Use the shared helpers
/// below to keep parse diagnostics and span behavior consistent.
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
    adapter_for_path_in(path, &ADAPTERS)
}

/// Find the adapter responsible for a path in a caller-supplied registry.
pub fn adapter_for_path_in<'a>(
    path: &Path,
    adapters: &'a [&dyn LanguageAdapter],
) -> Option<&'a dyn LanguageAdapter> {
    let extension = path.extension()?.to_str()?;
    adapters.iter().copied().find(|adapter| {
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
    parser_for_in(path, &ADAPTERS)
}

/// Configure a parser using a caller-supplied adapter registry.
pub fn parser_for_in<'a>(
    path: &Path,
    adapters: &'a [&dyn LanguageAdapter],
) -> anyhow::Result<Option<(Parser, &'a dyn LanguageAdapter)>> {
    let Some(adapter) = adapter_for_path_in(path, adapters) else {
        return Ok(None);
    };
    let mut parser = Parser::new();
    parser
        .set_language(&adapter.language())
        .map_err(anyhow::Error::msg)
        .map_err(|error| error.context(format!("failed to initialize {} grammar", adapter.id())))?;
    Ok(Some((parser, adapter)))
}

/// Validate identifiers and extensions in an adapter registry.
pub fn validate_registry(adapters: &[&dyn LanguageAdapter]) -> anyhow::Result<()> {
    let mut ids = std::collections::HashSet::new();
    let mut extensions = std::collections::HashSet::new();
    for adapter in adapters {
        let id = adapter.id();
        if id.is_empty()
            || !id
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
        {
            anyhow::bail!("adapter id {id:?} must use lowercase ASCII letters, digits, '_' or '-'");
        }
        if !ids.insert(id) {
            anyhow::bail!("adapter id {id:?} is registered more than once");
        }
        if adapter.extensions().is_empty() {
            anyhow::bail!("adapter {id:?} must register at least one file extension");
        }
        for extension in adapter.extensions() {
            let normalized = extension.to_ascii_lowercase();
            if normalized.is_empty() || !normalized.chars().all(|ch| ch.is_ascii_alphanumeric()) {
                anyhow::bail!("adapter {id:?} has invalid extension {extension:?}");
            }
            if !extensions.insert(normalized.clone()) {
                anyhow::bail!("file extension {normalized:?} is registered by multiple adapters");
            }
        }
    }
    Ok(())
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
