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

//! Generated filebeat transforms for the packages from `azure` to `cisa`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/azure_activitylogs/mod.rs"]
pub mod azure_activitylogs;
#[path = "../../dfe-transforms/src/filebeat/azure_ai_foundry_logs/mod.rs"]
pub mod azure_ai_foundry_logs;
#[path = "../../dfe-transforms/src/filebeat/azure_ai_foundry_metrics/mod.rs"]
pub mod azure_ai_foundry_metrics;
#[path = "../../dfe-transforms/src/filebeat/azure_app_service_app_service_logs/mod.rs"]
pub mod azure_app_service_app_service_logs;
#[path = "../../dfe-transforms/src/filebeat/azure_application_gateway/mod.rs"]
pub mod azure_application_gateway;
#[path = "../../dfe-transforms/src/filebeat/azure_application_insights_app_insights/mod.rs"]
pub mod azure_application_insights_app_insights;
#[path = "../../dfe-transforms/src/filebeat/azure_auditlogs/mod.rs"]
pub mod azure_auditlogs;
#[path = "../../dfe-transforms/src/filebeat/azure_eventhub/mod.rs"]
pub mod azure_eventhub;
#[path = "../../dfe-transforms/src/filebeat/azure_events/mod.rs"]
pub mod azure_events;
#[path = "../../dfe-transforms/src/filebeat/azure_firewall_logs/mod.rs"]
pub mod azure_firewall_logs;
#[path = "../../dfe-transforms/src/filebeat/azure_frontdoor_access/mod.rs"]
pub mod azure_frontdoor_access;
#[path = "../../dfe-transforms/src/filebeat/azure_frontdoor_health_probe/mod.rs"]
pub mod azure_frontdoor_health_probe;
#[path = "../../dfe-transforms/src/filebeat/azure_frontdoor_waf/mod.rs"]
pub mod azure_frontdoor_waf;
#[path = "../../dfe-transforms/src/filebeat/azure_functions_functionapplogs/mod.rs"]
pub mod azure_functions_functionapplogs;
#[path = "../../dfe-transforms/src/filebeat/azure_functions_metrics/mod.rs"]
pub mod azure_functions_metrics;
#[path = "../../dfe-transforms/src/filebeat/azure_graphactivitylogs/mod.rs"]
pub mod azure_graphactivitylogs;
#[path = "../../dfe-transforms/src/filebeat/azure_identity_protection/mod.rs"]
pub mod azure_identity_protection;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_compute_vm/mod.rs"]
pub mod azure_metrics_compute_vm;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_compute_vm_scaleset/mod.rs"]
pub mod azure_metrics_compute_vm_scaleset;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_container_instance/mod.rs"]
pub mod azure_metrics_container_instance;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_container_registry/mod.rs"]
pub mod azure_metrics_container_registry;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_container_service/mod.rs"]
pub mod azure_metrics_container_service;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_database_account/mod.rs"]
pub mod azure_metrics_database_account;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_monitor/mod.rs"]
pub mod azure_metrics_monitor;
#[path = "../../dfe-transforms/src/filebeat/azure_metrics_storage_account/mod.rs"]
pub mod azure_metrics_storage_account;
#[path = "../../dfe-transforms/src/filebeat/azure_network_watcher_nsg_log/mod.rs"]
pub mod azure_network_watcher_nsg_log;
#[path = "../../dfe-transforms/src/filebeat/azure_network_watcher_vnet_log/mod.rs"]
pub mod azure_network_watcher_vnet_log;
#[path = "../../dfe-transforms/src/filebeat/azure_openai_logs/mod.rs"]
pub mod azure_openai_logs;
#[path = "../../dfe-transforms/src/filebeat/azure_openai_metrics/mod.rs"]
pub mod azure_openai_metrics;
#[path = "../../dfe-transforms/src/filebeat/azure_platformlogs/mod.rs"]
pub mod azure_platformlogs;
#[path = "../../dfe-transforms/src/filebeat/azure_provisioning/mod.rs"]
pub mod azure_provisioning;
#[path = "../../dfe-transforms/src/filebeat/azure_signinlogs/mod.rs"]
pub mod azure_signinlogs;
#[path = "../../dfe-transforms/src/filebeat/azure_springcloudlogs/mod.rs"]
pub mod azure_springcloudlogs;
#[path = "../../dfe-transforms/src/filebeat/backstage_logs/mod.rs"]
pub mod backstage_logs;
#[path = "../../dfe-transforms/src/filebeat/barracuda_cloudgen_firewall_log/mod.rs"]
pub mod barracuda_cloudgen_firewall_log;
#[path = "../../dfe-transforms/src/filebeat/barracuda_waf/mod.rs"]
pub mod barracuda_waf;
#[path = "../../dfe-transforms/src/filebeat/bbot_asm_intel/mod.rs"]
pub mod bbot_asm_intel;
#[path = "../../dfe-transforms/src/filebeat/beelzebub_logs/mod.rs"]
pub mod beelzebub_logs;
#[path = "../../dfe-transforms/src/filebeat/beyondinsight_password_safe_asset/mod.rs"]
pub mod beyondinsight_password_safe_asset;
#[path = "../../dfe-transforms/src/filebeat/beyondinsight_password_safe_managedaccount/mod.rs"]
pub mod beyondinsight_password_safe_managedaccount;
#[path = "../../dfe-transforms/src/filebeat/beyondinsight_password_safe_managedsystem/mod.rs"]
pub mod beyondinsight_password_safe_managedsystem;
#[path = "../../dfe-transforms/src/filebeat/beyondinsight_password_safe_session/mod.rs"]
pub mod beyondinsight_password_safe_session;
#[path = "../../dfe-transforms/src/filebeat/beyondinsight_password_safe_useraudit/mod.rs"]
pub mod beyondinsight_password_safe_useraudit;
#[path = "../../dfe-transforms/src/filebeat/beyondtrust_epm_audit/mod.rs"]
pub mod beyondtrust_epm_audit;
#[path = "../../dfe-transforms/src/filebeat/beyondtrust_epm_event/mod.rs"]
pub mod beyondtrust_epm_event;
#[path = "../../dfe-transforms/src/filebeat/beyondtrust_isi_incident/mod.rs"]
pub mod beyondtrust_isi_incident;
#[path = "../../dfe-transforms/src/filebeat/beyondtrust_pra_access_session/mod.rs"]
pub mod beyondtrust_pra_access_session;
#[path = "../../dfe-transforms/src/filebeat/bitdefender_push_configuration/mod.rs"]
pub mod bitdefender_push_configuration;
#[path = "../../dfe-transforms/src/filebeat/bitdefender_push_notifications/mod.rs"]
pub mod bitdefender_push_notifications;
#[path = "../../dfe-transforms/src/filebeat/bitdefender_push_statistics/mod.rs"]
pub mod bitdefender_push_statistics;
#[path = "../../dfe-transforms/src/filebeat/bitsight_vulnerability/mod.rs"]
pub mod bitsight_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/bitwarden_collection/mod.rs"]
pub mod bitwarden_collection;
#[path = "../../dfe-transforms/src/filebeat/bitwarden_event/mod.rs"]
pub mod bitwarden_event;
#[path = "../../dfe-transforms/src/filebeat/bitwarden_group/mod.rs"]
pub mod bitwarden_group;
#[path = "../../dfe-transforms/src/filebeat/bitwarden_member/mod.rs"]
pub mod bitwarden_member;
#[path = "../../dfe-transforms/src/filebeat/bitwarden_policy/mod.rs"]
pub mod bitwarden_policy;
#[path = "../../dfe-transforms/src/filebeat/blacklens_alerts/mod.rs"]
pub mod blacklens_alerts;
#[path = "../../dfe-transforms/src/filebeat/bluecoat_director/mod.rs"]
pub mod bluecoat_director;
#[path = "../../dfe-transforms/src/filebeat/box_events_events/mod.rs"]
pub mod box_events_events;
#[path = "../../dfe-transforms/src/filebeat/canva_audit/mod.rs"]
pub mod canva_audit;
#[path = "../../dfe-transforms/src/filebeat/carbon_black_cloud_alert_v7/mod.rs"]
pub mod carbon_black_cloud_alert_v7;
#[path = "../../dfe-transforms/src/filebeat/carbon_black_cloud_asset_vulnerability_summary/mod.rs"]
pub mod carbon_black_cloud_asset_vulnerability_summary;
#[path = "../../dfe-transforms/src/filebeat/carbon_black_cloud_audit/mod.rs"]
pub mod carbon_black_cloud_audit;
#[path = "../../dfe-transforms/src/filebeat/carbon_black_cloud_endpoint_event/mod.rs"]
pub mod carbon_black_cloud_endpoint_event;
#[path = "../../dfe-transforms/src/filebeat/carbon_black_cloud_watchlist_hit/mod.rs"]
pub mod carbon_black_cloud_watchlist_hit;
#[path = "../../dfe-transforms/src/filebeat/carbonblack_edr_log/mod.rs"]
pub mod carbonblack_edr_log;
#[path = "../../dfe-transforms/src/filebeat/cassandra_log/mod.rs"]
pub mod cassandra_log;
#[path = "../../dfe-transforms/src/filebeat/cassandra_metrics/mod.rs"]
pub mod cassandra_metrics;
#[path = "../../dfe-transforms/src/filebeat/cato_networks_audit/mod.rs"]
pub mod cato_networks_audit;
#[path = "../../dfe-transforms/src/filebeat/cato_networks_event/mod.rs"]
pub mod cato_networks_event;
#[path = "../../dfe-transforms/src/filebeat/cef_log/mod.rs"]
pub mod cef_log;
#[path = "../../dfe-transforms/src/filebeat/ceph_cluster_disk/mod.rs"]
pub mod ceph_cluster_disk;
#[path = "../../dfe-transforms/src/filebeat/ceph_cluster_health/mod.rs"]
pub mod ceph_cluster_health;
#[path = "../../dfe-transforms/src/filebeat/ceph_cluster_status/mod.rs"]
pub mod ceph_cluster_status;
#[path = "../../dfe-transforms/src/filebeat/ceph_osd_performance/mod.rs"]
pub mod ceph_osd_performance;
#[path = "../../dfe-transforms/src/filebeat/ceph_osd_pool_stats/mod.rs"]
pub mod ceph_osd_pool_stats;
#[path = "../../dfe-transforms/src/filebeat/ceph_osd_tree/mod.rs"]
pub mod ceph_osd_tree;
#[path = "../../dfe-transforms/src/filebeat/ceph_pool_disk/mod.rs"]
pub mod ceph_pool_disk;
#[path = "../../dfe-transforms/src/filebeat/checkpoint/mod.rs"]
pub mod checkpoint;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_email_event/mod.rs"]
pub mod checkpoint_email_event;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_harmony_endpoint_antibot/mod.rs"]
pub mod checkpoint_harmony_endpoint_antibot;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_harmony_endpoint_antimalware/mod.rs"]
pub mod checkpoint_harmony_endpoint_antimalware;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_harmony_endpoint_forensics/mod.rs"]
pub mod checkpoint_harmony_endpoint_forensics;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_harmony_endpoint_threatemulation/mod.rs"]
pub mod checkpoint_harmony_endpoint_threatemulation;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_harmony_endpoint_threatextraction/mod.rs"]
pub mod checkpoint_harmony_endpoint_threatextraction;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_harmony_endpoint_urlfiltering/mod.rs"]
pub mod checkpoint_harmony_endpoint_urlfiltering;
#[path = "../../dfe-transforms/src/filebeat/checkpoint_harmony_endpoint_zerophishing/mod.rs"]
pub mod checkpoint_harmony_endpoint_zerophishing;
#[path = "../../dfe-transforms/src/filebeat/cisa_kevs_vulnerability/mod.rs"]
pub mod cisa_kevs_vulnerability;
