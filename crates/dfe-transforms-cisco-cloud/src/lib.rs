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

//! Generated filebeat transforms for the packages from `cisco` to `cloud`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/cisco_aironet_log/mod.rs"]
pub mod cisco_aironet_log;
#[path = "../../dfe-transforms/src/filebeat/cisco_asa/mod.rs"]
pub mod cisco_asa;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_activity/mod.rs"]
pub mod cisco_duo_activity;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_admin/mod.rs"]
pub mod cisco_duo_admin;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_auth/mod.rs"]
pub mod cisco_duo_auth;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_offline_enrollment/mod.rs"]
pub mod cisco_duo_offline_enrollment;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_summary/mod.rs"]
pub mod cisco_duo_summary;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_telephony/mod.rs"]
pub mod cisco_duo_telephony;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_telephony_v2/mod.rs"]
pub mod cisco_duo_telephony_v2;
#[path = "../../dfe-transforms/src/filebeat/cisco_duo_trust_monitor/mod.rs"]
pub mod cisco_duo_trust_monitor;
#[path = "../../dfe-transforms/src/filebeat/cisco_ftd/mod.rs"]
pub mod cisco_ftd;
#[path = "../../dfe-transforms/src/filebeat/cisco_ios/mod.rs"]
pub mod cisco_ios;
#[path = "../../dfe-transforms/src/filebeat/cisco_ise_log/mod.rs"]
pub mod cisco_ise_log;
#[path = "../../dfe-transforms/src/filebeat/cisco_meraki/mod.rs"]
pub mod cisco_meraki;
#[path = "../../dfe-transforms/src/filebeat/cisco_meraki_events/mod.rs"]
pub mod cisco_meraki_events;
#[path = "../../dfe-transforms/src/filebeat/cisco_meraki_metrics_device_health/mod.rs"]
pub mod cisco_meraki_metrics_device_health;
#[path = "../../dfe-transforms/src/filebeat/cisco_nexus/mod.rs"]
pub mod cisco_nexus;
#[path = "../../dfe-transforms/src/filebeat/cisco_secure_email_gateway_log/mod.rs"]
pub mod cisco_secure_email_gateway_log;
#[path = "../../dfe-transforms/src/filebeat/cisco_secure_endpoint_event/mod.rs"]
pub mod cisco_secure_endpoint_event;
#[path = "../../dfe-transforms/src/filebeat/cisco_umbrella/mod.rs"]
pub mod cisco_umbrella;
#[path = "../../dfe-transforms/src/filebeat/citrix_adc_interface/mod.rs"]
pub mod citrix_adc_interface;
#[path = "../../dfe-transforms/src/filebeat/citrix_adc_lbvserver/mod.rs"]
pub mod citrix_adc_lbvserver;
#[path = "../../dfe-transforms/src/filebeat/citrix_adc_log/mod.rs"]
pub mod citrix_adc_log;
#[path = "../../dfe-transforms/src/filebeat/citrix_adc_service/mod.rs"]
pub mod citrix_adc_service;
#[path = "../../dfe-transforms/src/filebeat/citrix_adc_system/mod.rs"]
pub mod citrix_adc_system;
#[path = "../../dfe-transforms/src/filebeat/citrix_adc_vpn/mod.rs"]
pub mod citrix_adc_vpn;
#[path = "../../dfe-transforms/src/filebeat/citrix_waf_log/mod.rs"]
pub mod citrix_waf_log;
#[path = "../../dfe-transforms/src/filebeat/claroty_ctd_asset/mod.rs"]
pub mod claroty_ctd_asset;
#[path = "../../dfe-transforms/src/filebeat/claroty_ctd_baseline/mod.rs"]
pub mod claroty_ctd_baseline;
#[path = "../../dfe-transforms/src/filebeat/claroty_ctd_event/mod.rs"]
pub mod claroty_ctd_event;
#[path = "../../dfe-transforms/src/filebeat/claroty_xdome_alert/mod.rs"]
pub mod claroty_xdome_alert;
#[path = "../../dfe-transforms/src/filebeat/claroty_xdome_event/mod.rs"]
pub mod claroty_xdome_event;
#[path = "../../dfe-transforms/src/filebeat/claroty_xdome_vulnerability/mod.rs"]
pub mod claroty_xdome_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/claude_code_events/mod.rs"]
pub mod claude_code_events;
#[path = "../../dfe-transforms/src/filebeat/claude_cowork_events/mod.rs"]
pub mod claude_cowork_events;
#[path = "../../dfe-transforms/src/filebeat/cloud_asset_inventory_asset_inventory/mod.rs"]
pub mod cloud_asset_inventory_asset_inventory;
#[path = "../../dfe-transforms/src/filebeat/cloud_defend_alerts/mod.rs"]
pub mod cloud_defend_alerts;
#[path = "../../dfe-transforms/src/filebeat/cloud_defend_file/mod.rs"]
pub mod cloud_defend_file;
#[path = "../../dfe-transforms/src/filebeat/cloud_defend_process/mod.rs"]
pub mod cloud_defend_process;
#[path = "../../dfe-transforms/src/filebeat/cloud_security_posture_findings/mod.rs"]
pub mod cloud_security_posture_findings;
#[path = "../../dfe-transforms/src/filebeat/cloud_security_posture_vulnerabilities/mod.rs"]
pub mod cloud_security_posture_vulnerabilities;
