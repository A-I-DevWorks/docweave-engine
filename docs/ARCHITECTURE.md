# Architecture Proposal

## Status

This document describes the proposed architecture, not components already implemented.

## Pipeline

1. **Configuration** loads settings and validates options.
2. **Discovery** walks configured source roots, applies include/exclude rules, and identifies candidate files.
3. **Language adapters** parse files and extract symbols, documentation comments, signatures, and source locations.
4. **Intermediate model** represents files, modules, symbols, parameters, return values, examples, and diagnostics in a language-neutral form.
5. **Validation** checks documentation consistency and reports issues without inventing intent.
6. **Renderers** transform the shared model into HTML, Markdown, or JSON.

## Boundaries

- The core must not depend on a particular programming language.
- Adapters must not own HTML or Markdown presentation.
- Renderers must not re-parse source code.
- Parsing untrusted source must not execute it.
- Network access should not be required for local generation.

## Intermediate model

The initial model should consider these entities:

- Project and source file
- Module or namespace
- Symbol (function, method, class, type, constant, or field)
- Signature and parameters
- Explicit summary and long description
- Return value, examples, tags, and references
- Source span (path, start/end line and column)
- Diagnostics with severity and stable codes

Only populate fields actually supported by an adapter. Unknown values should remain unknown rather than being fabricated.

## Adapter contract

Each adapter should declare supported languages and documentation conventions, determine whether it can process a file, and return normalized symbols plus diagnostics. Parser errors should be recoverable where practical so one malformed file does not necessarily prevent documentation for the rest of the project.

## Renderer contract

Renderers consume only the intermediate model and produce deterministic output where practical. They must escape untrusted source text correctly, generate stable links, and avoid embedding executable source content.

## Configuration

Configuration should cover source roots, include/exclude patterns, output formats and destination, private/internal symbol handling, and strictness. Define precedence between configuration files, command-line flags, and defaults before implementation.

## Initial milestones

1. Select the implementation language and package/build system.
2. Define and test the intermediate model.
3. Implement file discovery and diagnostics.
4. Implement one language adapter and one renderer end-to-end.
5. Add fixtures and integration tests.
6. Add further adapters and output formats based on real use cases.
