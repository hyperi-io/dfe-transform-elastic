// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `tests/compare-policy.yaml`, as the comparison harness reads it.
//!
//! The file is the single definition of which differences are defects, shared
//! with `scripts/compat.py`. Two definitions drift, so this one is read rather
//! than restated -- a rule added for the compat corpus takes effect here on the
//! next run, and a rule removed stops hiding anything on both sides at once.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use serde::Deserialize;

/// Where the policy sits, relative to this crate.
const POLICY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/compare-policy.yaml"
);

#[derive(Debug, Deserialize)]
struct PolicyFile {
    #[serde(default)]
    nondeterministic: Vec<Rule>,
    #[serde(default)]
    not_emitted: Vec<Rule>,
    #[serde(default)]
    known_different: Vec<Rule>,
    #[serde(default)]
    corrected: Vec<Rule>,
    #[serde(default)]
    unordered: Vec<String>,
}

/// One rule. `path` and `prefix` are the two spellings the file uses; both
/// match on a dotted path, so they are read into one field.
#[derive(Debug, Deserialize)]
struct Rule {
    #[serde(alias = "prefix")]
    path: String,
    /// The sources this rule holds for. EMPTY means every source, which is
    /// right for a fact about the field itself -- `event.ingested` is
    /// nondeterministic everywhere. A rule that describes ONE source's quirk
    /// names it, or it blinds the other twenty-one: `event.kind` and
    /// `error.message` were excluded corpus-wide for a `cisco_nexus` date
    /// failure, so a transform emitting `pipeline_error` anywhere else did not
    /// show up as a difference at all.
    #[serde(default)]
    sources: Vec<String>,
}

impl Rule {
    /// Whether this rule covers a path, for a comparison that may or may not
    /// know which source it is looking at.
    ///
    /// A comparison with no source cannot honour scoping, so every rule
    /// applies -- the committed-fixture tests run that way and their floors
    /// depend on it.
    fn covers(&self, source: Option<&str>, path: &str) -> bool {
        let in_scope = self.sources.is_empty()
            || source.is_none_or(|name| self.sources.iter().any(|s| s == name));
        in_scope && prefixes(&self.path, path)
    }
}

/// Whether `prefix` is `path` or names one of its ancestors.
fn prefixes(prefix: &str, path: &str) -> bool {
    path.strip_prefix(prefix)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('.'))
}

/// The comparison policy: what to skip, and which fields are sets.
#[derive(Debug, Default)]
pub struct Policy {
    /// Paths whose differences are not defects, with the sources they hold for.
    skip: Vec<Rule>,
    /// Dotted paths whose values are sets, so member order is not a difference.
    ///
    /// Not scoped: these are facts about the FIELD -- `event.category` is a set
    /// in ECS whatever wrote it -- rather than one source's quirk.
    pub unordered: BTreeSet<String>,
}

impl Policy {
    /// Whether a dotted path is excluded from comparison for a given source.
    #[must_use]
    pub fn skips(&self, source: Option<&str>, path: &str) -> bool {
        self.skip.iter().any(|rule| rule.covers(source, path))
    }

    /// How many rules the file defines, for the parse check.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.skip.len()
    }

    /// Whether a dotted path holds a set rather than a sequence.
    ///
    /// An array not named in the policy is compared IN ORDER, deliberately --
    /// that is the file's stated position, not an oversight.
    #[must_use]
    pub fn is_unordered(&self, path: &str) -> bool {
        self.unordered.iter().any(|p| prefixes(p, path))
    }
}

/// The policy, parsed once per process.
///
/// # Panics
///
/// If the file is missing or malformed. A comparison harness running against
/// no policy would silently compare the wrong things, which is worse than a
/// failed test run.
#[must_use]
#[allow(
    clippy::panic,
    reason = "the panic is the contract stated above, and this is harness support behind the testutil feature rather than shipped code"
)]
pub fn policy() -> &'static Policy {
    static LOADED: OnceLock<Policy> = OnceLock::new();
    LOADED.get_or_init(|| {
        let text = std::fs::read_to_string(POLICY).unwrap_or_else(|e| panic!("read {POLICY}: {e}"));
        let file: PolicyFile =
            serde_yaml_ng::from_str(&text).unwrap_or_else(|e| panic!("parse {POLICY}: {e}"));

        let skip = file
            .nondeterministic
            .into_iter()
            .chain(file.not_emitted)
            .chain(file.known_different)
            .chain(file.corrected)
            .collect();

        Policy {
            skip,
            unordered: file.unordered.into_iter().collect(),
        }
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn the_policy_file_parses_and_is_not_empty() {
        let policy = policy();
        assert!(policy.rule_count() > 20, "{} rules", policy.rule_count());
        assert!(!policy.unordered.is_empty());
    }

    /// The three groups all mean "not a defect here", so all three land in the
    /// skip set -- `known_different` is debt, but it is not a test failure.
    #[test]
    fn every_group_reaches_the_skip_set() {
        let policy = policy();
        assert!(policy.skips(None, "event.ingested"), "nondeterministic");
        assert!(
            policy.skips(None, "agent.version"),
            "not_emitted, by prefix"
        );
        assert!(
            policy.skips(None, "source.geo.country_name"),
            "known_different"
        );
    }

    #[test]
    fn a_field_the_policy_does_not_name_is_compared() {
        assert!(!policy().skips(None, "source.ip"));
        assert!(!policy().is_unordered("network.community_id"));
    }

    /// A rule naming its sources holds for those and no others, and a rule
    /// naming none holds everywhere.
    #[test]
    fn a_scoped_rule_only_covers_the_sources_it_names() {
        let scoped = Rule {
            path: "event.kind".to_string(),
            sources: vec!["cisco_nexus".to_string()],
        };
        assert!(scoped.covers(Some("cisco_nexus"), "event.kind"));
        assert!(!scoped.covers(Some("gcp"), "event.kind"));
        assert!(scoped.covers(Some("cisco_nexus"), "event.kind.nested"));
        assert!(!scoped.covers(Some("cisco_nexus"), "event.kindly"));

        // A comparison that does not know its source cannot honour the scope.
        assert!(scoped.covers(None, "event.kind"));

        let global = Rule {
            path: "event.ingested".to_string(),
            sources: Vec::new(),
        };
        assert!(global.covers(Some("anything"), "event.ingested"));
    }

    #[test]
    fn the_named_sets_are_unordered() {
        assert!(policy().is_unordered("event.category"));
        assert!(policy().is_unordered("related.ip"));
    }
}
