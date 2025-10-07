use ruff_db::{diagnostic::Diagnostic, files::File, parsed::parsed_module};
use ruff_python_ast::visitor::source_order::SourceOrderVisitor;
use ty_project::Db;

use crate::{Context, rule::RuleSelection};

mod annotation_checker;
mod ast_checker;
mod overload_checker;

pub(crate) use ast_checker::ASTChecker;

pub fn check_file(db: &dyn Db, file: File, rule_selection: &RuleSelection) -> Vec<Diagnostic> {
    let context = Context::new(db, file, rule_selection);

    let mut ast_checker = ASTChecker::new(db, &context, file);

    let ast = parsed_module(db, file).load(db);

    ast_checker.visit_body(ast.suite());

    context.into_diagnostics()
}
