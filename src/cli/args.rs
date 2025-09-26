use std::path::PathBuf;

use clap::Parser;

use crate::cli::logging::Verbosity;

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

    /// Display Karva's version
    Version,
}

#[derive(Debug, Parser)]
pub struct CheckCommand {
    /// List of files or directories to check.
    #[clap(
        help = "List of files or directories to check [default: the project root]",
        value_name = "PATH"
    )]
    pub(crate) paths: Vec<PathBuf>,

    #[clap(flatten)]
    pub(crate) verbosity: Verbosity,
}
