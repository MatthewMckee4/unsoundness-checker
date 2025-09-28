use core::fmt;
use std::{fmt::Formatter, hash::Hasher};

use itertools::Itertools;
use ruff_db::diagnostic::{DiagnosticId, LintName, Severity};
use rustc_hash::FxHashMap;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct RuleMetadata {
    /// The unique identifier for the rule.
    pub name: LintName,

    /// A one-sentence summary of what the rule catches.
    pub summary: &'static str,

    /// An in depth explanation of the rule in markdown. Covers what the rule does, why it's bad and possible fixes.
    ///
    /// The documentation may require post-processing to be rendered correctly. For example, lines
    /// might have leading or trailing whitespace that should be removed.
    pub raw_documentation: &'static str,

    /// The default level of the rule if the user doesn't specify one.
    pub default_level: Level,

    pub status: RuleStatus,

    /// The source file in which the rule is declared.
    pub file: &'static str,
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

impl Level {
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }

    pub const fn is_warn(self) -> bool {
        matches!(self, Self::Warn)
    }

    pub const fn is_ignore(self) -> bool {
        matches!(self, Self::Ignore)
    }
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
    pub const fn name(&self) -> LintName {
        self.name
    }

    pub const fn summary(&self) -> &str {
        self.summary
    }

    /// Returns the documentation line by line with one leading space and all trailing whitespace removed.
    pub fn documentation_lines(&self) -> impl Iterator<Item = &str> {
        self.raw_documentation.lines().map(|line| {
            line.strip_prefix(char::is_whitespace)
                .unwrap_or(line)
                .trim_end()
        })
    }

    /// Returns the documentation as a single string.
    pub fn documentation(&self) -> String {
        self.documentation_lines().join("\n")
    }

    pub const fn default_level(&self) -> Level {
        self.default_level
    }

    pub const fn status(&self) -> &RuleStatus {
        &self.status
    }

    pub const fn file(&self) -> &str {
        self.file
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RuleStatus {
    /// The rule has been added to the ruleer, but is not yet stable.
    Preview {
        /// The version in which the rule was added.
        since: &'static str,
    },

    /// The rule is stable.
    Stable {
        /// The version in which the rule was stabilized.
        since: &'static str,
    },

    /// The rule is deprecated and no longer recommended for use.
    Deprecated {
        /// The version in which the rule was deprecated.
        since: &'static str,

        /// The reason why the rule has been deprecated.
        ///
        /// This should explain why the rule has been deprecated and if there's a replacement rule that users
        /// can use instead.
        reason: &'static str,
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
    pub const fn preview(since: &'static str) -> Self {
        Self::Preview { since }
    }

    pub const fn stable(since: &'static str) -> Self {
        Self::Stable { since }
    }

    pub const fn deprecated(since: &'static str, reason: &'static str) -> Self {
        Self::Deprecated { since, reason }
    }

    pub const fn removed(since: &'static str, reason: &'static str) -> Self {
        Self::Removed { since, reason }
    }

    pub const fn is_removed(&self) -> bool {
        matches!(self, Self::Removed { .. })
    }

    pub const fn is_deprecated(&self) -> bool {
        matches!(self, Self::Deprecated { .. })
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
            file: file!(),
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
    pub const fn of(definition: &'static RuleMetadata) -> Self {
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
pub struct RuleRegistryBuilder {
    /// Registered rules that haven't been removed.
    rules: Vec<RuleId>,

    /// Rules indexed by name, including aliases and removed rules.
    by_name: FxHashMap<&'static str, RuleEntry>,
}

impl RuleRegistryBuilder {
    #[track_caller]
    pub fn register_rule(&mut self, rule: &'static RuleMetadata) {
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

    #[track_caller]
    pub fn register_alias(&mut self, from: LintName, to: &'static RuleMetadata) {
        let target = match self.by_name.get(to.name.as_str()) {
            Some(RuleEntry::Rule(target) | RuleEntry::Removed(target)) => target,
            Some(RuleEntry::Alias(target)) => {
                panic!(
                    "rule alias {from} -> {to:?} points to another alias {target:?}",
                    target = target.name()
                )
            }
            None => panic!(
                "rule alias {from} -> {to} points to non-registered rule",
                to = to.name
            ),
        };

        assert_eq!(
            self.by_name
                .insert(from.as_str(), RuleEntry::Alias(*target)),
            None,
            "duplicate rule registration for '{from}'",
        );
    }

    pub fn build(self) -> RuleRegistry {
        RuleRegistry {
            rules: self.rules,
            by_name: self.by_name,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct RuleRegistry {
    rules: Vec<RuleId>,
    by_name: FxHashMap<&'static str, RuleEntry>,
}

impl RuleRegistry {
    /// Looks up a rule by its name.
    pub fn get(&self, code: &str) -> Result<RuleId, GetRuleError> {
        match self.by_name.get(code) {
            Some(RuleEntry::Rule(metadata)) => Ok(*metadata),
            Some(RuleEntry::Alias(rule)) => {
                if rule.status.is_removed() {
                    Err(GetRuleError::Removed(rule.name()))
                } else {
                    Ok(*rule)
                }
            }
            Some(RuleEntry::Removed(rule)) => Err(GetRuleError::Removed(rule.name())),
            None => {
                if let Some(without_prefix) = DiagnosticId::strip_category(code)
                    && let Some(entry) = self.by_name.get(without_prefix)
                {
                    return Err(GetRuleError::PrefixedWithCategory {
                        prefixed: code.to_string(),
                        suggestion: entry.id().name.to_string(),
                    });
                }

                Err(GetRuleError::Unknown(code.to_string()))
            }
        }
    }

    /// Returns all registered, non-removed rules.
    pub fn rules(&self) -> &[RuleId] {
        &self.rules
    }

    /// Returns an iterator over all known aliases and to their target rules.
    ///
    /// This iterator includes aliases that point to removed rules.
    pub fn aliases(&self) -> impl Iterator<Item = (LintName, RuleId)> + '_ {
        self.by_name.iter().filter_map(|(key, value)| {
            if let RuleEntry::Alias(alias) = value {
                Some((LintName::of(key), *alias))
            } else {
                None
            }
        })
    }

    /// Iterates over all removed rules.
    pub fn removed(&self) -> impl Iterator<Item = RuleId> + '_ {
        self.by_name.iter().filter_map(|(_, value)| {
            if let RuleEntry::Removed(metadata) = value {
                Some(*metadata)
            } else {
                None
            }
        })
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GetRuleError {
    /// The name maps to this removed rule.
    #[error("rule `{0}` has been removed")]
    Removed(LintName),

    /// No rule with the given name is known.
    #[error("unknown rule `{0}`")]
    Unknown(String),

    /// The name uses the full qualified diagnostic id `rule:<rule>` instead of just `rule`.
    /// The String is the name without the `rule:` category prefix.
    #[error("unknown rule `{prefixed}`. Did you mean `{suggestion}`?")]
    PrefixedWithCategory {
        prefixed: String,
        suggestion: String,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RuleEntry {
    /// An existing rule rule. Can be in preview, stable or deprecated.
    Rule(RuleId),
    /// A rule rule that has been removed.
    Removed(RuleId),
    Alias(RuleId),
}

impl RuleEntry {
    const fn id(self) -> RuleId {
        match self {
            Self::Rule(id) | Self::Removed(id) | Self::Alias(id) => id,
        }
    }
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
pub struct RuleSelection {
    /// Map with the severity for each enabled rule rule.
    ///
    /// If a rule isn't present in this map, then it should be considered disabled.
    rules: FxHashMap<RuleId, (Severity, RuleSource)>,
}

impl RuleSelection {
    /// Creates a new rule selection from all known rules in the registry that are enabled
    /// according to their default severity.
    pub fn from_registry(registry: &RuleRegistry) -> Self {
        Self::from_registry_with_default(registry, None)
    }

    /// Creates a new rule selection from all known rules in the registry, including rules that are default by default.
    /// Rules that are disabled by default use the `default_severity`.
    pub fn all(registry: &RuleRegistry, default_severity: Severity) -> Self {
        Self::from_registry_with_default(registry, Some(default_severity))
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

    /// Returns an iterator over all enabled rules.
    pub fn enabled(&self) -> impl Iterator<Item = RuleId> + '_ {
        self.rules.keys().copied()
    }

    /// Returns an iterator over all enabled rules and their severity.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (RuleId, Severity)> + '_ {
        self.rules
            .iter()
            .map(|(&rule, &(severity, _))| (rule, severity))
    }

    /// Returns the configured severity for the rule with the given id or `None` if the rule is disabled.
    pub fn severity(&self, rule: RuleId) -> Option<Severity> {
        self.rules.get(&rule).map(|(severity, _)| *severity)
    }

    pub fn get(&self, rule: RuleId) -> Option<(Severity, RuleSource)> {
        self.rules.get(&rule).copied()
    }

    /// Returns `true` if the `rule` is enabled.
    pub fn is_enabled(&self, rule: RuleId) -> bool {
        self.severity(rule).is_some()
    }

    /// Enables `rule` and configures with the given `severity`.
    ///
    /// Overrides any previous configuration for the rule.
    pub fn enable(&mut self, rule: RuleId, severity: Severity, source: RuleSource) {
        self.rules.insert(rule, (severity, source));
    }

    /// Disables `rule` if it was previously enabled.
    pub fn disable(&mut self, rule: RuleId) {
        self.rules.remove(&rule);
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
pub enum RuleSource {
    /// The user didn't enable the rule explicitly, instead it's enabled by default.
    #[default]
    Default,

    /// The rule was enabled by using a CLI argument
    Cli,

    /// The rule was enabled in a configuration file.
    File,
}
