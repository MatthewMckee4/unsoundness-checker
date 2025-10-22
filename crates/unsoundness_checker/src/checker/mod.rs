use ruff_db::{diagnostic::Diagnostic, files::File};
use ty_project::Db;

use crate::{
    Context,
    checker::{ast_checker::check_ast, tokens_checker::check_tokens},
    rule::RuleSelection,
};

mod annotation_checker;
mod assignment_checker;
mod ast_checker;
mod overload_checker;
mod tokens_checker;

pub fn check_file(db: &dyn Db, file: File, rule_selection: &RuleSelection) -> Vec<Diagnostic> {
    let context = Context::new(db, file, rule_selection);

    check_ast(db, &context, file);

    check_tokens(db, &context, file);

    context.into_diagnostics()
}
