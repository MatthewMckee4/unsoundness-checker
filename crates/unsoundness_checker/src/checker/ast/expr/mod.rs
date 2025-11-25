use ruff_python_ast::Expr;
use ty_python_semantic::SemanticModel;

use crate::Context;

mod attribute;
mod call;

use attribute::check_attribute_expression;
use call::check_call_expression;

pub(super) fn check_expr<'ast>(
    context: &Context,
    model: &'ast SemanticModel<'ast>,
    expr: &'ast Expr,
) {
    match expr {
        Expr::Call(expr_call) => {
            check_call_expression(context, model, expr_call);
        }
        Expr::Attribute(attr_expr) => {
            check_attribute_expression(context, model, attr_expr);
        }
        _ => {}
    }
}
