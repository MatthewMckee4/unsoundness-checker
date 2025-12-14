use ruff_python_ast::{Expr, StmtIf};
use ty_python_semantic::SemanticModel;

use crate::Context;
use crate::rules::report_if_type_checking_used;

pub(super) fn check_if_statement<'ast>(
    context: &Context<'_>,
    _model: &'ast SemanticModel<'ast>,
    stmt_if: &'ast StmtIf,
) {
    if is_type_checking_test(&stmt_if.test) {
        report_if_type_checking_used(context, &stmt_if.test);
    }
}

/// Checks if an expression is a reference to `TYPE_CHECKING` from the typing module
fn is_type_checking_test(expr: &Expr) -> bool {
    match expr {
        // Direct reference: if TYPE_CHECKING:
        Expr::Name(name_expr) => name_expr.id.as_str() == "TYPE_CHECKING",

        // Attribute reference: if typing.TYPE_CHECKING:
        Expr::Attribute(attr_expr) => attr_expr.attr.as_str() == "TYPE_CHECKING",

        // Handle negation: if not TYPE_CHECKING:
        Expr::UnaryOp(unary_expr) => is_type_checking_test(&unary_expr.operand),

        _ => false,
    }
}
