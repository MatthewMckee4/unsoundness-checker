use ruff_python_ast::{
    Expr,
    visitor::source_order::{self, SourceOrderVisitor},
};
use ty_python_semantic::{
    HasType, SemanticModel,
    types::{DynamicType, Type},
};

use crate::{checker::Context, rules::report_typing_any_used};

struct AnnotationChecker<'db, 'ctx> {
    context: &'ctx Context<'db>,

    model: &'db SemanticModel<'db>,
}

impl<'db, 'ctx> AnnotationChecker<'db, 'ctx> {
    #[must_use]
    pub(crate) const fn new(context: &'ctx Context<'db>, model: &'db SemanticModel<'db>) -> Self {
        Self { context, model }
    }
}

impl SourceOrderVisitor<'_> for AnnotationChecker<'_, '_> {
    fn visit_expr(&mut self, expr: &'_ Expr) {
        let ty = expr.inferred_type(self.model);

        if matches!(ty, Type::Dynamic(DynamicType::Any)) {
            report_typing_any_used(self.context, expr);
        }

        source_order::walk_expr(self, expr);
    }
}

pub(super) fn check_annotation<'ast>(
    context: &Context<'_>,
    model: &'ast SemanticModel<'ast>,
    expr: &'ast Expr,
) {
    let mut annotation_checker = AnnotationChecker::new(context, model);

    annotation_checker.visit_expr(expr);
}
