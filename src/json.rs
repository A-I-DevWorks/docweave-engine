//! JSON renderer for the language-neutral documentation model.

use anyhow::{Context, Result};

use crate::model::DocProject;

/// Serialize a project model as pretty-printed JSON.
pub fn render(project: &DocProject) -> Result<String> {
    serde_json::to_string_pretty(project).context("failed to serialize documentation as JSON")
}
