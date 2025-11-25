use ruff_db::{files::File, parsed::parsed_module};
use ruff_linter::Locator;
use ruff_python_index::Indexer;
use ty_project::Db;

use crate::{Context, rules::report_type_checking_directive_used};

/// Type checking directives from various Python type checkers
static TYPE_CHECKING_DIRECTIVES: &[&str] = &[
    // mypy / Standard (PEP 484)
    "type: ignore",
    // pyright
    "pyright: ignore",
    // ty
    "ty: ignore",
    // pyrefly
    "pyrefly: ignore",
];

pub fn check_tokens<'db>(db: &'db dyn Db, context: &Context<'db>, file: File) {
    tracing::debug!("Checking tokens");
    let Ok(file_content) = file.read_to_string(db) else {
        return;
    };

    let ast = parsed_module(db, file).load(db);

    let locator = Locator::new(&file_content);

    let indexer = Indexer::from_tokens(ast.tokens(), locator.contents());

    for range in indexer.comment_ranges() {
        let line = locator.line_str(range.start());

        // Check if the comment contains any type checking directive
        if let Some(directive) = find_type_checking_directive(line) {
            report_type_checking_directive_used(context, range, directive);
        }
    }
}

/// Finds a type checking directive in a comment line
fn find_type_checking_directive(line: &str) -> Option<&'static str> {
    // Find the comment start
    let comment_start = line.find("# ")?;
    let comment_content = &line[comment_start + 2..];

    // Check if any directive is present in the comment
    TYPE_CHECKING_DIRECTIVES
        .iter()
        .find(|&&directive| comment_content.starts_with(directive))
        .copied()
}
