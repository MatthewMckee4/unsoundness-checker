use ruff_python_ast::Stmt;
use ty_python_semantic::SemanticModel;

use super::annotation::check_annotation;
use crate::Context;

mod assignment_statement;
mod function_definition;
mod if_statement;

use assignment_statement::check_assignment;
use function_definition::check_function_definition_statement;
use if_statement::check_if_statement;

pub(super) fn check_statement(context: &Context, model: &SemanticModel, stmt: &Stmt) {
    match stmt {
        Stmt::FunctionDef(stmt_function_def) => {
            check_function_definition_statement(context, model, stmt_function_def);
        }
        Stmt::AnnAssign(stmt_ann_assign) => {
            check_annotation(context, model, stmt_ann_assign.annotation.as_ref());
        }
        Stmt::Assign(stmt_assign) => check_assignment(context, model, stmt_assign),
        Stmt::If(stmt_if) => check_if_statement(context, model, stmt_if),
        _ => {}
    }
}
