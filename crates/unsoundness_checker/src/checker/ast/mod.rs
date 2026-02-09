use annotation::check_annotation;
use ruff_python_ast::visitor::source_order::{self, SourceOrderVisitor};
use ruff_python_ast::{Expr, Stmt};

use crate::Context;
use crate::checker::ast::expr::check_expr;
use crate::checker::ast::statement::check_statement;

mod annotation;
mod expr;
mod statement;
mod utils;

pub struct ASTChecker<'db, 'ctx> {
    context: &'ctx Context<'db>,
}

impl<'db, 'ctx> ASTChecker<'db, 'ctx> {
    pub(crate) const fn new(context: &'ctx Context<'db>) -> Self {
        Self { context }
    }
}

impl SourceOrderVisitor<'_> for ASTChecker<'_, '_> {
    fn visit_stmt(&mut self, stmt: &'_ Stmt) {
        check_statement(self.context, stmt);
        source_order::walk_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'_ Expr) {
        check_expr(self.context, expr);
        source_order::walk_expr(self, expr);
    }
}

pub fn check_ast(context: &Context) {
    let mut ast_checker = ASTChecker::new(context);

    ast_checker.visit_body(context.ast().suite());
}
