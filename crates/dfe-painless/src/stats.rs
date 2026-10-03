// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How much Painless the runtime actually executes.
//!
//! [`crate::plan::painless_exec`] runs the scripts it recognises and
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

/// Per script, how often it ran and how often it was skipped.
type Reach = HashMap<String, [u64; 2]>;

fn catalogue_store() -> &'static RwLock<Reach> {
    static STORE: OnceLock<RwLock<Reach>> = OnceLock::new();
    STORE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Record that a script was recognised and run.
pub fn record_handled(script: &str) {
    HANDLED.fetch_add(1, Ordering::Relaxed);
    tally(script, 0);
}

/// Record that a script was skipped because nothing recognised it.
pub fn record_unhandled(script: &str) {
    UNHANDLED.fetch_add(1, Ordering::Relaxed);
    tally(script, 1);
}

/// Count one outcome for a script: slot 0 ran, slot 1 was skipped.
///
/// Both outcomes are recorded because a skip count alone cannot tell a pattern
/// that NEVER applies from one that declines on the events missing its source
/// field. Only the first is a defect, and the difference is the ratio.
fn tally(script: &str, slot: usize) {
    // A relaxed bool read is the whole cost when the catalogue is off, which
    // is the production case.
    if !CATALOGUE_ON.load(Ordering::Relaxed) {
        return;
    }
    let key: String = script.chars().take(CATALOGUE_KEY_LEN).collect();
    if let Ok(mut store) = catalogue_store().write() {
        store.entry(key).or_insert([0, 0])[slot] += 1;
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
    reach()
        .into_iter()
        .filter(|(_, _, skipped)| *skipped > 0)
        .map(|(script, _, skipped)| (script, skipped))
        .collect()
}

/// Every catalogued script as `(script, ran, skipped)`, most skipped first.
///
/// A script with `ran == 0` was never once applied, which is the pattern a
/// matcher claims and cannot honour. One with both counts non-zero declines
/// only on some events, which is ordinary.
#[must_use]
pub fn reach() -> Vec<(String, u64, u64)> {
    let Ok(store) = catalogue_store().read() else {
        return Vec::new();
    };
    let mut entries: Vec<(String, u64, u64)> = store
        .iter()
        .map(|(script, counts)| (script.clone(), counts[0], counts[1]))
        .collect();
    entries.sort_unstable_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
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

/// The counters are process-global, so every test in the crate that records
/// or reads them takes this lock rather than racing the ones that count.
#[cfg(test)]
pub(crate) fn serialised() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn counts_both_outcomes() {
        let _guard = serialised();
        reset();

        record_handled("a script");
        record_handled("a script");
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

    /// A script that never once ran is a matcher claiming what it cannot
    /// honour; one that ran and sometimes declined is ordinary. A skip count
    /// alone cannot tell them apart, which is what `reach` is for.
    #[test]
    fn reach_separates_a_pattern_that_never_applies_from_one_that_sometimes_declines() {
        let _guard = serialised();
        reset();
        enable_catalogue(true);

        record_unhandled("never applies");
        record_unhandled("never applies");
        record_handled("declines sometimes");
        record_handled("declines sometimes");
        record_unhandled("declines sometimes");
        enable_catalogue(false);

        let reach = reach();
        assert_eq!(reach[0], ("never applies".to_string(), 0, 2));
        assert_eq!(reach[1], ("declines sometimes".to_string(), 2, 1));

        // The skip-only view keeps its pattern for the coverage test, which reads
        // it to rank what is worth teaching the runtime next.
        assert_eq!(catalogue().len(), 2);
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
        record_handled("a script");
        record_unhandled("s");

        reset();
        enable_catalogue(false);

        assert_eq!(total(), 0);
        assert_eq!(catalogue(), [] as [(std::string::String, u64); 0]);
    }
}
