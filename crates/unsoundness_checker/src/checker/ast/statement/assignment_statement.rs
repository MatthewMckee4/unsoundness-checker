use ruff_python_ast::name::Name;
use ruff_python_ast::{Expr, StmtAssign};
use ty_python_semantic::types::{Type, TypeContext};
use ty_python_semantic::{HasType, SemanticModel};

use crate::Context;
use crate::rules::{
    report_mutating_function_code_attribute, report_mutating_globals_dict,
    report_setting_function_defaults_attribute,
};

pub(super) fn check_assignment<'ast>(
    context: &Context<'_>,
    model: &'ast SemanticModel<'ast>,
    stmt_assign: &'ast StmtAssign,
) {
    for target in &stmt_assign.targets {
        match target {
            Expr::Subscript(subscript_expr) => {
                if is_globals_call(&subscript_expr.value)
                    && let Some(expr_string_literal) = subscript_expr.slice.as_string_literal_expr()
                {
                    let members = model.members_in_scope_at(stmt_assign.into());

                    if let Some(current_symbol_definition) =
                        members.get(&Name::new(expr_string_literal.value.to_str()))
                    {
                        let value_type = target.inferred_type(model);

                        let current_type = current_symbol_definition.ty;

                        // Try promote literal types like `Literal[1]`.
                        //
                        // A definition like `x = 1` gives x type `Literal[1]`. But we want to be able to assign `Literal[2]` to it.
                        let current_promotion =
                            current_type.promote_literals(model.db(), TypeContext::default());

                        if !value_type.is_assignable_to(context.db(), current_promotion) {
                            report_mutating_globals_dict(context, target);
                        }
                    }
                }
            }
            Expr::Attribute(attr_expr) => {
                let inferred_value_type = attr_expr.value.inferred_type(model);

                if let Type::FunctionLiteral(function_ty) = inferred_value_type {
                    match attr_expr.attr.as_str() {
                        "__defaults__" => {
                            let (_, Some(implementation)) =
                                function_ty.overloads_and_implementation(context.db())
                            else {
                                continue;
                            };

                            let signature = implementation.signature(context.db());

                            let annotated_types = signature.parameter_annotated_types();

                            let default_types = signature.parameter_default_types();

                            let default_types: Vec<_> = default_types
                                .iter()
                                .filter(|default_type| default_type.is_some())
                                .collect();

                            let inferred_target_type = target.inferred_type(model);

                            // Setting `__default__` to an object of type `None` on a function with no default parameters
                            // is fine as the current `__default__` is of type `tuple[None]`
                            // If there are some default parameters, we can emit an error now.
                            if inferred_target_type.is_none(context.db())
                                && !default_types.is_empty()
                            {
                                report_setting_function_defaults_attribute(
                                    context,
                                    target,
                                    &inferred_target_type,
                                );
                            }

                            let Some(tuple_spec) =
                                inferred_target_type.tuple_instance_spec(context.db())
                            else {
                                continue;
                            };

                            let target_tuple_elements: Vec<_> =
                                tuple_spec.fixed_elements().collect();

                            // Setting the default arguments to less than there already are can lead to runtime errors.
                            if target_tuple_elements.len() < default_types.len() {
                                report_setting_function_defaults_attribute(
                                    context,
                                    target,
                                    &inferred_target_type,
                                );
                                continue;
                            }

                            for (target_ty, annotated_ty) in
                                target_tuple_elements.iter().zip(annotated_types.iter())
                            {
                                let Some(annotated_ty) = annotated_ty else {
                                    continue;
                                };

                                // If we want to "replace" a default argument, it must be assignable to the annotated type.
                                if !target_ty.is_assignable_to(context.db(), *annotated_ty) {
                                    report_setting_function_defaults_attribute(
                                        context,
                                        target,
                                        &inferred_target_type,
                                    );
                                    break;
                                }
                            }
                        }
                        "__code__" => {
                            report_mutating_function_code_attribute(context, target);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

/// Checks if an expression is a call to `globals()`
fn is_globals_call(expr: &Expr) -> bool {
    match expr {
        Expr::Call(call_expr) => {
            if let Expr::Name(name_expr) = call_expr.func.as_ref() {
                name_expr.id.as_str() == "globals"
            } else {
                false
            }
        }
        _ => false,
    }
}
