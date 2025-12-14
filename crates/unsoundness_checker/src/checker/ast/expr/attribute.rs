use ruff_python_ast::ExprAttribute;
use ty_python_semantic::types::Type;
use ty_python_semantic::{HasType, SemanticModel};

use crate::Context;
use crate::rules::report_mangled_dunder_instance_variable;

pub(super) fn check_attribute_expression(
    context: &Context,
    model: &SemanticModel,
    attr_expr: &ExprAttribute,
) {
    check_possibly_mangled_dunder_variable_access(context, model, attr_expr);
}

fn check_possibly_mangled_dunder_variable_access(
    context: &Context,
    model: &SemanticModel,
    expr_attribute: &ExprAttribute,
) {
    let inferred_type = expr_attribute.value.inferred_type(model);
    let class_name = match inferred_type {
        // Possibly in an instance method where `self` is inferred as `Self@__init__` for example
        Type::TypeVar(inferred_value_type) => {
            let typevar = inferred_value_type.typevar(context.db());
            let Some(upper_bound) = typevar.upper_bound(context.db()) else {
                return;
            };

            upper_bound.display(context.db())
        }
        // Accessing the attribute via `Foo().<mangled_name>`
        Type::NominalInstance(_) => inferred_type.display(context.db()),
        _ => return,
    };

    let attr_name = expr_attribute.attr.as_str();
    if is_mangled_dunder_variable(attr_name, &class_name.to_string()) {
        report_mangled_dunder_instance_variable(context, expr_attribute, attr_name);
    }
}

/// Checks if an attribute name is a mangled dunder variable.
///
/// Python mangles double-underscore instance variables to `_ClassName__variable`.
/// This function detects explicit usage of the mangled form.
fn is_mangled_dunder_variable(attr_name: &str, class_name: &str) -> bool {
    // Pattern: _<ClassName>__<variable>
    // Must start with underscore followed by the class name, then __, then variable name
    let expected_prefix = format!("_{class_name}__");

    if !attr_name.starts_with(&expected_prefix) {
        return false;
    }

    // Ensure there's at least one character after the prefix (variable name)
    attr_name.len() > expected_prefix.len()
}
