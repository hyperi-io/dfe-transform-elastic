// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The broker round-trip and deployment checks live in the service crate,
//! which owns the service loop and the deployment contract:
//!
//! - `tests/broker.rs` -- events in one topic, transformed events out another,
//!   against a real broker.
//! - `src/deployment.rs` -- contract tests, plus drift checks on the committed
//!   Dockerfile and artefacts.
//!
//! Nothing belongs here: this crate holds transforms, and a transform is
//! exercised by `tests/integration.rs` and `tests/smoke.rs` without a broker.
//! This file is a signpost so the next reader does not add a broker test to the
//! wrong crate. It previously held two empty `#[ignore]` bodies that asserted
//! nothing and read as coverage.
