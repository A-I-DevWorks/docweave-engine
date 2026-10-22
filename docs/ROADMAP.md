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

## Milestone 5 — Validate extraction in supported languages

- [x] Define the declarations and documentation forms each adapter is expected to recognize.
- [x] Add focused, representative fixtures for Rust, C, C++, Python, and Lua.
- [x] Cover missing documentation, invalid syntax, nested code, and complex signatures.
- [x] Document each language's known limitations.

## Milestone 6 — Improve the user experience

- [ ] Improve CLI errors for invalid configuration and paths.
- [ ] Add CLI options for output directories and file filters.
- [ ] Support configuration of documentation titles and metadata.
- [ ] Document installation, configuration, and end-to-end usage.

## Milestone 7 — Prepare outputs for publishing

- [ ] Add a generated landing page with navigation by file or module.
- [ ] Improve HTML theme and navigation.
- [ ] Add useful metadata to JSON and document its format.
- [ ] Add consistency and security checks for source-derived content.

## Milestone 8 — Prepare for community language adapters

- [ ] Define a stable public interface for language adapters.
- [ ] Separate shared adapter conventions from language-specific behavior.
- [ ] Provide a minimal, standalone adapter example.
- [ ] Document how to implement, register, and validate an adapter.

## Milestone 9 — Prepare a usable release

- [ ] Choose and add a project license.
- [ ] Define how to install and distribute the CLI.
- [ ] Document configuration and JSON format versioning.
- [ ] Define release criteria for version 1.0.
