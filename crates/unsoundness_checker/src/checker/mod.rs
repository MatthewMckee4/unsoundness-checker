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
mod expr_checker;
mod function_checker;
mod if_checker;
mod tokens_checker;
mod utils;

pub fn check_file(db: &dyn Db, file: File, rule_selection: &RuleSelection) -> Vec<Diagnostic> {
    let context = Context::new(db, file, rule_selection);

    check_ast(db, &context, file);

    check_tokens(db, &context, file);

    context.into_diagnostics()
}
