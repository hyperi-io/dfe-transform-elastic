// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Test utilities for transform validation.
//!
//! Feature-gated behind `testutil`. Provides JSON flattening,
//! structured diff, and a test harness for comparing transform
//! output against expected fixtures.

pub mod diff;
pub mod flatten;
pub mod harness;
pub mod policy;

/// Run `body` on a thread with a stack deep enough for a generated transform.
///
/// A transform is ONE function per pipeline and its locals share a single
/// frame, so the largest of them -- `symantec_endpoint_security`'s 89,400
/// lines -- overflows the test harness thread's default stack in a debug
/// build. Any test that drives a transform over real fixtures needs this.
///
/// The panic is RESUMED rather than unwrapped, so a failure's own message
/// survives the hop instead of arriving as `Any { .. }`.
///
/// # Panics
///
/// Propagates whatever `body` panicked with, and panics if the thread cannot
/// be spawned.
#[allow(
    clippy::expect_used,
    reason = "a test that cannot start its own thread has nothing to report"
)]
pub fn on_a_deep_stack<F: FnOnce() + Send + 'static>(body: F) {
    const STACK: usize = 64 * 1024 * 1024;

    let finished = std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(body)
        .expect("spawn a deep-stacked test thread")
        .join();
    if let Err(panic) = finished {
        std::panic::resume_unwind(panic);
    }
}

pub use diff::{JsonDiff, MatchMode};
pub use flatten::{flatten_value, unflatten_value};
pub use harness::{
    load_expected_outputs, load_integration_events, load_integration_expected, load_test_events,
    run_integration_test, run_transform_test,
};
pub use policy::{Policy, policy};
