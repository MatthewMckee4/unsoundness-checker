use ruff_python_ast::{Expr, ExprAttribute, ExprCall};
use ty_python_semantic::{
    HasType, SemanticModel,
    types::{KnownFunction, Type, TypeContext, ide_support::all_members},
};

use crate::{
    Context,
    rules::{
        report_invalid_setattr, report_mangled_dunder_instance_variable, report_typing_cast_used,
    },
};

pub(super) fn check_expr<'ast>(
    context: &Context,
    model: &'ast SemanticModel<'ast>,
    expr: &'ast Expr,
) {
    match expr {
        Expr::Call(expr_call) => {
            if is_setattr_call(&expr_call.func) {
                check_setattr_call(context, model, expr_call);
                return;
            }

            let func_ty = expr_call.func.inferred_type(model);

            if let Type::FunctionLiteral(function_type) = func_ty {
                if function_type.is_known(context.db(), KnownFunction::Cast) {
                    let Some(first_argument) = expr_call.arguments.find_positional(0) else {
                        return;
                    };

                    let Some(second_argument) = expr_call.arguments.find_positional(1) else {
                        return;
                    };

                    let value_type = second_argument.inferred_type(model);

                    let casting_type = first_argument.inferred_type(model);

                    let current_promotion =
                        casting_type.promote_literals(model.db(), TypeContext::default());

                    if !value_type.is_assignable_to(context.db(), current_promotion) {
                        report_typing_cast_used(context, &expr_call.func);
                    }
                }
            }
        }
        Expr::Attribute(attr_expr) => {
            check_possibly_mangled_dunder_variable_access(context, model, attr_expr);
        }
        _ => {}
    }
}

fn check_possibly_mangled_dunder_variable_access(
    context: &Context,
    model: &SemanticModel,
    expr: &ExprAttribute,
) {
    let inferred_type = expr.value.inferred_type(model);
    let class_name = match inferred_type {
        // Possibly in an instance method where `self` is inferred as `Self@__init__` for example
        Type::TypeVar(inferred_value_type) => {
            let typevar = inferred_value_type.typevar(context.db());
            let Some(upper_bound) = typevar.upper_bound(context.db()) else {
                return;
            };

            upper_bound.display(context.db())
        }
        // Accessing the attribute via `Foo().<mangled_name>`
        Type::NominalInstance(_) => inferred_type.display(context.db()),
        _ => return,
    };

    let attr_name = expr.attr.as_str();
    if is_mangled_dunder_variable(attr_name, &class_name.to_string()) {
        report_mangled_dunder_instance_variable(context, expr, attr_name);
    }
}

/// Checks if an attribute name is a mangled dunder variable.
///
/// Python mangles double-underscore instance variables to `_ClassName__variable`.
/// This function detects explicit usage of the mangled form.
fn is_mangled_dunder_variable(attr_name: &str, class_name: &str) -> bool {
    // Pattern: _<ClassName>__<variable>
    // Must start with underscore followed by the class name, then __, then variable name
    let expected_prefix = format!("_{class_name}__");

    if !attr_name.starts_with(&expected_prefix) {
        return false;
    }

    // Ensure there's at least one character after the prefix (variable name)
    attr_name.len() > expected_prefix.len()
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
