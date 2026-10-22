use std::process::Command;

#[test]
fn cli_generates_markdown_with_anchors_docs_and_source_links() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let source_dir = temp.path().join("src");
    std::fs::create_dir(&source_dir).expect("source directory");
    std::fs::write(
        source_dir.join("lib.rs"),
        "//! Library overview.\n/// Adds two values.\npub fn add(left: i32, right: i32) -> i32 { left + right }",
    )
    .expect("source fixture");
    let config_path = temp.path().join("docweave.toml");
    std::fs::write(
        &config_path,
        "title = \"Example API\"\nsource = \"src\"\noutput = \"generated/api.md\"\n",
    )
    .expect("config fixture");

    let result = Command::new(env!("CARGO_BIN_EXE_docweave"))
        .args(["generate", "--config"])
        .arg(&config_path)
        .output()
        .expect("run docweave");

    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let generated = std::fs::read_to_string(temp.path().join("generated/api.md"))
        .expect("read generated Markdown");
    assert!(generated.contains("# Example API"));
    assert!(generated.contains("Library overview."));
    assert!(generated.contains("Adds two values."));
    assert!(generated.contains("<a id=\"symbol-lib-rs-function-add\"></a>"));
    assert!(generated.contains("[View source](../src/lib.rs#L3)"));
}

#[test]
fn cli_rejects_invalid_configuration_without_creating_output() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let config_path = temp.path().join("bad.toml");
    std::fs::write(&config_path, "title = \"\"\noutput = \"bad.pdf\"\n").expect("config fixture");

    let result = Command::new(env!("CARGO_BIN_EXE_docweave"))
        .args(["generate", "--config"])
        .arg(&config_path)
        .output()
        .expect("run docweave");

    assert!(!result.status.success());
    assert!(!temp.path().join("bad.pdf").exists());
}

#[test]
fn cli_applies_output_directory_metadata_and_file_extension_filters() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let source_dir = temp.path().join("src");
    std::fs::create_dir(&source_dir).expect("source directory");
    std::fs::write(
        source_dir.join("api.rs"),
        "/// Rust docs.\npub fn run() {}\n",
    )
    .expect("Rust source");
    std::fs::write(
        source_dir.join("api.py"),
        "\"\"\"Python docs.\"\"\"\ndef run():\n    pass\n",
    )
    .expect("Python source");
    let config_path = temp.path().join("docweave.toml");
    std::fs::write(
        &config_path,
        "title = \"Filtered API\"\ndescription = \"Generated overview.\"\nsource = \"src\"\noutput = \"docs/api.md\"\n",
    )
    .expect("config fixture");
    let output_dir = temp.path().join("site");

    let result = Command::new(env!("CARGO_BIN_EXE_docweave"))
        .args(["generate", "--config"])
        .arg(&config_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .args(["--include-extension", "rs"])
        .output()
        .expect("run docweave");

    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let generated =
        std::fs::read_to_string(output_dir.join("api.md")).expect("read generated Markdown");
    assert!(generated.contains("# Filtered API"));
    assert!(generated.contains("Generated overview."));
    assert!(generated.contains("Rust docs."));
    assert!(!generated.contains("Python docs."));
}
