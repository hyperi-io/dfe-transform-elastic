// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How much Painless the runtime actually executes.
//!
//! [`crate::codegen_api::painless_exec`] runs the scripts it recognises and
//! skips the rest. Skipping is the right behaviour -- a script the runtime
//! cannot execute must not fail the event -- but until it was counted the
//! skips were invisible, and an invisible skip is indistinguishable from a
//! script that did nothing.
//!
//! Two tiers, so production pays for the first only:
//!
//! - **Counters** are two atomics, incremented on every call. Always on.
//! - **The catalogue** records which distinct scripts were skipped, and is off
//!   unless [`enable_catalogue`] is called. It takes a lock, so it is for
//!   tests and analysis, never the hot path.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{OnceLock, RwLock};

static HANDLED: AtomicU64 = AtomicU64::new(0);
static UNHANDLED: AtomicU64 = AtomicU64::new(0);
static CATALOGUE_ON: AtomicBool = AtomicBool::new(false);

/// How many characters of a script identify it in the catalogue.
///
/// Painless scripts share long preambles, so a short prefix would collapse
/// distinct scripts into one entry.
const CATALOGUE_KEY_LEN: usize = 200;

fn catalogue_store() -> &'static RwLock<HashMap<String, u64>> {
    static STORE: OnceLock<RwLock<HashMap<String, u64>>> = OnceLock::new();
    STORE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Record that a script was recognised and run.
pub fn record_handled() {
    HANDLED.fetch_add(1, Ordering::Relaxed);
}

/// Record that a script was skipped because nothing recognised it.
pub fn record_unhandled(script: &str) {
    UNHANDLED.fetch_add(1, Ordering::Relaxed);

    // A relaxed bool read is the whole cost when the catalogue is off, which
    // is the production case.
    if !CATALOGUE_ON.load(Ordering::Relaxed) {
        return;
    }
    let key: String = script.chars().take(CATALOGUE_KEY_LEN).collect();
    if let Ok(mut store) = catalogue_store().write() {
        *store.entry(key).or_insert(0) += 1;
    }
}

/// Scripts recognised and run since the last [`reset`].
#[must_use]
pub fn handled() -> u64 {
    HANDLED.load(Ordering::Relaxed)
}

/// Scripts skipped since the last [`reset`].
#[must_use]
pub fn unhandled() -> u64 {
    UNHANDLED.load(Ordering::Relaxed)
}

/// Every script the runtime saw, handled or not.
#[must_use]
pub fn total() -> u64 {
    handled() + unhandled()
}

/// Fraction of scripts the runtime actually executed, 0.0 to 1.0.
///
/// Returns 1.0 when nothing has run: a run that saw no Painless has skipped
/// none of it.
#[must_use]
pub fn coverage() -> f64 {
    let total = total();
    if total == 0 {
        return 1.0;
    }
    #[allow(clippy::cast_precision_loss)]
    {
        handled() as f64 / total as f64
    }
}

/// Start recording WHICH scripts are skipped, not just how many.
///
/// Takes a lock per skipped script, so this is for tests and analysis.
pub fn enable_catalogue(on: bool) {
    CATALOGUE_ON.store(on, Ordering::Relaxed);
}

/// The distinct skipped scripts and their counts, most frequent first.
///
/// Empty unless [`enable_catalogue`] was on while they were skipped.
#[must_use]
pub fn catalogue() -> Vec<(String, u64)> {
    let Ok(store) = catalogue_store().read() else {
        return Vec::new();
    };
    let mut entries: Vec<(String, u64)> = store
        .iter()
        .map(|(script, count)| (script.clone(), *count))
        .collect();
    entries.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    entries
}

/// Clear the counters and the catalogue.
pub fn reset() {
    HANDLED.store(0, Ordering::Relaxed);
    UNHANDLED.store(0, Ordering::Relaxed);
    if let Ok(mut store) = catalogue_store().write() {
        store.clear();
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// The counters are process-global, so the tests that read them share one
    /// lock rather than racing each other.
    fn serialised() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    #[test]
    fn counts_both_outcomes() {
        let _guard = serialised();
        reset();

        record_handled();
        record_handled();
        record_unhandled("script one");

        assert_eq!(handled(), 2);
        assert_eq!(unhandled(), 1);
        assert_eq!(total(), 3);
        assert!((coverage() - 2.0 / 3.0).abs() < f64::EPSILON);
    }

    /// A run that saw no Painless has not skipped any.
    #[test]
    fn coverage_of_nothing_is_complete() {
        let _guard = serialised();
        reset();
        assert!((coverage() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn the_catalogue_is_off_by_default() {
        let _guard = serialised();
        reset();
        enable_catalogue(false);

        record_unhandled("a script nobody handles");

        assert_eq!(unhandled(), 1, "the count is always on");
        assert!(catalogue().is_empty(), "the catalogue is not");
    }

    #[test]
    fn the_catalogue_groups_by_script_and_ranks_by_frequency() {
        let _guard = serialised();
        reset();
        enable_catalogue(true);

        record_unhandled("common script");
        record_unhandled("common script");
        record_unhandled("common script");
        record_unhandled("rare script");
        enable_catalogue(false);

        let entries = catalogue();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], ("common script".to_string(), 3));
        assert_eq!(entries[1], ("rare script".to_string(), 1));
    }

    /// Scripts sharing a long preamble must not collapse into one entry.
    #[test]
    fn long_scripts_are_distinguished_past_their_preamble() {
        let _guard = serialised();
        reset();
        enable_catalogue(true);

        let preamble = "x".repeat(CATALOGUE_KEY_LEN - 1);
        record_unhandled(&format!("{preamble}A tail that differs"));
        record_unhandled(&format!("{preamble}B tail that differs"));
        enable_catalogue(false);

        assert_eq!(catalogue().len(), 2);
    }

    #[test]
    fn reset_clears_both_tiers() {
        let _guard = serialised();
        reset();
        enable_catalogue(true);
        record_handled();
        record_unhandled("s");

        reset();
        enable_catalogue(false);

        assert_eq!(total(), 0);
        assert!(catalogue().is_empty());
    }
}
