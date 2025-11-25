use clap::{Parser, ValueEnum};

use crate::cli::logging::Verbosity;

/// Summary output mode for diagnostics
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum SummaryMode {
    /// Don't show summary (default)
    #[default]
    False,
    /// Show diagnostics and summary
    True,
    /// Show only summary, not individual diagnostics
    Only,
}

#[derive(Debug, Parser)]
#[command(
    author,
    name = crate::NAME,
    about = "A Python unsoundness checker."
)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Run tests.
    Check(CheckCommand),

    /// Display the version
    Version,
}

#[derive(Debug, Parser)]
pub struct CheckCommand {
    /// List of files or directories to check.
    #[clap(
        help = "List of files or directories to check [default: the project root]",
        value_name = "PATH"
    )]
    pub(crate) paths: Vec<camino::Utf8PathBuf>,

    /// Show summary of diagnostic counts by rule
    #[clap(long, value_enum, default_value = "false")]
    pub(crate) summary: SummaryMode,

    #[clap(flatten)]
    pub(crate) verbosity: Verbosity,
}
