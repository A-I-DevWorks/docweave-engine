# DocWeave Engine

**A language-agnostic engine for generating structured, navigable documentation from source code.**

DocWeave analyzes source files, extracts code structure and documentation comments, and aims to publish consistent documentation in multiple formats.

> **Project status:** Early development. Rust scanning and extraction into a shared documentation model are implemented; configuration and output rendering are planned.

## Current implementation

- Rust 2024 edition.
- Tree-sitter-based Rust syntax parsing.
- Recursive discovery of `.rs` files.
- Skips common generated/dependency directories: `.git`, `target`, `node_modules`, and `vendor`.
- Extracts Rust modules, structs, enums, traits, functions, methods, and constants into a serializable language-neutral model.
- Preserves explicit `///` and `//!` documentation and records source-located syntax diagnostics.
- Unit tests and fixtures for source discovery, extraction, and malformed syntax.
- Validated TOML configuration and Markdown documentation generation with symbol anchors and source links.

## Quick start

Install the Rust stable toolchain, then run:

```sh
cargo test
cargo run -- scan .
cargo run -- generate --config docweave.toml
```

Scan a specific project directory:

```sh
cargo run -- scan ./path/to/project
```

The first build downloads dependencies from crates.io. Source files are parsed as data; DocWeave does not execute analyzed source code.

Copy [docweave.toml.example](docweave.toml.example) to `docweave.toml`, edit its title and paths, then run `cargo run -- generate --config docweave.toml`. Without a config file, generation uses the current directory as source and writes `docs.md`; `--output` can override the output file. The generated Markdown includes a contents list, stable file and symbol anchors, source-line links, documentation comments, signatures, and parse diagnostics. Set `source_url` to link to a hosted source repository instead of local files.

## Planned architecture

```text
Configuration and source discovery
              |
              v
     Language adapters
       (Tree-sitter)
              |
              v
 Language-neutral document model
              |
       +------+------+
       v      v      v
     HTML  Markdown  JSON
```

The core should remain independent of individual languages and output formats. Language-specific behavior belongs in adapters; presentation belongs in renderers.

## Design principles

1. **Correctness over guesswork.** Preserve source locations and distinguish explicit documentation from inferred metadata.
2. **Modularity.** Parsers and renderers should be independently testable and replaceable.
3. **Safe defaults.** Never execute analyzed source code; exclude generated directories where appropriate.
4. **Reproducibility.** Identical source and configuration should produce stable output wherever practical.
5. **Useful diagnostics.** Explain unsupported syntax and malformed documentation with file and location context.

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md) for milestones and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for design boundaries.
For Rust documentation-comment syntax and examples, see [docs/RUST_DOC_COMMENTS.md](docs/RUST_DOC_COMMENTS.md).

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing changes.

## License

The repository does not yet have a finalized license. Until one is selected and added, assume all rights are reserved.
