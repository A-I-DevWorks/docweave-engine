# Architecture

## Status

Rust is the implementation language and Tree-sitter is the parsing technology. The repository contains adapters for Rust, C, C++, Python, and Lua; a language-neutral documentation model; validated TOML configuration; and Markdown, JSON, and HTML renderers. HTML escapes source-derived text and treats raw HTML in doc comments as text. See [ADDING_LANGUAGE_ADAPTERS.md](ADDING_LANGUAGE_ADAPTERS.md) for the adapter contract and implementation guide.

## Initial pipeline

1. **CLI** parses commands and arguments.
2. **Discovery** walks configured roots, skips common dependency/build directories, and selects supported files.
3. **Language adapters** parse source files with Tree-sitter grammars and extract symbols, comments, signatures, and source locations.
4. **Intermediate model** represents projects, files, modules, symbols, documentation fields, and diagnostics in a language-neutral form.
5. **Validation** checks documentation consistency without inventing missing intent.
6. **Renderers** transform the shared model into Markdown, JSON, or HTML selected by output extension.

## Module boundaries

- `cli`: command-line interface and exit codes.
- `discovery`: safe recursive source discovery and ignore rules.
- `parsers`: language-specific Tree-sitter adapters.
- `model`: serializable language-neutral documentation types.
- `validate`: documentation rules and stable diagnostics.
- `renderers`: output backends, independent of parser implementations.
- `config`: configuration parsing, validation, and precedence.

These are logical boundaries; the implementation has a shared adapter registry, five language adapters, a model, TOML config loader, Markdown/JSON/HTML renderers, and CLI in focused modules.

## Tree-sitter adapters

Each adapter declares supported file extensions and language conventions, parses source without executing it, and returns normalized symbols plus diagnostics. Parser errors should be recoverable where practical so one malformed file does not necessarily prevent documentation for the rest of a project. The current adapters support C, C++, Python, and Lua alongside Rust; extraction scope and doc-comment conventions are listed in [ADDING_LANGUAGE_ADAPTERS.md](ADDING_LANGUAGE_ADAPTERS.md).

Tree-sitter provides syntax trees, not complete semantic analysis. The engine must not claim type resolution or compiler-level guarantees unless a language-specific semantic backend supplies them.

## Intermediate model

The initial model should support:

- Project and source file
- Module or namespace
- Symbol (function, method, class, type, constant, or field)
- Signature and parameters
- Explicit summary and long description
- Return value, examples, tags, and references
- Source span (path, start/end line and column)
- Diagnostics with severity and stable codes

Only populate fields actually supported by an adapter. Unknown values should remain unknown rather than being fabricated.

## Configuration

The current `docweave.toml` config supports a title, optional description, source directory, Markdown, JSON, or HTML output path, optional hosted source URL, and include/exclude filters by file extension. The output extension selects the format. Relative config paths resolve from the config file's directory; CLI output overrides resolve from the working directory. Configuration is validated before scanning or writing. Visibility controls and strict mode remain future work.

PDF generation is deferred. The evaluation recommends prototyping an optional HTML-to-PDF path when users need PDF export; ordinary generation must not download or require a browser. See [PDF_OUTPUT_EVALUATION.md](PDF_OUTPUT_EVALUATION.md).

## Security and determinism

- Treat source files as untrusted data and never execute them.
- Do not follow directory symlinks by default.
- Escape source-derived text in generated HTML.
- Avoid network access during ordinary local generation.
- Keep output ordering stable and diagnostics actionable.
