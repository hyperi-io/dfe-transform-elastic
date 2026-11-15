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

//! Generated filebeat transforms for the packages from `cloudflare` to `extrahop`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/cloudflare_audit/mod.rs"]
pub mod cloudflare_audit;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpull/mod.rs"]
pub mod cloudflare_logpull;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_access_request/mod.rs"]
pub mod cloudflare_logpush_access_request;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_audit/mod.rs"]
pub mod cloudflare_logpush_audit;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_casb/mod.rs"]
pub mod cloudflare_logpush_casb;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_device_posture/mod.rs"]
pub mod cloudflare_logpush_device_posture;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_dlp_forensic_copies/mod.rs"]
pub mod cloudflare_logpush_dlp_forensic_copies;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_dns/mod.rs"]
pub mod cloudflare_logpush_dns;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_dns_firewall/mod.rs"]
pub mod cloudflare_logpush_dns_firewall;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_email_security_alerts/mod.rs"]
pub mod cloudflare_logpush_email_security_alerts;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_firewall_event/mod.rs"]
pub mod cloudflare_logpush_firewall_event;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_gateway_dns/mod.rs"]
pub mod cloudflare_logpush_gateway_dns;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_gateway_http/mod.rs"]
pub mod cloudflare_logpush_gateway_http;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_gateway_network/mod.rs"]
pub mod cloudflare_logpush_gateway_network;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_http_request/mod.rs"]
pub mod cloudflare_logpush_http_request;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_magic_ids/mod.rs"]
pub mod cloudflare_logpush_magic_ids;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_nel_report/mod.rs"]
pub mod cloudflare_logpush_nel_report;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_network_analytics/mod.rs"]
pub mod cloudflare_logpush_network_analytics;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_network_session/mod.rs"]
pub mod cloudflare_logpush_network_session;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_page_shield_events/mod.rs"]
pub mod cloudflare_logpush_page_shield_events;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_sinkhole_http/mod.rs"]
pub mod cloudflare_logpush_sinkhole_http;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_spectrum_event/mod.rs"]
pub mod cloudflare_logpush_spectrum_event;
#[path = "../../dfe-transforms/src/filebeat/cloudflare_logpush_workers_trace/mod.rs"]
pub mod cloudflare_logpush_workers_trace;
#[path = "../../dfe-transforms/src/filebeat/cockroachdb_status/mod.rs"]
pub mod cockroachdb_status;
#[path = "../../dfe-transforms/src/filebeat/contrast_security_attack_event/mod.rs"]
pub mod contrast_security_attack_event;
#[path = "../../dfe-transforms/src/filebeat/contrast_security_incident/mod.rs"]
pub mod contrast_security_incident;
#[path = "../../dfe-transforms/src/filebeat/contrast_security_issue/mod.rs"]
pub mod contrast_security_issue;
#[path = "../../dfe-transforms/src/filebeat/coredns_log/mod.rs"]
pub mod coredns_log;
#[path = "../../dfe-transforms/src/filebeat/couchbase_bucket/mod.rs"]
pub mod couchbase_bucket;
#[path = "../../dfe-transforms/src/filebeat/couchbase_cache/mod.rs"]
pub mod couchbase_cache;
#[path = "../../dfe-transforms/src/filebeat/couchbase_cbl_replication/mod.rs"]
pub mod couchbase_cbl_replication;
#[path = "../../dfe-transforms/src/filebeat/couchbase_cluster/mod.rs"]
pub mod couchbase_cluster;
#[path = "../../dfe-transforms/src/filebeat/couchbase_database_stats/mod.rs"]
pub mod couchbase_database_stats;
#[path = "../../dfe-transforms/src/filebeat/couchbase_miscellaneous/mod.rs"]
pub mod couchbase_miscellaneous;
#[path = "../../dfe-transforms/src/filebeat/couchbase_node/mod.rs"]
pub mod couchbase_node;
#[path = "../../dfe-transforms/src/filebeat/couchbase_query_index/mod.rs"]
pub mod couchbase_query_index;
#[path = "../../dfe-transforms/src/filebeat/couchbase_resource/mod.rs"]
pub mod couchbase_resource;
#[path = "../../dfe-transforms/src/filebeat/couchbase_xdcr/mod.rs"]
pub mod couchbase_xdcr;
#[path = "../../dfe-transforms/src/filebeat/couchdb_server/mod.rs"]
pub mod couchdb_server;
#[path = "../../dfe-transforms/src/filebeat/crowdstrike/mod.rs"]
pub mod crowdstrike;
#[path = "../../dfe-transforms/src/filebeat/crowdstrike_alert/mod.rs"]
pub mod crowdstrike_alert;
#[path = "../../dfe-transforms/src/filebeat/crowdstrike_host/mod.rs"]
pub mod crowdstrike_host;
#[path = "../../dfe-transforms/src/filebeat/crowdstrike_identity_protection_assessment/mod.rs"]
pub mod crowdstrike_identity_protection_assessment;
#[path = "../../dfe-transforms/src/filebeat/crowdstrike_identity_protection_timeline/mod.rs"]
pub mod crowdstrike_identity_protection_timeline;
#[path = "../../dfe-transforms/src/filebeat/crowdstrike_vulnerability/mod.rs"]
pub mod crowdstrike_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/cursor_audit/mod.rs"]
pub mod cursor_audit;
#[path = "../../dfe-transforms/src/filebeat/cyberark_epm_admin_audit/mod.rs"]
pub mod cyberark_epm_admin_audit;
#[path = "../../dfe-transforms/src/filebeat/cyberark_epm_aggregated_event/mod.rs"]
pub mod cyberark_epm_aggregated_event;
#[path = "../../dfe-transforms/src/filebeat/cyberark_epm_policyaudit_aggregated_event/mod.rs"]
pub mod cyberark_epm_policyaudit_aggregated_event;
#[path = "../../dfe-transforms/src/filebeat/cyberark_epm_policyaudit_raw_event/mod.rs"]
pub mod cyberark_epm_policyaudit_raw_event;
#[path = "../../dfe-transforms/src/filebeat/cyberark_epm_raw_event/mod.rs"]
pub mod cyberark_epm_raw_event;
#[path = "../../dfe-transforms/src/filebeat/cyberark_pta_events/mod.rs"]
pub mod cyberark_pta_events;
#[path = "../../dfe-transforms/src/filebeat/cyberarkpas_audit/mod.rs"]
pub mod cyberarkpas_audit;
#[path = "../../dfe-transforms/src/filebeat/cyberarkpas_monitor/mod.rs"]
pub mod cyberarkpas_monitor;
#[path = "../../dfe-transforms/src/filebeat/cybereason_logon_session/mod.rs"]
pub mod cybereason_logon_session;
#[path = "../../dfe-transforms/src/filebeat/cybereason_malop_connection/mod.rs"]
pub mod cybereason_malop_connection;
#[path = "../../dfe-transforms/src/filebeat/cybereason_malop_process/mod.rs"]
pub mod cybereason_malop_process;
#[path = "../../dfe-transforms/src/filebeat/cybereason_malware/mod.rs"]
pub mod cybereason_malware;
#[path = "../../dfe-transforms/src/filebeat/cybereason_poll_malop/mod.rs"]
pub mod cybereason_poll_malop;
#[path = "../../dfe-transforms/src/filebeat/cybereason_suspicions_process/mod.rs"]
pub mod cybereason_suspicions_process;
#[path = "../../dfe-transforms/src/filebeat/cyera_audit/mod.rs"]
pub mod cyera_audit;
#[path = "../../dfe-transforms/src/filebeat/cyera_classification/mod.rs"]
pub mod cyera_classification;
#[path = "../../dfe-transforms/src/filebeat/cyera_datastore/mod.rs"]
pub mod cyera_datastore;
#[path = "../../dfe-transforms/src/filebeat/cyera_event/mod.rs"]
pub mod cyera_event;
#[path = "../../dfe-transforms/src/filebeat/cyera_issue/mod.rs"]
pub mod cyera_issue;
#[path = "../../dfe-transforms/src/filebeat/cylance_protect/mod.rs"]
pub mod cylance_protect;
#[path = "../../dfe-transforms/src/filebeat/darktrace_ai_analyst_alert/mod.rs"]
pub mod darktrace_ai_analyst_alert;
#[path = "../../dfe-transforms/src/filebeat/darktrace_model_breach_alert/mod.rs"]
pub mod darktrace_model_breach_alert;
#[path = "../../dfe-transforms/src/filebeat/darktrace_system_status_alert/mod.rs"]
pub mod darktrace_system_status_alert;
#[path = "../../dfe-transforms/src/filebeat/dataminr_pulse_alerts/mod.rs"]
pub mod dataminr_pulse_alerts;
#[path = "../../dfe-transforms/src/filebeat/digital_guardian_arc/mod.rs"]
pub mod digital_guardian_arc;
#[path = "../../dfe-transforms/src/filebeat/doppel_alerts/mod.rs"]
pub mod doppel_alerts;
#[path = "../../dfe-transforms/src/filebeat/doppler_activity/mod.rs"]
pub mod doppler_activity;
#[path = "../../dfe-transforms/src/filebeat/doppler_secret_read/mod.rs"]
pub mod doppler_secret_read;
#[path = "../../dfe-transforms/src/filebeat/ece_adminconsole/mod.rs"]
pub mod ece_adminconsole;
#[path = "../../dfe-transforms/src/filebeat/elastic_agent_elastic_agent_logs/mod.rs"]
pub mod elastic_agent_elastic_agent_logs;
#[path = "../../dfe-transforms/src/filebeat/elastic_agent_status_change_logs/mod.rs"]
pub mod elastic_agent_status_change_logs;
#[path = "../../dfe-transforms/src/filebeat/elastic_package_registry_metrics/mod.rs"]
pub mod elastic_package_registry_metrics;
#[path = "../../dfe-transforms/src/filebeat/elastic_security_alert/mod.rs"]
pub mod elastic_security_alert;
#[path = "../../dfe-transforms/src/filebeat/elasticsearch_audit/mod.rs"]
pub mod elasticsearch_audit;
#[path = "../../dfe-transforms/src/filebeat/elasticsearch_deprecation/mod.rs"]
pub mod elasticsearch_deprecation;
#[path = "../../dfe-transforms/src/filebeat/elasticsearch_gc/mod.rs"]
pub mod elasticsearch_gc;
#[path = "../../dfe-transforms/src/filebeat/elasticsearch_ingest_pipeline/mod.rs"]
pub mod elasticsearch_ingest_pipeline;
#[path = "../../dfe-transforms/src/filebeat/elasticsearch_querylog/mod.rs"]
pub mod elasticsearch_querylog;
#[path = "../../dfe-transforms/src/filebeat/elasticsearch_server/mod.rs"]
pub mod elasticsearch_server;
#[path = "../../dfe-transforms/src/filebeat/elasticsearch_slowlog/mod.rs"]
pub mod elasticsearch_slowlog;
#[path = "../../dfe-transforms/src/filebeat/endace_flow/mod.rs"]
pub mod endace_flow;
#[path = "../../dfe-transforms/src/filebeat/endace_log/mod.rs"]
pub mod endace_log;
#[path = "../../dfe-transforms/src/filebeat/entityanalytics_ad_entity/mod.rs"]
pub mod entityanalytics_ad_entity;
#[path = "../../dfe-transforms/src/filebeat/entityanalytics_entra_id/mod.rs"]
pub mod entityanalytics_entra_id;
#[path = "../../dfe-transforms/src/filebeat/entityanalytics_okta_entity/mod.rs"]
pub mod entityanalytics_okta_entity;
#[path = "../../dfe-transforms/src/filebeat/entro_audit/mod.rs"]
pub mod entro_audit;
#[path = "../../dfe-transforms/src/filebeat/envoyproxy_log/mod.rs"]
pub mod envoyproxy_log;
#[path = "../../dfe-transforms/src/filebeat/envoyproxy_stats/mod.rs"]
pub mod envoyproxy_stats;
#[path = "../../dfe-transforms/src/filebeat/eset_protect_detection/mod.rs"]
pub mod eset_protect_detection;
#[path = "../../dfe-transforms/src/filebeat/eset_protect_device_task/mod.rs"]
pub mod eset_protect_device_task;
#[path = "../../dfe-transforms/src/filebeat/eset_protect_device_vulnerability/mod.rs"]
pub mod eset_protect_device_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/eset_protect_event/mod.rs"]
pub mod eset_protect_event;
#[path = "../../dfe-transforms/src/filebeat/ess_billing_billing/mod.rs"]
pub mod ess_billing_billing;
#[path = "../../dfe-transforms/src/filebeat/ess_billing_credits/mod.rs"]
pub mod ess_billing_credits;
#[path = "../../dfe-transforms/src/filebeat/etcd_metrics/mod.rs"]
pub mod etcd_metrics;
#[path = "../../dfe-transforms/src/filebeat/etcd_self/mod.rs"]
pub mod etcd_self;
#[path = "../../dfe-transforms/src/filebeat/etcd_store/mod.rs"]
pub mod etcd_store;
#[path = "../../dfe-transforms/src/filebeat/extrahop_detection/mod.rs"]
pub mod extrahop_detection;
#[path = "../../dfe-transforms/src/filebeat/extrahop_investigation/mod.rs"]
pub mod extrahop_investigation;
