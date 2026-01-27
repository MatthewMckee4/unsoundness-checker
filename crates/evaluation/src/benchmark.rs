use std::collections::BTreeMap;
use std::time::Instant;

use anyhow::Result;
use colored::Colorize;
use ruff_db::diagnostic::DiagnosticId;

use crate::real_world_projects::Benchmark;

use crate::real_world_projects::CheckResult;
use crate::real_world_projects::run_checker;

struct BenchmarkResult {
    project_name: String,
    check_duration: std::time::Duration,
    diagnostics: BTreeMap<String, usize>,
    lines_of_code: usize,
}

pub fn run_performance_benchmarks(project_names: &[String], show_summary: bool) -> Result<()> {
    println!("{}", "Running performance benchmarks...".bold().blue());
    println!();

    let all_benchmarks = [
        ("pydantic", Benchmark::pydantic()),
        ("pytest", Benchmark::pytest()),
        ("fastapi", Benchmark::fastapi()),
        ("black", Benchmark::black()),
        ("flask", Benchmark::flask()),
    ];

    // If no projects specified, run all
    let benchmarks_to_run: Vec<_> = if project_names.is_empty() {
        all_benchmarks.iter().map(|(_, b)| b).collect()
    } else {
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

    println!();
    println!("{}", "Benchmark Results".bold().blue());
    println!("{}", "-".repeat(60).bold());
    println!();

    for result in &results {
        println!(
            "{} {}",
            "Project:".bold(),
            result.project_name.bold().green()
        );
        println!("  Check time:  {:.2}s", result.check_duration.as_secs_f64());
        println!("  Lines of code: {}", result.lines_of_code);

        if show_summary {
            println!("  Diagnostic summary:");
            let mut sorted_diagnostics: Vec<_> = result.diagnostics.iter().collect();
            sorted_diagnostics.sort_by(|a, b| b.1.cmp(a.1));

            for (kind, count) in &sorted_diagnostics {
                println!("    {kind}: {count}");
            }
        } else {
            let total: usize = result.diagnostics.values().sum();
            println!("  Total diagnostics: {total}");
        }
        println!();
    }

    Ok(())
}

fn run_single_benchmark(benchmark: &Benchmark) -> Result<BenchmarkResult> {
    let project_name = benchmark.project_name();
    println!("{} {}", "Benchmarking:".bold(), project_name.bold().green());

    println!("  {} project...", "Setting up".dimmed());
    let installed_project = benchmark.setup()?;

    println!("  {} unsoundness checker...", "Running".dimmed());
    let check_start = Instant::now();
    let CheckResult {
        diagnostics,
        lines_of_code,
    } = run_checker(&installed_project)?;
    let check_duration = check_start.elapsed();
    println!("  {}", "done".green(),);
    println!();

    let mut summary: BTreeMap<String, usize> = BTreeMap::new();
    for diagnostic in diagnostics {
        if let DiagnosticId::Lint(lint_name) = diagnostic.id() {
            *summary.entry(lint_name.as_str().to_string()).or_insert(0) += 1;
        }
    }

    Ok(BenchmarkResult {
        project_name: project_name.to_string(),
        check_duration,
        diagnostics: summary,
        lines_of_code,
    })
}
