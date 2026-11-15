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

//! Generated filebeat transforms for the packages from `qualys` to `stan`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/qualys_gav_asset/mod.rs"]
pub mod qualys_gav_asset;
#[path = "../../dfe-transforms/src/filebeat/qualys_vmdr_asset_host_detection/mod.rs"]
pub mod qualys_vmdr_asset_host_detection;
#[path = "../../dfe-transforms/src/filebeat/qualys_vmdr_knowledge_base/mod.rs"]
pub mod qualys_vmdr_knowledge_base;
#[path = "../../dfe-transforms/src/filebeat/qualys_vmdr_user_activity/mod.rs"]
pub mod qualys_vmdr_user_activity;
#[path = "../../dfe-transforms/src/filebeat/qualys_was_vulnerability/mod.rs"]
pub mod qualys_was_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/rabbitmq_log/mod.rs"]
pub mod rabbitmq_log;
#[path = "../../dfe-transforms/src/filebeat/radware_defensepro/mod.rs"]
pub mod radware_defensepro;
#[path = "../../dfe-transforms/src/filebeat/rapid7_insightvm_asset/mod.rs"]
pub mod rapid7_insightvm_asset;
#[path = "../../dfe-transforms/src/filebeat/rapid7_insightvm_asset_vulnerability/mod.rs"]
pub mod rapid7_insightvm_asset_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/rapid7_insightvm_vulnerability/mod.rs"]
pub mod rapid7_insightvm_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/redis_log/mod.rs"]
pub mod redis_log;
#[path = "../../dfe-transforms/src/filebeat/redisenterprise_node/mod.rs"]
pub mod redisenterprise_node;
#[path = "../../dfe-transforms/src/filebeat/redisenterprise_proxy/mod.rs"]
pub mod redisenterprise_proxy;
#[path = "../../dfe-transforms/src/filebeat/rubrik_drives/mod.rs"]
pub mod rubrik_drives;
#[path = "../../dfe-transforms/src/filebeat/rubrik_filesets/mod.rs"]
pub mod rubrik_filesets;
#[path = "../../dfe-transforms/src/filebeat/rubrik_global_cluster_performance/mod.rs"]
pub mod rubrik_global_cluster_performance;
#[path = "../../dfe-transforms/src/filebeat/rubrik_managed_volumes/mod.rs"]
pub mod rubrik_managed_volumes;
#[path = "../../dfe-transforms/src/filebeat/rubrik_monitoring_jobs/mod.rs"]
pub mod rubrik_monitoring_jobs;
#[path = "../../dfe-transforms/src/filebeat/rubrik_mssql_databases/mod.rs"]
pub mod rubrik_mssql_databases;
#[path = "../../dfe-transforms/src/filebeat/rubrik_node_statistics/mod.rs"]
pub mod rubrik_node_statistics;
#[path = "../../dfe-transforms/src/filebeat/rubrik_physical_hosts/mod.rs"]
pub mod rubrik_physical_hosts;
#[path = "../../dfe-transforms/src/filebeat/rubrik_sla_domains/mod.rs"]
pub mod rubrik_sla_domains;
#[path = "../../dfe-transforms/src/filebeat/rubrik_tasks/mod.rs"]
pub mod rubrik_tasks;
#[path = "../../dfe-transforms/src/filebeat/rubrik_unmanaged_objects/mod.rs"]
pub mod rubrik_unmanaged_objects;
#[path = "../../dfe-transforms/src/filebeat/rubrik_virtual_machines/mod.rs"]
pub mod rubrik_virtual_machines;
#[path = "../../dfe-transforms/src/filebeat/sailpoint_identity_sc_events/mod.rs"]
pub mod sailpoint_identity_sc_events;
#[path = "../../dfe-transforms/src/filebeat/salesforce_apex/mod.rs"]
pub mod salesforce_apex;
#[path = "../../dfe-transforms/src/filebeat/salesforce_login/mod.rs"]
pub mod salesforce_login;
#[path = "../../dfe-transforms/src/filebeat/salesforce_logout/mod.rs"]
pub mod salesforce_logout;
#[path = "../../dfe-transforms/src/filebeat/salesforce_setupaudittrail/mod.rs"]
pub mod salesforce_setupaudittrail;
#[path = "../../dfe-transforms/src/filebeat/santa_log/mod.rs"]
pub mod santa_log;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_activity/mod.rs"]
pub mod sentinel_one_activity;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_agent/mod.rs"]
pub mod sentinel_one_agent;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_alert/mod.rs"]
pub mod sentinel_one_alert;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_application/mod.rs"]
pub mod sentinel_one_application;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_application_risk/mod.rs"]
pub mod sentinel_one_application_risk;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_cloud_funnel_event/mod.rs"]
pub mod sentinel_one_cloud_funnel_event;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_group/mod.rs"]
pub mod sentinel_one_group;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_threat/mod.rs"]
pub mod sentinel_one_threat;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_threat_event/mod.rs"]
pub mod sentinel_one_threat_event;
#[path = "../../dfe-transforms/src/filebeat/sentinel_one_unified_alert/mod.rs"]
pub mod sentinel_one_unified_alert;
#[path = "../../dfe-transforms/src/filebeat/servicenow_event/mod.rs"]
pub mod servicenow_event;
#[path = "../../dfe-transforms/src/filebeat/slack_audit/mod.rs"]
pub mod slack_audit;
#[path = "../../dfe-transforms/src/filebeat/snort_log/mod.rs"]
pub mod snort_log;
#[path = "../../dfe-transforms/src/filebeat/snyk_audit_logs/mod.rs"]
pub mod snyk_audit_logs;
#[path = "../../dfe-transforms/src/filebeat/snyk_issues/mod.rs"]
pub mod snyk_issues;
#[path = "../../dfe-transforms/src/filebeat/sonicwall_firewall_log/mod.rs"]
pub mod sonicwall_firewall_log;
#[path = "../../dfe-transforms/src/filebeat/sophos_central_alert/mod.rs"]
pub mod sophos_central_alert;
#[path = "../../dfe-transforms/src/filebeat/sophos_central_event/mod.rs"]
pub mod sophos_central_event;
#[path = "../../dfe-transforms/src/filebeat/sophos_utm/mod.rs"]
pub mod sophos_utm;
#[path = "../../dfe-transforms/src/filebeat/sophos_xg/mod.rs"]
pub mod sophos_xg;
#[path = "../../dfe-transforms/src/filebeat/splunk_alert/mod.rs"]
pub mod splunk_alert;
#[path = "../../dfe-transforms/src/filebeat/splunk_search/mod.rs"]
pub mod splunk_search;
#[path = "../../dfe-transforms/src/filebeat/spring_boot_audit_events/mod.rs"]
pub mod spring_boot_audit_events;
#[path = "../../dfe-transforms/src/filebeat/spring_boot_gc/mod.rs"]
pub mod spring_boot_gc;
#[path = "../../dfe-transforms/src/filebeat/spring_boot_http_trace/mod.rs"]
pub mod spring_boot_http_trace;
#[path = "../../dfe-transforms/src/filebeat/spring_boot_memory/mod.rs"]
pub mod spring_boot_memory;
#[path = "../../dfe-transforms/src/filebeat/spring_boot_threading/mod.rs"]
pub mod spring_boot_threading;
#[path = "../../dfe-transforms/src/filebeat/spycloud_breach_catalog/mod.rs"]
pub mod spycloud_breach_catalog;
#[path = "../../dfe-transforms/src/filebeat/spycloud_breach_record/mod.rs"]
pub mod spycloud_breach_record;
#[path = "../../dfe-transforms/src/filebeat/spycloud_compass/mod.rs"]
pub mod spycloud_compass;
#[path = "../../dfe-transforms/src/filebeat/squid_log/mod.rs"]
pub mod squid_log;
#[path = "../../dfe-transforms/src/filebeat/stan_log/mod.rs"]
pub mod stan_log;
