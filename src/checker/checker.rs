use ruff_python_ast::{Expr, visitor::source_order::SourceOrderVisitor};

use crate::checker::context::Context;

pub struct ASTChecker {
    context: Context,
}

impl ASTChecker {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

impl SourceOrderVisitor<'_> for ASTChecker {
    fn visit_expr(&mut self, expr: &'_ Expr) {}
}
