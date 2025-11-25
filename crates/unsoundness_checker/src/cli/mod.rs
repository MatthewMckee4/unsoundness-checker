use std::{
    collections::BTreeMap,
    io::{self, BufWriter, Write},
    process::{ExitCode, Termination},
};

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use ruff_db::{
    diagnostic::{DiagnosticId, DisplayDiagnosticConfig, DisplayDiagnostics},
    system::{OsSystem, SystemPath, SystemPathBuf},
};
use ty_project::{
    Db, ProjectDatabase, ProjectMetadata, metadata::options::ProjectOptionsOverrides,
};

use crate::{
    checker::check_file,
    cli::{
        args::{CheckCommand, Command, SummaryMode},
        logging::setup_tracing,
    },
    default_rule_registry,
    rule::RuleSelection,
    version::{self},
};

mod args;
mod logging;

pub(crate) use args::Args;

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
        .map(|path| {
            SystemPath::absolute(SystemPath::from_std_path(path.as_std_path()).unwrap(), &cwd)
        })
        .collect();

    let system = OsSystem::new(&cwd);

    let mut project_metadata = ProjectMetadata::discover(&cwd, &system)?;

    let rules = project_metadata.options().rules.clone();

    project_metadata.apply_configuration_files(&system)?;

    let project_options_overrides = ProjectOptionsOverrides::default();
    project_metadata.apply_overrides(&project_options_overrides);

    let mut db = ProjectDatabase::new(project_metadata, system)?;

    if !check_paths.is_empty() {
        db.project().set_included_paths(&mut db, check_paths);
    }

    let files = db.project().files(&db);

    let display_config = DisplayDiagnosticConfig::default();

    let rule_registry = default_rule_registry();

    let (rule_selection, rule_diagnostics) =
        RuleSelection::from_rules_selection(rule_registry, rules.as_ref(), &db);

    let mut stdout = io::stdout();

    write!(
        stdout,
        "{}",
        DisplayDiagnostics::new(&db, &display_config, &rule_diagnostics)
    )?;

    let mut diagnostics = Vec::new();

    for file in &files {
        diagnostics.extend(check_file(&db, file, &rule_selection));
    }

    if diagnostics.is_empty() {
        writeln!(stdout, "All checks passed")?;
    } else {
        // Display individual diagnostics unless summary mode is "only"
        if !matches!(args.summary, SummaryMode::Only) {
            write!(
                stdout,
                "{}",
                DisplayDiagnostics::new(&db, &display_config, &diagnostics)
            )?;
        }

        let num_diagnostics = diagnostics.len();

        // Display diagnostic count unless summary mode is "only"
        if !matches!(args.summary, SummaryMode::Only) {
            writeln!(
                stdout,
                "Found {} diagnostic{}",
                num_diagnostics,
                if num_diagnostics > 1 { "s" } else { "" }
            )?;
        }

        // Display summary if summary mode is "true" or "only"
        if matches!(args.summary, SummaryMode::True | SummaryMode::Only) {
            // Count diagnostics by rule name
            let mut summary: BTreeMap<String, usize> = BTreeMap::new();
            for diagnostic in &diagnostics {
                if let DiagnosticId::Lint(lint_name) = diagnostic.id() {
                    *summary.entry(lint_name.as_str().to_string()).or_insert(0) += 1;
                }
            }

            // Display summary
            if !summary.is_empty() {
                writeln!(stdout)?;
                writeln!(stdout, "Summary:")?;
                for (rule_name, count) in &summary {
                    writeln!(stdout, "  {rule_name}: {count}")?;
                }
                writeln!(
                    stdout,
                    "\nTotal: {num_diagnostics} diagnostic{}",
                    if num_diagnostics > 1 { "s" } else { "" }
                )?;
            }
        }
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
