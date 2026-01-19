use ruff_db::diagnostic::Diagnostic;
use ruff_db::files::File;
use ty_project::{Db, ProjectDatabase};

use crate::Context;
use crate::checker::ast::check_ast;
use crate::checker::tokens::check_tokens;
use crate::rule::RuleSelection;

mod ast;
mod tokens;

pub fn check_file(db: &dyn Db, file: File, rule_selection: &RuleSelection) -> Vec<Diagnostic> {
    tracing::info!("Checking file: {}", file.path(db));

    let context = Context::new(db, file, rule_selection);

    check_ast(&context);

    check_tokens(&context);

    context.into_diagnostics()
}

pub fn check_project(db: &ProjectDatabase, rule_selection: &RuleSelection) -> Vec<Diagnostic> {
    use rayon::prelude::*;

    let check_start = std::time::Instant::now();

    let all_diagnostics: Vec<Diagnostic> = db
        .project()
        .files(db)
        .iter()
        .map(|&file| (file, db.clone()))
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|(file, db)| check_file(&db, file, rule_selection))
        .collect();

    tracing::debug!(
        "Checking all files took {:.3}s",
        check_start.elapsed().as_secs_f64(),
    );

    all_diagnostics
}
