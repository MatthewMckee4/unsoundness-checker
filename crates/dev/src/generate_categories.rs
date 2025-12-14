use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anyhow::{Result, bail};
use pretty_assertions::StrComparison;

use crate::ROOT_DIR;
use crate::generate_all::{Mode, REGENERATE_ALL_COMMAND};

#[derive(clap::Args)]
pub(crate) struct Args {
    #[arg(long, default_value_t, value_enum)]
    pub(crate) mode: Mode,
}

pub(crate) fn main(args: &Args) -> Result<()> {
    let markdown = generate_markdown();
    let filename = "docs/categories.md";
    let schema_path = PathBuf::from(ROOT_DIR).join(filename);

    match args.mode {
        Mode::DryRun => {
            println!("{markdown}");
        }
        Mode::Check => {
            let current = fs::read_to_string(schema_path)?;
            if current == markdown {
                println!("Up-to-date: {filename}");
            } else {
                let comparison = StrComparison::new(&current, &markdown);
                bail!("{filename} changed, please run `{REGENERATE_ALL_COMMAND}`:\n{comparison}");
            }
        }
        Mode::Write => {
            let current = fs::read_to_string(&schema_path).unwrap_or_default();
            if current == markdown {
                println!("Up-to-date: {filename}");
            } else {
                println!("Updating: {filename}");
                fs::write(schema_path, markdown.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn generate_markdown() -> String {
    let registry = unsoundness_checker::default_rule_registry();
    let categories = unsoundness_checker::categories::ALL_CATEGORIES;

    let mut output = String::new();

    let _ = writeln!(&mut output, "# Categories\n");
    let _ = writeln!(
        &mut output,
        "This page describes the different categories of type system unsoundness that the checker can detect.\n"
    );

    for category in categories {
        let _ = writeln!(&mut output, "## {}\n", category.name);

        // Write category documentation - strip leading whitespace from each line
        for line in category.documentation.lines() {
            let trimmed = line.trim_start();
            let _ = writeln!(&mut output, "{}", trimmed);
        }
        let _ = writeln!(&mut output);

        // Find all rules that belong to this category
        let rules_in_category: Vec<_> = registry
            .rules()
            .iter()
            .filter(|rule| {
                rule.categories
                    .iter()
                    .any(|cat| std::ptr::eq(*cat, *category))
            })
            .collect();

        if rules_in_category.is_empty() {
            let _ = writeln!(&mut output, "*No rules in this category.*\n");
        } else {
            let _ = writeln!(&mut output, "### Rules in this category\n");
            for rule in rules_in_category {
                let _ = writeln!(
                    &mut output,
                    "- [`{}`](rules.md#{}) - {}",
                    rule.name(),
                    rule.name(),
                    rule.summary
                );
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Args, main};
    use crate::generate_all::Mode;

    #[test]
    #[cfg(unix)]
    fn categories_up_to_date() -> Result<()> {
        main(&Args { mode: Mode::Check })
    }
}
