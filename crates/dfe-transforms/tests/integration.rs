// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests -- single binary, submodule per source.
//!
//! All integration tests compile as ONE binary (one link cycle).
//! This gives ~3x compile-time reduction vs separate test files.
//!
//! Adding a new source? Add `mod source_name;` in the integration/ directory
//! and declare it here.
//!
//! **A broker test does not belong in this crate.** This one holds transforms,
//! and a transform is exercised here and in `smoke.rs` without a broker. The
//! round-trip and deployment checks live in the service crate, which owns the
//! service loop and the contract:
//!
//! - `tests/broker.rs` -- events in one topic, transformed events out another,
//!   against a real broker.
//! - `src/deployment.rs` -- contract tests, plus drift checks on the committed
//!   Dockerfile and artefacts.
//!
//! That signpost had its own test binary (`e2e.rs` plus an `e2e/deployment.rs`
//! holding nothing but this note), which linked against a crate that compiles
//! as one ~11.6 GB rustc and ran zero tests. A comment does not need a link
//! cycle.

mod common;

#[path = "integration/azure.rs"]
mod azure;

#[path = "integration/crowdstrike.rs"]
mod crowdstrike;

#[path = "integration/okta.rs"]
mod okta;

#[path = "integration/remaining.rs"]
mod remaining;
