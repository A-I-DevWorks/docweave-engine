# Adding a language adapter

DocWeave separates parsing from rendering. A language adapter turns one
language's Tree-sitter syntax tree and documentation conventions into the
shared [`DocFile` / `DocSymbol` model](../src/model.rs). The renderers then work
without knowing which language produced that model.

## Built-in adapters

| Language | Extensions | Documentation convention | Extracted declarations |
| --- | --- | --- | --- |
| Rust | `.rs` | `///` and `//!` | modules, structs, enums, traits, functions, methods, constants |
| C | `.c`, `.h` | `///`, `//!`, `/** ... */`, `/*! ... */` | structs, enums, functions and declarations |
| C++ | `.cc`, `.cpp`, `.cxx`, `.hh`, `.hpp`, `.hxx` | `///`, `//!`, `/** ... */`, `/*! ... */` | namespaces, classes/structs, enums, functions and methods |
| Python | `.py`, `.pyi` | module, class and function docstrings | classes, functions, methods, uppercase module assignments |
| Lua | `.lua` | `---` comments | named functions and colon-style methods |

The `.h` extension is assigned to C to avoid ambiguous dispatch. Use `.hpp`,
`.hh`, or another listed C++ extension for C++ headers. These adapters extract
only the constructs listed above; they do not resolve types, infer undocumented
meaning, or claim compiler-level semantic analysis.

## Adapter contract

The registry is in [`src/adapters/mod.rs`](../src/adapters/mod.rs). Implement
`LanguageAdapter` with four pieces of information:

```rust,ignore
pub trait LanguageAdapter: Sync {
    fn id(&self) -> &'static str;
    fn extensions(&self) -> &'static [&'static str];
    fn language(&self) -> tree_sitter::Language;
    fn extract(
        &self,
        path: &std::path::Path,
        source: &str,
        tree: &tree_sitter::Tree,
    ) -> DocFile;
}
```

- `id` is a stable lowercase identifier used in diagnostic codes.
- `extensions` lists unambiguous filename extensions without dots.
- `language` returns the grammar supplied by the selected Tree-sitter crate.
- `extract` maps declarations, docs, signatures, spans, and parse diagnostics
  into the shared model.

## Implementation steps

1. **Choose the grammar and extension set.** Add the Rust grammar crate to
   `Cargo.toml`. Check that it supports the repository's Tree-sitter runtime
   version. Keep extensions unique; `adapter_for_path` selects the first match.
2. **Create an adapter module.** Add `src/adapters/<language>.rs` with a
   zero-sized adapter struct and its `LanguageAdapter` implementation. Declare
   the module and add a static instance to `ADAPTERS` in `src/adapters/mod.rs`.
3. **Inspect the grammar's node types.** Use representative source files to
   identify declarations, name fields, body nodes, and comment nodes. Tree-
   sitter gives syntax, not resolved types or runtime values.
4. **Map only explicit information.** Use `SymbolKind` where it fits. For a
   language construct without a matching kind, choose the closest documented
   mapping and explain it; do not synthesize a summary or signature facts that
   the syntax tree does not contain.
5. **Preserve source locations.** Use `source_span` for one-based lines and
   columns. End positions are exclusive. Store paths relative to the scanned
   project root, as the existing adapters do.
6. **Keep documentation conventions adapter-specific.** Normalize comment
   markers or docstrings to Markdown text, but leave Markdown content intact.
   Ignore ordinary comments unless that language's documented convention says
   they are documentation.
7. **Report syntax problems consistently.** Use the shared `diagnostics`
   helper or emit equivalent stable codes such as `<language>.syntax_error` and
   `<language>.missing_syntax`. A malformed file should produce diagnostics
   whenever Tree-sitter can recover its tree.
8. **Add a showcase source and adapter-focused checks.** Cover a module/file
   doc, representative declaration kinds, signatures, comments/docstrings,
   Unicode paths or identifiers where supported, and malformed syntax. Then
   generate Markdown, JSON, and HTML from a mixed-language source directory.
9. **Update this guide and the roadmap.** Record the language's extensions,
   comment convention, supported declaration kinds, and known limitations here
   whenever the adapter changes.

The shared helpers `parser_for`, `adapter_for_path`, `node_text`, `source_span`,
and `diagnostics` are available from `docweave_engine::adapters`. Existing
adapters under `src/adapters/` provide working examples. Rust's historical
extractor remains in `src/rust_extract.rs` and is wrapped by
`src/adapters/rust.rs`.

## Local workflow

Add a source file with the selected extension beneath a small fixture or
showcase directory, then run:

```sh
cargo run -- generate --config showcase/docweave.toml.showcase --output /tmp/showcase.md
cargo run -- generate --config showcase/docweave.toml.showcase --output /tmp/showcase.json
cargo run -- generate --config showcase/docweave.toml.showcase --output /tmp/showcase.html
```

The output extension chooses the renderer. The scanner discovers supported
extensions recursively and does not execute analyzed source files.
