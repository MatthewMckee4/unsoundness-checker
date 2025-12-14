use ruff_python_ast::Expr;
use ruff_python_ast::visitor::source_order::{self, SourceOrderVisitor};
use ty_python_semantic::types::{DynamicType, Type};
use ty_python_semantic::{HasType, SemanticModel};

use crate::checker::Context;
use crate::rules::{report_callable_ellipsis_used, report_typing_any_used};

struct DynamicAnnotationChecker<'db, 'ctx> {
    context: &'ctx Context<'db>,

    model: &'db SemanticModel<'db>,
}

impl<'db, 'ctx> DynamicAnnotationChecker<'db, 'ctx> {
    pub(crate) const fn new(context: &'ctx Context<'db>, model: &'db SemanticModel<'db>) -> Self {
        Self { context, model }
    }
}

impl SourceOrderVisitor<'_> for DynamicAnnotationChecker<'_, '_> {
    fn visit_expr(&mut self, expr: &'_ Expr) {
        let ty = expr.inferred_type(self.model);

        if matches!(ty, Type::Dynamic(DynamicType::Any)) {
            report_typing_any_used(self.context, expr);
        }

        if matches!(ty, Type::Callable(_))
            && let Expr::Subscript(expr_subscript) = expr
            && let slice = expr_subscript.slice.as_ref()
            && let Expr::Tuple(tuple) = slice
            && let Some(first_element) = tuple.elts.first()
            && let Expr::EllipsisLiteral(_) = first_element
        {
            report_callable_ellipsis_used(self.context, expr);
        }

        source_order::walk_expr(self, expr);
    }
}

struct GenericAnnotationChecker<'db> {
    model: &'db SemanticModel<'db>,

    contains_generic: bool,
}

impl<'db> GenericAnnotationChecker<'db> {
    pub(crate) const fn new(model: &'db SemanticModel<'db>) -> Self {
        Self {
            model,
            contains_generic: false,
        }
    }
}

impl SourceOrderVisitor<'_> for GenericAnnotationChecker<'_> {
    fn visit_expr(&mut self, expr: &'_ Expr) {
        let ty = expr.inferred_type(self.model);

        if matches!(ty, Type::TypeVar(_)) {
            self.contains_generic = true;
        }

        source_order::walk_expr(self, expr);
    }
}

pub(super) fn check_annotation<'ast>(
    context: &Context<'_>,
    model: &'ast SemanticModel<'ast>,
    expr: &'ast Expr,
) {
    let mut annotation_checker = DynamicAnnotationChecker::new(context, model);

    annotation_checker.visit_expr(expr);
}

pub(super) fn is_generic_annotation(model: &SemanticModel<'_>, expr: &Expr) -> bool {
    let mut generic_checker = GenericAnnotationChecker::new(model);

    generic_checker.visit_expr(expr);

    generic_checker.contains_generic
}
