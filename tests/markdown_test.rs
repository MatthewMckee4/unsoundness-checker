use std::fs;

mod common;

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

    for rule_name in rule_files {
        let results = run_rule_tests(&rule_name);

        let snapshots_dir = format!("tests/snapshots/{rule_name}");

        fs::create_dir_all(&snapshots_dir)
            .unwrap_or_else(|_| panic!("Failed to create snapshots directory for {rule_name}"));

        for (snippet_name, output) in results {
            insta::with_settings!({
                snapshot_path => format!("snapshots/{}", rule_name)
            }, {
                insta::assert_snapshot!(snippet_name.clone(), output);
            });
        }
    }
}
