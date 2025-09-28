use std::{borrow::Cow, fmt::Write as _, fs, path::PathBuf};

use anyhow::{Result, bail};
use itertools::Itertools as _;
use pretty_assertions::StrComparison;

use crate::{
    ROOT_DIR,
    generate_all::{Mode, REGENERATE_ALL_COMMAND},
};

#[derive(clap::Args)]
pub(crate) struct Args {
    #[arg(long, default_value_t, value_enum)]
    pub(crate) mode: Mode,
}

pub(crate) fn main(args: &Args) -> Result<()> {
    let markdown = generate_markdown();
    let filename = "docs/rules.md";
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
            let current = fs::read_to_string(&schema_path)?;
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

    let mut output = String::new();

    let _ = writeln!(&mut output, "# Rules\n");

    let mut rules: Vec<_> = registry.rules().iter().collect();
    rules.sort_by(|a, b| {
        a.default_level()
            .cmp(&b.default_level())
            .reverse()
            .then_with(|| a.name().cmp(&b.name()))
    });

    for rule in rules {
        let _ = writeln!(&mut output, "## `{rule_name}`\n", rule_name = rule.name());

        // Reformat headers as bold text
        let mut in_code_fence = false;
        let documentation = rule
            .documentation_lines()
            .map(|line| {
                // Toggle the code fence state if we encounter a boundary
                if line.starts_with("```") {
                    in_code_fence = !in_code_fence;
                }
                if !in_code_fence && line.starts_with('#') {
                    Cow::Owned(format!(
                        "**{line}**\n",
                        line = line.trim_start_matches('#').trim_start()
                    ))
                } else {
                    Cow::Borrowed(line)
                }
            })
            .join("\n");

        let _ = writeln!(
            &mut output,
            r#"<small>
Default level: [`{level}`](../rules.md#rule-levels "This lint has a default level of '{level}'.") ·
</small>

{documentation}
"#,
            level = rule.default_level(),
        );
    }

    output
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Args, main};
    use crate::generate_all::Mode;

    #[test]
    fn rules_up_to_date() -> Result<()> {
        main(&Args { mode: Mode::Check })
    }
}
