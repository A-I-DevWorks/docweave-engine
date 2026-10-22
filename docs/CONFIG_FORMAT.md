# Configuration format

DocWeave reads TOML configuration files. `config_version` identifies the
configuration contract:

- Version `1` is the current format. It may be written explicitly as
  `config_version = 1`.
- If `config_version` is omitted, DocWeave treats the file as version 1 for
  compatibility with configurations created before the field existed.
- Unknown versions are rejected with an error that names the supported version.
- Unknown keys are rejected so misspellings do not silently change behavior.

The complete version 1 key set is shown in
[`docweave.toml.example`](../docweave.toml.example). To make an incompatible
change to key meaning, path resolution, or required values, introduce a new
configuration version and document migration behavior. Additive optional
settings should have defaults that preserve existing output.

Paths in the config resolve relative to the config file. Command-line output
overrides resolve relative to the current working directory. Output format is
selected by `.md`, `.json`, or `.html` extension.
