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

//! Generated filebeat transforms for the packages from `1password` to `authentik`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/_1password_audit_events/mod.rs"]
pub mod _1password_audit_events;
#[path = "../../dfe-transforms/src/filebeat/_1password_item_usages/mod.rs"]
pub mod _1password_item_usages;
#[path = "../../dfe-transforms/src/filebeat/_1password_signin_attempts/mod.rs"]
pub mod _1password_signin_attempts;
#[path = "../../dfe-transforms/src/filebeat/abnormal_security_ai_security_mailbox/mod.rs"]
pub mod abnormal_security_ai_security_mailbox;
#[path = "../../dfe-transforms/src/filebeat/abnormal_security_ai_security_mailbox_not_analyzed/mod.rs"]
pub mod abnormal_security_ai_security_mailbox_not_analyzed;
#[path = "../../dfe-transforms/src/filebeat/abnormal_security_audit/mod.rs"]
pub mod abnormal_security_audit;
#[path = "../../dfe-transforms/src/filebeat/abnormal_security_case/mod.rs"]
pub mod abnormal_security_case;
#[path = "../../dfe-transforms/src/filebeat/abnormal_security_threat/mod.rs"]
pub mod abnormal_security_threat;
#[path = "../../dfe-transforms/src/filebeat/abnormal_security_vendor_case/mod.rs"]
pub mod abnormal_security_vendor_case;
#[path = "../../dfe-transforms/src/filebeat/activemq_audit/mod.rs"]
pub mod activemq_audit;
#[path = "../../dfe-transforms/src/filebeat/activemq_broker/mod.rs"]
pub mod activemq_broker;
#[path = "../../dfe-transforms/src/filebeat/activemq_log/mod.rs"]
pub mod activemq_log;
#[path = "../../dfe-transforms/src/filebeat/activemq_queue/mod.rs"]
pub mod activemq_queue;
#[path = "../../dfe-transforms/src/filebeat/activemq_topic/mod.rs"]
pub mod activemq_topic;
#[path = "../../dfe-transforms/src/filebeat/admin_by_request_epm_auditlog/mod.rs"]
pub mod admin_by_request_epm_auditlog;
#[path = "../../dfe-transforms/src/filebeat/admin_by_request_epm_events/mod.rs"]
pub mod admin_by_request_epm_events;
#[path = "../../dfe-transforms/src/filebeat/agentless_hello_world_generic/mod.rs"]
pub mod agentless_hello_world_generic;
#[path = "../../dfe-transforms/src/filebeat/agentless_hello_world_mock_counter/mod.rs"]
pub mod agentless_hello_world_mock_counter;
#[path = "../../dfe-transforms/src/filebeat/airflow_statsd/mod.rs"]
pub mod airflow_statsd;
#[path = "../../dfe-transforms/src/filebeat/airlock_digital_agent/mod.rs"]
pub mod airlock_digital_agent;
#[path = "../../dfe-transforms/src/filebeat/airlock_digital_execution_histories/mod.rs"]
pub mod airlock_digital_execution_histories;
#[path = "../../dfe-transforms/src/filebeat/airlock_digital_server_activities/mod.rs"]
pub mod airlock_digital_server_activities;
#[path = "../../dfe-transforms/src/filebeat/akamai_siem/mod.rs"]
pub mod akamai_siem;
#[path = "../../dfe-transforms/src/filebeat/amazon_security_lake_event/mod.rs"]
pub mod amazon_security_lake_event;
#[path = "../../dfe-transforms/src/filebeat/anthropic_audit/mod.rs"]
pub mod anthropic_audit;
#[path = "../../dfe-transforms/src/filebeat/anthropic_metrics_cost/mod.rs"]
pub mod anthropic_metrics_cost;
#[path = "../../dfe-transforms/src/filebeat/anthropic_metrics_rate_limit/mod.rs"]
pub mod anthropic_metrics_rate_limit;
#[path = "../../dfe-transforms/src/filebeat/anthropic_metrics_usage/mod.rs"]
pub mod anthropic_metrics_usage;
#[path = "../../dfe-transforms/src/filebeat/apache_access/mod.rs"]
pub mod apache_access;
#[path = "../../dfe-transforms/src/filebeat/apache_error/mod.rs"]
pub mod apache_error;
#[path = "../../dfe-transforms/src/filebeat/apache_spark_application/mod.rs"]
pub mod apache_spark_application;
#[path = "../../dfe-transforms/src/filebeat/apache_spark_driver/mod.rs"]
pub mod apache_spark_driver;
#[path = "../../dfe-transforms/src/filebeat/apache_spark_executor/mod.rs"]
pub mod apache_spark_executor;
#[path = "../../dfe-transforms/src/filebeat/apache_spark_node/mod.rs"]
pub mod apache_spark_node;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_access/mod.rs"]
pub mod apache_tomcat_access;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_cache/mod.rs"]
pub mod apache_tomcat_cache;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_catalina/mod.rs"]
pub mod apache_tomcat_catalina;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_connection_pool/mod.rs"]
pub mod apache_tomcat_connection_pool;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_localhost/mod.rs"]
pub mod apache_tomcat_localhost;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_memory/mod.rs"]
pub mod apache_tomcat_memory;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_request/mod.rs"]
pub mod apache_tomcat_request;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_session/mod.rs"]
pub mod apache_tomcat_session;
#[path = "../../dfe-transforms/src/filebeat/apache_tomcat_thread_pool/mod.rs"]
pub mod apache_tomcat_thread_pool;
#[path = "../../dfe-transforms/src/filebeat/arista_ngfw_log/mod.rs"]
pub mod arista_ngfw_log;
#[path = "../../dfe-transforms/src/filebeat/armis_alert/mod.rs"]
pub mod armis_alert;
#[path = "../../dfe-transforms/src/filebeat/armis_device/mod.rs"]
pub mod armis_device;
#[path = "../../dfe-transforms/src/filebeat/armis_vulnerability/mod.rs"]
pub mod armis_vulnerability;
#[path = "../../dfe-transforms/src/filebeat/atlassian_bitbucket_audit/mod.rs"]
pub mod atlassian_bitbucket_audit;
#[path = "../../dfe-transforms/src/filebeat/atlassian_cloud_audit/mod.rs"]
pub mod atlassian_cloud_audit;
#[path = "../../dfe-transforms/src/filebeat/atlassian_confluence_audit/mod.rs"]
pub mod atlassian_confluence_audit;
#[path = "../../dfe-transforms/src/filebeat/atlassian_jira_audit/mod.rs"]
pub mod atlassian_jira_audit;
#[path = "../../dfe-transforms/src/filebeat/auditd_log/mod.rs"]
pub mod auditd_log;
#[path = "../../dfe-transforms/src/filebeat/auditd_manager_auditd/mod.rs"]
pub mod auditd_manager_auditd;
#[path = "../../dfe-transforms/src/filebeat/auth0_logs/mod.rs"]
pub mod auth0_logs;
#[path = "../../dfe-transforms/src/filebeat/authentik_event/mod.rs"]
pub mod authentik_event;
#[path = "../../dfe-transforms/src/filebeat/authentik_group/mod.rs"]
pub mod authentik_group;
#[path = "../../dfe-transforms/src/filebeat/authentik_user/mod.rs"]
pub mod authentik_user;
