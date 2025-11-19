pub use checker::check_file;

use crate::rule::{RuleRegistry, RuleRegistryBuilder};
pub(crate) use crate::{context::Context, rules::register_rules};

pub mod categories;
pub mod checker;
pub mod cli;
pub mod context;
pub mod rule;
pub mod rules;
pub mod version;

pub(crate) const NAME: &str = "Unsoundness Checker";

/// Returns the default registry with all known semantic rules.
pub fn default_rule_registry() -> &'static RuleRegistry {
    static REGISTRY: std::sync::LazyLock<RuleRegistry> = std::sync::LazyLock::new(|| {
        let mut registry = RuleRegistryBuilder::default();
        register_rules(&mut registry);
        registry.build()
    });

    &REGISTRY
}
