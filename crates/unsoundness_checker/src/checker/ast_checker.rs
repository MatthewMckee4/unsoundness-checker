use ruff_db::{files::File, parsed::parsed_module};
use ruff_python_ast::{
    Expr, Stmt,
    visitor::source_order::{self, SourceOrderVisitor},
};
use ty_project::Db;
use ty_python_semantic::SemanticModel;

use crate::{
    Context,
    checker::{annotation_checker, overload_checker},
    rules::{report_setting_function_code_attribute, report_setting_function_defaults_attribute},
};

pub struct ASTChecker<'db, 'ctx> {
    context: &'ctx Context<'db>,

    model: SemanticModel<'db>,
}

impl<'db, 'ctx> ASTChecker<'db, 'ctx> {
    pub(crate) fn new(db: &'db dyn Db, context: &'ctx Context<'db>, file: File) -> Self {
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
                overload_checker::check_function_statement(
                    self.context,
                    &self.model,
                    stmt_function_def,
                );

                for parameter in &stmt_function_def.parameters {
                    if let Some(annotation) = parameter.annotation() {
                        annotation_checker::check_annotation(self.context, &self.model, annotation);
                    }
                }

                if let Some(return_annotation) = stmt_function_def.returns.as_ref() {
                    annotation_checker::check_annotation(
                        self.context,
                        &self.model,
                        return_annotation,
                    );
                }
            }
            Stmt::AnnAssign(stmt_ann_assign) => {
                let annotation = stmt_ann_assign.annotation.as_ref();

                annotation_checker::check_annotation(self.context, &self.model, annotation);
            }
            Stmt::Assign(stmt_assign) => {
                // Check if we're assigning to __defaults__ or __code__
                for target in &stmt_assign.targets {
                    if let Expr::Attribute(attr_expr) = target {
                        match attr_expr.attr.as_str() {
                            "__defaults__" => {
                                report_setting_function_defaults_attribute(self.context, target);
                            }
                            "__code__" => {
                                report_setting_function_code_attribute(self.context, target);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        source_order::walk_stmt(self, stmt);
    }
}

pub fn check_ast<'db>(db: &'db dyn Db, context: &Context<'db>, file: File) {
    let mut ast_checker = ASTChecker::new(db, context, file);

    let ast = parsed_module(db, file).load(db);

    ast_checker.visit_body(ast.suite());
}
