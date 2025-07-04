use anyhow::Result;
use clap::Parser as _;
use cli::Args;

mod cli;
mod gitinfo;
#[cfg(test)]
mod tests;
mod util;

/// Entry point for the git-statuses CLI tool.
/// Parses arguments, scans for repositories, prints their status and a summary.
fn main() -> Result<()> {
    let args = Args::parse();
    let mut repos = util::find_repositories(&args)?;

    util::print_repositories(&mut repos, &args);
    if args.summary {
        util::print_summary(&repos);
    }

    Ok(())
}
