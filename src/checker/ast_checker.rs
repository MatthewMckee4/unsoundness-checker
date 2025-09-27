use ruff_db::files::File;
use ruff_python_ast::{
    Stmt,
    visitor::source_order::{self, SourceOrderVisitor},
};
use ty_project::Db;
use ty_python_semantic::SemanticModel;

use crate::{Context, checker::AnnotationChecker};

pub struct ASTChecker<'db, 'ctx> {
    context: &'ctx Context,

    model: SemanticModel<'db>,
}

impl<'db, 'ctx> ASTChecker<'db, 'ctx> {
    pub fn new(db: &'db dyn Db, context: &'ctx Context, file: File) -> Self {
        Self {
            context,
            model: SemanticModel::new(db, file),
        }
    }
}

impl SourceOrderVisitor<'_> for ASTChecker<'_, '_> {
    fn visit_stmt(&mut self, stmt: &'_ Stmt) {
        match stmt {
            Stmt::FunctionDef(stmt_function_def) => {
                for parameter in &stmt_function_def.parameters {
                    let Some(annotation) = parameter.annotation() else {
                        continue;
                    };

                    let mut annotation_checker = AnnotationChecker::new(self.context, &self.model);

                    annotation_checker.visit_expr(annotation);
                }

                if let Some(return_annotation) = stmt_function_def.returns.as_ref() {
                    let mut annotation_checker = AnnotationChecker::new(self.context, &self.model);

                    annotation_checker.visit_expr(return_annotation);
                }

                source_order::walk_body(self, &stmt_function_def.body);
            }
            Stmt::AnnAssign(stmt_ann_assign) => {
                let annotation = stmt_ann_assign.annotation.as_ref();

                let mut annotation_checker = AnnotationChecker::new(self.context, &self.model);

                annotation_checker.visit_expr(annotation);
            }
            _ => {
                source_order::walk_stmt(self, stmt);
            }
        }
    }
}
