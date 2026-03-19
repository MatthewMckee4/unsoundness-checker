use std::collections::BTreeMap;
use std::time::Instant;

use anyhow::Result;
use colored::Colorize;
use ruff_db::diagnostic::DiagnosticId;

use crate::real_world_projects::{
    Benchmark, count_lines_of_code, run_checker, run_type_checker, setup_project_db,
};

struct BenchmarkResult {
    project_name: String,
    checker_durations: Vec<std::time::Duration>,
    type_checker_durations: Vec<std::time::Duration>,
    diagnostics: BTreeMap<String, usize>,
    lines_of_code: usize,
}

pub fn run_performance_benchmarks(
    project_names: &[String],
    iterations: usize,
    show_summary: bool,
    latex: bool,
) -> Result<()> {
    println!("{}", "Running performance benchmarks...".bold().blue());
    if iterations > 1 {
        println!("  Running each project {iterations} times");
    }
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
        results.push(run_single_benchmark(benchmark, iterations)?);
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

        print_durations("Unsoundness checker", &result.checker_durations);
        print_durations("Type checker", &result.type_checker_durations);

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

    if latex {
        print_latex_table(&results);
        if show_summary {
            print_latex_diagnostics_table(&results);
        }
    }

    Ok(())
}

fn print_latex_table(results: &[BenchmarkResult]) {
    println!("\\begin{{table}}[htbp]");
    println!("\\centering");
    println!(
        "\\begin{{tabular}}{{l@{{\\hspace{{2em}}}}r@{{\\hspace{{2em}}}}r@{{\\hspace{{2em}}}}r@{{\\hspace{{2em}}}}r}}"
    );
    println!("\\toprule");
    println!("Project & Check (ms) & \\texttt{{ty}} (ms) & LoC & Diagnostics \\\\");
    println!("\\midrule");

    for result in results {
        let check_time = mean_duration_ms(&result.checker_durations);
        let type_check_time = mean_duration_ms(&result.type_checker_durations);
        let total_diagnostics: usize = result.diagnostics.values().sum();

        println!(
            "\\texttt{{{}}} & {:.2} & {:.2} & {} & {} \\\\",
            result.project_name,
            check_time,
            type_check_time,
            format_with_commas(result.lines_of_code),
            format_with_commas(total_diagnostics),
        );
    }

    println!("\\bottomrule");
    println!("\\end{{tabular}}");
    println!("\\caption{{Benchmark Summary Statistics}}");
    println!("\\label{{tab:benchmark-summary}}");
    println!("\\end{{table}}");
}

fn print_latex_diagnostics_table(results: &[BenchmarkResult]) {
    use std::collections::BTreeSet;

    let all_diagnostics: BTreeSet<&str> = results
        .iter()
        .flat_map(|r| r.diagnostics.keys().map(String::as_str))
        .collect();

    // Sort diagnostics by total count descending
    let mut sorted_diagnostics: Vec<&str> = all_diagnostics.into_iter().collect();
    sorted_diagnostics.sort_by(|a, b| {
        let total_a: usize = results.iter().filter_map(|r| r.diagnostics.get(*a)).sum();
        let total_b: usize = results.iter().filter_map(|r| r.diagnostics.get(*b)).sum();
        total_b.cmp(&total_a)
    });

    let col_spec = format!("l{}", "r".repeat(results.len()));

    println!();
    println!("\\begin{{table}}[htbp]");
    println!("\\centering");
    println!("\\begin{{tabular}}{{{col_spec}}}");
    println!("\\toprule");

    // Header row
    print!("Diagnostic");
    for (i, result) in results.iter().enumerate() {
        if i < results.len() - 1 {
            print!(" & \\texttt{{{}}} &", result.project_name);
        } else {
            print!(" & \\texttt{{{}}}", result.project_name);
        }
    }
    println!(" \\\\");
    println!("\\midrule");

    for diag in &sorted_diagnostics {
        print!("{diag}");
        for result in results {
            match result.diagnostics.get(*diag) {
                Some(&count) => print!(" & {}", format_with_commas(count)),
                None => print!(" & ---"),
            }
        }
        println!(" \\\\");
    }

    println!("\\bottomrule");
    println!("\\end{{tabular}}");
    println!("\\caption{{Diagnostic Counts by Project}}");
    println!("\\label{{tab:benchmark-diagnostics}}");
    println!("\\end{{table}}");
}

fn mean_duration_ms(durations: &[std::time::Duration]) -> f64 {
    let sum: f64 = durations.iter().map(|d| d.as_secs_f64() * 1000.0).sum();
    #[allow(clippy::cast_precision_loss)]
    let mean = sum / durations.len() as f64;
    mean
}

fn format_with_commas(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn print_durations(label: &str, durations: &[std::time::Duration]) {
    if durations.len() == 1 {
        println!("  {label}:  {:.2}ms", durations[0].as_secs_f64() * 1000.0);
    } else {
        let times: Vec<f64> = durations.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
        let min = times.iter().copied().reduce(f64::min).unwrap();
        let max = times.iter().copied().reduce(f64::max).unwrap();
        #[allow(clippy::cast_precision_loss)]
        let mean = times.iter().sum::<f64>() / times.len() as f64;
        println!(
            "  {label}:  {:.2}ms min / {:.2}ms mean / {:.2}ms max ({} runs)",
            min,
            mean,
            max,
            times.len()
        );
    }
}

fn run_single_benchmark(benchmark: &Benchmark, iterations: usize) -> Result<BenchmarkResult> {
    let project_name = benchmark.project_name();
    println!("{} {}", "Benchmarking:".bold(), project_name.bold().green());

    println!("  {} project...", "Setting up".dimmed());
    let installed_project = benchmark.setup()?;

    let lines_of_code = count_lines_of_code(&setup_project_db(&installed_project));

    let mut checker_durations = Vec::with_capacity(iterations);
    let mut type_checker_durations = Vec::with_capacity(iterations);
    let mut last_diagnostics = Vec::new();

    for i in 0..iterations {
        if iterations > 1 {
            println!(
                "  {} unsoundness checker (run {}/{})...",
                "Running".dimmed(),
                i + 1,
                iterations
            );
        } else {
            println!("  {} unsoundness checker...", "Running".dimmed());
        }

        let db = setup_project_db(&installed_project);
        let check_start = Instant::now();
        let diagnostics = run_checker(&db);
        checker_durations.push(check_start.elapsed());
        last_diagnostics = diagnostics;

        if iterations > 1 {
            println!(
                "  {} type checker (run {}/{})...",
                "Running".dimmed(),
                i + 1,
                iterations
            );
        } else {
            println!("  {} type checker...", "Running".dimmed());
        }

        let db = setup_project_db(&installed_project);
        let check_start = Instant::now();
        let type_checker_diagnostics = run_type_checker(&db);
        type_checker_durations.push(check_start.elapsed());
        assert!(
            !type_checker_diagnostics.is_empty(),
            "Type checker produced no diagnostics for project '{project_name}'"
        );
    }

    println!("  {}", "done".green());
    println!();

    let mut summary: BTreeMap<String, usize> = BTreeMap::new();
    for diagnostic in last_diagnostics {
        if let DiagnosticId::Lint(lint_name) = diagnostic.id() {
            *summary.entry(lint_name.as_str().to_string()).or_insert(0) += 1;
        }
    }

    Ok(BenchmarkResult {
        project_name: project_name.to_string(),
        checker_durations,
        type_checker_durations,
        diagnostics: summary,
        lines_of_code,
    })
}
