//! Core library for DocWeave Engine.
//!
//! The first milestone validates the Tree-sitter parsing pipeline for Rust.
//! Documentation extraction and language-neutral rendering will be layered on
//! top of this API in subsequent milestones.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tree_sitter::Tree;
use walkdir::WalkDir;

pub mod adapters;
pub mod config;
pub mod html;
pub mod json;
pub mod markdown;
pub mod model;
mod rust_extract;

pub use model::{
    Diagnostic, DiagnosticSeverity, DocFile, DocProject, DocSymbol, SourceSpan, SymbolKind,
};

/// A source file discovered and parsed by DocWeave.
#[derive(Debug)]
pub struct ParsedFile {
    pub path: PathBuf,
    pub source: String,
    pub tree: Tree,
    /// Stable identifier of the adapter used to parse this file.
    pub adapter_id: String,
}

/// Recursively discover Rust files and parse them with Tree-sitter.
///
/// Directories commonly containing generated files or dependencies are skipped.
/// Individual unreadable or malformed files return an error with path context.
pub fn scan_rust(root: &Path) -> Result<Vec<ParsedFile>> {
    scan_sources(root, Some("rust"))
}

/// Recursively discover supported C, C++, Python, Lua, and Rust files and parse
/// each file with its registered Tree-sitter adapter.
pub fn scan_project(root: &Path) -> Result<Vec<ParsedFile>> {
    scan_sources(root, None)
}

fn scan_sources(root: &Path, only_adapter: Option<&str>) -> Result<Vec<ParsedFile>> {
    if !root.exists() {
        anyhow::bail!("source directory does not exist: {}", root.display());
    }
    if !root.is_dir() {
        anyhow::bail!("source path is not a directory: {}", root.display());
    }
    let mut parsed = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            if !entry.file_type().is_dir() {
                return true;
            }

            !matches!(
                entry.file_name().to_str(),
                Some(".git" | "target" | "node_modules" | "vendor")
            )
        })
    {
        let entry = entry.with_context(|| format!("failed while walking {}", root.display()))?;

        if !entry.file_type().is_file() {
            continue;
        }

        let Some(adapter) = adapters::adapter_for_path(entry.path()) else {
            continue;
        };
        if only_adapter.is_some_and(|id| id != adapter.id()) {
            continue;
        }
        let Some((mut parser, adapter)) = adapters::parser_for(entry.path())? else {
            continue;
        };

        let source = std::fs::read_to_string(entry.path())
            .with_context(|| format!("failed to read {}", entry.path().display()))?;

        let tree = parser.parse(&source, None).with_context(|| {
            format!(
                "Tree-sitter {} parser failed to parse {}",
                adapter.id(),
                entry.path().display()
            )
        })?;

        parsed.push(ParsedFile {
            path: entry.path().to_path_buf(),
            source,
            tree,
            adapter_id: adapter.id().to_owned(),
        });
    }

    parsed.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(parsed)
}

/// Scan a Rust project and extract its language-neutral documentation model.
pub fn document_rust(root: &Path) -> Result<DocProject> {
    let root = root
        .canonicalize()
        .with_context(|| format!("failed to resolve source directory {}", root.display()))?;
    let files = scan_rust(&root)?;
    let files = files
        .iter()
        .map(|file| {
            let path = file.path.strip_prefix(&root).unwrap_or(&file.path);
            rust_extract::extract_rust(path, &file.source, &file.tree)
        })
        .collect();
    Ok(DocProject {
        root: root.display().to_string(),
        files,
    })
}

/// Scan all supported source languages and extract the shared documentation model.
pub fn document_project(root: &Path) -> Result<DocProject> {
    let root = root
        .canonicalize()
        .with_context(|| format!("failed to resolve source directory {}", root.display()))?;
    let files = scan_project(&root)?;
    let files = files
        .iter()
        .filter_map(|file| {
            let adapter = adapters::adapter_by_id(&file.adapter_id)?;
            let path = file.path.strip_prefix(&root).unwrap_or(&file.path);
            Some(adapter.extract(path, &file.source, &file.tree))
        })
        .collect();
    Ok(DocProject {
        root: root.display().to_string(),
        files,
    })
}

