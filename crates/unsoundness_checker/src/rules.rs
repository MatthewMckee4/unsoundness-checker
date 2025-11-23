use ruff_python_ast::{Decorator, Expr, ExprAttribute, ExprCall, StmtReturn};
use ruff_text_size::Ranged;
use ty_python_semantic::types::Type;

use crate::{
    Context,
    categories::{RUNTIME_MODIFICATION, TYPE_CHECKING_SUPPRESSION},
    declare_rule,
    rule::{Level, RuleRegistryBuilder, RuleStatus},
};

pub(crate) fn register_rules(registry: &mut RuleRegistryBuilder) {
    registry.register_rule(&TYPING_ANY_USED);
    registry.register_rule(&INVALID_OVERLOAD_IMPLEMENTATION);
    registry.register_rule(&TYPING_OVERLOAD_USED);
    registry.register_rule(&TYPE_CHECKING_DIRECTIVE_USED);
    registry.register_rule(&IF_TYPE_CHECKING_USED);
    registry.register_rule(&INVALID_FUNCTION_DEFAULTS);
    registry.register_rule(&MUTATING_FUNCTION_CODE_ATTRIBUTE);
    registry.register_rule(&TYPING_CAST_USED);
    registry.register_rule(&MUTATING_GLOBALS_DICT);
    registry.register_rule(&TYPING_TYPE_IS_USED);
    registry.register_rule(&CALLABLE_ELLIPSIS_USED);
    registry.register_rule(&MUTABLE_GENERIC_DEFAULT);
    registry.register_rule(&MANGLED_DUNDER_INSTANCE_VARIABLE);
    registry.register_rule(&INVALID_SETATTR);
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of `typing.Any` in type annotations.
    ///
    /// ## Why is this bad?
    /// Using `typing.Any` in type annotations can lead to runtime errors.
    ///
    /// ## Examples
    /// ```python
    /// from typing import Any
    ///
    /// def foo(x: Any) -> Any:
    ///     return x + 1
    ///
    /// foo("1")
    /// ```
    pub (crate) static TYPING_ANY_USED = {
        summary: "detects usage of `typing.Any` in type annotations",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&TYPE_CHECKING_SUPPRESSION],
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of `...` (ellipsis) in the first argument of `Callable` type annotations.
    ///
    /// ## Why is this bad?
    /// Using `Callable[..., ReturnType]` indicates that the callable accepts any number
    /// of arguments of any type, which bypasses parameter type checking. This can lead to
    /// runtime type errors as the type checker cannot verify argument types or counts.
    ///
    /// ## Examples
    /// ```python
    /// from typing import Callable
    ///
    /// def foo(callback: Callable[..., int]) -> int:
    ///     return callback("wrong", "types")
    ///
    /// def bar(x: int) -> int:
    ///     return x
    ///
    /// # This passes type checking but fails at runtime.
    /// foo(bar)
    /// ```
    pub (crate) static CALLABLE_ELLIPSIS_USED = {
        summary: "detects usage of `...` in the first argument of `Callable` type annotations",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&TYPE_CHECKING_SUPPRESSION],
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for generic functions or methods that accept mutable objects as default parameter values.
    ///
    /// ## Why is this bad?
    /// When a generic function uses a mutable default value (like a list, dict, or set), that default
    /// is shared across all invocations of the function. This creates a scenario where the mutable
    /// object can accumulate values of different types from different calls.
    ///
    /// Type checkers assume that `list[T]` only contains values of type `T`. However, when a mutable
    /// default is reused across calls with different type parameters, the list can contain values of
    /// multiple different types, leading to runtime type errors.
    ///
    /// ## Examples
    /// ```python
    /// def append_and_return[T](x: T, items: list[T] = []) -> list[T]:
    ///     items.append(x)
    ///     return items
    ///
    /// int_list = append_and_return(42)
    /// str_list = append_and_return("hello")
    ///
    /// # This is a int at runtime but str at type check time.
    /// value: str = str_list[0]
    /// ```
    pub (crate) static MUTABLE_GENERIC_DEFAULT = {
        summary: "detects mutable default arguments in generic functions",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Error,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for invalid overload implementation.
    ///
    /// ## Why is this bad?
    /// Invalid overload implementation can lead to runtime errors.
    ///
    /// ## Examples
    /// ```python
    /// from typing import overload
    ///
    /// @overload
    /// def foo(x: int) -> str: ...
    /// @overload
    /// def foo(x: str) -> int: ...
    /// def foo(x: int | str) -> int | str:
    ///     return x
    ///
    /// foo("1")
    /// ```
    pub (crate) static INVALID_OVERLOAD_IMPLEMENTATION = {
        summary: "detects invalid overload implementation",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Error,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of overloaded functions.
    ///
    /// ## Why is this bad?
    /// Using overloaded functions can lead to runtime errors.
    /// When users don't follow the correct overload implementation, it can lead to unexpected behavior.
    ///
    /// ## Examples
    /// ```python
    /// from typing import overload
    ///
    /// @overload
    /// def foo(x: int) -> str: ...
    /// @overload
    /// def foo(x: str) -> int: ...
    /// def foo(x: int | str) -> int | str:
    ///     return x
    /// ```
    pub (crate) static TYPING_OVERLOAD_USED = {
        summary: "detects usage of overloaded functions",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of type checking directives in comments.
    ///
    /// ## Why is this bad?
    /// Type checking directives like `# type: ignore` suppress type checker warnings,
    /// which can hide potential type errors that may lead to runtime failures.
    /// These directives bypass the safety guarantees that type checking provides.
    ///
    /// ## Examples
    /// ```python
    /// # mypy / standard (PEP 484)
    /// x = "string" + 123  # type: ignore
    /// y = foo()  # type: ignore[attr-defined]
    /// ```
    pub (crate) static TYPE_CHECKING_DIRECTIVE_USED = {
        summary: "detects usage of type checking directives in comments",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&TYPE_CHECKING_SUPPRESSION],
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of `if TYPE_CHECKING:` blocks from the `typing` module.
    ///
    /// ## Why is this bad?
    /// `TYPE_CHECKING` is `False` at runtime but `True` during static type checking.
    /// When used with an `else` clause where signatures don't match, the type checker
    /// validates against the `if TYPE_CHECKING` branch, but at runtime the `else` branch
    /// executes, causing runtime type errors that the type checker can't catch.
    ///
    /// ## Examples
    /// ```python
    /// from typing import TYPE_CHECKING
    ///
    /// if TYPE_CHECKING:
    ///     def get_value() -> int:
    ///         ...
    /// else:
    ///     def get_value() -> str:
    ///         return "hello"
    ///
    /// result: int = get_value()  # Type checks, but returns str at runtime!
    /// ```
    pub (crate) static IF_TYPE_CHECKING_USED = {
        summary: "detects usage of `if TYPE_CHECKING:` blocks",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&TYPE_CHECKING_SUPPRESSION],
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for invalid mutations of the `__defaults__` attribute of a function.
    ///
    /// ## Why is this bad?
    /// Modifying the `__defaults__` attribute with types different to the parameters
    /// can lead to runtime type errors.
    ///
    /// ## Examples
    /// ```python
    /// def foo(x: int = 1) -> int:
    ///     return x
    ///
    /// foo.__defaults__ = ("string",)
    /// result = foo()  # Returns "string" but type checker thinks it's int
    /// ```
    pub (crate) static INVALID_FUNCTION_DEFAULTS = {
        summary: "detects invalid mutation of the `__defaults__` attribute of a function",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&RUNTIME_MODIFICATION],
        default_level: Level::Error,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for mutating the `__code__` attribute of a function.
    ///
    /// ## Why is this bad?
    /// Modifying the `__code__` attribute allows runtime modification
    /// of function internals, which can bypass type checking and lead to runtime type errors.
    /// Type checkers cannot analyze or verify operations performed through code objects.
    ///
    /// ## Examples
    /// ```python
    /// def foo(x: int) -> int:
    ///     return 1
    ///
    /// def bar(x: str) -> str:
    ///     return "bar"
    ///
    /// foo.__code__ = bar.__code__
    /// # Now foo will return a string
    /// ```
    pub (crate) static MUTATING_FUNCTION_CODE_ATTRIBUTE = {
        summary: "detects mutating the `__code__` attribute of a function",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&RUNTIME_MODIFICATION],
        default_level: Level::Error,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of `typing.cast()` function calls.
    ///
    /// ## Why is this bad?
    /// `typing.cast()` tells the type checker to treat a value as a specific type
    /// without any runtime checks or validation. This can lead to runtime type errors
    /// if the cast is incorrect. Type checkers trust casts completely, so incorrect
    /// casts bypass all type safety guarantees.
    ///
    /// ## Examples
    /// ```python
    /// from typing import cast
    ///
    /// def get_value() -> int | str:
    ///     return "hello"
    ///
    /// result = cast(int, get_value())
    /// result + 1  # Type checks, but fails at runtime!
    /// ```
    pub (crate) static TYPING_CAST_USED = {
        summary: "detects usage of `typing.cast()` function calls",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&TYPE_CHECKING_SUPPRESSION],
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for mutations to the `globals()` dictionary.
    ///
    /// ## Why is this bad?
    /// Modifying the `globals()` dictionary allows runtime modification
    /// of global variables, which can bypass type checking and lead to runtime type errors.
    /// Type checkers cannot track or verify modifications made through the globals dictionary.
    ///
    /// ## Examples
    /// ```python
    /// x: int = 5
    ///
    /// globals()['x'] = "hello"
    ///
    /// # Type checker thinks `x` is an `int`, but it is now a string
    /// result: int = x
    /// ```
    pub (crate) static MUTATING_GLOBALS_DICT = {
        summary: "detects mutations to the `globals()` dictionary",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&RUNTIME_MODIFICATION],
        default_level: Level::Error,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for return types that use `typing.TypeIs`.
    ///
    /// ## Why is this bad?
    /// Using `typing.TypeIs` in return type annotations can lead to runtime type errors.
    /// Type checkers use `TypeIs` to narrow types, but incorrect implementation can bypass
    /// type safety guarantees.
    ///
    /// ## Examples
    /// ```python
    /// from typing import TypeIs
    ///
    /// def is_int(x: object) -> TypeIs[int]:
    ///     return True
    ///
    /// value = "hello"
    ///
    /// if is_int(value):
    ///     result = value + 1  # Type checks but fails at runtime!
    /// ```
    pub (crate) static TYPING_TYPE_IS_USED = {
        summary: "detects usage of `typing.TypeIs` in return type annotations",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&TYPE_CHECKING_SUPPRESSION],
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for explicit usage of mangled dunder instance variables in attribute access.
    ///
    /// ## Why is this bad?
    /// Python automatically mangles double-underscore (dunder) instance variables to
    /// `_ClassName__variable` to provide name privacy. When code explicitly uses the
    /// mangled form, it can bypass type checking by assigning different types to the
    /// mangled name than what the non-mangled variable expects.
    ///
    /// ## Examples
    /// ```python
    /// class HiddenDunderVariables:
    ///     def __init__(self, x: int) -> None:
    ///         self.__str_x = str(x)
    ///         self._HiddenDunderVariables__str_x = x
    ///
    ///     def get_str_x(self) -> str:
    ///         return self.__str_x
    ///
    /// # Here, x is a string at type check time, but an integer at runtime.
    /// x = hidden_dunder_variables.get_str_x()
    /// ```
    pub (crate) static MANGLED_DUNDER_INSTANCE_VARIABLE = {
        summary: "detects explicit usage of mangled dunder instance variables",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&TYPE_CHECKING_SUPPRESSION],
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for invalid `setattr()` usage.
    ///
    /// ## Why is this bad?
    /// `setattr()` bypasses type checking by allowing "dynamic" attribute assignment.
    /// You can assign any type to any attribute, which can lead to runtime type errors
    /// when the actual type doesn't match the declared type annotation.
    ///
    /// ## Examples
    /// ```python
    /// class Foo:
    ///     def __init__(self) -> None:
    ///         self.x: str = "hello"
    ///
    /// foo = Foo()
    /// setattr(foo, "x", 1)
    /// ```
    pub (crate) static INVALID_SETATTR = {
        summary: "detects invalid usage of `setattr()` built-in function",
        status: RuleStatus::stable("1.0.0"),
        categories: &[&RUNTIME_MODIFICATION],
        default_level: Level::Warn,
    }
}

pub(crate) fn report_typing_any_used(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&TYPING_ANY_USED, expr.range()) else {
        return;
    };

    builder.into_diagnostic("Using `typing.Any` in type annotations can lead to runtime errors.");
}

pub(crate) fn report_invalid_overload_implementation<'db>(
    context: &Context<'db>,
    return_statement: &StmtReturn,
    return_type: Option<&'db Type<'db>>,
    overload_return_types: &[Option<Type<'db>>],
) {
    let Some(builder) =
        context.report_lint(&INVALID_OVERLOAD_IMPLEMENTATION, return_statement.range())
    else {
        return;
    };

    let mut diagnostic =
        builder.into_diagnostic("Invalid overload implementation can lead to runtime errors.");

    let get_return_type_display = |ty: Option<&Type<'_>>| {
        format!(
            "`{}`",
            ty.map_or_else(
                || "None".to_string(),
                |ty| ty.display(context.db()).to_string(),
            )
        )
    };

    let return_type_display = get_return_type_display(return_type);

    let overload_return_types_display = overload_return_types
        .iter()
        .map(|ty| get_return_type_display(ty.as_ref()))
        .collect::<Vec<_>>()
        .join(", ");

    diagnostic.info(format_args!(
        "This overload implementation is invalid as {return_type_display} is not \
        assignable to any of the overload return types ({overload_return_types_display})"
    ));
}

pub(crate) fn report_typing_overload_used(context: &Context, decorator: &Decorator) {
    let Some(builder) = context.report_lint(&TYPING_OVERLOAD_USED, decorator.range()) else {
        return;
    };

    builder.into_diagnostic("Using `typing.overload` can lead to runtime errors.");
}

pub(crate) fn report_type_checking_directive_used(
    context: &Context,
    range: ruff_text_size::TextRange,
    directive: &str,
) {
    let Some(builder) = context.report_lint(&TYPE_CHECKING_DIRECTIVE_USED, range) else {
        return;
    };

    builder.into_diagnostic(format!(
        "Type checking directive `{directive}` suppresses type checker warnings, which may hide potential type errors.",
    ));
}

pub(crate) fn report_setting_function_defaults_attribute(
    context: &Context,
    expr: &Expr,
    new_defaults: &Type,
) {
    let Some(builder) = context.report_lint(&INVALID_FUNCTION_DEFAULTS, expr.range()) else {
        return;
    };

    builder.into_diagnostic(format!(
        "Setting `__defaults__` to an object of type `{}` on a function may lead to runtime type errors.",
        new_defaults.display(context.db())
    ));
}

pub(crate) fn report_mutating_function_code_attribute(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&MUTATING_FUNCTION_CODE_ATTRIBUTE, expr.range()) else {
        return;
    };

    builder.into_diagnostic(
        "Mutating `__code__` attribute on a function may lead to runtime type errors.",
    );
}

