# Release process and 1.0 criteria

## Install and distribute

For a local checkout, install the `docweave` executable with:

```sh
cargo install --path .
```

For a published release, install the package with:

```sh
cargo install docweave-engine --locked
```

Before publishing to crates.io, update the package version in `Cargo.toml`,
run the project checks, inspect `cargo package --list`, and run
`cargo publish --dry-run`. Publish only after the release review. Tag the same
version as `vX.Y.Z` in Git and attach the source archive and release notes to
the hosted release. Do not commit credentials or publish artifacts from an
unreviewed working tree.

## Versioning contracts

- Follow semantic versioning for the CLI and Rust library once 1.0 is released.
- The TOML `config_version` and JSON `format_version` are independent format
  contracts. Their current rules are described in
  [CONFIG_FORMAT.md](CONFIG_FORMAT.md) and [JSON_FORMAT.md](JSON_FORMAT.md).
- Incompatible configuration or JSON changes require a format-version change
  and migration notes. New optional fields should preserve existing behavior.
- Document user-visible CLI, adapter, output, and format changes in release
  notes.

## Version 1.0 release criteria

Treat 1.0 as ready only when all of the following are true:

- The MIT license is present in source control and included in the package.
- `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all-targets`, and package verification pass on stable Rust.
- README installation and end-to-end examples work from a clean checkout.
- Every built-in language has representative extraction, malformed-syntax,
  and missing-documentation coverage with documented limits.
- Markdown, HTML, and JSON outputs are deterministic for identical inputs and
  configuration; HTML escaping and unsafe-link checks have regression tests.
- The public adapter contract, CLI behavior, TOML version, and JSON version are
  documented, and breaking API/output changes have migration guidance.
- Release notes identify supported platforms, known limitations, and upgrade
  steps. The release artifact installs and `docweave --help` runs successfully.
- No known high-severity correctness, security, or data-loss issue remains.
