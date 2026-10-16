use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser as ClapParser, Subcommand};
use docweave_engine::{scan_rust, syntax_issue_count};

#[derive(Debug, ClapParser)]
#[command(name = "docweave", version, about = "Generate structured documentation from source code.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Recursively parse Rust source files and report syntax diagnostics.
    Scan {
        /// Root directory to scan.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Scan { path } => {
            let files = scan_rust(&path)?;
            let mut total_issues = 0;
            for file in &files {
                let issues = syntax_issue_count(&file.tree);
                total_issues += issues;
                println!(
                    "{}: {} syntax issue(s)",
                    file.path.display(),
                    issues
                );
            }
            println!(
                "Scanned {} Rust file(s); {} syntax issue(s) found.",
                files.len(),
                total_issues
            );
            if total_issues > 0 {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
