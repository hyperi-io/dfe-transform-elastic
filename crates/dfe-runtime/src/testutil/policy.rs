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
    unordered: Vec<String>,
}

/// One rule. `path` and `prefix` are the two spellings the file uses; both
/// match on a dotted path, so they are read into one field.
#[derive(Debug, Deserialize)]
struct Rule {
    #[serde(alias = "prefix")]
    path: String,
}

/// The comparison policy: what to skip, and which fields are sets.
#[derive(Debug, Default)]
pub struct Policy {
    /// Dotted paths whose differences are not defects, by prefix.
    pub skip: BTreeSet<String>,
    /// Dotted paths whose values are sets, so member order is not a difference.
    pub unordered: BTreeSet<String>,
}

impl Policy {
    /// Whether a dotted path is excluded from comparison.
    #[must_use]
    pub fn skips(&self, path: &str) -> bool {
        self.skip
            .iter()
            .any(|p| path == p || path.starts_with(&format!("{p}.")))
    }

    /// Whether a dotted path holds a set rather than a sequence.
    ///
    /// An array not named in the policy is compared IN ORDER, deliberately --
    /// that is the file's stated position, not an oversight.
    #[must_use]
    pub fn is_unordered(&self, path: &str) -> bool {
        self.unordered
            .iter()
            .any(|p| path == p || path.starts_with(&format!("{p}.")))
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
pub fn policy() -> &'static Policy {
    static LOADED: OnceLock<Policy> = OnceLock::new();
    LOADED.get_or_init(|| {
        #[allow(clippy::expect_used)]
        let text = std::fs::read_to_string(POLICY).unwrap_or_else(|e| panic!("read {POLICY}: {e}"));
        #[allow(clippy::expect_used)]
        let file: PolicyFile =
            serde_yaml_ng::from_str(&text).unwrap_or_else(|e| panic!("parse {POLICY}: {e}"));

        let skip = file
            .nondeterministic
            .iter()
            .chain(&file.not_emitted)
            .chain(&file.known_different)
            .map(|r| r.path.clone())
            .collect();

        Policy {
            skip,
            unordered: file.unordered.into_iter().collect(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_policy_file_parses_and_is_not_empty() {
        let policy = policy();
        assert!(policy.skip.len() > 20, "{} rules", policy.skip.len());
        assert!(!policy.unordered.is_empty());
    }

    /// The three groups all mean "not a defect here", so all three land in the
    /// skip set -- `known_different` is debt, but it is not a test failure.
    #[test]
    fn every_group_reaches_the_skip_set() {
        let policy = policy();
        assert!(policy.skips("event.ingested"), "nondeterministic");
        assert!(policy.skips("agent.version"), "not_emitted, by prefix");
        assert!(policy.skips("source.geo.country_name"), "known_different");
    }

    #[test]
    fn a_field_the_policy_does_not_name_is_compared() {
        assert!(!policy().skips("source.ip"));
        assert!(!policy().is_unordered("network.community_id"));
    }

    #[test]
    fn the_named_sets_are_unordered() {
        assert!(policy().is_unordered("event.category"));
        assert!(policy().is_unordered("related.ip"));
    }
}
