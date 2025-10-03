use ruff_db::files::File;
use ruff_python_ast::{
    AnyParameterRef, Expr, ExprContext, ExprName, Stmt, StmtFunctionDef, StmtReturn,
    name::Name,
    visitor::source_order::{self, SourceOrderVisitor},
};
use ruff_text_size::{Ranged, TextRange, TextSize};
use ty_python_semantic::{
    Db, HasType, SemanticModel,
    semantic_index::semantic_index,
    types::{Type, infer_scope_types},
};

use crate::{Context, rules::report_invalid_overload_implementation};

pub(super) fn check_function_statement<'ast>(
    context: &Context,
    model: &'ast SemanticModel<'ast>,
    stmt_function_def: &StmtFunctionDef,
) {
    let function_ty = stmt_function_def.inferred_type(model);

    let Type::FunctionLiteral(function_type_ty) = function_ty else {
        return;
    };

    let (overloads, _) = function_type_ty.overloads_and_implementation(model.db());

    if overloads.is_empty() {
        return;
    }

    let overload_signatures = overloads
        .iter()
        .map(|overload| {
            overload.signature(
                model.db(),
                function_type_ty
                    .literal(model.db())
                    .inherited_generic_context(model.db()),
            )
        })
        .collect::<Vec<_>>();

    let overload_return_types = overload_signatures
        .iter()
        .map(|overload| overload.return_ty)
        .collect::<Vec<_>>();

    let return_statements = get_return_statements(stmt_function_def);

    let parameters = &stmt_function_def
        .parameters
        .iter()
        .map(AnyParameterRef::as_parameter)
        .collect::<Vec<_>>();

    for return_statement in return_statements {
        let return_type = return_statement
            .value
            .as_ref()
            .map(|value| value.inferred_type(model));

        let overload_matches_return_type =
            |overload_return_type: &Option<Type>| match (return_type, overload_return_type) {
                (Some(return_type), Some(overload_return_type)) => {
                    return_type.is_assignable_to(model.db(), *overload_return_type)
                }
                (None, None) => true,
                _ => false,
            };

        let is_return_type_assignable_to_an_overload = overload_return_types
            .iter()
            .any(&overload_matches_return_type);

        if !is_return_type_assignable_to_an_overload {
            report_invalid_overload_implementation(
                context,
                return_statement,
                return_type.as_ref(),
                &overload_return_types,
            );

            continue;
        }

        // Return type is assignable to at least one overload return type
        let _possible_overload_matches = overload_signatures
            .iter()
            .filter(|signature| overload_matches_return_type(&signature.return_ty))
            .collect::<Vec<_>>();
    }
}

fn inferred_type<'db>(
    db: &'db dyn Db,
    file: File,
    node_expr: &Expr,
    actual_expr: &Expr,
) -> Type<'db> {
    let index = semantic_index(db, file);
    let file_scope = index.expression_scope_id(node_expr);
    let scope = file_scope.to_scope_id(db, file);

    infer_scope_types(db, scope).expression_type(actual_expr)
}

fn get_return_statements(stmt_function_def: &StmtFunctionDef) -> Vec<&StmtReturn> {
    let mut return_statement_finder = ReturnStatementFinder::new();

    source_order::walk_body(&mut return_statement_finder, &stmt_function_def.body);

    return_statement_finder.return_statements
}

struct ReturnStatementFinder<'ast> {
    return_statements: Vec<&'ast StmtReturn>,
}

impl ReturnStatementFinder<'_> {
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            return_statements: Vec::new(),
        }
    }
}

impl<'ast> SourceOrderVisitor<'ast> for ReturnStatementFinder<'ast> {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if let Stmt::Return(stmt_return) = stmt {
            self.return_statements.push(stmt_return);
        }

        source_order::walk_stmt(self, stmt);
    }
}