pub(crate) fn report_if_type_checking_used(context: &Context, if_typing_checking_expr: &Expr) {
    let Some(builder) =
        context.report_lint(&IF_TYPE_CHECKING_USED, if_typing_checking_expr.range())
    else {
        return;
    };

    builder.into_diagnostic(
        "Using `if TYPE_CHECKING:` blocks can lead to runtime errors if imports or definitions are incorrectly referenced at runtime.",
    );
}

pub(crate) fn report_typing_cast_used(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&TYPING_CAST_USED, expr.range()) else {
        return;
    };

    let mut diagnostic = builder.into_diagnostic(
        "Using `typing.cast()` bypasses type checking and can lead to runtime type errors.",
    );

    diagnostic.info("Consider using `isinstance` checks to ensure types at runtime.");
}

pub(crate) fn report_mutating_globals_dict(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&MUTATING_GLOBALS_DICT, expr.range()) else {
        return;
    };

    builder.into_diagnostic("Mutating the `globals()` dictionary may lead to runtime type errors.");
}

pub(crate) fn report_typing_type_is_used(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&TYPING_TYPE_IS_USED, expr.range()) else {
        return;
    };

    builder.into_diagnostic("Using `typing.TypeIs` can lead to runtime type errors.");
}

pub(crate) fn report_callable_ellipsis_used(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&CALLABLE_ELLIPSIS_USED, expr.range()) else {
        return;
    };

    builder.into_diagnostic(
        "Using `...` in `Callable` type annotations can lead to runtime type errors.",
    );
}

