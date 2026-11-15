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

//! Generated filebeat transforms for the packages from `o365` to `qnap`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/o365/mod.rs"]
pub mod o365;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_active_users_services_user_counts/mod.rs"]
pub mod o365_metrics_active_users_services_user_counts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_app_registrations/mod.rs"]
pub mod o365_metrics_app_registrations;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_entra_agent/mod.rs"]
pub mod o365_metrics_entra_agent;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_entra_alerts/mod.rs"]
pub mod o365_metrics_entra_alerts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_entra_features/mod.rs"]
pub mod o365_metrics_entra_features;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_entra_id_users/mod.rs"]
pub mod o365_metrics_entra_id_users;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_groups_activity_group_detail/mod.rs"]
pub mod o365_metrics_groups_activity_group_detail;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_mailbox_usage_detail/mod.rs"]
pub mod o365_metrics_mailbox_usage_detail;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_mailbox_usage_quota_status/mod.rs"]
pub mod o365_metrics_mailbox_usage_quota_status;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_onedrive_usage_account_counts/mod.rs"]
pub mod o365_metrics_onedrive_usage_account_counts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_onedrive_usage_account_detail/mod.rs"]
pub mod o365_metrics_onedrive_usage_account_detail;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_onedrive_usage_file_counts/mod.rs"]
pub mod o365_metrics_onedrive_usage_file_counts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_onedrive_usage_storage/mod.rs"]
pub mod o365_metrics_onedrive_usage_storage;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_outlook_activity/mod.rs"]
pub mod o365_metrics_outlook_activity;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_outlook_app_usage_version_counts/mod.rs"]
pub mod o365_metrics_outlook_app_usage_version_counts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_service_health/mod.rs"]
pub mod o365_metrics_service_health;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_sharepoint_site_usage_detail/mod.rs"]
pub mod o365_metrics_sharepoint_site_usage_detail;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_sharepoint_site_usage_storage/mod.rs"]
pub mod o365_metrics_sharepoint_site_usage_storage;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_subscriptions/mod.rs"]
pub mod o365_metrics_subscriptions;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_teams_call_quality/mod.rs"]
pub mod o365_metrics_teams_call_quality;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_teams_device_usage_user_counts/mod.rs"]
pub mod o365_metrics_teams_device_usage_user_counts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_teams_user_activity_user_counts/mod.rs"]
pub mod o365_metrics_teams_user_activity_user_counts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_teams_user_activity_user_detail/mod.rs"]
pub mod o365_metrics_teams_user_activity_user_detail;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_tenant_settings/mod.rs"]
pub mod o365_metrics_tenant_settings;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_viva_engage_device_usage_user_counts/mod.rs"]
pub mod o365_metrics_viva_engage_device_usage_user_counts;
#[path = "../../dfe-transforms/src/filebeat/o365_metrics_viva_engage_groups_activity_group_detail/mod.rs"]
pub mod o365_metrics_viva_engage_groups_activity_group_detail;
#[path = "../../dfe-transforms/src/filebeat/okta/mod.rs"]
pub mod okta;
#[path = "../../dfe-transforms/src/filebeat/openai_audio_speeches/mod.rs"]
pub mod openai_audio_speeches;
#[path = "../../dfe-transforms/src/filebeat/openai_audio_transcriptions/mod.rs"]
pub mod openai_audio_transcriptions;
#[path = "../../dfe-transforms/src/filebeat/openai_audit/mod.rs"]
pub mod openai_audit;
#[path = "../../dfe-transforms/src/filebeat/openai_code_interpreter_sessions/mod.rs"]
pub mod openai_code_interpreter_sessions;
#[path = "../../dfe-transforms/src/filebeat/openai_completions/mod.rs"]
pub mod openai_completions;
#[path = "../../dfe-transforms/src/filebeat/openai_embeddings/mod.rs"]
pub mod openai_embeddings;
#[path = "../../dfe-transforms/src/filebeat/openai_images/mod.rs"]
pub mod openai_images;
#[path = "../../dfe-transforms/src/filebeat/openai_moderations/mod.rs"]
pub mod openai_moderations;
#[path = "../../dfe-transforms/src/filebeat/openai_rate_limits/mod.rs"]
pub mod openai_rate_limits;
#[path = "../../dfe-transforms/src/filebeat/openai_vector_stores/mod.rs"]
pub mod openai_vector_stores;
#[path = "../../dfe-transforms/src/filebeat/opencanary_events/mod.rs"]
pub mod opencanary_events;
#[path = "../../dfe-transforms/src/filebeat/oracle_database_audit/mod.rs"]
pub mod oracle_database_audit;
#[path = "../../dfe-transforms/src/filebeat/oracle_memory/mod.rs"]
pub mod oracle_memory;
#[path = "../../dfe-transforms/src/filebeat/oracle_performance/mod.rs"]
pub mod oracle_performance;
#[path = "../../dfe-transforms/src/filebeat/oracle_sysmetric/mod.rs"]
pub mod oracle_sysmetric;
#[path = "../../dfe-transforms/src/filebeat/oracle_system_statistics/mod.rs"]
pub mod oracle_system_statistics;
#[path = "../../dfe-transforms/src/filebeat/oracle_tablespace/mod.rs"]
pub mod oracle_tablespace;
#[path = "../../dfe-transforms/src/filebeat/oracle_weblogic_access/mod.rs"]
pub mod oracle_weblogic_access;
#[path = "../../dfe-transforms/src/filebeat/oracle_weblogic_admin_server/mod.rs"]
pub mod oracle_weblogic_admin_server;
#[path = "../../dfe-transforms/src/filebeat/oracle_weblogic_deployed_application/mod.rs"]
pub mod oracle_weblogic_deployed_application;
#[path = "../../dfe-transforms/src/filebeat/oracle_weblogic_domain/mod.rs"]
pub mod oracle_weblogic_domain;
#[path = "../../dfe-transforms/src/filebeat/oracle_weblogic_managed_server/mod.rs"]
pub mod oracle_weblogic_managed_server;
#[path = "../../dfe-transforms/src/filebeat/oracle_weblogic_threadpool/mod.rs"]
pub mod oracle_weblogic_threadpool;
#[path = "../../dfe-transforms/src/filebeat/osquery_manager_action_responses/mod.rs"]
pub mod osquery_manager_action_responses;
#[path = "../../dfe-transforms/src/filebeat/osquery_manager_query_profile/mod.rs"]
pub mod osquery_manager_query_profile;
#[path = "../../dfe-transforms/src/filebeat/osquery_manager_result/mod.rs"]
pub mod osquery_manager_result;
#[path = "../../dfe-transforms/src/filebeat/osquery_result/mod.rs"]
pub mod osquery_result;
#[path = "../../dfe-transforms/src/filebeat/panw/mod.rs"]
pub mod panw;
#[path = "../../dfe-transforms/src/filebeat/panw_cortex_xdr_alerts/mod.rs"]
pub mod panw_cortex_xdr_alerts;
#[path = "../../dfe-transforms/src/filebeat/panw_cortex_xdr_event/mod.rs"]
pub mod panw_cortex_xdr_event;
#[path = "../../dfe-transforms/src/filebeat/panw_cortex_xdr_incidents/mod.rs"]
pub mod panw_cortex_xdr_incidents;
#[path = "../../dfe-transforms/src/filebeat/panw_metrics_system/mod.rs"]
pub mod panw_metrics_system;
#[path = "../../dfe-transforms/src/filebeat/pfsense_log/mod.rs"]
pub mod pfsense_log;
#[path = "../../dfe-transforms/src/filebeat/php_fpm_pool/mod.rs"]
pub mod php_fpm_pool;
#[path = "../../dfe-transforms/src/filebeat/php_fpm_process/mod.rs"]
pub mod php_fpm_process;
#[path = "../../dfe-transforms/src/filebeat/ping_federate_admin/mod.rs"]
pub mod ping_federate_admin;
#[path = "../../dfe-transforms/src/filebeat/ping_federate_audit/mod.rs"]
pub mod ping_federate_audit;
#[path = "../../dfe-transforms/src/filebeat/ping_one_audit/mod.rs"]
pub mod ping_one_audit;
#[path = "../../dfe-transforms/src/filebeat/platform_observability_kibana_audit/mod.rs"]
pub mod platform_observability_kibana_audit;
#[path = "../../dfe-transforms/src/filebeat/platform_observability_kibana_log/mod.rs"]
pub mod platform_observability_kibana_log;
#[path = "../../dfe-transforms/src/filebeat/postgresql_activity/mod.rs"]
pub mod postgresql_activity;
#[path = "../../dfe-transforms/src/filebeat/postgresql_log/mod.rs"]
pub mod postgresql_log;
#[path = "../../dfe-transforms/src/filebeat/pps_log/mod.rs"]
pub mod pps_log;
#[path = "../../dfe-transforms/src/filebeat/prisma_access_event/mod.rs"]
pub mod prisma_access_event;
#[path = "../../dfe-transforms/src/filebeat/prisma_cloud_alert/mod.rs"]
pub mod prisma_cloud_alert;
#[path = "../../dfe-transforms/src/filebeat/prisma_cloud_audit/mod.rs"]
pub mod prisma_cloud_audit;
#[path = "../../dfe-transforms/src/filebeat/prisma_cloud_host/mod.rs"]
pub mod prisma_cloud_host;
#[path = "../../dfe-transforms/src/filebeat/prisma_cloud_host_profile/mod.rs"]
pub mod prisma_cloud_host_profile;
#[path = "../../dfe-transforms/src/filebeat/prisma_cloud_incident_audit/mod.rs"]
pub mod prisma_cloud_incident_audit;
#[path = "../../dfe-transforms/src/filebeat/prisma_cloud_misconfiguration/mod.rs"]
pub mod prisma_cloud_misconfiguration;
#[path = "../../dfe-transforms/src/filebeat/prisma_cloud_vulnerability/mod.rs"]
pub mod prisma_cloud_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/prometheus_collector/mod.rs"]
pub mod prometheus_collector;
#[path = "../../dfe-transforms/src/filebeat/prometheus_query/mod.rs"]
pub mod prometheus_query;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_365totalprotection_email/mod.rs"]
pub mod proofpoint_365totalprotection_email;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_essentials_threat/mod.rs"]
pub mod proofpoint_essentials_threat;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_itm_report/mod.rs"]
pub mod proofpoint_itm_report;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_on_demand_audit/mod.rs"]
pub mod proofpoint_on_demand_audit;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_on_demand_mail/mod.rs"]
pub mod proofpoint_on_demand_mail;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_on_demand_message/mod.rs"]
pub mod proofpoint_on_demand_message;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_tap_clicks_blocked/mod.rs"]
pub mod proofpoint_tap_clicks_blocked;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_tap_clicks_permitted/mod.rs"]
pub mod proofpoint_tap_clicks_permitted;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_tap_message_blocked/mod.rs"]
pub mod proofpoint_tap_message_blocked;
#[path = "../../dfe-transforms/src/filebeat/proofpoint_tap_message_delivered/mod.rs"]
pub mod proofpoint_tap_message_delivered;
#[path = "../../dfe-transforms/src/filebeat/proxysg_log/mod.rs"]
pub mod proxysg_log;
#[path = "../../dfe-transforms/src/filebeat/pulse_connect_secure_log/mod.rs"]
pub mod pulse_connect_secure_log;
#[path = "../../dfe-transforms/src/filebeat/qnap_nas_log/mod.rs"]
pub mod qnap_nas_log;
