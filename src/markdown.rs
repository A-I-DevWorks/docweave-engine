//! Deterministic Markdown renderer for the language-neutral documentation model.

use std::path::{Component, Path, PathBuf};

use crate::model::{DocProject, SourceSpan};

/// Render a project as Markdown, linking declarations to local source or a source URL.
pub fn render(
    project: &DocProject,
    title: &str,
    description: Option<&str>,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String {
    let anchors = Anchors::new(project);
    let mut output = format!("# {}\n\n", escape_heading(title));
    if let Some(description) = description {
        output.push_str(description.trim());
        output.push_str("\n\n");
    }
    output.push_str("## Contents\n\n");

    for (file_index, file) in project.files.iter().enumerate() {
        output.push_str(&format!(
            "- [{}](#{})\n",
            code_span(
                &project_relative_path(project, &file.path)
                    .display()
                    .to_string()
            ),
            anchors.files[file_index]
        ));
        for (symbol_index, symbol) in file.symbols.iter().enumerate() {
            output.push_str(&format!(
                "  - [{} {}](#{})\n",
                kind_label(symbol.kind),
                code_span(&symbol.name),
                anchors.symbols[file_index][symbol_index]
            ));
        }
    }
    let modules: Vec<_> = project
        .files
        .iter()
        .enumerate()
        .flat_map(|(file_index, file)| {
            file.symbols
                .iter()
                .enumerate()
                .filter(|(_, symbol)| symbol.kind == crate::model::SymbolKind::Module)
                .map(move |(symbol_index, symbol)| (file_index, symbol_index, file, symbol))
        })
        .collect();
    if !modules.is_empty() {
        output.push_str("## Modules\n\n");
        for (file_index, symbol_index, file, symbol) in modules {
            output.push_str(&format!(
                "- [{}](#{}) — {}\n",
                code_span(&symbol.name),
                anchors.symbols[file_index][symbol_index],
                code_span(
                    &project_relative_path(project, &file.path)
                        .display()
                        .to_string()
                )
            ));
        }
        output.push('\n');
    }
    output.push('\n');

    for (file_index, file) in project.files.iter().enumerate() {
        output.push_str(&format!(
            "<a id=\"{}\"></a>\n## {}\n\n",
            anchors.files[file_index],
            code_span(
                &project_relative_path(project, &file.path)
                    .display()
                    .to_string()
            )
        ));
        if let Some(documentation) = &file.documentation {
            output.push_str(documentation.trim());
            output.push_str("\n\n");
        }

        for (symbol_index, symbol) in file.symbols.iter().enumerate() {
            output.push_str(&format!(
                "<a id=\"{}\"></a>\n### {}: {}\n\n",
                anchors.symbols[file_index][symbol_index],
                kind_label(symbol.kind),
                code_span(&symbol.name)
            ));
            if let Some(signature) = &symbol.signature {
                output.push_str(&format!(
                    "{}\n\n",
                    fenced_code(signature, syntax_language(&file.path))
                ));
            }
            if let Some(documentation) = &symbol.documentation {
                output.push_str(documentation.trim());
                output.push_str("\n\n");
            }
            output.push_str(&format!(
                "[View source]({})\n\n",
                source_link(&symbol.span, project, output_parent, source_url)
            ));
        }

        if !file.diagnostics.is_empty() {
            output.push_str("### Diagnostics\n\n");
            for diagnostic in &file.diagnostics {
                output.push_str(&format!("- `{}` — {}", diagnostic.code, diagnostic.message));
                if let Some(span) = &diagnostic.span {
                    output.push_str(&format!(
                        " ([line {}]({}))",
                        span.start_line,
                        source_link(span, project, output_parent, source_url)
                    ));
                }
                output.push('\n');
            }
            output.push('\n');
        }
    }

    format!("{}\n", output.trim_end_matches('\n'))
}

struct Anchors {
    files: Vec<String>,
    symbols: Vec<Vec<String>>,
}

impl Anchors {
    fn new(project: &DocProject) -> Self {
        let mut used = std::collections::HashMap::<String, usize>::new();
        let mut files = Vec::with_capacity(project.files.len());
        let mut symbols = Vec::with_capacity(project.files.len());
        for file in &project.files {
            let file_path = project_relative_path(project, &file.path)
                .display()
                .to_string();
            files.push(unique_anchor(
                &mut used,
                format!("file-{}", slug(&file_path)),
            ));
            let file_symbols = file
                .symbols
                .iter()
                .map(|symbol| {
                    unique_anchor(
                        &mut used,
                        format!(
                            "symbol-{}-{}-{}",
                            slug(&file_path),
                            slug(kind_label(symbol.kind)),
                            slug(&symbol.name)
                        ),
                    )
                })
                .collect();
            symbols.push(file_symbols);
        }
        Self { files, symbols }
    }
}

fn unique_anchor(used: &mut std::collections::HashMap<String, usize>, base: String) -> String {
    let count = used.entry(base.clone()).or_default();
    *count += 1;
    if *count == 1 {
        base
    } else {
        format!("{base}-{}", *count)
    }
}

fn slug(value: &str) -> String {
    let mut slug = String::new();
    let mut dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash && !slug.is_empty() {
            slug.push('-');
            dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        "item".to_owned()
    } else {
        slug
    }
}

fn kind_label(kind: crate::model::SymbolKind) -> &'static str {
    match kind {
        crate::model::SymbolKind::Module => "Module",
        crate::model::SymbolKind::Struct => "Struct",
        crate::model::SymbolKind::Enum => "Enum",
        crate::model::SymbolKind::Trait => "Trait",
        crate::model::SymbolKind::Function => "Function",
        crate::model::SymbolKind::Method => "Method",
        crate::model::SymbolKind::Constant => "Constant",
    }
}

