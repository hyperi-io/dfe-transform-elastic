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

//! Generated filebeat transforms for the packages from `stormshield` to `swimlane`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/stormshield_log/mod.rs"]
pub mod stormshield_log;
#[path = "../../dfe-transforms/src/filebeat/sublime_security_audit/mod.rs"]
pub mod sublime_security_audit;
#[path = "../../dfe-transforms/src/filebeat/sublime_security_email_message/mod.rs"]
pub mod sublime_security_email_message;
#[path = "../../dfe-transforms/src/filebeat/sublime_security_message_event/mod.rs"]
pub mod sublime_security_message_event;
#[path = "../../dfe-transforms/src/filebeat/suricata_eve/mod.rs"]
pub mod suricata_eve;
#[path = "../../dfe-transforms/src/filebeat/swimlane_audit_logs/mod.rs"]
pub mod swimlane_audit_logs;
#[path = "../../dfe-transforms/src/filebeat/swimlane_swimlane_api/mod.rs"]
pub mod swimlane_swimlane_api;
#[path = "../../dfe-transforms/src/filebeat/swimlane_tenant_api/mod.rs"]
pub mod swimlane_tenant_api;
#[path = "../../dfe-transforms/src/filebeat/swimlane_turbine_api/mod.rs"]
pub mod swimlane_turbine_api;
