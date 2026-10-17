//! Language-neutral data structures produced by source adapters.

use serde::{Deserialize, Serialize};

/// A complete documentation model for one scanned project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocProject {
    pub root: String,
    pub files: Vec<DocFile>,
}

/// Documentation and diagnostics extracted from one source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocFile {
    pub path: String,
    pub documentation: Option<String>,
    pub symbols: Vec<DocSymbol>,
    pub diagnostics: Vec<Diagnostic>,
}

/// A documented declaration found in source code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocSymbol {
    pub kind: SymbolKind,
    pub name: String,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub span: SourceSpan,
}

/// Shared declaration kinds used by the built-in language adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Module,
    Struct,
    Enum,
    Trait,
    Function,
    Method,
    Constant,
}

/// A one-based source range. End positions are exclusive, matching Tree-sitter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub path: String,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// A stable, source-located issue found while parsing or extracting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: String,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub span: Option<SourceSpan>,
}

/// Diagnostic importance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Warning,
    Error,
}
