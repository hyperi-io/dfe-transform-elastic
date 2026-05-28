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

pub use diff::{JsonDiff, MatchMode};
pub use flatten::{flatten_value, unflatten_value};
pub use harness::{
    load_expected_outputs, load_integration_events, load_integration_expected, load_test_events,
    run_integration_test, run_transform_test,
};
