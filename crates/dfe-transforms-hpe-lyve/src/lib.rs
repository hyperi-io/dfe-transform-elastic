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

//! Generated filebeat transforms for the packages from `hpe` to `lyve`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/hpe_aruba_cx_log/mod.rs"]
pub mod hpe_aruba_cx_log;
#[path = "../../dfe-transforms/src/filebeat/ibm_qradar_offense/mod.rs"]
pub mod ibm_qradar_offense;
#[path = "../../dfe-transforms/src/filebeat/ibmmq_errorlog/mod.rs"]
pub mod ibmmq_errorlog;
#[path = "../../dfe-transforms/src/filebeat/ibmmq_qmgr/mod.rs"]
pub mod ibmmq_qmgr;
#[path = "../../dfe-transforms/src/filebeat/iis_access/mod.rs"]
pub mod iis_access;
#[path = "../../dfe-transforms/src/filebeat/iis_error/mod.rs"]
pub mod iis_error;
#[path = "../../dfe-transforms/src/filebeat/imperva_cloud_waf_event/mod.rs"]
pub mod imperva_cloud_waf_event;
#[path = "../../dfe-transforms/src/filebeat/imperva_securesphere/mod.rs"]
pub mod imperva_securesphere;
#[path = "../../dfe-transforms/src/filebeat/influxdb_advstatus/mod.rs"]
pub mod influxdb_advstatus;
#[path = "../../dfe-transforms/src/filebeat/influxdb_status/mod.rs"]
pub mod influxdb_status;
#[path = "../../dfe-transforms/src/filebeat/infoblox_bloxone_ddi_dhcp_lease/mod.rs"]
pub mod infoblox_bloxone_ddi_dhcp_lease;
#[path = "../../dfe-transforms/src/filebeat/infoblox_bloxone_ddi_dns_config/mod.rs"]
pub mod infoblox_bloxone_ddi_dns_config;
#[path = "../../dfe-transforms/src/filebeat/infoblox_bloxone_ddi_dns_data/mod.rs"]
pub mod infoblox_bloxone_ddi_dns_data;
#[path = "../../dfe-transforms/src/filebeat/infoblox_nios_log/mod.rs"]
pub mod infoblox_nios_log;
#[path = "../../dfe-transforms/src/filebeat/infoblox_threat_defense_event/mod.rs"]
pub mod infoblox_threat_defense_event;
#[path = "../../dfe-transforms/src/filebeat/iptables_log/mod.rs"]
pub mod iptables_log;
#[path = "../../dfe-transforms/src/filebeat/ironscales_incident/mod.rs"]
pub mod ironscales_incident;
#[path = "../../dfe-transforms/src/filebeat/island_browser_admin_actions/mod.rs"]
pub mod island_browser_admin_actions;
#[path = "../../dfe-transforms/src/filebeat/island_browser_audit/mod.rs"]
pub mod island_browser_audit;
#[path = "../../dfe-transforms/src/filebeat/island_browser_compromised_credential/mod.rs"]
pub mod island_browser_compromised_credential;
#[path = "../../dfe-transforms/src/filebeat/island_browser_device/mod.rs"]
pub mod island_browser_device;
#[path = "../../dfe-transforms/src/filebeat/island_browser_user/mod.rs"]
pub mod island_browser_user;
#[path = "../../dfe-transforms/src/filebeat/istio_access_logs/mod.rs"]
pub mod istio_access_logs;
#[path = "../../dfe-transforms/src/filebeat/istio_istiod_metrics/mod.rs"]
pub mod istio_istiod_metrics;
#[path = "../../dfe-transforms/src/filebeat/istio_proxy_metrics/mod.rs"]
pub mod istio_proxy_metrics;
#[path = "../../dfe-transforms/src/filebeat/jamf_compliance_reporter_log/mod.rs"]
pub mod jamf_compliance_reporter_log;
#[path = "../../dfe-transforms/src/filebeat/jamf_pro_events/mod.rs"]
pub mod jamf_pro_events;
#[path = "../../dfe-transforms/src/filebeat/jamf_pro_inventory/mod.rs"]
pub mod jamf_pro_inventory;
#[path = "../../dfe-transforms/src/filebeat/jamf_protect_alerts/mod.rs"]
pub mod jamf_protect_alerts;
#[path = "../../dfe-transforms/src/filebeat/jamf_protect_telemetry/mod.rs"]
pub mod jamf_protect_telemetry;
#[path = "../../dfe-transforms/src/filebeat/jamf_protect_web_threat_events/mod.rs"]
pub mod jamf_protect_web_threat_events;
#[path = "../../dfe-transforms/src/filebeat/jamf_protect_web_traffic_events/mod.rs"]
pub mod jamf_protect_web_traffic_events;
#[path = "../../dfe-transforms/src/filebeat/jumpcloud_events/mod.rs"]
pub mod jumpcloud_events;
#[path = "../../dfe-transforms/src/filebeat/juniper_junos_log/mod.rs"]
pub mod juniper_junos_log;
#[path = "../../dfe-transforms/src/filebeat/juniper_netscreen_log/mod.rs"]
pub mod juniper_netscreen_log;
#[path = "../../dfe-transforms/src/filebeat/juniper_srx_log/mod.rs"]
pub mod juniper_srx_log;
#[path = "../../dfe-transforms/src/filebeat/jupiter_one_asset/mod.rs"]
pub mod jupiter_one_asset;
#[path = "../../dfe-transforms/src/filebeat/kafka_connect_client/mod.rs"]
pub mod kafka_connect_client;
#[path = "../../dfe-transforms/src/filebeat/kafka_connect_connector/mod.rs"]
pub mod kafka_connect_connector;
#[path = "../../dfe-transforms/src/filebeat/kafka_connect_task/mod.rs"]
pub mod kafka_connect_task;
#[path = "../../dfe-transforms/src/filebeat/kafka_connect_worker/mod.rs"]
pub mod kafka_connect_worker;
#[path = "../../dfe-transforms/src/filebeat/kafka_consumer/mod.rs"]
pub mod kafka_consumer;
#[path = "../../dfe-transforms/src/filebeat/kafka_controller/mod.rs"]
pub mod kafka_controller;
#[path = "../../dfe-transforms/src/filebeat/kafka_jvm/mod.rs"]
pub mod kafka_jvm;
#[path = "../../dfe-transforms/src/filebeat/kafka_log/mod.rs"]
pub mod kafka_log;
#[path = "../../dfe-transforms/src/filebeat/kafka_log_manager/mod.rs"]
pub mod kafka_log_manager;
#[path = "../../dfe-transforms/src/filebeat/kafka_network/mod.rs"]
pub mod kafka_network;
#[path = "../../dfe-transforms/src/filebeat/kafka_producer/mod.rs"]
pub mod kafka_producer;
#[path = "../../dfe-transforms/src/filebeat/kafka_raft/mod.rs"]
pub mod kafka_raft;
#[path = "../../dfe-transforms/src/filebeat/kafka_replica_manager/mod.rs"]
pub mod kafka_replica_manager;
#[path = "../../dfe-transforms/src/filebeat/kafka_topic/mod.rs"]
pub mod kafka_topic;
#[path = "../../dfe-transforms/src/filebeat/keeper_security_siem_integration_audit/mod.rs"]
pub mod keeper_security_siem_integration_audit;
#[path = "../../dfe-transforms/src/filebeat/keycloak_log/mod.rs"]
pub mod keycloak_log;
#[path = "../../dfe-transforms/src/filebeat/kibana_audit/mod.rs"]
pub mod kibana_audit;
#[path = "../../dfe-transforms/src/filebeat/kibana_log/mod.rs"]
pub mod kibana_log;
#[path = "../../dfe-transforms/src/filebeat/kolide_audit/mod.rs"]
pub mod kolide_audit;
#[path = "../../dfe-transforms/src/filebeat/kolide_auth/mod.rs"]
pub mod kolide_auth;
#[path = "../../dfe-transforms/src/filebeat/kolide_deprovisioned_person/mod.rs"]
pub mod kolide_deprovisioned_person;
#[path = "../../dfe-transforms/src/filebeat/kolide_device/mod.rs"]
pub mod kolide_device;
#[path = "../../dfe-transforms/src/filebeat/kolide_device_check/mod.rs"]
pub mod kolide_device_check;
#[path = "../../dfe-transforms/src/filebeat/kolide_issues/mod.rs"]
pub mod kolide_issues;
#[path = "../../dfe-transforms/src/filebeat/kolide_osquery_result/mod.rs"]
pub mod kolide_osquery_result;
#[path = "../../dfe-transforms/src/filebeat/kolide_osquery_status/mod.rs"]
pub mod kolide_osquery_status;
#[path = "../../dfe-transforms/src/filebeat/kolide_people/mod.rs"]
pub mod kolide_people;
#[path = "../../dfe-transforms/src/filebeat/kolide_request/mod.rs"]
pub mod kolide_request;
#[path = "../../dfe-transforms/src/filebeat/kubernetes_audit_logs/mod.rs"]
pub mod kubernetes_audit_logs;
#[path = "../../dfe-transforms/src/filebeat/kubernetes_container_logs/mod.rs"]
pub mod kubernetes_container_logs;
#[path = "../../dfe-transforms/src/filebeat/lastpass_detailed_shared_folder/mod.rs"]
pub mod lastpass_detailed_shared_folder;
#[path = "../../dfe-transforms/src/filebeat/lastpass_event_report/mod.rs"]
pub mod lastpass_event_report;
#[path = "../../dfe-transforms/src/filebeat/lastpass_user/mod.rs"]
pub mod lastpass_user;
#[path = "../../dfe-transforms/src/filebeat/logstash_log/mod.rs"]
pub mod logstash_log;
#[path = "../../dfe-transforms/src/filebeat/logstash_pipeline/mod.rs"]
pub mod logstash_pipeline;
#[path = "../../dfe-transforms/src/filebeat/logstash_plugins/mod.rs"]
pub mod logstash_plugins;
#[path = "../../dfe-transforms/src/filebeat/logstash_slowlog/mod.rs"]
pub mod logstash_slowlog;
#[path = "../../dfe-transforms/src/filebeat/lumos_activity_logs/mod.rs"]
pub mod lumos_activity_logs;
#[path = "../../dfe-transforms/src/filebeat/lyve_cloud_audit/mod.rs"]
pub mod lyve_cloud_audit;
