# DocWeave Engine

**A language-agnostic engine for generating structured, navigable documentation from source code.**

DocWeave analyzes source files, extracts code structure and documentation comments, and aims to publish consistent documentation in multiple formats.

> **Project status:** Early development. The initial Rust scanning milestone is underway; full documentation extraction and rendering are not implemented yet.

## Current implementation

- Rust 2024 edition.
- Tree-sitter-based Rust syntax parsing.
- Recursive discovery of `.rs` files.
- Skips common generated/dependency directories: `.git`, `target`, `node_modules`, and `vendor`.
- Reports syntax-error and missing-node counts.
- Unit tests for source discovery and syntax diagnostics.

## Quick start

Install the Rust stable toolchain, then run:

```sh
cargo test
cargo run -- scan .
```

Scan a specific project directory:

```sh
cargo run -- scan ./path/to/project
```

The first build downloads dependencies from crates.io. Source files are parsed as data; DocWeave does not execute analyzed source code.

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

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing changes.

## License

The repository does not yet have a finalized license. Until one is selected and added, assume all rights are reserved.
