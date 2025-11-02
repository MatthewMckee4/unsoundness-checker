use ruff_python_ast::Expr;
use ty_python_semantic::{
    HasType, SemanticModel,
    types::{KnownFunction, Type},
};

use crate::{Context, rules::report_typing_cast_used};

pub(super) fn check_expr<'ast>(
    context: &Context,
    model: &'ast SemanticModel<'ast>,
    expr: &'ast Expr,
) {
    #[allow(clippy::single_match)]
    match expr {
        Expr::Call(expr_call) => {
            let func_ty = expr_call.func.inferred_type(model);

            if let Type::FunctionLiteral(function_type) = func_ty {
                if function_type.is_known(context.db(), KnownFunction::Cast) {
                    report_typing_cast_used(context, &expr_call.func);
                }
            }
        }
        _ => {}
    }
}
