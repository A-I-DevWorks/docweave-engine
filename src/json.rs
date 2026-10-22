//! JSON renderer for the language-neutral documentation model.

use anyhow::{Context, Result};
use serde::Serialize;

use crate::model::DocProject;

/// Serialize a versioned documentation document as pretty-printed JSON.
pub fn render(
    project: &DocProject,
    title: &str,
    description: Option<&str>,
    source_url: Option<&str>,
) -> Result<String> {
    let document = JsonDocument {
        format_version: 1,
        generator: Generator {
            name: "DocWeave Engine",
            version: env!("CARGO_PKG_VERSION"),
        },
        metadata: Metadata {
            title,
            description,
            source_url,
        },
        project,
    };
    serde_json::to_string_pretty(&document).context("failed to serialize documentation as JSON")
}

#[derive(Serialize)]
struct JsonDocument<'a> {
    format_version: u32,
    generator: Generator,
    metadata: Metadata<'a>,
    project: &'a DocProject,
}

#[derive(Serialize)]
struct Generator {
    name: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct Metadata<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_url: Option<&'a str>,
}
