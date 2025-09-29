use ruff_db::{diagnostic::Diagnostic, files::File, parsed::parsed_module};
use ruff_python_ast::visitor::source_order::SourceOrderVisitor;
use ty_project::Db;

use crate::{Context, default_rule_registry, rule::RuleSelection};

mod annotation_checker;
mod ast_checker;
mod overload_checker;

pub(crate) use ast_checker::ASTChecker;

pub fn check_file(db: &dyn Db, file: File) -> Vec<Diagnostic> {
    let rule_registry = default_rule_registry();

    let context = Context::new(db, file, RuleSelection::from_registry(rule_registry));

    let mut ast_checker = ASTChecker::new(db, &context, file);

    let ast = parsed_module(db, file).load(db);

    ast_checker.visit_body(ast.suite());

    context.into_diagnostics()
}
