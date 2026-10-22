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
    let symbol_count: usize = project.files.iter().map(|file| file.symbols.len()).sum();
    let diagnostic_count: usize = project
        .files
        .iter()
        .map(|file| file.diagnostics.len())
        .sum();
    let mut output = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{}</title>\n<style>:root{{color-scheme:light dark;--bg:#fff;--surface:#f5f7fb;--text:#202633;--muted:#5e6878;--link:#245ec7;--line:#dce2ea;--warning:#9b342e}}@media(prefers-color-scheme:dark){{:root{{--bg:#151922;--surface:#202632;--text:#e8edf5;--muted:#a5afbf;--link:#8db5ff;--line:#3a4352;--warning:#ffaaa2}}}}*{{box-sizing:border-box}}body{{margin:0;background:var(--bg);color:var(--text);font:16px/1.65 system-ui,sans-serif}}a{{color:var(--link)}}.skip-link{{position:absolute;left:-9999px}}.skip-link:focus{{left:1rem;top:1rem;background:var(--bg);padding:.5rem;z-index:5}}.layout{{display:grid;grid-template-columns:19rem minmax(0,1fr);min-height:100vh}}.sidebar{{position:sticky;top:0;height:100vh;overflow:auto;padding:1.5rem;background:var(--surface);border-right:1px solid var(--line)}}.brand{{font-size:1.2rem;font-weight:700;text-decoration:none;color:var(--text)}}nav h2{{font-size:.8rem;text-transform:uppercase;letter-spacing:.08em;color:var(--muted);margin:1.8rem 0 .4rem}}nav ul{{list-style:none;margin:0;padding:0}}nav ul ul{{padding-left:1rem}}nav li{{margin:.2rem 0}}nav a{{text-decoration:none;overflow-wrap:anywhere}}nav a:hover{{text-decoration:underline}}main{{max-width:76rem;width:100%;margin:0 auto;padding:2rem clamp(1rem,4vw,4rem)}}.page-header{{padding:1rem 0 1.5rem;border-bottom:1px solid var(--line)}}.page-header h1{{font-size:clamp(2rem,4vw,3rem);line-height:1.15;margin:.2rem 0}}.stats{{display:flex;gap:.75rem;flex-wrap:wrap;margin:1.5rem 0}}.stat{{background:var(--surface);border:1px solid var(--line);border-radius:.7rem;padding:.7rem 1rem;min-width:8rem}}.stat strong{{display:block;font-size:1.3rem}}.file-section{{padding:1rem 0;border-bottom:1px solid var(--line)}}article{{padding:1rem 0;border-top:1px solid var(--line)}}pre{{overflow:auto;background:var(--surface);padding:1rem;border:1px solid var(--line);border-radius:.5rem}}code{{font-family:ui-monospace,monospace;overflow-wrap:anywhere}}.source{{font-size:.9em;color:var(--muted)}}.diagnostics{{color:var(--warning);background:var(--surface);padding:.5rem 1rem;border-radius:.5rem}}@media(max-width:850px){{.layout{{grid-template-columns:1fr}}.sidebar{{position:static;height:auto;max-height:40vh;border-right:0;border-bottom:1px solid var(--line)}}main{{padding:1.25rem}}}}</style>\n</head>\n<body>\n<a class=\"skip-link\" href=\"#main\">Skip to documentation</a>\n<div class=\"layout\"><aside class=\"sidebar\"><a class=\"brand\" href=\"#top\">{}</a>\n<nav aria-label=\"Documentation navigation\"><h2>Files</h2><ul>\n",
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
    output.push_str("</ul><h2>Modules</h2><ul>\n");
    for (file_index, file) in project.files.iter().enumerate() {
        for (symbol_index, symbol) in file.symbols.iter().enumerate() {
            if symbol.kind == crate::model::SymbolKind::Module {
                output.push_str(&format!(
                    "<li><a href=\"#symbol-{file_index}-{symbol_index}\"><code>{}</code> <span class=\"muted\">{}</span></a></li>\n",
                    escape_html(&symbol.name),
                    escape_html(&file.path)
                ));
            }
        }
    }
    output.push_str(
        "</ul></nav></aside>\n<main id=\"main\"><header class=\"page-header\"><h1 id=\"top\">",
    );
    output.push_str(&escape_html(title));
    output.push_str("</h1></header>\n<section class=\"stats\" aria-label=\"Project summary\">");
    output.push_str(&format!(
        "<div class=\"stat\"><strong>{}</strong> files</div><div class=\"stat\"><strong>{}</strong> declarations</div><div class=\"stat\"><strong>{}</strong> diagnostics</div></section>\n",
        project.files.len(), symbol_count, diagnostic_count
    ));
    if let Some(description) = description {
        output.push_str(&render_markdown(description));
    }

    for (file_index, file) in project.files.iter().enumerate() {
        output.push_str(&format!(
            "<section class=\"file-section\" id=\"file-{file_index}\"><h2><code>{}</code></h2>\n",
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

pub(crate) fn safe_url(url: &str) -> bool {
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
    let relative = safe_project_path(project, &span.path);
    let target = if let Some(base) = source_url {
        format!("{}/{}", base.trim_end_matches('/'), encode_path(&relative))
    } else {
        let source_root = Path::new(&project.root);
        let file = source_root.join(&relative);
        encode_path(&relative_path(output_parent, &file))
    };
    format!("{target}#L{}", span.start_line)
}

fn safe_project_path(project: &DocProject, path: &str) -> std::path::PathBuf {
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
        std::path::PathBuf::from("source-unavailable")
    }
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
