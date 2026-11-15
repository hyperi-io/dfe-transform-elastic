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

//! Generated filebeat transforms for the `zscaler` package.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_alerts/mod.rs"]
pub mod zscaler_zia_alerts;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_audit/mod.rs"]
pub mod zscaler_zia_audit;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_dns/mod.rs"]
pub mod zscaler_zia_dns;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_email_dlp/mod.rs"]
pub mod zscaler_zia_email_dlp;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_endpoint_dlp/mod.rs"]
pub mod zscaler_zia_endpoint_dlp;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_firewall/mod.rs"]
pub mod zscaler_zia_firewall;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_saas_security/mod.rs"]
pub mod zscaler_zia_saas_security;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_saas_security_activity/mod.rs"]
pub mod zscaler_zia_saas_security_activity;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_sandbox_report/mod.rs"]
pub mod zscaler_zia_sandbox_report;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_sandbox_verdict/mod.rs"]
pub mod zscaler_zia_sandbox_verdict;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_tunnel/mod.rs"]
pub mod zscaler_zia_tunnel;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zia_web/mod.rs"]
pub mod zscaler_zia_web;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zpa_app_connector_status/mod.rs"]
pub mod zscaler_zpa_app_connector_status;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zpa_audit/mod.rs"]
pub mod zscaler_zpa_audit;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zpa_browser_access/mod.rs"]
pub mod zscaler_zpa_browser_access;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zpa_user_activity/mod.rs"]
pub mod zscaler_zpa_user_activity;
#[path = "../../dfe-transforms/src/filebeat/zscaler_zpa_user_status/mod.rs"]
pub mod zscaler_zpa_user_status;
