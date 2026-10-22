# DocWeave Engine

**A language-agnostic engine for generating structured, navigable documentation from source code.**

DocWeave analyzes source files, extracts code structure and documentation comments, and aims to publish consistent documentation in multiple formats.

> **Project status:** Early development. Rust, C, C++, Python, and Lua source files can be scanned into a shared documentation model and rendered as Markdown, JSON, or HTML.

## Current implementation

- Rust 2024 edition.
- Tree-sitter-based syntax parsing for the supported languages.
- Recursive discovery of supported source extensions.
- Skips common generated/dependency directories: `.git`, `target`, `node_modules`, and `vendor`.
- Extracts Rust modules, structs, enums, traits, functions, methods, and constants into a serializable language-neutral model.
- Includes initial Tree-sitter adapters for C, C++, Python, and Lua.
- Preserves each language's explicit documentation comments or docstrings and records source-located syntax diagnostics.
- Unit tests and fixtures for source discovery, extraction, and malformed syntax.
- Validated TOML configuration and Markdown, JSON, and safe HTML documentation export.

## Quick start

Install the [Rust stable toolchain](https://www.rust-lang.org/tools/install), then build and install the CLI from the repository root:

```sh
cargo install --path .
docweave --help
```

To run directly from the checkout without installing:

```sh
cargo run -- scan ./path/to/project
```

The first build downloads dependencies from crates.io. Source files are parsed as data; DocWeave does not execute analyzed source code.

Copy [docweave.toml.example](docweave.toml.example) to `docweave.toml`, edit its title and paths, then run `cargo run -- generate --config docweave.toml`. The output extension selects Markdown (`.md`), JSON (`.json`), or HTML (`.html`). Without a config file, generation uses the current directory as source and writes `docs.md`; `--output` can override the output file. Markdown and HTML include documentation comments, signatures, and parse diagnostics; JSON wraps the language-neutral model in a versioned metadata envelope. HTML escapes source-derived text and treats raw HTML in doc comments as text. Set `source_url` to link to a hosted source repository instead of local files.

Create a config file such as `docweave.toml`:

```toml
title = "Example API"
description = "Reference documentation for the example project."
source = "./src"
output = "./site/api.md"
source_url = "https://github.com/owner/repository/blob/main"
include_extensions = ["rs", "py"]
exclude_extensions = ["pyi"]
```

Then generate the docs:

```sh
docweave generate --config docweave.toml
```

The title is used in generated Markdown and HTML; description is introductory Markdown in those formats. JSON output serializes the language-neutral documentation model. `source_url` adds hosted source links to Markdown and HTML. Include filters allow only listed extensions; exclude filters then remove matching extensions. Extension matching ignores case and an optional leading dot. CLI options `--output-dir`, `--include-extension`, and `--exclude-extension` override those settings for one run; repeated extension options accept multiple values. For example:

```sh
docweave generate --config docweave.toml --output-dir ./site --include-extension rs --include-extension py
```

`--output` selects a complete output path and format by extension; `--output-dir` preserves the configured filename and changes its directory. Use `docweave --help` or `docweave generate --help` for command options. Relative config paths resolve from the config file's directory; CLI output paths resolve from the current working directory. The output directory is created when needed. To check a checkout before publishing, run `cargo test`, then run `docweave scan ./path/to/project` and `docweave generate --config docweave.toml`.

The supported languages currently have focused initial extraction coverage; see [the language adapter guide](docs/ADDING_LANGUAGE_ADAPTERS.md) for extensions, comment conventions, supported declarations, and how to implement a custom adapter. JSON output includes a versioned metadata envelope; see [the JSON format guide](docs/JSON_FORMAT.md).

PDF generation is deferred; see the [PDF output evaluation](docs/PDF_OUTPUT_EVALUATION.md) for the current decision.

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
The generated reference for this repository is [docs/PROJECT_DOCUMENTATION.md](docs/PROJECT_DOCUMENTATION.md); regenerate it with `cargo run -- generate --config docweave.toml.example --output docs/PROJECT_DOCUMENTATION.md`.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing changes.

## License

The repository does not yet have a finalized license. Until one is selected and added, assume all rights are reserved.
