# JSON output format

DocWeave JSON output is a versioned document. The top-level object has four
fields:

- `format_version`: integer version for the serialized document contract.
- `generator`: the generator name and DocWeave package version.
- `metadata`: configured title plus optional description and hosted source URL.
- `project`: the language-neutral source model, including root, files, symbols,
  spans, and diagnostics.

Version 1 has this shape:

```json
{
  "format_version": 1,
  "generator": {
    "name": "DocWeave Engine",
    "version": "0.1.0"
  },
  "metadata": {
    "title": "Example API",
    "description": "Reference documentation for the project.",
    "source_url": "https://github.com/owner/project/blob/main"
  },
  "project": {
    "root": "/workspace/project/src",
    "files": []
  }
}
```

Optional metadata fields are omitted when unset. `project.files` entries use
the shared model defined in [`src/model.rs`](../src/model.rs). Source spans use
one-based line and column numbers with exclusive end positions. Diagnostics
carry stable codes and severities; consumers should preserve unknown codes and
ignore fields they do not need. The format version changes when the document
shape or field meaning changes incompatibly.