pub(crate) fn report_mutable_generic_default(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&MUTABLE_GENERIC_DEFAULT, expr.range()) else {
        return;
    };

    builder.into_diagnostic(
        "Using a mutable default argument for a generic parameter in a function can lead to runtime type errors.",
    );
}

pub(crate) fn report_mangled_dunder_instance_variable(
    context: &Context,
    expr: &ExprAttribute,
    attr_name: &str,
) {
    let Some(builder) = context.report_lint(&MANGLED_DUNDER_INSTANCE_VARIABLE, expr.range()) else {
        return;
    };

    builder.into_diagnostic(format!(
        "Explicit use of mangled attribute `{attr_name}` can bypass type checking and lead to runtime type errors.",
    ));
}

pub(crate) fn report_invalid_setattr(
    context: &Context,
    expr: &ExprCall,
    original_ty: Type<'_>,
    new_type: Type<'_>,
) {
    let Some(builder) = context.report_lint(&INVALID_SETATTR, expr.range()) else {
        return;
    };

    let mut diagnostic = builder.into_diagnostic(
        "Using `setattr()` bypasses type checking and can lead to runtime type errors.",
    );

    diagnostic.info(format!(
        "Object of type {:?} is not assignable to type {:?}",
        original_ty.display(context.db()),
        new_type.display(context.db())
    ));
}
