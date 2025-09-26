use ruff_db::files::File;
use ruff_python_ast::{Stmt, visitor::source_order::SourceOrderVisitor};
use ty_project::Db;
use ty_python_semantic::{
    HasType, SemanticModel,
    types::{DynamicType, Type},
};

use crate::{checker::context::Context, diagnostic::report_typing_any_used};

pub struct ASTChecker<'db, 'ctx> {
    _db: &'db dyn Db,

    context: &'ctx Context,

    model: SemanticModel<'db>,
}

impl<'db, 'ctx> ASTChecker<'db, 'ctx> {
    pub fn new(db: &'db dyn Db, context: &'ctx Context, file: File) -> Self {
        Self {
            _db: db,
            context,
            model: SemanticModel::new(db, file),
        }
    }
}

impl SourceOrderVisitor<'_> for ASTChecker<'_, '_> {
    fn visit_stmt(&mut self, stmt: &'_ Stmt) {
        #[allow(clippy::single_match)]
        match stmt {
            Stmt::FunctionDef(stmt_function_def) => {
                for parameter in &stmt_function_def.parameters {
                    let Some(annotation) = parameter.annotation() else {
                        continue;
                    };

                    let ty = annotation.inferred_type(&self.model);

                    if matches!(ty, Type::Dynamic(DynamicType::Any)) {
                        report_typing_any_used(self.context, annotation);
                    }
                }
            }
            _ => {}
        }
    }
}
