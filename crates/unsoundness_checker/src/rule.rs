use core::fmt;
use std::{fmt::Formatter, hash::Hasher};

use itertools::Itertools;
use ruff_db::diagnostic::{LintName, Severity};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct RuleMetadata {
    /// The unique identifier for the rule.
    pub(crate) name: LintName,

    /// A one-sentence summary of what the rule catches.
    pub summary: &'static str,

    /// An in depth explanation of the rule in markdown. Covers what the rule does, why it's bad and possible fixes.
    ///
    /// The documentation may require post-processing to be rendered correctly. For example, lines
    /// might have leading or trailing whitespace that should be removed.
    pub(crate) raw_documentation: &'static str,

    /// The default level of the rule if the user doesn't specify one.
    pub(crate) default_level: Level,

    pub(crate) status: RuleStatus,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Level {
    /// # Ignore
    ///
    /// The rule is disabled and should not run.
    Ignore,

    /// # Warn
    ///
    /// The rule is enabled and diagnostic should have a warning severity.
    Warn,

    /// # Error
    ///
    /// The rule is enabled and diagnostics have an error severity.
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ignore => f.write_str("ignore"),
            Self::Warn => f.write_str("warn"),
            Self::Error => f.write_str("error"),
        }
    }
}

impl TryFrom<Level> for Severity {
    type Error = ();

    fn try_from(level: Level) -> Result<Self, ()> {
        match level {
            Level::Ignore => Err(()),
            Level::Warn => Ok(Self::Warning),
            Level::Error => Ok(Self::Error),
        }
    }
}

impl RuleMetadata {
    #[must_use]
    pub const fn name(&self) -> LintName {
        self.name
    }

    /// Returns the documentation line by line with one leading space and all trailing whitespace removed.
    pub fn documentation_lines(&self) -> impl Iterator<Item = &str> {
        self.raw_documentation.lines().map(|line| {
            line.strip_prefix(char::is_whitespace)
                .unwrap_or(line)
                .trim_end()
        })
    }

    #[must_use]
    pub const fn default_level(&self) -> Level {
        self.default_level
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RuleStatus {
    /// The rule is stable.
    Stable {
        /// The version in which the rule was stabilized.
        since: &'static str,
    },

    /// The rule has been removed and can no longer be used.
    Removed {
        /// The version in which the rule was removed.
        since: &'static str,

        /// The reason why the rule has been removed.
        reason: &'static str,
    },
}

impl RuleStatus {
    pub(crate) const fn stable(since: &'static str) -> Self {
        Self::Stable { since }
    }

    pub(crate) const fn is_removed(&self) -> bool {
        matches!(self, Self::Removed { .. })
    }
}

#[macro_export]
macro_rules! declare_rule {
    (
        $(#[doc = $doc:literal])+
        $vis: vis static $name: ident = {
            summary: $summary: literal,
            status: $status: expr,
            // Optional properties
            $( $key:ident: $value:expr, )*
        }
    ) => {
        $( #[doc = $doc] )+
        $vis static $name: $crate::rule::RuleMetadata = $crate::rule::RuleMetadata {
            name: ruff_db::diagnostic::LintName::of(ruff_macros::kebab_case!($name)),
            summary: $summary,
            raw_documentation: concat!($($doc, '\n',)+),
            status: $status,
            $( $key: $value, )*
        };
    };
}

/// A unique identifier for a rule.
///
/// Implements `PartialEq`, `Eq`, and `Hash` based on the `RuleMetadata` pointer
/// for fast comparison and lookup.
#[derive(Debug, Clone, Copy)]
pub struct RuleId {
    definition: &'static RuleMetadata,
}

impl RuleId {
    pub(crate) const fn of(definition: &'static RuleMetadata) -> Self {
        Self { definition }
    }
}

impl PartialEq for RuleId {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.definition, other.definition)
    }
}

impl Eq for RuleId {}

impl std::hash::Hash for RuleId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.definition, state);
    }
}

impl std::ops::Deref for RuleId {
    type Target = RuleMetadata;

    fn deref(&self) -> &Self::Target {
        self.definition
    }
}

#[derive(Default, Debug)]
pub(crate) struct RuleRegistryBuilder {
    /// Registered rules that haven't been removed.
    rules: Vec<RuleId>,

    /// Rules indexed by name, including aliases and removed rules.
    by_name: FxHashMap<&'static str, RuleEntry>,
}

impl RuleRegistryBuilder {
    #[track_caller]
    pub(crate) fn register_rule(&mut self, rule: &'static RuleMetadata) {
        assert_eq!(
            self.by_name.insert(&*rule.name, rule.into()),
            None,
            "duplicate rule registration for '{name}'",
            name = rule.name
        );

        if !rule.status.is_removed() {
            self.rules.push(RuleId::of(rule));
        }
    }

    pub(crate) fn build(self) -> RuleRegistry {
        RuleRegistry { rules: self.rules }
    }
}

#[derive(Default, Debug, Clone)]
pub struct RuleRegistry {
    rules: Vec<RuleId>,
}

impl RuleRegistry {
    /// Returns all registered, non-removed rules.
    #[must_use]
    pub fn rules(&self) -> &[RuleId] {
        &self.rules
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum RuleEntry {
    /// An existing rule rule. Can be in preview, stable or deprecated.
    Rule(RuleId),
    /// A rule rule that has been removed.
    Removed(RuleId),
}

impl From<&'static RuleMetadata> for RuleEntry {
    fn from(metadata: &'static RuleMetadata) -> Self {
        if metadata.status.is_removed() {
            Self::Removed(RuleId::of(metadata))
        } else {
            Self::Rule(RuleId::of(metadata))
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub(crate) struct RuleSelection {
    /// Map with the severity for each enabled rule rule.
    ///
    /// If a rule isn't present in this map, then it should be considered disabled.
    rules: FxHashMap<RuleId, (Severity, RuleSource)>,
}

impl RuleSelection {
    /// Creates a new rule selection from all known rules in the registry that are enabled
    /// according to their default severity.
    pub(crate) fn from_registry(registry: &RuleRegistry) -> Self {
        Self::from_registry_with_default(registry, None)
    }

    fn from_registry_with_default(
        registry: &RuleRegistry,
        default_severity: Option<Severity>,
    ) -> Self {
        let rules = registry
            .rules()
            .iter()
            .filter_map(|rule| {
                Severity::try_from(rule.default_level())
                    .ok()
                    .or(default_severity)
                    .map(|severity| (*rule, (severity, RuleSource::Default)))
            })
            .collect();

        Self { rules }
    }

    pub(crate) fn get(&self, rule: RuleId) -> Option<(Severity, RuleSource)> {
        self.rules.get(&rule).copied()
    }
}

// The default `RuleId` debug implementation prints the entire rule metadata.
// This is way too verbose.
impl fmt::Debug for RuleSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rules = self.rules.iter().sorted_by_key(|(rule, _)| rule.name);

        if f.alternate() {
            let mut f = f.debug_map();

            for (rule, (severity, source)) in rules {
                f.entry(
                    &rule.name().as_str(),
                    &format_args!("{severity:?} ({source:?})"),
                );
            }

            f.finish()
        } else {
            let mut f = f.debug_set();

            for (rule, _) in rules {
                f.entry(&rule.name());
            }

            f.finish()
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum RuleSource {
    /// The user didn't enable the rule explicitly, instead it's enabled by default.
    #[default]
    Default,

    /// The rule was enabled by using a CLI argument
    #[expect(dead_code)]
    Cli,

    /// The rule was enabled in a configuration file.
    #[expect(dead_code)]
    File,
}
