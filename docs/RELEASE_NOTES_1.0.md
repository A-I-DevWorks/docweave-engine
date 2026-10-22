# DocWeave Engine 1.0.0 — planned release notes

> Draft only. Do not publish until the 1.0 release criteria in
> [RELEASING.md](RELEASING.md) pass and the platform matrix is verified.

## Highlights

- Generate navigable Markdown, JSON, and HTML documentation from Rust, C,
  C++, Python, and Lua source.
- Extend scanning and generation through the public `LanguageAdapter` API.
- Configure output metadata and filters with TOML or CLI options.
- Use versioned TOML configuration (`config_version = 1`) and versioned JSON
  output (`format_version = 1`).

## Platform support

- Linux x86_64: exercised by the local release checks and Ubuntu CI workflow.
- macOS and Windows: added to the CI matrix, but still unverified until that
  workflow runs successfully on both platforms.

## Known limitations

- Adapters provide syntax-based extraction, not compiler-level type or symbol
  resolution. See [the adapter guide](ADDING_LANGUAGE_ADAPTERS.md) for
  language-specific extraction limits.
- PDF output is deferred; generate HTML and use a browser's print function.
- The Rust adapter currently extracts line-style `///` and `//!` docs, not
  block-style Rust doc comments.

## Upgrade notes

- This is the first planned stable release; no compatibility is promised with
  pre-1.0 library APIs or generated output.
- Existing TOML files without `config_version` remain accepted as version 1.
- JSON consumers should read `format_version` and the nested `project` object;
  version 1 wraps the model with generator and project metadata.

## Verification before publication

- Replace this draft notice with the final release date and exact version.
- Confirm the macOS and Windows CI jobs pass, then confirm package contents,
  install workflow, and upgrade instructions from a clean checkout.
