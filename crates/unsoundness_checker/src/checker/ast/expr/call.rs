use ruff_python_ast::{Expr, ExprCall};
use ty_python_semantic::HasType;
use ty_python_semantic::types::list_members::all_members;
use ty_python_semantic::types::{KnownFunction, Type, TypeContext};

use crate::Context;
use crate::rules::{report_invalid_setattr, report_typing_cast_used};

pub(super) fn check_call_expression(context: &Context, expr_call: &ExprCall) {
    if is_setattr_call(&expr_call.func) {
        check_setattr_call(context, expr_call);
        return;
    }

    let Some(func_ty) = expr_call.func.inferred_type(context.model()) else {
        return;
    };

    let Type::FunctionLiteral(function_type) = func_ty else {
        return;
    };

    if function_type.is_known(context.db(), KnownFunction::Cast) {
        let Some(casting_type) = expr_call
            .arguments
            .find_positional(0)
            .and_then(|arg| arg.inferred_type(context.model()))
        else {
            return;
        };

        let Some(value_type) = expr_call
            .arguments
            .find_positional(1)
            .and_then(|arg| arg.inferred_type(context.model()))
        else {
            return;
        };

        let current_promotion = casting_type.promote_literals(context.db(), TypeContext::default());

        if !value_type.is_assignable_to(context.db(), current_promotion) {
            report_typing_cast_used(context, &expr_call.func);
        }
    }
}

/// Checks if an expression is a call to `setattr()`
fn is_setattr_call(expr: &Expr) -> bool {
    match expr {
        Expr::Name(name_expr) => name_expr.id.as_str() == "setattr",
        _ => false,
    }
}

fn check_setattr_call(context: &Context, expr_call: &ExprCall) {
    let _ = check_setattr_call_inner(context, expr_call);
}

fn check_setattr_call_inner(context: &Context, expr_call: &ExprCall) -> Option<()> {
    let first_argument = expr_call.arguments.find_positional(0)?;
    let second_argument = expr_call.arguments.find_positional(1)?;
    let third_argument = expr_call.arguments.find_positional(2)?;

    let attr_name = second_argument.as_string_literal_expr()?;
    let first_ty = first_argument.inferred_type(context.model())?;

    let type_of_attribute = all_members(context.db(), first_ty)
        .iter()
        .find(|m| m.name == attr_name.value.to_str())
        .map(|m| m.ty)?;

    let promoted = type_of_attribute.promote_literals(context.db(), TypeContext::default());
    let value_type = third_argument.inferred_type(context.model())?;

    if !value_type.is_assignable_to(context.db(), promoted) {
        report_invalid_setattr(context, expr_call, promoted, value_type);
    }

    Some(())
}
