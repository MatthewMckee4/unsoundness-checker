use ruff_python_ast::{Expr, ExprCall};
use ty_python_semantic::types::ide_support::all_members;
use ty_python_semantic::types::{KnownFunction, Type, TypeContext};
use ty_python_semantic::{HasType, SemanticModel};

use crate::Context;
use crate::rules::{report_invalid_setattr, report_typing_cast_used};

pub(super) fn check_call_expression(
    context: &Context,
    model: &SemanticModel,
    expr_call: &ExprCall,
) {
    if is_setattr_call(&expr_call.func) {
        check_setattr_call(context, model, expr_call);
        return;
    }

    let func_ty = expr_call.func.inferred_type(model);

    let Type::FunctionLiteral(function_type) = func_ty else {
        return;
    };

    if function_type.is_known(context.db(), KnownFunction::Cast) {
        let Some(first_argument) = expr_call.arguments.find_positional(0) else {
            return;
        };

        let Some(second_argument) = expr_call.arguments.find_positional(1) else {
            return;
        };

        let value_type = second_argument.inferred_type(model);

        let casting_type = first_argument.inferred_type(model);

        let current_promotion = casting_type.promote_literals(model.db(), TypeContext::default());

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

fn check_setattr_call(context: &Context, model: &SemanticModel, expr_call: &ExprCall) {
    let Some(first_argument) = expr_call.arguments.find_positional(0) else {
        return;
    };

    let Some(second_argument) = expr_call.arguments.find_positional(1) else {
        return;
    };

    let Some(third_argument) = expr_call.arguments.find_positional(2) else {
        return;
    };

    let Some(second_argument_string) = second_argument.as_string_literal_expr() else {
        return;
    };

    let first_ty = first_argument.inferred_type(model);

    let members = all_members(context.db(), first_ty);

    let Some(type_of_attribute) = members
        .iter()
        .find(|member| member.name == second_argument_string.value.to_str())
        .map(|member| member.ty)
    else {
        return;
    };

    let current_attribute_promotion =
        type_of_attribute.promote_literals(model.db(), TypeContext::default());

    let value_type = third_argument.inferred_type(model);

    if !value_type.is_assignable_to(context.db(), current_attribute_promotion) {
        report_invalid_setattr(context, expr_call, current_attribute_promotion, value_type);
    }
}
