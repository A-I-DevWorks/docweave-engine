//! Consistency and source-content checks applied before documentation export.

use std::path::{Component, Path};

use pulldown_cmark::{Event, Options, Parser, Tag};

use crate::model::{Diagnostic, DiagnosticSeverity, DocProject, SourceSpan};

/// Append warnings for inconsistent model data and risky source-derived text.
pub fn append_consistency_diagnostics(project: &mut DocProject) {
    let mut seen_paths = std::collections::HashSet::new();
    for file in &mut project.files {
        if !is_safe_relative_path(&file.path) {
            file.diagnostics.push(warning(
                "validation.unsafe_source_path",
                "Source path is absolute or contains parent-directory traversal.",
                None,
            ));
        }
        if !seen_paths.insert(file.path.clone()) {
            file.diagnostics.push(warning(
                "validation.duplicate_file_path",
                "More than one source file has the same path in the documentation model.",
                None,
            ));
        }
        check_control_characters(&file.path, None, &mut file.diagnostics);
        if let Some(documentation) = &file.documentation {
            check_markdown(documentation, None, &mut file.diagnostics);
        }
        for symbol in &mut file.symbols {
            if symbol.name.trim().is_empty() {
                file.diagnostics.push(warning(
                    "validation.empty_symbol_name",
                    "Extracted declaration has an empty name.",
                    Some(symbol.span.clone()),
                ));
            }
            if symbol.span.path != file.path {
                file.diagnostics.push(warning(
                    "validation.span_path_mismatch",
                    "Declaration source span points to a different file path.",
                    Some(symbol.span.clone()),
                ));
            }
            if !valid_span(&symbol.span) {
                file.diagnostics.push(warning(
                    "validation.invalid_source_span",
                    "Declaration has an invalid or zero-based source range.",
                    Some(symbol.span.clone()),
                ));
            }
            check_control_characters(&symbol.name, Some(&symbol.span), &mut file.diagnostics);
            if let Some(signature) = &symbol.signature {
                check_control_characters(signature, Some(&symbol.span), &mut file.diagnostics);
            }
            if let Some(documentation) = &symbol.documentation {
                check_markdown(documentation, Some(&symbol.span), &mut file.diagnostics);
            }
        }
        if file.diagnostics.iter().any(|diagnostic| {
            diagnostic
                .span
                .as_ref()
                .is_some_and(|span| !valid_span(span))
        }) {
            file.diagnostics.push(warning(
                "validation.invalid_diagnostic_span",
                "A diagnostic contains an invalid or zero-based source range.",
                None,
            ));
        }
    }
}

fn is_safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

fn valid_span(span: &SourceSpan) -> bool {
    span.start_line > 0
        && span.start_column > 0
        && span.end_line > 0
        && span.end_column > 0
        && (span.end_line, span.end_column) >= (span.start_line, span.start_column)
}

fn check_markdown(text: &str, span: Option<&SourceSpan>, diagnostics: &mut Vec<Diagnostic>) {
    check_control_characters(text, span, diagnostics);
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    for event in Parser::new_ext(text, options) {
        let destination = match event {
            Event::Start(Tag::Link { dest_url, .. })
            | Event::Start(Tag::Image { dest_url, .. }) => Some(dest_url),
            _ => None,
        };
        if destination.is_some_and(|url| !crate::html::safe_url(&url)) {
            diagnostics.push(warning(
                "content.unsafe_link",
                "Documentation contains a link with a URL scheme that HTML export blocks.",
                span.cloned(),
            ));
        }
    }
}

fn check_control_characters(
    text: &str,
    span: Option<&SourceSpan>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if text
        .chars()
        .any(|ch| ch.is_control() && !matches!(ch, '\n' | '\r' | '\t'))
    {
        diagnostics.push(warning(
            "content.control_character",
            "Source-derived text contains a control character that may disrupt generated output.",
            span.cloned(),
        ));
    }
}

fn warning(code: &str, message: &str, span: Option<SourceSpan>) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        severity: DiagnosticSeverity::Warning,
        message: message.to_owned(),
        span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DocFile, DocSymbol, SymbolKind};

    #[test]
    fn reports_unsafe_paths_inconsistent_spans_and_control_characters() {
        let mut project = DocProject {
            root: "/project".to_owned(),
            files: vec![DocFile {
                path: "../escape.rs".to_owned(),
                documentation: None,
                symbols: vec![DocSymbol {
                    kind: SymbolKind::Function,
                    name: "bad\u{0001}name".to_owned(),
                    signature: None,
                    documentation: None,
                    span: SourceSpan {
                        path: "other.rs".to_owned(),
                        start_line: 0,
                        start_column: 0,
                        end_line: 0,
                        end_column: 0,
                    },
                }],
                diagnostics: Vec::new(),
            }],
        };

        append_consistency_diagnostics(&mut project);
        let codes: Vec<_> = project.files[0]
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"validation.unsafe_source_path"));
        assert!(codes.contains(&"validation.span_path_mismatch"));
        assert!(codes.contains(&"validation.invalid_source_span"));
        assert!(codes.contains(&"content.control_character"));
    }
}
