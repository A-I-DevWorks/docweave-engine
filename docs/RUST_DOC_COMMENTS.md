# Rust documentation comments

This guide describes the Rust documentation comments that DocWeave currently extracts and how they appear in its documentation model.

## Outer comments: `///`

Use `///` to document the declaration immediately below the comment:

```rust
/// Adds two signed integers.
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

DocWeave associates `Adds two signed integers.` with the `add` function. The same form works for modules, structs, enums, traits, functions, methods, and constants:

```rust
/// A container that holds one value.
#[derive(Debug)]
pub struct Container<T> {
    pub value: T,
}

/// Converts a value to a string.
pub trait ToText {
    fn to_text(&self) -> String;
}

impl<T: std::fmt::Debug> Container<T> {
    /// Formats the stored value for display.
    pub fn display(&self) -> String {
        format!("{:?}", self.value)
    }
}
```

Consecutive `///` lines become one documentation string, joined with newline characters. Attributes between the documentation and declaration are allowed, as in the `Container` example.

## Inner comments: `//!`

Use `//!` to document the file or inline module that contains the comment. At the top of a source file, it becomes that file's documentation:

```rust
//! Helpers for working with paths.
//! The text is available as the file's documentation in the model.

/// Returns the file extension, if present.
pub fn extension(path: &std::path::Path) -> Option<&std::ffi::OsStr> {
    path.extension()
}
```

Inside an inline module, `//!` documents that module. If the module also has an outer `///` comment, DocWeave combines the two texts in that order, separated by a newline:

```rust
/// Path-related helpers.
pub mod paths {
    //! Functions for inspecting file paths.

    /// Returns the file extension, if present.
    pub fn extension(path: &std::path::Path) -> Option<&std::ffi::OsStr> {
        path.extension()
    }
}
```

The module's documentation in the model is:

```text
Path-related helpers.
Functions for inspecting file paths.
```

## Reading the extracted documentation

The library API returns a `DocProject`. File-level `//!` text is in `DocFile.documentation`; each extracted declaration has its own optional `DocSymbol.documentation`:

```rust,ignore
use std::path::Path;

let project = docweave_engine::document_rust(Path::new("src"))?;
for file in project.files {
    if let Some(text) = file.documentation {
        println!("File docs: {text}");
    }
    for symbol in file.symbols {
        if let Some(text) = symbol.documentation {
            println!("{}: {text}", symbol.name);
        }
    }
}
# Ok::<(), anyhow::Error>(())
```

DocWeave keeps documentation absent when no supported comment is present; it does not invent descriptions. The CLI can render the extracted model as Markdown, JSON, or HTML; see the README for configuration and generation commands.

## Current support and limitations

- Supported: line-style outer comments (`///`) and inner comments (`//!`).
- Not currently extracted: block-style doc comments (`/** ... */` and `/*! ... */`).
- The extracted text is comment content with the marker and one optional following space removed. Consecutive comment lines are joined with `\n`.
- Rust syntax errors are recorded as diagnostics with source spans; extraction can still return declarations that Tree-sitter recognized in a malformed file.
- DocWeave does not execute Rust code or infer undocumented intent.
