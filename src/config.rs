//! TOML configuration loading and validation.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// Validated settings for a documentation generation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub title: String,
    pub source: PathBuf,
    pub output: PathBuf,
    pub source_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct FileConfig {
    title: String,
    source: PathBuf,
    output: PathBuf,
    source_url: Option<String>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            title: "Project Documentation".to_owned(),
            source: PathBuf::from("."),
            output: PathBuf::from("docs.md"),
            source_url: None,
        }
    }
}

impl Config {
    /// Load and validate a TOML file. Relative paths are resolved from its directory.
    /// If no path is supplied, use defaults rooted at the current working directory.
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let (file, base) = if let Some(path) = path {
            let text = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read config {}", path.display()))?;
            let file: FileConfig = toml::from_str(&text)
                .with_context(|| format!("invalid TOML config {}", path.display()))?;
            let parent = path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let base = if parent.is_absolute() {
                parent.to_path_buf()
            } else {
                std::env::current_dir()?.join(parent)
            };
            (file, base)
        } else {
            (FileConfig::default(), std::env::current_dir()?)
        };

        let config = Self {
            title: file.title,
            source: resolve(&base, file.source),
            output: resolve(&base, file.output),
            source_url: file
                .source_url
                .map(|url| url.trim_end_matches('/').to_owned()),
        };
        config.validate()?;
        Ok(config)
    }

    /// Override the output path, resolving relative values from the current directory.
    pub fn with_output(mut self, output: PathBuf) -> Result<Self> {
        self.output = if output.is_absolute() {
            output
        } else {
            std::env::current_dir()?.join(output)
        };
        self.validate()?;
        Ok(self)
    }

    /// Validate all public settings before a generation run.
    pub fn validate(&self) -> Result<()> {
        if self.title.trim().is_empty() {
            bail!("config title must not be empty");
        }
        if self.title.contains(['\n', '\r']) {
            bail!("config title must be a single line");
        }
        if !self.source.is_dir() {
            bail!("source path is not a directory: {}", self.source.display());
        }
        if self.output.as_os_str().is_empty()
            || !self
                .output
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        {
            bail!("output path must have a .md extension");
        }
        if let Some(url) = &self.source_url
            && (!url.starts_with("https://") && !url.starts_with("http://")
                || url
                    .trim_start_matches("https://")
                    .trim_start_matches("http://")
                    .split('/')
                    .next()
                    .is_none_or(str::is_empty)
                || url.chars().any(char::is_whitespace)
                || url.contains(['?', '#']))
        {
            bail!("source_url must be an http(s) base URL without query or fragment");
        }
        Ok(())
    }
}

fn resolve(base: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        base.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_partial_config_and_resolves_paths_from_config_directory() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::create_dir(temp.path().join("source")).unwrap();
        let config_path = temp.path().join("docweave.toml");
        std::fs::write(
            &config_path,
            "title = \"Example\"\nsource = \"source\"\noutput = \"out/index.md\"\nsource_url = \"https://example.test/repo/blob/main/\"\n",
        )
        .unwrap();

        let config = Config::load(Some(&config_path)).unwrap();

        assert_eq!(config.title, "Example");
        assert_eq!(config.source, temp.path().join("source"));
        assert_eq!(config.output, temp.path().join("out/index.md"));
        assert_eq!(
            config.source_url.as_deref(),
            Some("https://example.test/repo/blob/main")
        );
    }

    #[test]
    fn rejects_unknown_keys_and_invalid_values() {
        let temp = tempfile::tempdir().unwrap();
        let config_path = temp.path().join("bad.toml");
        std::fs::write(&config_path, "title = \"\"\nunknown = true\n").unwrap();
        assert!(Config::load(Some(&config_path)).is_err());

        std::fs::write(&config_path, "title = \"Valid\"\noutput = \"site.html\"\n").unwrap();
        assert!(Config::load(Some(&config_path)).is_err());
    }
}
