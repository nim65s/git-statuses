use std::path::PathBuf;

use clap::{ArgAction, Parser};

/// Scan the given directory for Git repositories and display their status.
/// A Repository turns red if it has unpushed changes.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to scan
    #[arg(default_value = ".")]
    pub dir: PathBuf,
    /// Recursively scan all subdirectories
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub all: bool,
    /// Show remote URL
    #[arg(short = 'r', long, action = ArgAction::SetTrue)]
    pub remote: bool,
    /// Show a summary of the scan
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub summary: bool,
}
