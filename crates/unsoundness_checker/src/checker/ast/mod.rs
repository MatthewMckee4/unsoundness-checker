use annotation::check_annotation;
use ruff_db::{files::File, parsed::parsed_module};
use ruff_python_ast::{
    Expr, Stmt,
    visitor::source_order::{self, SourceOrderVisitor},
};
use ty_project::Db;
use ty_python_semantic::SemanticModel;

use crate::{
    Context,
    checker::ast::{expr::check_expr, statement::check_statement},
};

mod annotation;
mod expr;
mod statement;
mod utils;

pub struct ASTChecker<'db, 'ctx> {
    context: &'ctx Context<'db>,

    model: SemanticModel<'db>,
}

impl<'db, 'ctx> ASTChecker<'db, 'ctx> {
    pub(crate) fn new(db: &'db dyn Db, context: &'ctx Context<'db>, file: File) -> Self {
        Self {
            context,
            model: SemanticModel::new(db, file),
        }
    }
}

impl SourceOrderVisitor<'_> for ASTChecker<'_, '_> {
    fn visit_stmt(&mut self, stmt: &'_ Stmt) {
        check_statement(self.context, &self.model, stmt);
        source_order::walk_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'_ Expr) {
        check_expr(self.context, &self.model, expr);
        source_order::walk_expr(self, expr);
    }
}

pub fn check_ast<'db>(db: &'db dyn Db, context: &Context<'db>, file: File) {
    tracing::debug!("Checking ast");

    let mut ast_checker = ASTChecker::new(db, context, file);

    let ast = parsed_module(db, file).load(db);

    ast_checker.visit_body(ast.suite());
}
