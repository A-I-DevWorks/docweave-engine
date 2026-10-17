//! Core library for DocWeave Engine.
//!
//! The first milestone validates the Tree-sitter parsing pipeline for Rust.
//! Documentation extraction and language-neutral rendering will be layered on
//! top of this API in subsequent milestones.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tree_sitter::{Parser, Tree};
use walkdir::WalkDir;

pub mod config;
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
}

/// Recursively discover Rust files and parse them with Tree-sitter.
///
/// Directories commonly containing generated files or dependencies are skipped.
/// Individual unreadable or malformed files return an error with path context.
pub fn scan_rust(root: &Path) -> Result<Vec<ParsedFile>> {
    let mut parser = Parser::new();

    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .context("failed to initialize the Tree-sitter Rust grammar")?;

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

        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|ext| ext.to_str()) != Some("rs")
        {
            continue;
        }

        let source = std::fs::read_to_string(entry.path())
            .with_context(|| format!("failed to read {}", entry.path().display()))?;

        let tree = parser
            .parse(&source, None)
            .with_context(|| format!("Tree-sitter failed to parse {}", entry.path().display()))?;

        parsed.push(ParsedFile {
            path: entry.path().to_path_buf(),
            source,
            tree,
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

/// Generate Markdown documentation using validated configuration.
pub fn generate_markdown(config: &config::Config) -> Result<PathBuf> {
    config.validate()?;
    let project = document_rust(&config.source)?;
    let parent = config.output.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    let output_parent = parent
        .canonicalize()
        .with_context(|| format!("failed to resolve output directory {}", parent.display()))?;
    let markdown = markdown::render(
        &project,
        &config.title,
        &output_parent,
        config.source_url.as_deref(),
    );
    std::fs::write(&config.output, markdown).with_context(|| {
        format!(
            "failed to write Markdown output {}",
            config.output.display()
        )
    })?;
    Ok(config.output.clone())
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
