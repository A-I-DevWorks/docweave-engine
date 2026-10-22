//! HTML renderer with escaped source text and safe Markdown conversion.

use std::path::{Component, Path};

use pulldown_cmark::{Event, Options, Parser, Tag};

use crate::model::{DocProject, SourceSpan};

/// Render a project as a self-contained HTML document.
pub fn render(
    project: &DocProject,
    title: &str,
    description: Option<&str>,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String {
    let mut output = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{}</title>\n<style>body{{font:16px/1.6 system-ui,sans-serif;max-width:72rem;margin:2rem auto;padding:0 1rem;color:#222}}pre{{overflow:auto;background:#f5f5f5;padding:1rem}}code{{font-family:ui-monospace,monospace}}.source{{font-size:.9em}}.diagnostics{{color:#8a2d2d}}</style>\n</head>\n<body>\n<h1>{}</h1>\n<nav aria-label=\"Contents\"><h2>Contents</h2><ul>\n",
        escape_html(title),
        escape_html(title)
    );

    for (file_index, file) in project.files.iter().enumerate() {
        let file_id = format!("file-{file_index}");
        output.push_str(&format!(
            "<li><a href=\"#{file_id}\"><code>{}</code></a><ul>\n",
            escape_html(&file.path)
        ));
        for (symbol_index, symbol) in file.symbols.iter().enumerate() {
            output.push_str(&format!(
                "<li><a href=\"#symbol-{file_index}-{symbol_index}\">{} <code>{}</code></a></li>\n",
                kind_label(symbol.kind),
                escape_html(&symbol.name)
            ));
        }
        output.push_str("</ul></li>\n");
    }
    output.push_str("</ul></nav>\n<main>\n");
    if let Some(description) = description {
        output.push_str(&render_markdown(description));
    }

    for (file_index, file) in project.files.iter().enumerate() {
        output.push_str(&format!(
            "<section id=\"file-{file_index}\"><h2><code>{}</code></h2>\n",
            escape_html(&file.path)
        ));
        if let Some(documentation) = &file.documentation {
            output.push_str(&render_markdown(documentation));
        }
        for (symbol_index, symbol) in file.symbols.iter().enumerate() {
            output.push_str(&format!(
                "<article id=\"symbol-{file_index}-{symbol_index}\"><h3>{}: <code>{}</code></h3>\n",
                kind_label(symbol.kind),
                escape_html(&symbol.name)
            ));
            if let Some(signature) = &symbol.signature {
                output.push_str(&format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>\n",
                    syntax_language(&file.path),
                    escape_html(signature)
                ));
            }
            if let Some(documentation) = &symbol.documentation {
                output.push_str(&render_markdown(documentation));
            }
            output.push_str(&format!(
                "<p class=\"source\"><a href=\"{}\">View source (line {})</a></p>\n",
                escape_html(&source_link(
                    &symbol.span,
                    project,
                    output_parent,
                    source_url
                )),
                symbol.span.start_line
            ));
            output.push_str("</article>\n");
        }
        if !file.diagnostics.is_empty() {
            output.push_str("<section class=\"diagnostics\"><h3>Diagnostics</h3><ul>\n");
            for diagnostic in &file.diagnostics {
                output.push_str(&format!(
                    "<li><code>{}</code>: {}",
                    escape_html(&diagnostic.code),
                    escape_html(&diagnostic.message)
                ));
                if let Some(span) = &diagnostic.span {
                    output.push_str(&format!(
                        " <a href=\"{}\">line {}</a>",
                        escape_html(&source_link(span, project, output_parent, source_url)),
                        span.start_line
                    ));
                }
                output.push_str("</li>\n");
            }
            output.push_str("</ul></section>\n");
        }
        output.push_str("</section>\n");
    }
    output.push_str("</main>\n</body>\n</html>\n");
    output
}

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options
        .insert(Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        // Raw HTML in comments is treated as text so it cannot inject markup or scripts.
        Event::Html(html) | Event::InlineHtml(html) => Event::Text(html),
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Link {
            link_type,
            dest_url: if safe_url(dest_url.as_ref()) {
                dest_url
            } else {
                "".into()
            },
            title,
            id,
        }),
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Image {
            link_type,
            dest_url: if safe_url(dest_url.as_ref()) {
                dest_url
            } else {
                "".into()
            },
            title,
            id,
        }),
        event => event,
    });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

fn safe_url(url: &str) -> bool {
    let trimmed = url.trim();
    if trimmed.starts_with('#')
        || trimmed.starts_with('/')
        || trimmed.starts_with("./")
        || trimmed.starts_with("../")
    {
        return true;
    }
    match trimmed.split_once(':') {
        None => true,
        Some((scheme, _)) => matches!(
            scheme.to_ascii_lowercase().as_str(),
            "http" | "https" | "mailto"
        ),
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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

fn source_link(
    span: &SourceSpan,
    project: &DocProject,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String {
    let relative = Path::new(&span.path);
    let target = if let Some(base) = source_url {
        format!("{}/{}", base.trim_end_matches('/'), encode_path(relative))
    } else {
        let source_root = Path::new(&project.root);
        let file = if relative.is_absolute() {
            relative.to_path_buf()
        } else {
            source_root.join(relative)
        };
        encode_path(&relative_path(output_parent, &file))
    };
    format!("{target}#L{}", span.start_line)
}

fn relative_path(from: &Path, to: &Path) -> std::path::PathBuf {
    let from_components: Vec<_> = from.components().collect();
    let to_components: Vec<_> = to.components().collect();
    let common = from_components
        .iter()
        .zip(&to_components)
        .take_while(|(left, right)| left == right)
        .count();
    let mut result = std::path::PathBuf::new();
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
