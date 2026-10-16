# Roadmap

## Milestone 1 — Rust parsing foundation

- [x] Select Rust and Tree-sitter.
- [x] Add a Cargo package and `docweave scan` command.
- [x] Recursively discover Rust files and skip common generated/dependency directories.
- [x] Report Tree-sitter syntax errors and missing nodes.
- [x] Add initial unit tests.
- [ ] Run formatting, tests, and lint checks in CI.

## Milestone 2 — Documentation model

- [ ] Define serializable project, file, symbol, source-span, and diagnostic types.
- [ ] Extract Rust modules, structs, enums, traits, functions, methods, and constants.
- [ ] Preserve Rust doc comments (`///` and `//! `) without inventing missing documentation.
- [ ] Add fixtures for nested modules, generics, attributes, and malformed files.

## Milestone 3 — First output

- [ ] Add validated configuration (TOML).
- [ ] Implement Markdown rendering.
- [ ] Add stable anchors and links to source locations.
- [ ] Add end-to-end tests.

## Milestone 4 — Extensibility

- [ ] Add JSON export.
- [ ] Add HTML output with safe escaping.
- [ ] Add Tree-sitter adapters for additional languages based on real use cases.
- [ ] Evaluate PDF only after the model and renderers stabilize.
