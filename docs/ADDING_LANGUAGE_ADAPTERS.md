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

## Current extraction scope and limitations

- **Rust:** extracts named modules, structs, enums, traits, functions, methods,
  constants, and statics. Documentation comes from `///` and `//!`; block doc
  comments are not currently extracted. The adapter preserves syntax text and
  does not resolve macros, imports, types, or conditional compilation.
- **C:** extracts named structs, enums, function definitions, and declarations
  with function declarators. It recognizes `///`, `//!`, `/** ... */`, and
  `/*! ... */` comments when they directly precede a declaration. It does not
  model fields, typedef aliases, macros, or C semantic distinctions.
- **C++:** extracts namespaces, classes/structs, enums, functions, and methods.
  It recognizes the same comment forms as C. Classes map to the shared `struct`
  kind; namespace nesting and overload resolution are not represented in the
  shared model.
- **Python:** extracts classes, functions, methods, and uppercase module-level
  assignments as constants. Only the first string expression in a module,
  class, or function body is treated as a docstring. Decorator semantics,
  assignments other than module constants, and type resolution are not modeled.
- **Lua:** extracts named function declarations, including dotted functions
  and colon-style methods. Only leading `---` comments are documentation;
  anonymous functions and table-based function assignments are not extracted.
  Nested named declarations are returned as symbols without a parent scope in
  the shared model.

All adapters report Tree-sitter syntax errors with source spans when the grammar
recovers an error or missing node. Signatures are source text, not normalized
or semantically validated. Nested declarations may be emitted as independent
symbols because the shared model currently has no parent/qualified-scope field.

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

The public interface is `docweave_engine::adapters::LanguageAdapter`. In the
current 0.x series, changes to this trait or the shared model may require
adapter updates; after 1.0, compatibility follows semantic versioning. Keep
language-specific syntax and comment interpretation inside `extract`. Keep
shared path, diagnostic, and rendering rules in DocWeave and its shared
helpers. Adapter IDs must be unique lowercase identifiers; file extensions
must be unique case-insensitively and use ASCII letters or digits without a
leading dot.

## Use a custom adapter

External adapters do not modify DocWeave's built-in registry. Pass an explicit
registry to `document_project_with_adapters` or
`generate_documentation_with_adapters`. Compose built-ins with a custom adapter
by copying the built-in registry and appending your adapter:

```rust,ignore
use docweave_engine::adapters::LanguageAdapter;
use docweave_engine::document_project_with_adapters;

static CUSTOM: MyAdapter = MyAdapter;
let mut adapters = docweave_engine::adapters::ADAPTERS.to_vec();
adapters.push(&CUSTOM);
let project = document_project_with_adapters(source_root, &adapters)?;
```

The registry is checked before scanning. Duplicate IDs, duplicate extensions,
invalid identifiers, and invalid extensions return an error instead of
silently dispatching to the first adapter. The public helpers
`adapter_for_path_in`, `parser_for_in`, and `validate_registry` support custom
registries directly. `scan_project_with_adapters` returns parsed trees when an
adapter needs a separate extraction stage.

See the compiling, standalone [`custom_adapter` example](../examples/custom_adapter.rs)
and its [sample source](../examples/demo-sources/api.demo):

```sh
cargo run --example custom_adapter -- examples/demo-sources
```

The example uses the Rust grammar only to keep the example self-contained; a
real adapter should add and return its own Tree-sitter grammar.

## Implementation steps

1. **Choose the grammar and extension set.** Add the Tree-sitter grammar crate to
   `Cargo.toml`. Check that it supports the repository's Tree-sitter runtime
   version. Keep extensions unique; registry validation rejects ambiguous
   dispatch.
2. **Create an adapter.** Add a zero-sized adapter type and its
   `LanguageAdapter` implementation. For an external adapter, pass it in an
   explicit registry. For a built-in adapter, add the module and static
   instance to `ADAPTERS` in `src/adapters/mod.rs`.
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
and `diagnostics` are available from `docweave_engine::adapters`; registry-aware
variants support custom adapter lists. Existing adapters under `src/adapters/`
provide language-specific examples. Rust's historical extractor remains in
`src/rust_extract.rs` and is wrapped by `src/adapters/rust.rs`.

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
