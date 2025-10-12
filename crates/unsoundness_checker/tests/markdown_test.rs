use std::{fs, panic, path::Path};

pub mod common;

use common::run_rule_tests;

#[test]
fn test_all_rules_from_markdown() {
    let rules_dir = "resources/rules";

    let entries = fs::read_dir(rules_dir).expect("Failed to read rules directory");
    let mut rule_files = Vec::new();

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md")
            && let Some(rule_name) = path.file_stem().and_then(|s| s.to_str())
        {
            rule_files.push(rule_name.to_string());
        }
    }

    assert!(
        !rule_files.is_empty(),
        "No rule markdown files found in {rules_dir}",
    );

    rule_files.sort();

    let mut failures = Vec::new();

    for rule_name in rule_files {
        let results = run_rule_tests(&rule_name);

        let snapshots_dir = format!("tests/snapshots/{rule_name}");

        fs::create_dir_all(&snapshots_dir)
            .unwrap_or_else(|_| panic!("Failed to create snapshots directory for {rule_name}"));

        for (temp_path, snippet_name, output) in results {
            let temp_filter = tempdir_filter(&temp_path);

            eprint!("test {rule_name}/{snippet_name}");

            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path(format!("snapshots/{rule_name}"));
            settings.add_filter(&temp_filter, "<temp_dir>/");

            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                settings.bind(|| {
                    insta::assert_snapshot!(snippet_name.clone(), output);
                });
            }));

            if let Err(err) = result {
                let error_message = err.downcast_ref::<String>().map_or_else(
                    || {
                        err.downcast_ref::<&str>()
                            .map_or_else(|| "Unknown panic".to_string(), |s| (*s).to_string())
                    },
                    Clone::clone,
                );
                failures.push(format!("{rule_name}/{snippet_name}: {error_message}"));
                eprintln!(" failed");
            } else {
                eprintln!(" passed");
            }
        }
    }

    assert!(
        failures.is_empty(),
        "Snapshot assertions failed:\n{}",
        failures.join("\n")
    );
}

fn tempdir_filter(path: &Path) -> String {
    format!(r"{}\\?/?", regex::escape(path.to_str().unwrap()))
}
