// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Counts that may only go down, held as DATA rather than as constants.
//!
//! A ratchet moves on every improvement, so a hand-typed one grows a changelog
//! beside it explaining each move -- history that git already holds, and churn
//! in every review. These live in `tests/ratchets.json` and are rewritten by
//! the test that measures them, under `DFE_UPDATE_RATCHETS=1`, the same way
//! `shapes.lock` is rewritten under `DFE_UPDATE_SHAPE_LOCK=1`.
//!
//! A FLOOR is the opposite and stays a `const`: it proves a scan was not
//! empty, moves only on a structural change, and carries its reason in a
//! comment that does not churn.
//!
//! These are counts over the GENERATED TREE, so they hold on a fresh clone
//! with no corpus. Corpus-scoped numbers belong in `tests/compat-baseline.json`
//! instead, which is skipped whenever the corpus provenance does not match.

// Every failure here aborts the test that called it, which is the point: a
// ratchet that cannot load, or one that rose, must stop the run rather than
// return an error a caller could drop. The module ships only under the
// `testutil` feature and has no production caller.
#![allow(clippy::panic, clippy::expect_used)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{Map, Value};

/// The environment variable that rewrites the file instead of asserting.
pub const UPDATE: &str = "DFE_UPDATE_RATCHETS";

/// Every ratchet, loaded once and written back whole.
pub struct Ratchets {
    path: PathBuf,
    values: Map<String, Value>,
    updating: bool,
    changed: Vec<String>,
}

impl Ratchets {
    /// Read `tests/ratchets.json`, relative to the workspace root.
    ///
    /// # Panics
    /// If the file is missing or unreadable. It is committed, and a test that
    /// silently skipped its own ratchet would be the very thing these exist to
    /// stop.
    #[must_use]
    pub fn load(workspace_root: &std::path::Path) -> Self {
        let path = workspace_root.join("tests/ratchets.json");
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let Ok(Value::Object(values)) = serde_json::from_str(&text) else {
            panic!("{} is not a JSON object", path.display())
        };
        Self {
            path,
            values,
            updating: std::env::var_os(UPDATE).is_some(),
            changed: Vec::new(),
        }
    }

    /// Assert `measured` has not risen above the recorded count, or record it.
    ///
    /// A FALL is recorded only under the update flag, so a run that improves
    /// something reports the new number rather than silently keeping the old.
    ///
    /// # Panics
    /// If the count rose and the update flag is not set, with `why` explaining
    /// what a rise means.
    pub fn check(&mut self, key: &str, measured: usize, why: &str) {
        let recorded = self.count(key);
        if measured == recorded {
            return;
        }
        assert!(
            self.updating || measured < recorded,
            "{key} rose from {recorded} to {measured}. {why}\n  \
             If the rise is correct and understood, re-run with {UPDATE}=1 and \
             say in the commit WHY the number got worse."
        );
        self.values.insert(key.to_string(), Value::from(measured));
        self.changed
            .push(format!("{key}: {recorded} -> {measured}"));
    }

    /// The recorded count for `key`.
    ///
    /// # Panics
    /// If the key is absent: a ratchet nothing records is not a ratchet.
    #[must_use]
    pub fn count(&self, key: &str) -> usize {
        let Some(value) = self.values.get(key).and_then(Value::as_u64) else {
            panic!("{} holds no count for {key}", self.path.display())
        };
        usize::try_from(value).unwrap_or(usize::MAX)
    }

    /// Assert a per-name table has not risen anywhere, or record it whole.
    ///
    /// The table exists so a fix in one source cannot be cancelled out by a
    /// regression in another and still pass a total.
    ///
    /// # Panics
    /// If any name rose, or a name with a count has no entry.
    pub fn check_table(&mut self, key: &str, measured: &BTreeMap<String, usize>, why: &str) {
        let recorded = self.table(key);
        if !self.updating {
            for (name, count) in measured {
                let against = recorded.get(name).copied().unwrap_or(0);
                assert!(
                    *count <= against,
                    "{key}[{name}] rose from {against} to {count}. {why}"
                );
            }
        }
        if recorded != *measured {
            let entries: Map<String, Value> = measured
                .iter()
                .map(|(name, count)| (name.clone(), Value::from(*count)))
                .collect();
            self.values.insert(key.to_string(), Value::Object(entries));
            self.changed
                .push(format!("{key}: {} names", measured.len()));
        }
    }

    /// The recorded table for `key`, empty when absent.
    #[must_use]
    pub fn table(&self, key: &str) -> BTreeMap<String, usize> {
        self.values
            .get(key)
            .and_then(Value::as_object)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|(name, count)| count.as_u64().map(|n| (name.clone(), n as usize)))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Write the file back if anything moved and the update flag is set.
    ///
    /// # Panics
    /// If the file cannot be written.
    pub fn save(&self) {
        if self.changed.is_empty() {
            return;
        }
        if !self.updating {
            println!("  ratchets that FELL, re-run with {UPDATE}=1 to record:");
            for line in &self.changed {
                println!("    {line}");
            }
            return;
        }
        let text = serde_json::to_string_pretty(&Value::Object(self.values.clone()))
            .expect("ratchets are counts and names");
        std::fs::write(&self.path, format!("{text}\n"))
            .unwrap_or_else(|e| panic!("write {}: {e}", self.path.display()));
        for line in &self.changed {
            println!("  recorded {line}");
        }
    }
}
