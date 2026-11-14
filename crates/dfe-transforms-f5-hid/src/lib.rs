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

//! Generated filebeat transforms for the packages from `f5` to `hid`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/f5_bigip_log/mod.rs"]
pub mod f5_bigip_log;
#[path = "../../dfe-transforms/src/filebeat/falco_alerts/mod.rs"]
pub mod falco_alerts;
#[path = "../../dfe-transforms/src/filebeat/fim_event/mod.rs"]
pub mod fim_event;
#[path = "../../dfe-transforms/src/filebeat/fireeye_nx/mod.rs"]
pub mod fireeye_nx;
#[path = "../../dfe-transforms/src/filebeat/first_epss_vulnerability/mod.rs"]
pub mod first_epss_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/forcepoint_web_logs/mod.rs"]
pub mod forcepoint_web_logs;
#[path = "../../dfe-transforms/src/filebeat/forescout_event/mod.rs"]
pub mod forescout_event;
#[path = "../../dfe-transforms/src/filebeat/forescout_host/mod.rs"]
pub mod forescout_host;
#[path = "../../dfe-transforms/src/filebeat/forgerock_am_access/mod.rs"]
pub mod forgerock_am_access;
#[path = "../../dfe-transforms/src/filebeat/forgerock_am_activity/mod.rs"]
pub mod forgerock_am_activity;
#[path = "../../dfe-transforms/src/filebeat/forgerock_am_authentication/mod.rs"]
pub mod forgerock_am_authentication;
#[path = "../../dfe-transforms/src/filebeat/forgerock_am_config/mod.rs"]
pub mod forgerock_am_config;
#[path = "../../dfe-transforms/src/filebeat/forgerock_am_core/mod.rs"]
pub mod forgerock_am_core;
#[path = "../../dfe-transforms/src/filebeat/forgerock_idm_access/mod.rs"]
pub mod forgerock_idm_access;
#[path = "../../dfe-transforms/src/filebeat/forgerock_idm_activity/mod.rs"]
pub mod forgerock_idm_activity;
#[path = "../../dfe-transforms/src/filebeat/forgerock_idm_authentication/mod.rs"]
pub mod forgerock_idm_authentication;
#[path = "../../dfe-transforms/src/filebeat/forgerock_idm_config/mod.rs"]
pub mod forgerock_idm_config;
#[path = "../../dfe-transforms/src/filebeat/forgerock_idm_core/mod.rs"]
pub mod forgerock_idm_core;
#[path = "../../dfe-transforms/src/filebeat/forgerock_idm_sync/mod.rs"]
pub mod forgerock_idm_sync;
#[path = "../../dfe-transforms/src/filebeat/fortinet/mod.rs"]
pub mod fortinet;
#[path = "../../dfe-transforms/src/filebeat/fortinet_forticlient_log/mod.rs"]
pub mod fortinet_forticlient_log;
#[path = "../../dfe-transforms/src/filebeat/fortinet_fortiedr_log/mod.rs"]
pub mod fortinet_fortiedr_log;
#[path = "../../dfe-transforms/src/filebeat/fortinet_fortimail_log/mod.rs"]
pub mod fortinet_fortimail_log;
#[path = "../../dfe-transforms/src/filebeat/fortinet_fortimanager_log/mod.rs"]
pub mod fortinet_fortimanager_log;
#[path = "../../dfe-transforms/src/filebeat/fortinet_fortiproxy_log/mod.rs"]
pub mod fortinet_fortiproxy_log;
#[path = "../../dfe-transforms/src/filebeat/gcp_audit/mod.rs"]
pub mod gcp_audit;
#[path = "../../dfe-transforms/src/filebeat/gcp_billing/mod.rs"]
pub mod gcp_billing;
#[path = "../../dfe-transforms/src/filebeat/gcp_cloudrun_metrics/mod.rs"]
pub mod gcp_cloudrun_metrics;
#[path = "../../dfe-transforms/src/filebeat/gcp_cloudsql_mysql/mod.rs"]
pub mod gcp_cloudsql_mysql;
#[path = "../../dfe-transforms/src/filebeat/gcp_cloudsql_postgresql/mod.rs"]
pub mod gcp_cloudsql_postgresql;
#[path = "../../dfe-transforms/src/filebeat/gcp_cloudsql_sqlserver/mod.rs"]
pub mod gcp_cloudsql_sqlserver;
#[path = "../../dfe-transforms/src/filebeat/gcp_compute/mod.rs"]
pub mod gcp_compute;
#[path = "../../dfe-transforms/src/filebeat/gcp_dataproc/mod.rs"]
pub mod gcp_dataproc;
#[path = "../../dfe-transforms/src/filebeat/gcp_dns/mod.rs"]
pub mod gcp_dns;
#[path = "../../dfe-transforms/src/filebeat/gcp_firestore/mod.rs"]
pub mod gcp_firestore;
#[path = "../../dfe-transforms/src/filebeat/gcp_firewall/mod.rs"]
pub mod gcp_firewall;
#[path = "../../dfe-transforms/src/filebeat/gcp_gke/mod.rs"]
pub mod gcp_gke;
#[path = "../../dfe-transforms/src/filebeat/gcp_loadbalancing_logs/mod.rs"]
pub mod gcp_loadbalancing_logs;
#[path = "../../dfe-transforms/src/filebeat/gcp_loadbalancing_metrics/mod.rs"]
pub mod gcp_loadbalancing_metrics;
#[path = "../../dfe-transforms/src/filebeat/gcp_pubsub/mod.rs"]
pub mod gcp_pubsub;
#[path = "../../dfe-transforms/src/filebeat/gcp_redis/mod.rs"]
pub mod gcp_redis;
#[path = "../../dfe-transforms/src/filebeat/gcp_storage/mod.rs"]
pub mod gcp_storage;
#[path = "../../dfe-transforms/src/filebeat/gcp_vertexai_auditlogs/mod.rs"]
pub mod gcp_vertexai_auditlogs;
#[path = "../../dfe-transforms/src/filebeat/gcp_vertexai_metrics/mod.rs"]
pub mod gcp_vertexai_metrics;
#[path = "../../dfe-transforms/src/filebeat/gcp_vertexai_prompt_response_logs/mod.rs"]
pub mod gcp_vertexai_prompt_response_logs;
#[path = "../../dfe-transforms/src/filebeat/gcp_vpcflow/mod.rs"]
pub mod gcp_vpcflow;
#[path = "../../dfe-transforms/src/filebeat/gdacs_events/mod.rs"]
pub mod gdacs_events;
#[path = "../../dfe-transforms/src/filebeat/gigamon_ami/mod.rs"]
pub mod gigamon_ami;
#[path = "../../dfe-transforms/src/filebeat/github_audit/mod.rs"]
pub mod github_audit;
#[path = "../../dfe-transforms/src/filebeat/github_code_scanning/mod.rs"]
pub mod github_code_scanning;
#[path = "../../dfe-transforms/src/filebeat/github_dependabot/mod.rs"]
pub mod github_dependabot;
#[path = "../../dfe-transforms/src/filebeat/github_issues/mod.rs"]
pub mod github_issues;
#[path = "../../dfe-transforms/src/filebeat/github_secret_scanning/mod.rs"]
pub mod github_secret_scanning;
#[path = "../../dfe-transforms/src/filebeat/github_security_advisories/mod.rs"]
pub mod github_security_advisories;
#[path = "../../dfe-transforms/src/filebeat/gitlab_api/mod.rs"]
pub mod gitlab_api;
#[path = "../../dfe-transforms/src/filebeat/gitlab_application/mod.rs"]
pub mod gitlab_application;
#[path = "../../dfe-transforms/src/filebeat/gitlab_audit/mod.rs"]
pub mod gitlab_audit;
#[path = "../../dfe-transforms/src/filebeat/gitlab_auth/mod.rs"]
pub mod gitlab_auth;
#[path = "../../dfe-transforms/src/filebeat/gitlab_pages/mod.rs"]
pub mod gitlab_pages;
#[path = "../../dfe-transforms/src/filebeat/gitlab_production/mod.rs"]
pub mod gitlab_production;
#[path = "../../dfe-transforms/src/filebeat/gitlab_sidekiq/mod.rs"]
pub mod gitlab_sidekiq;
#[path = "../../dfe-transforms/src/filebeat/goflow2_sflow/mod.rs"]
pub mod goflow2_sflow;
#[path = "../../dfe-transforms/src/filebeat/golang_expvar/mod.rs"]
pub mod golang_expvar;
#[path = "../../dfe-transforms/src/filebeat/golang_heap/mod.rs"]
pub mod golang_heap;
#[path = "../../dfe-transforms/src/filebeat/google_scc_asset/mod.rs"]
pub mod google_scc_asset;
#[path = "../../dfe-transforms/src/filebeat/google_scc_audit/mod.rs"]
pub mod google_scc_audit;
#[path = "../../dfe-transforms/src/filebeat/google_scc_finding/mod.rs"]
pub mod google_scc_finding;
#[path = "../../dfe-transforms/src/filebeat/google_scc_source/mod.rs"]
pub mod google_scc_source;
#[path = "../../dfe-transforms/src/filebeat/google_secops_alert/mod.rs"]
pub mod google_secops_alert;
#[path = "../../dfe-transforms/src/filebeat/google_secops_alert_v2/mod.rs"]
pub mod google_secops_alert_v2;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_access_transparency/mod.rs"]
pub mod google_workspace_access_transparency;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_admin/mod.rs"]
pub mod google_workspace_admin;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_alert/mod.rs"]
pub mod google_workspace_alert;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_calendar/mod.rs"]
pub mod google_workspace_calendar;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_chat/mod.rs"]
pub mod google_workspace_chat;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_chrome/mod.rs"]
pub mod google_workspace_chrome;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_context_aware_access/mod.rs"]
pub mod google_workspace_context_aware_access;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_data_studio/mod.rs"]
pub mod google_workspace_data_studio;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_device/mod.rs"]
pub mod google_workspace_device;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_drive/mod.rs"]
pub mod google_workspace_drive;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_gcp/mod.rs"]
pub mod google_workspace_gcp;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_gmail/mod.rs"]
pub mod google_workspace_gmail;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_group_enterprise/mod.rs"]
pub mod google_workspace_group_enterprise;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_groups/mod.rs"]
pub mod google_workspace_groups;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_keep/mod.rs"]
pub mod google_workspace_keep;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_login/mod.rs"]
pub mod google_workspace_login;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_meet/mod.rs"]
pub mod google_workspace_meet;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_rules/mod.rs"]
pub mod google_workspace_rules;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_saml/mod.rs"]
pub mod google_workspace_saml;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_token/mod.rs"]
pub mod google_workspace_token;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_user_accounts/mod.rs"]
pub mod google_workspace_user_accounts;
#[path = "../../dfe-transforms/src/filebeat/google_workspace_vault/mod.rs"]
pub mod google_workspace_vault;
#[path = "../../dfe-transforms/src/filebeat/grafana_logs/mod.rs"]
pub mod grafana_logs;
#[path = "../../dfe-transforms/src/filebeat/grafana_metrics/mod.rs"]
pub mod grafana_metrics;
#[path = "../../dfe-transforms/src/filebeat/greenhouse_audit/mod.rs"]
pub mod greenhouse_audit;
#[path = "../../dfe-transforms/src/filebeat/hackerone_report/mod.rs"]
pub mod hackerone_report;
#[path = "../../dfe-transforms/src/filebeat/hadoop_application/mod.rs"]
pub mod hadoop_application;
#[path = "../../dfe-transforms/src/filebeat/hadoop_cluster/mod.rs"]
pub mod hadoop_cluster;
#[path = "../../dfe-transforms/src/filebeat/hadoop_datanode/mod.rs"]
pub mod hadoop_datanode;
#[path = "../../dfe-transforms/src/filebeat/hadoop_namenode/mod.rs"]
pub mod hadoop_namenode;
#[path = "../../dfe-transforms/src/filebeat/hadoop_node_manager/mod.rs"]
pub mod hadoop_node_manager;
#[path = "../../dfe-transforms/src/filebeat/haproxy_log/mod.rs"]
pub mod haproxy_log;
#[path = "../../dfe-transforms/src/filebeat/hashicorp_vault_audit/mod.rs"]
pub mod hashicorp_vault_audit;
#[path = "../../dfe-transforms/src/filebeat/hashicorp_vault_log/mod.rs"]
pub mod hashicorp_vault_log;
#[path = "../../dfe-transforms/src/filebeat/hashicorp_vault_metrics/mod.rs"]
pub mod hashicorp_vault_metrics;
#[path = "../../dfe-transforms/src/filebeat/hid_bravura_monitor_log/mod.rs"]
pub mod hid_bravura_monitor_log;
#[path = "../../dfe-transforms/src/filebeat/hid_bravura_monitor_winlog/mod.rs"]
pub mod hid_bravura_monitor_winlog;
