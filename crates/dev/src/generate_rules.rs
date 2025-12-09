// MIT License
//
// Copyright (c) 2022 Charles Marsh
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{borrow::Cow, fmt::Write as _, fs, path::PathBuf};

use anyhow::{Result, bail};
use heck::ToSnakeCase;
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

        // Format categories with links to categories page
        let categories = if rule.categories.is_empty() {
            String::from("None")
        } else {
            rule.categories
                .iter()
                .map(|cat| format!("[`{}`](categories.md#{})", cat.name, cat.name))
                .join(", ")
        };

        let _ = writeln!(
            &mut output,
            r#"
{documentation}

<small>
Default level: `{level}`.
</small>

<small>
Categories: {categories}.
</small>

[See more](rules/{snake_case_name}.md)
"#,
            level = rule.default_level(),
            categories = categories,
            snake_case_name = rule.name().to_string().to_snake_case()
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
    #[cfg(unix)]
    fn rules_up_to_date() -> Result<()> {
        main(&Args { mode: Mode::Check })
    }
}
