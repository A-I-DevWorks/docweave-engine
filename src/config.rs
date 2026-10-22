//! TOML configuration loading and validation.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// Validated settings for a documentation generation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub title: String,
    pub description: Option<String>,
    pub source: PathBuf,
    pub output: PathBuf,
    pub source_url: Option<String>,
    pub include_extensions: Vec<String>,
    pub exclude_extensions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct FileConfig {
    title: String,
    description: Option<String>,
    source: PathBuf,
    output: PathBuf,
    source_url: Option<String>,
    include_extensions: Vec<String>,
    exclude_extensions: Vec<String>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            title: "Project Documentation".to_owned(),
            description: None,
            source: PathBuf::from("."),
            output: PathBuf::from("docs.md"),
            source_url: None,
            include_extensions: Vec::new(),
            exclude_extensions: Vec::new(),
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
            description: file.description,
            source: resolve(&base, file.source),
            output: resolve(&base, file.output),
            source_url: file
                .source_url
                .map(|url| url.trim_end_matches('/').to_owned()),
            include_extensions: normalize_extensions(file.include_extensions),
            exclude_extensions: normalize_extensions(file.exclude_extensions),
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

    /// Set the output directory while preserving the configured output filename.
    pub fn with_output_dir(mut self, directory: PathBuf) -> Result<Self> {
        let directory = if directory.is_absolute() {
            directory
        } else {
            std::env::current_dir()?.join(directory)
        };
        let filename = self.output.file_name().ok_or_else(|| {
            anyhow::anyhow!(
                "configured output path has no filename: {}",
                self.output.display()
            )
        })?;
        self.output = directory.join(filename);
        self.validate()?;
        Ok(self)
    }

    /// Override configured extension filters from CLI values when supplied.
    pub fn with_extension_filters(
        mut self,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
    ) -> Result<Self> {
        if let Some(include) = include {
            self.include_extensions = normalize_extensions(include);
        }
        if let Some(exclude) = exclude {
            self.exclude_extensions = normalize_extensions(exclude);
        }
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
        if self
            .description
            .as_ref()
            .is_some_and(|text| text.trim().is_empty())
        {
            bail!("config description must not be empty when provided");
        }
        if !self.source.exists() {
            bail!("source directory does not exist: {}", self.source.display());
        }
        if !self.source.is_dir() {
            bail!("source path is not a directory: {}", self.source.display());
        }
        if self.output.as_os_str().is_empty()
            || !self.output.extension().is_some_and(|extension| {
                ["md", "json", "html"]
                    .iter()
                    .any(|supported| extension.eq_ignore_ascii_case(supported))
            })
        {
            bail!("output path must use a supported .md, .json, or .html extension");
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
        if self
            .include_extensions
            .iter()
            .chain(&self.exclude_extensions)
            .any(|extension| {
                extension.is_empty() || !extension.chars().all(|ch| ch.is_ascii_alphanumeric())
            })
        {
            bail!("file extensions must contain only ASCII letters and digits");
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

fn normalize_extensions(extensions: Vec<String>) -> Vec<String> {
    extensions
        .into_iter()
        .map(|extension| {
            extension
                .trim()
                .trim_start_matches('.')
                .to_ascii_lowercase()
        })
        .collect()
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

        std::fs::write(&config_path, "title = \"Valid\"\noutput = \"site.pdf\"\n").unwrap();
        assert!(Config::load(Some(&config_path)).is_err());
    }
}
