// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
#![warn(clippy::all)]
// These modules mirror the structure of an Elastic ingest pipeline processor
// for processor, which is deliberately not idiomatic Rust. Correctness lints
// stay on; only the style ones that fight that shape are off.
#![allow(
    clippy::needless_return,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::redundant_closure_call,
    unused_assignments,
    unused_variables,
    unused_imports,
    dead_code
)]

//! Generated filebeat transforms for the `symantec` package.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/symantec_endpoint_log/mod.rs"]
pub mod symantec_endpoint_log;
#[path = "../../dfe-transforms/src/filebeat/symantec_endpoint_security_event/mod.rs"]
pub mod symantec_endpoint_security_event;
#[path = "../../dfe-transforms/src/filebeat/symantec_endpoint_security_incident/mod.rs"]
pub mod symantec_endpoint_security_incident;