/// Generate documentation in the format selected by the output extension.
pub fn generate_documentation(config: &config::Config) -> Result<PathBuf> {
    config.validate()?;
    let mut project = document_project(&config.source)?;
    project.files.retain(|file| {
        let extension = Path::new(&file.path)
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let included = config.include_extensions.is_empty()
            || config
                .include_extensions
                .iter()
                .any(|candidate| candidate == &extension);
        included
            && !config
                .exclude_extensions
                .iter()
                .any(|candidate| candidate == &extension)
    });
    let parent = config.output.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    let output_parent = parent
        .canonicalize()
        .with_context(|| format!("failed to resolve output directory {}", parent.display()))?;
    let content = match config
        .output
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("md") => markdown::render(
            &project,
            &config.title,
            config.description.as_deref(),
            &output_parent,
            config.source_url.as_deref(),
        ),
        Some("json") => json::render(&project)?,
        Some("html") => html::render(
            &project,
            &config.title,
            config.description.as_deref(),
            &output_parent,
            config.source_url.as_deref(),
        ),
        _ => unreachable!("config validation rejects unsupported output extensions"),
    };
    std::fs::write(&config.output, content).with_context(|| {
        format!(
            "failed to write generated documentation {}",
            config.output.display()
        )
    })?;
    Ok(config.output.clone())
}

/// Generate Markdown documentation using validated configuration.
pub fn generate_markdown(config: &config::Config) -> Result<PathBuf> {
    if !config
        .output
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
    {
        anyhow::bail!("Markdown output requires a .md extension");
    }
    generate_documentation(config)
}

/// Count syntax error and missing nodes in a parsed syntax tree.
pub fn syntax_issue_count(tree: &Tree) -> usize {
    fn visit(node: tree_sitter::Node<'_>, count: &mut usize) {
        if node.is_error() || node.is_missing() {
            *count += 1;
        }

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            visit(child, count);
        }
    }

    let mut count = 0;
    visit(tree.root_node(), &mut count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tree_sitter::Parser;

    #[test]
    fn scans_and_parses_rust_files() {
        let temp = tempfile::tempdir().expect("temporary directory");

        fs::write(
            temp.path().join("example.rs"),
            "/// Adds two values.\npub fn add(a: i32, b: i32) -> i32 { a + b }",
        )
        .expect("write fixture");

        fs::write(temp.path().join("ignored.txt"), "not Rust").expect("write non-Rust fixture");

        let files = scan_rust(temp.path()).expect("scan source");

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path.file_name().unwrap(), "example.rs");
        assert_eq!(syntax_issue_count(&files[0].tree), 0);
    }

    #[test]
    fn detects_invalid_rust_syntax() {
        let mut parser = Parser::new();

        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();

        let tree = parser.parse("fn broken( {", None).unwrap();

        assert!(syntax_issue_count(&tree) > 0);
    }

    #[test]
    fn builds_a_project_documentation_model() {
        let temp = tempfile::tempdir().expect("temporary directory");
        std::fs::write(
            temp.path().join("lib.rs"),
            "//! Project docs.\n/// Adds values.\npub fn add(a: i32, b: i32) -> i32 { a + b }",
        )
        .expect("write fixture");

        let project = document_rust(temp.path()).expect("document project");

        assert_eq!(project.files.len(), 1);
        assert_eq!(
            project.files[0].documentation.as_deref(),
            Some("Project docs.")
        );
        assert_eq!(project.files[0].symbols[0].name, "add");
        fn assert_serializable<T: serde::Serialize>() {}
        assert_serializable::<DocProject>();
    }
}
