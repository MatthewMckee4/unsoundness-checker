use ruff_python_ast::Expr;

use crate::Context;

mod attribute;
mod call;

use attribute::check_attribute_expression;
use call::check_call_expression;

pub(super) fn check_expr(context: &Context, expr: &Expr) {
    match expr {
        Expr::Call(expr_call) => {
            check_call_expression(context, expr_call);
        }
        Expr::Attribute(attr_expr) => {
            check_attribute_expression(context, attr_expr);
        }
        _ => {}
    }
}
