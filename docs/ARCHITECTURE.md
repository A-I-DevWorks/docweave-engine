# Architecture

## Status

Rust is the implementation language and Tree-sitter is the initial parsing technology. The repository contains a Rust source scanner, a language-neutral documentation model, Rust symbol and doc-comment extraction, and syntax diagnostics. Configuration and output renderers remain future work.

## Initial pipeline

1. **CLI** parses commands and arguments.
2. **Discovery** walks configured roots, skips common dependency/build directories, and selects supported files.
3. **Language adapters** parse source files with Tree-sitter grammars and extract symbols, comments, signatures, and source locations.
4. **Intermediate model** represents projects, files, modules, symbols, documentation fields, and diagnostics in a language-neutral form.
5. **Validation** checks documentation consistency without inventing missing intent.
6. **Renderers** transform the shared model into HTML, Markdown, or JSON.

## Module boundaries

- `cli`: command-line interface and exit codes.
- `discovery`: safe recursive source discovery and ignore rules.
- `parsers`: language-specific Tree-sitter adapters.
- `model`: serializable language-neutral documentation types.
- `validate`: documentation rules and stable diagnostics.
- `renderers`: output backends, independent of parser implementations.
- `config`: configuration parsing, validation, and precedence.

These are logical boundaries; the initial implementation is intentionally smaller.

## Tree-sitter adapters

Each adapter should declare supported file extensions and language conventions, parse source without executing it, and return normalized symbols plus diagnostics. Parser errors should be recoverable where practical so one malformed file does not necessarily prevent documentation for the rest of a project.

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

Configuration should cover source roots, include/exclude patterns, output formats and destination, private/internal symbol handling, and strictness. Define precedence between configuration files, command-line flags, and defaults before implementation.

## Security and determinism

- Treat source files as untrusted data and never execute them.
- Do not follow directory symlinks by default.
- Escape source-derived text in generated HTML.
- Avoid network access during ordinary local generation.
- Keep output ordering stable and diagnostics actionable.
