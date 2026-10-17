use std::path::Path;

use tree_sitter::{Language, Tree};

use crate::model::DocFile;

use super::LanguageAdapter;

pub struct RustAdapter;

impl LanguageAdapter for RustAdapter {
    fn id(&self) -> &'static str {
        "rust"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["rs"]
    }

    fn language(&self) -> Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile {
        crate::rust_extract::extract_rust(path, source, tree)
    }
}
