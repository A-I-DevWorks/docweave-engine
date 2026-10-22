use std::path::Path;

use docweave_engine::{
    adapters::{LanguageAdapter, diagnostics},
    document_project_with_adapters, generate_documentation_with_adapters,
    model::DocFile,
};
use tree_sitter::{Language, Tree};

struct DemoAdapter;

impl LanguageAdapter for DemoAdapter {
    fn id(&self) -> &'static str {
        "demo"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["demo"]
    }
    fn language(&self) -> Language {
        tree_sitter_rust::LANGUAGE.into()
    }
    fn extract(&self, path: &Path, _source: &str, tree: &Tree) -> DocFile {
        let path = path.display().to_string();
        DocFile {
            path: path.clone(),
            documentation: None,
            symbols: Vec::new(),
            diagnostics: diagnostics(tree.root_node(), &path, self.id()),
        }
    }
}

#[test]
fn caller_supplied_adapter_can_scan_and_extract_a_new_extension() {
    let temp = tempfile::tempdir().expect("temporary directory");
    std::fs::write(temp.path().join("sample.demo"), "fn demo() {}\n").expect("source fixture");
    std::fs::write(temp.path().join("ignored.rs"), "fn built_in() {}\n")
        .expect("unregistered source fixture");
    let registry: [&dyn LanguageAdapter; 1] = [&DemoAdapter];

    let project = document_project_with_adapters(temp.path(), &registry).expect("extract project");

    assert_eq!(project.files.len(), 1);
    assert_eq!(project.files[0].path, "sample.demo");
    assert!(project.files[0].diagnostics.is_empty());

    let config_path = temp.path().join("docweave.toml");
    std::fs::write(
        &config_path,
        "source = \".\"\noutput = \"api.json\"\ntitle = \"Custom API\"\n",
    )
    .expect("config fixture");
    let config = docweave_engine::config::Config::load(Some(&config_path)).expect("config");
    let output = generate_documentation_with_adapters(&config, &registry).expect("generate docs");
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output).expect("read generated JSON"))
            .expect("parse generated JSON");
    assert_eq!(json["metadata"]["title"], "Custom API");
    assert_eq!(json["project"]["files"][0]["path"], "sample.demo");
}

#[test]
fn adapter_registry_rejects_duplicate_extensions() {
    struct OtherAdapter;
    impl LanguageAdapter for OtherAdapter {
        fn id(&self) -> &'static str {
            "other"
        }
        fn extensions(&self) -> &'static [&'static str] {
            &["demo"]
        }
        fn language(&self) -> Language {
            tree_sitter_rust::LANGUAGE.into()
        }
        fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile {
            DemoAdapter.extract(path, source, tree)
        }
    }
    let registry: [&dyn LanguageAdapter; 2] = [&DemoAdapter, &OtherAdapter];
    let error = docweave_engine::adapters::validate_registry(&registry)
        .expect_err("duplicate extension must be rejected");
    assert!(
        error
            .to_string()
            .contains("registered by multiple adapters")
    );
}
