# DocWeave Engine

**A language-agnostic engine for generating structured, navigable documentation from source code.**

DocWeave Engine aims to analyze source files, extract documentation and code symbols, and publish consistent documentation in multiple formats.

> **Project status:** Early development. Architecture and APIs may change. Features described as goals are not necessarily implemented yet.

## Goals

- Support multiple programming languages through independent parser adapters.
- Normalize extracted symbols and documentation into a shared intermediate model.
- Generate documentation through replaceable output backends.
- Make source discovery, exclusions, and output settings configurable.
- Report parsing issues and missing documentation instead of silently guessing.
- Run locally and integrate into automated CI workflows.

## Proposed architecture

```text
Source files
    |
    v
Configuration and discovery
    |
    v
Language adapters / parsers
    |
    v
Intermediate documentation model
    |
    +--> HTML renderer
    +--> Markdown renderer
    +--> JSON exporter
```

The core should remain independent of individual languages and output formats. Language-specific behavior belongs in adapters; presentation belongs in renderers.

## Design principles

1. **Correctness over guesswork.** Preserve source locations and distinguish explicit documentation from inferred metadata.
2. **Modularity.** Parsers and renderers should be independently testable and replaceable.
3. **Safe defaults.** Never execute analyzed source code; exclude generated directories where appropriate.
4. **Reproducibility.** Identical source and configuration should produce stable output wherever practical.
5. **Useful diagnostics.** Explain unsupported syntax and malformed documentation with file and location context.

## Planned outputs

- **HTML** — navigable documentation.
- **Markdown** — repository-friendly documentation.
- **JSON** — machine-readable export.
- **PDF** — a possible later output, after the core pipeline stabilizes.

## Repository layout

- `docs/` — architecture and design notes.
- `examples/` — small input projects and fixtures.
- `tests/` — unit and integration tests.
- `.github/workflows/` — automated checks.
- `src/` — implementation, once the language and build system are selected.

## Development

The implementation language and package/build system have not yet been finalized. See [the architecture proposal](docs/ARCHITECTURE.md) and [roadmap](docs/ROADMAP.md).

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing changes.

## License

No license has been selected yet. Until a license is added, assume the contents are all rights reserved; do not assume permission to reuse, distribute, or modify them.
