use ruff_python_ast::{Expr, StmtAssign};
use ty_python_semantic::{HasType, SemanticModel, types::Type};

use crate::{
    Context,
    rules::{report_setting_function_code_attribute, report_setting_function_defaults_attribute},
};

pub(super) fn check_assignment<'ast>(
    context: &Context<'_>,
    model: &'ast SemanticModel<'ast>,
    stmt_assign: &'ast StmtAssign,
) {
    for target in &stmt_assign.targets {
        if let Expr::Attribute(attr_expr) = target {
            let inferred_value_type = attr_expr.value.inferred_type(model);

            if let Type::FunctionLiteral(function_ty) = inferred_value_type {
                match attr_expr.attr.as_str() {
                    "__defaults__" => {
                        let (_, Some(implementation)) =
                            function_ty.overloads_and_implementation(context.db())
                        else {
                            continue;
                        };

                        let function_literal = function_ty.literal(context.db());

                        let signature = implementation.signature(
                            context.db(),
                            function_literal.inherited_generic_context(context.db()),
                        );

                        let annotated_types = signature.parameter_annotated_types();

                        let default_types = signature.parameter_default_types();

                        let default_types: Vec<_> = default_types
                            .iter()
                            .filter(|default_type| default_type.is_some())
                            .collect();

                        let inferred_target_type = target.inferred_type(model);

                        if inferred_target_type.is_none(context.db()) && !default_types.is_empty() {
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

                        let target_tuple_elements: Vec<_> = tuple_spec.fixed_elements().collect();

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
                        report_setting_function_code_attribute(context, target);
                    }
                    _ => {}
                }
            }
        }
    }
}
