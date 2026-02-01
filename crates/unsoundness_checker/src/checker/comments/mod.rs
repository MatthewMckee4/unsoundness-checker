use ruff_linter::Locator;
use ruff_python_index::Indexer;

use crate::Context;
use crate::rules::report_type_checking_directive_used;

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

pub fn check_comments(context: &Context) {
    let Ok(file_content) = context.file().read_to_string(context.db()) else {
        return;
    };

    let locator = Locator::new(&file_content);

    let indexer = Indexer::from_tokens(context.ast().tokens(), locator.contents());

    for range in indexer.comment_ranges() {
        let line = locator.slice(range);

        // Check if the comment contains any type checking directive
        if let Some(directive) = find_type_checking_directive(line) {
            report_type_checking_directive_used(context, range, directive);
        }
    }
}

/// Finds a type checking directive in a comment line
fn find_type_checking_directive(line: &str) -> Option<&'static str> {
    // Find the content of the comment.
    // It is assumed that all comments start with "# ".
    // But we be safe here and explicitly check for it.
    let comment_start = line.find("# ")?;
    let comment_content = &line[comment_start + 2..];

    let comment_content = comment_content.trim();

    // Check if any directive is present in the comment
    TYPE_CHECKING_DIRECTIVES
        .iter()
        .find(|&&directive| comment_content.starts_with(directive))
        .copied()
}
