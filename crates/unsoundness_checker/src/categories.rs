use crate::declare_category;

/// Metadata for an unsoundness category.
#[derive(Debug, Clone)]
pub struct CategoryMetadata {
    /// The unique identifier for the category (kebab-case).
    pub name: &'static str,

    /// An in-depth explanation of the category in markdown.
    pub documentation: &'static str,
}

declare_category! {
    /// Runtime code modifications that escape static type checker analysis.
    ///
    /// Examples: modifying `__code__`, `__defaults__`, or other runtime attributes
    /// that change behavior in ways type checkers cannot detect.
    pub(crate) static RUNTIME_MODIFICATION = {
        name: "runtime-modification",
    }
}

declare_category! {
    /// Mechanisms that suppress or bypass type checker warnings.
    ///
    /// Examples: `typing.Any`, `# type: ignore` directives, or other escape hatches
    /// that silence type checking without fixing underlying type issues.
    pub(crate) static TYPE_CHECKING_SUPPRESSION = {
        name: "type-checking-suppression",
    }
}

/// All registered categories.
pub static ALL_CATEGORIES: &[&CategoryMetadata] =
    &[&RUNTIME_MODIFICATION, &TYPE_CHECKING_SUPPRESSION];

#[macro_export]
macro_rules! declare_category {
    (
        $(#[doc = $doc:literal])+
        $vis: vis static $name: ident = {
            name: $cat_name: literal,
        }
    ) => {
        $( #[doc = $doc] )+
        $vis static $name: CategoryMetadata = CategoryMetadata {
            name: $cat_name,
            documentation: concat!($($doc, '\n',)+),
        };
    };
}
