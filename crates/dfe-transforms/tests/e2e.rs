// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! End-to-end tests requiring real infrastructure (Kafka, GeoIP DBs).
//!
//! All tests here are `#[ignore]` by default.
//! Run with: `cargo nextest run -- --ignored`
//! Or in CI: e2e stage only runs on PR to `release`.

mod common;

#[path = "e2e/deployment.rs"]
mod deployment;
