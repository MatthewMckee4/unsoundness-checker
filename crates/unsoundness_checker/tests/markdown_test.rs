use std::{fs, panic, path::Path};

use dir_test::{Fixture, dir_test};

pub mod common;

use common::run_rule_tests;

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/resources/rules",
    glob: "**/*.md"
)]
#[expect(clippy::needless_pass_by_value)]
fn test_all_rules_from_markdown(fixture: Fixture<&str>) {
    let rule_path = fixture.path();

    let rule_name = Path::new(rule_path).file_stem().unwrap().to_str().unwrap();

    let results = run_rule_tests(rule_name);

    let snapshots_dir = format!("tests/snapshots/{rule_name}");

    fs::create_dir_all(&snapshots_dir)
        .unwrap_or_else(|_| panic!("Failed to create snapshots directory for {rule_name}"));

    for (temp_path, snippet_name, output) in results {
        let temp_filter = tempdir_filter(&temp_path);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(format!("snapshots/{rule_name}"));
        settings.add_filter(&temp_filter, "<temp_dir>/");

        settings.bind(|| {
            insta::assert_snapshot!(snippet_name.clone(), output);
        });
    }
}

fn tempdir_filter(path: &Path) -> String {
    format!(r"{}\\?/?", regex::escape(path.to_str().unwrap()))
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/resources/rules",
    glob: "**/*.md"
)]
#[expect(clippy::needless_pass_by_value)]
fn test_rule_has_what_gets_flagged_heading(fixture: Fixture<&str>) {
    let rule_path = fixture.path();
    let content = fs::read_to_string(rule_path)
        .unwrap_or_else(|_| panic!("Failed to read rule file: {rule_path}"));

    assert!(
        content.contains("## What gets flagged"),
        "Rule file '{rule_path}' is missing the '## What gets flagged' heading"
    );
}