fn source_link(
    span: &SourceSpan,
    project: &DocProject,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String {
    let relative = project_relative_path(project, &span.path);
    let target = if let Some(base) = source_url {
        format!(
            "{}/{}",
            encode_url_base(base.trim_end_matches('/')),
            encode_path(&relative)
        )
    } else {
        let root = Path::new(&project.root);
        let file = if relative.is_absolute() {
            relative.clone()
        } else {
            root.join(&relative)
        };
        let relative_link = relative_path(output_parent, &file);
        encode_path(&relative_link)
    };
    format!("{target}#L{}", span.start_line)
}

fn project_relative_path(project: &DocProject, path: &str) -> PathBuf {
    let path = Path::new(path);
    let relative = if path.is_absolute() {
        path.strip_prefix(Path::new(&project.root)).ok()
    } else {
        Some(path)
    };
    if let Some(relative) = relative
        && relative
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
    {
        relative.to_path_buf()
    } else {
        PathBuf::from("source-unavailable")
    }
}

fn relative_path(from: &Path, to: &Path) -> PathBuf {
    let from_components: Vec<_> = from
        .components()
        .filter(|c| *c != Component::CurDir)
        .collect();
    let to_components: Vec<_> = to
        .components()
        .filter(|c| *c != Component::CurDir)
        .collect();
    let common = from_components
        .iter()
        .zip(&to_components)
        .take_while(|(left, right)| left == right)
        .count();
    let mut result = PathBuf::new();
    for component in &from_components[common..] {
        if matches!(component, Component::Normal(_)) {
            result.push("..");
        }
    }
    for component in &to_components[common..] {
        result.push(component.as_os_str());
    }
    result
}

fn encode_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn encode_url_base(base: &str) -> String {
    base.bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~:/%".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn code_span(value: &str) -> String {
    let fence = "`".repeat(max_backtick_run(value) + 1);
    let padding = if value.starts_with([' ', '`']) || value.ends_with([' ', '`']) {
        " "
    } else {
        ""
    };
    format!("{fence}{padding}{value}{padding}{fence}")
}

fn fenced_code(value: &str, language: &str) -> String {
    let longest = value.lines().map(max_backtick_run).max().unwrap_or(0);
    let fence = "`".repeat((longest + 1).max(3));
    format!("{fence}{language}\n{value}\n{fence}")
}

fn syntax_language(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "rs" => "rust",
        "c" | "h" => "c",
        "cc" | "cpp" | "cxx" | "hh" | "hpp" | "hxx" => "cpp",
        "py" | "pyi" => "python",
        "lua" => "lua",
        _ => "text",
    }
}

fn max_backtick_run(value: &str) -> usize {
    let mut longest = 0;
    let mut current = 0;
    for ch in value.chars() {
        if ch == '`' {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    longest
}

fn escape_heading(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        if "\\`*_{}[]<>#+-.!|".contains(ch) {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Diagnostic, DiagnosticSeverity, DocFile, DocSymbol, SymbolKind};

    fn fixture() -> DocProject {
        DocProject {
            root: "/repo".into(),
            files: vec![DocFile {
                path: "/repo/src/lib.rs".into(),
                documentation: Some("Library docs.".into()),
                symbols: vec![DocSymbol {
                    kind: SymbolKind::Function,
                    name: "hello-world".into(),
                    signature: Some("pub fn hello_world()".into()),
                    documentation: Some("Greets the reader.".into()),
                    span: SourceSpan {
                        path: "/repo/src/lib.rs".into(),
                        start_line: 7,
                        start_column: 1,
                        end_line: 9,
                        end_column: 2,
                    },
                }],
                diagnostics: vec![Diagnostic {
                    code: "rust.syntax_error".into(),
                    severity: DiagnosticSeverity::Error,
                    message: "Invalid syntax.".into(),
                    span: None,
                }],
            }],
        }
    }

    #[test]
    fn renders_deterministic_anchors_and_local_source_links() {
        let project = fixture();
        let first = render(&project, "API docs", None, Path::new("/repo/docs"), None);
        let second = render(&project, "API docs", None, Path::new("/repo/docs"), None);
        assert_eq!(first, second);
        assert!(first.contains("<a id=\"symbol-src-lib-rs-function-hello-world\"></a>"));
        assert!(first.contains("[View source](../src/lib.rs#L7)"));
        assert!(first.contains("Greets the reader."));
    }

    #[test]
    fn remote_links_encode_source_paths_and_duplicate_anchors_are_unique() {
        let mut project = fixture();
        project.files[0].path = "/repo/src/a b.rs".into();
        project.files[0].symbols[0].span.path = project.files[0].path.clone();
        let duplicate = project.files[0].symbols[0].clone();
        project.files[0].symbols.push(duplicate);
        let output = render(
            &project,
            "API docs",
            None,
            Path::new("/repo/docs"),
            Some("https://example.test/repo/blob/main"),
        );
        assert!(output.contains("a%20b.rs#L7"));
        assert!(output.contains("symbol-src-a-b-rs-function-hello-world-2"));
    }

    #[test]
    fn inline_code_delimiters_do_not_break_on_backticks_in_names() {
        assert_eq!(code_span("odd``name"), "```odd``name```");
    }
}
