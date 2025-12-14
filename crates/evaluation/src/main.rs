use anyhow::Result;
use clap::{Parser, Subcommand};

mod real_world_projects;

use crate::{benchmark::run_performance_benchmarks, unsoundness_suite::run_unsoundness_suite};

mod benchmark;
mod unsoundness_suite;

#[derive(Parser, Debug)]
#[command(name = "evaluation")]
#[command(about = "Evaluation tool for unsoundness-checker", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run performance benchmarks
    Benchmark {
        /// Specific projects to benchmark (e.g., pytest, pydantic).
        /// If not specified, all projects will be run.
        #[arg(long = "project")]
        projects: Vec<String>,
    },
    /// Run unsoundness suite
    UnsoundnessSuite,
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::default()
                .add_directive("ty=warn".parse().unwrap())
                .add_directive("ruff=warn".parse().unwrap())
                .add_directive(
                    "unsoundness_checker=warn"
                        .parse()
                        .expect("Hardcoded directive to be valid"),
                ),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Command::Benchmark { projects }) => {
            run_performance_benchmarks(&projects)?;
        }
        Some(Command::UnsoundnessSuite) => run_unsoundness_suite()?,
        _ => (),
    }

    Ok(())
}
