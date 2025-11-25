use std::time::Instant;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

mod real_world_projects;
use real_world_projects::Benchmark;

use crate::real_world_projects::run_checker;

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
    Performance {
        /// Specific projects to benchmark (e.g., pytest, pydantic).
        /// If not specified, all projects will be run.
        #[arg(long = "project")]
        projects: Vec<String>,
    },
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

    if let Some(Command::Performance { projects }) = cli.command {
        run_performance_benchmarks(&projects)?;
    }

    Ok(())
}

struct BenchmarkResult {
    project_name: String,
    check_duration: std::time::Duration,
    diagnostics: usize,
}

fn run_performance_benchmarks(project_names: &[String]) -> Result<()> {
    println!("{}", "Running performance benchmarks...".bold().blue());
    println!();

    // All available benchmarks
    let all_benchmarks = vec![
        ("pydantic", Benchmark::pydantic()),
        ("pytest", Benchmark::pytest()),
    ];

    // If no projects specified, run all
    let benchmarks_to_run: Vec<_> = if project_names.is_empty() {
        all_benchmarks.iter().map(|(_, b)| b).collect()
    } else {
        // Validate all project names first
        for name in project_names {
            if !all_benchmarks.iter().any(|(n, _)| n == name) {
                anyhow::bail!(
                    "Unknown project '{}'. Available projects: {}",
                    name,
                    all_benchmarks
                        .iter()
                        .map(|(n, _)| *n)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }

        // Collect the selected benchmarks
        all_benchmarks
            .iter()
            .filter(|(n, _)| project_names.contains(&(*n).to_string()))
            .map(|(_, b)| b)
            .collect()
    };

    let mut results = Vec::new();
    for benchmark in benchmarks_to_run {
        results.push(run_single_benchmark(benchmark)?);
    }

    // Display all results at the end
    println!();
    println!("{}", "=".repeat(60).bold());
    println!("{}", "Benchmark Results".bold().blue());
    println!("{}", "=".repeat(60).bold());
    println!();

    for result in &results {
        println!(
            "{} {}",
            "Project:".bold(),
            result.project_name.bold().green()
        );
        println!("  Check time:  {:.2}s", result.check_duration.as_secs_f64());
        println!("  Diagnostics: {}", result.diagnostics);
        println!();
    }

    Ok(())
}

fn run_single_benchmark(benchmark: &Benchmark) -> Result<BenchmarkResult> {
    let project_name = benchmark.project_name();
    println!("{} {}", "Benchmarking:".bold(), project_name.bold().green());

    // Setup phase (clone repo, install dependencies)
    println!("  {} project...", "Setting up".dimmed());
    let installed_project = benchmark.setup()?;

    // Run the unsoundness checker
    println!("  {} unsoundness checker...", "Running".dimmed());
    let check_start = Instant::now();
    let diagnostics = run_checker(&installed_project)?;
    let check_duration = check_start.elapsed();
    println!("  {}", "done".green(),);
    println!();

    Ok(BenchmarkResult {
        project_name: project_name.to_string(),
        check_duration,
        diagnostics,
    })
}
