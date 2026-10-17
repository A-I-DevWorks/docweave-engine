# Roadmap

## Milestone 1 — Rust parsing foundation

- [x] Select Rust and Tree-sitter.
- [x] Add a Cargo package and `docweave scan` command.
- [x] Recursively discover Rust files and skip common generated/dependency directories.
- [x] Report Tree-sitter syntax errors and missing nodes.
- [x] Add initial unit tests.
- [x] Run formatting, tests, and lint checks in CI.

## Milestone 2 — Documentation model

- [x] Define serializable project, file, symbol, source-span, and diagnostic types.
- [x] Extract Rust modules, structs, enums, traits, functions, methods, and constants.
- [x] Preserve Rust doc comments (`///` and `//! `) without inventing missing documentation.
- [x] Add fixtures for nested modules, generics, attributes, and malformed files.

## Milestone 3 — First output

- [x] Add validated configuration (TOML).
- [x] Implement Markdown rendering.
- [x] Add stable anchors and links to source locations.
- [x] Add end-to-end tests.

## Milestone 4 — Extensibility

- [x] Add JSON export.
- [x] Add HTML output with safe escaping.
- [x] Add Tree-sitter adapters for C, C++, Python, and Lua.
- [x] Document how to create an adapter for another language.
- [x] Generate documentation for the Docweave project itself using Docweave.
- [x] Evaluate PDF after the model and renderers stabilize; defer implementation to a later milestone (see [PDF_OUTPUT_EVALUATION.md](PDF_OUTPUT_EVALUATION.md)).
