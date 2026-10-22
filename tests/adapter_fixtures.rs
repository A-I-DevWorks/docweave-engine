use std::path::Path;

use docweave_engine::adapters::parser_for;
use docweave_engine::model::SymbolKind;

fn extract(path: &str, source: &str) -> docweave_engine::model::DocFile {
    let path = Path::new(path);
    let (mut parser, adapter) = parser_for(path)
        .expect("adapter setup")
        .expect("fixture extension has an adapter");
    let tree = parser.parse(source, None).expect("parse fixture");
    adapter.extract(path, source, &tree)
}

fn symbol<'a>(
    file: &'a docweave_engine::model::DocFile,
    name: &str,
) -> &'a docweave_engine::model::DocSymbol {
    file.symbols
        .iter()
        .find(|symbol| symbol.name == name)
        .unwrap_or_else(|| {
            panic!(
                "missing symbol {name:?}; found {:?}",
                file.symbols
                    .iter()
                    .map(|item| &item.name)
                    .collect::<Vec<_>>()
            )
        })
}

#[test]
fn c_fixture_covers_file_docs_complex_signature_and_undocumented_declarations() {
    let source = include_str!("fixtures/adapter_c.c");
    let file = extract("adapter_c.c", source);
    assert_eq!(
        file.documentation.as_deref(),
        Some("File-level C API documentation.")
    );
    let add = symbol(&file, "add_into");
    assert_eq!(add.kind, SymbolKind::Function);
    assert_eq!(
        add.documentation.as_deref(),
        Some("Adds two integers with an explicit output pointer.")
    );
    assert!(
        add.signature
            .as_deref()
            .is_some_and(|s| s.contains("int *result"))
    );
    assert!(symbol(&file, "undocumented").documentation.is_none());
    assert!(symbol(&file, "Entry").documentation.is_none());
    assert!(file.diagnostics.is_empty());
}

#[test]
fn cpp_fixture_covers_nested_scopes_and_complex_signatures() {
    let source = include_str!("fixtures/adapter_cpp.cpp");
    let file = extract("adapter_cpp.cpp", source);
    assert_eq!(
        file.documentation.as_deref(),
        Some("C++ API documentation.")
    );
    assert_eq!(symbol(&file, "api").kind, SymbolKind::Module);
    assert_eq!(symbol(&file, "Store").kind, SymbolKind::Struct);
    let find = symbol(&file, "find");
    assert_eq!(find.kind, SymbolKind::Method);
    assert_eq!(
        find.documentation.as_deref(),
        Some("Looks up a value using a complex parameter type.")
    );
    assert!(
        find.signature
            .as_deref()
            .is_some_and(|s| s.contains("std::vector<std::pair<int, std::string>>"))
    );
    assert!(symbol(&file, "undocumented_cpp").documentation.is_none());
    assert!(file.diagnostics.is_empty());
}

#[test]
fn python_fixture_covers_docstrings_constants_methods_and_missing_docs() {
    let source = include_str!("fixtures/adapter_python.py");
    let file = extract("adapter_python.py", source);
    assert_eq!(
        file.documentation.as_deref(),
        Some("Python module documentation.")
    );
    assert_eq!(symbol(&file, "MAX_SIZE").kind, SymbolKind::Constant);
    assert!(
        !file
            .symbols
            .iter()
            .any(|item| item.name == "ordinary_value")
    );
    assert_eq!(
        symbol(&file, "Store").documentation.as_deref(),
        Some("A documented store.")
    );
    let find = symbol(&file, "find");
    assert_eq!(find.kind, SymbolKind::Method);
    assert_eq!(
        find.documentation.as_deref(),
        Some("Find values for the requested keys.")
    );
    assert!(
        find.signature
            .as_deref()
            .is_some_and(|s| s.contains("list[tuple[int, str]]"))
    );
    assert!(symbol(&file, "undocumented").documentation.is_none());
    assert_eq!(symbol(&file, "normalize").kind, SymbolKind::Function);
    assert!(file.diagnostics.is_empty());
}

#[test]
fn lua_fixture_covers_doc_comments_methods_and_nested_code() {
    let source = include_str!("fixtures/adapter_lua.lua");
    let file = extract("adapter_lua.lua", source);
    assert_eq!(
        file.documentation.as_deref(),
        Some("Lua module documentation.")
    );
    let add = symbol(&file, "api.add");
    assert_eq!(add.documentation.as_deref(), Some("Adds two values."));
    assert!(
        add.signature
            .as_deref()
            .is_some_and(|s| s.contains("left, right"))
    );
    assert_eq!(symbol(&file, "api:label").kind, SymbolKind::Method);
    assert!(symbol(&file, "undocumented").documentation.is_none());
    assert!(file.symbols.iter().any(|item| item.name == "nested"));
    assert!(file.diagnostics.is_empty());
}

#[test]
fn malformed_non_rust_fixtures_emit_source_located_diagnostics() {
    for (path, source) in [
        ("bad.c", "int broken( { return 1; }"),
        ("bad.cpp", "namespace api { int broken( { }"),
        ("bad.py", "def broken(:\n    pass\n"),
        ("bad.lua", "function broken(\n"),
    ] {
        let file = extract(path, source);
        assert!(
            !file.diagnostics.is_empty(),
            "{path} should report a parse diagnostic"
        );
        assert!(
            file.diagnostics
                .iter()
                .all(|diagnostic| diagnostic.span.is_some()),
            "{path}"
        );
    }
}
