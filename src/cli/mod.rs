use std::{
    io::{self, BufWriter, Write},
    process::{ExitCode, Termination},
};

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use ruff_db::{
    diagnostic::{DisplayDiagnosticConfig, DisplayDiagnostics},
    system::{OsSystem, SystemPath, SystemPathBuf},
};
use ty_project::{
    Db, ProjectDatabase, ProjectMetadata, metadata::options::ProjectOptionsOverrides,
};

use crate::{
    checker::check_file,
    cli::{
        args::{CheckCommand, Command},
        logging::setup_tracing,
    },
    version::{self},
};

mod args;
mod logging;

pub use args::Args;

pub fn run() -> anyhow::Result<ExitStatus> {
    let args = wild::args_os();

    let args = argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX)
        .context("Failed to read CLI arguments from file")?;

    let args = Args::parse_from(args);

    match args.command {
        Command::Check(test_args) => test(&test_args),
        Command::Version => version().map(|()| ExitStatus::Success),
    }
}

pub(crate) fn version() -> Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "{} {}", crate::NAME, crate::version::version())?;

    Ok(())
}

pub(crate) fn test(args: &CheckCommand) -> Result<ExitStatus> {
    let verbosity = args.verbosity.level();
    let _guard = setup_tracing(verbosity);

    tracing::debug!("Version: {}", version::version());

    let cwd = {
        let cwd = std::env::current_dir().context("Failed to get the current working directory")?;
        SystemPathBuf::from_path_buf(cwd).map_err(|path| {
            anyhow!(
                "The current working directory `{}` contains non-Unicode characters.",
                path.display()
            )
        })?
    };

    let check_paths: Vec<_> = args
        .paths
        .iter()
        .map(|path| SystemPath::absolute(SystemPath::from_std_path(path).unwrap(), &cwd))
        .collect();

    let system = OsSystem::new(&cwd);

    let mut project_metadata = ProjectMetadata::discover(&cwd, &system)?;

    project_metadata.apply_configuration_files(&system)?;

    let project_options_overrides = ProjectOptionsOverrides::default();
    project_metadata.apply_overrides(&project_options_overrides);

    let mut db = ProjectDatabase::new(project_metadata, system)?;

    if !check_paths.is_empty() {
        db.project().set_included_paths(&mut db, check_paths);
    }

    let files = db.project().files(&db);

    let mut diagnostics = Vec::new();

    for file in &files {
        diagnostics.extend(check_file(&db, file));
    }
    let display_config = DisplayDiagnosticConfig::default();

    let mut stdout = io::stdout();

    if diagnostics.is_empty() {
        writeln!(stdout, "All checks passed")?;
    } else {
        write!(
            stdout,
            "{}",
            DisplayDiagnostics::new(&db, &display_config, &diagnostics)
        )?;

        let num_diagnostics = diagnostics.len();

        writeln!(
            stdout,
            "Found {} diagnostic{}",
            num_diagnostics,
            if num_diagnostics > 1 { "s" } else { "" }
        )?;
    }

    Ok(ExitStatus::Success)
}

#[derive(Copy, Clone)]
pub enum ExitStatus {
    /// Checking was successful and there were no errors.
    Success = 0,

    /// Checking was successful but there were errors.
    Failure = 1,

    /// Checking failed.
    Error = 2,
}

impl Termination for ExitStatus {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

impl ExitStatus {
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }
}
