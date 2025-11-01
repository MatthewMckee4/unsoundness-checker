use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod generate_all;
mod generate_categories;
mod generate_rules;

const ROOT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[allow(clippy::enum_variant_names)]
enum Command {
    GenerateRules(generate_rules::Args),
    GenerateCategories(generate_categories::Args),
    GenerateAll,
}

fn main() -> Result<ExitCode> {
    let Args { command } = Args::parse();
    match command {
        Command::GenerateRules(args) => generate_rules::main(&args)?,
        Command::GenerateCategories(args) => generate_categories::main(&args)?,
        Command::GenerateAll => {
            generate_rules::main(&generate_rules::Args {
                mode: generate_all::Mode::Write,
            })?;
            generate_categories::main(&generate_categories::Args {
                mode: generate_all::Mode::Write,
            })?;
        }
    }
    Ok(ExitCode::SUCCESS)
}
