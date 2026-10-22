# Contributing to DocWeave Engine

DocWeave Engine is in early development. Contributions should favor small, testable changes and clear separation of responsibilities.

## Guidelines

- Discuss large changes in an issue before starting work.
- Keep language-specific parsing in adapters and output formatting in renderers.
- Add tests for new behavior and bug fixes.
- Preserve source paths and line information when available.
- Never execute analyzed source code as part of documentation generation.
- Do not commit credentials, private source files, build outputs, or generated documentation unless explicitly required by a fixture.
- Update documentation when behavior or configuration changes.
- Whenever supported Rust documentation-comment syntax or its extraction behavior changes, update [docs/RUST_DOC_COMMENTS.md](docs/RUST_DOC_COMMENTS.md) and the related fixtures and tests.

## Pull requests

Include the motivation, implementation summary, validation performed, and known limitations. Do not claim a language or format is supported without representative tests.

## Project status

The engine is implemented in Rust and uses Cargo. Run `cargo test` before
submitting changes, and keep adapter and renderer behavior covered by
representative fixtures.
