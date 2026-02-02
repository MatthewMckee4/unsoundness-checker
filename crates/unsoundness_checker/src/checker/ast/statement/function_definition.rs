use ruff_python_ast::visitor::source_order::{self, SourceOrderVisitor};
use ruff_python_ast::{Stmt, StmtFunctionDef, StmtReturn};
use ty_python_semantic::types::{DynamicType, KnownFunction, Type, UnionBuilder};
use ty_python_semantic::{HasType, SemanticModel};

use crate::Context;
use crate::checker::ast::annotation::is_generic_annotation;
use crate::checker::ast::check_annotation;
use crate::checker::ast::utils::is_mutable_expr;
use crate::rules::{
    report_invalid_overload_implementation, report_mutable_generic_default,
    report_typing_overload_used, report_typing_type_is_used,
};

pub(super) fn check_overloads<'ast>(
    context: &Context,
    model: &'ast SemanticModel<'ast>,
    stmt_function_def: &StmtFunctionDef,
) {
    let function_ty = stmt_function_def.inferred_type(model);

    let Some(Type::FunctionLiteral(function_type_ty)) = function_ty else {
        return;
    };

    let (overloads, implementation) = function_type_ty.overloads_and_implementation(context.db());

    if implementation.is_some() {
        for overload in overloads {
            let overload_stmt = overload.node(context.db(), context.file(), context.ast());

            for decorator in &overload_stmt.decorator_list {
                let decorator_ty = decorator.expression.inferred_type(model);

                let Some(Type::FunctionLiteral(decorator_type_ty)) = decorator_ty else {
                    continue;
                };

                if decorator_type_ty.is_known(context.db(), KnownFunction::Overload) {
                    report_typing_overload_used(context, decorator);
                }
            }
        }
    }

    if overloads.is_empty() {
        return;
    }

    let overload_return_types = overloads
        .iter()
        .map(|overload| overload.signature(context.db()).return_ty)
        .collect::<Vec<_>>();

    if overload_return_types
        .iter()
        .any(|ty| matches!(ty, Type::Dynamic(DynamicType::Any | DynamicType::Unknown)))
    {
        return;
    }

    let union_of_overload_return_type = overload_return_types
        .iter()
        .fold(UnionBuilder::new(context.db()), |builder, ty| {
            builder.add(*ty)
        })
        .build();

    let return_statements = get_return_statements(stmt_function_def);

    for return_statement in return_statements {
        let Some(return_type) = return_statement
            .value
            .as_ref()
            .and_then(|value| value.inferred_type(model))
        else {
            continue;
        };

        let is_return_type_assignable_to_an_overload =
            return_type.is_assignable_to(model.db(), union_of_overload_return_type);

        if !is_return_type_assignable_to_an_overload {
            report_invalid_overload_implementation(
                context,
                return_statement,
                &return_type,
                &overload_return_types,
            );
        }
    }
}

fn get_return_statements(stmt_function_def: &StmtFunctionDef) -> Vec<&StmtReturn> {
    let mut return_statement_finder = ReturnStatementFinder::new();

    source_order::walk_body(&mut return_statement_finder, &stmt_function_def.body);

    return_statement_finder.return_statements
}

struct ReturnStatementFinder<'ast> {
    return_statements: Vec<&'ast StmtReturn>,
    inside_inner_function: bool,
}

impl ReturnStatementFinder<'_> {
    pub(crate) const fn new() -> Self {
        Self {
            return_statements: Vec::new(),
            inside_inner_function: false,
        }
    }
}

impl<'ast> SourceOrderVisitor<'ast> for ReturnStatementFinder<'ast> {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        match stmt {
            Stmt::Return(stmt_return) => {
                if !self.inside_inner_function {
                    self.return_statements.push(stmt_return);
                }
            }
            Stmt::FunctionDef(stmt_function_def) => {
                let previous = self.inside_inner_function;
                self.inside_inner_function = true;
                source_order::walk_body(self, &stmt_function_def.body);
                self.inside_inner_function = previous;
            }
            _ => {
                source_order::walk_stmt(self, stmt);
            }
        }
    }
}

pub(super) fn check_function_definition_statement(
    context: &Context,
    model: &SemanticModel,
    stmt_function_def: &StmtFunctionDef,
) {
    check_overloads(context, model, stmt_function_def);

    if let Some(return_type) = stmt_function_def.returns.as_ref() {
        let inferred_return_type = return_type.inferred_type(model);
        if let Some(Type::TypeIs(_)) = inferred_return_type {
            report_typing_type_is_used(context, return_type);
        }
    }

    for parameter in &stmt_function_def.parameters {
        if let Some(annotation) = parameter.annotation() {
            check_annotation(context, model, annotation);

            if let Some(default) = parameter.default()
                && is_mutable_expr(default)
                && is_generic_annotation(model, annotation)
            {
                report_mutable_generic_default(context, default);
            }
        }
    }

    if let Some(return_annotation) = stmt_function_def.returns.as_ref() {
        check_annotation(context, model, return_annotation);
    }
}
