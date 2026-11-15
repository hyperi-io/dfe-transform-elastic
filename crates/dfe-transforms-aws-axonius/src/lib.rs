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

//! Generated filebeat transforms for the packages from `aws` to `axonius`.
//!
//! The modules stay in `dfe-transforms/src/filebeat`, where the generator writes
//! them, and `dfe-transforms` re-exports this crate under `filebeat`.

#[path = "../../dfe-transforms/src/filebeat/aws_apigateway_logs/mod.rs"]
pub mod aws_apigateway_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_awshealth/mod.rs"]
pub mod aws_awshealth;
#[path = "../../dfe-transforms/src/filebeat/aws_bedrock_agentcore_gateway_application_logs/mod.rs"]
pub mod aws_bedrock_agentcore_gateway_application_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_bedrock_agentcore_memory_application_logs/mod.rs"]
pub mod aws_bedrock_agentcore_memory_application_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_bedrock_agentcore_metrics/mod.rs"]
pub mod aws_bedrock_agentcore_metrics;
#[path = "../../dfe-transforms/src/filebeat/aws_bedrock_agentcore_runtime_application_logs/mod.rs"]
pub mod aws_bedrock_agentcore_runtime_application_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_bedrock_guardrails/mod.rs"]
pub mod aws_bedrock_guardrails;
#[path = "../../dfe-transforms/src/filebeat/aws_bedrock_invocation/mod.rs"]
pub mod aws_bedrock_invocation;
#[path = "../../dfe-transforms/src/filebeat/aws_bedrock_runtime/mod.rs"]
pub mod aws_bedrock_runtime;
#[path = "../../dfe-transforms/src/filebeat/aws_billing/mod.rs"]
pub mod aws_billing;
#[path = "../../dfe-transforms/src/filebeat/aws_billing_cur/mod.rs"]
pub mod aws_billing_cur;
#[path = "../../dfe-transforms/src/filebeat/aws_cloudfront_logs/mod.rs"]
pub mod aws_cloudfront_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_cloudtrail/mod.rs"]
pub mod aws_cloudtrail;
#[path = "../../dfe-transforms/src/filebeat/aws_cloudwatch_logs/mod.rs"]
pub mod aws_cloudwatch_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_cloudwatch_metrics/mod.rs"]
pub mod aws_cloudwatch_metrics;
#[path = "../../dfe-transforms/src/filebeat/aws_config/mod.rs"]
pub mod aws_config;
#[path = "../../dfe-transforms/src/filebeat/aws_dynamodb/mod.rs"]
pub mod aws_dynamodb;
#[path = "../../dfe-transforms/src/filebeat/aws_ec2_logs/mod.rs"]
pub mod aws_ec2_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_ec2_metrics/mod.rs"]
pub mod aws_ec2_metrics;
#[path = "../../dfe-transforms/src/filebeat/aws_elb_logs/mod.rs"]
pub mod aws_elb_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_emr_logs/mod.rs"]
pub mod aws_emr_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_firewall_logs/mod.rs"]
pub mod aws_firewall_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_guardduty/mod.rs"]
pub mod aws_guardduty;
#[path = "../../dfe-transforms/src/filebeat/aws_inspector/mod.rs"]
pub mod aws_inspector;
#[path = "../../dfe-transforms/src/filebeat/aws_kafka_metrics/mod.rs"]
pub mod aws_kafka_metrics;
#[path = "../../dfe-transforms/src/filebeat/aws_kinesis/mod.rs"]
pub mod aws_kinesis;
#[path = "../../dfe-transforms/src/filebeat/aws_lambda/mod.rs"]
pub mod aws_lambda;
#[path = "../../dfe-transforms/src/filebeat/aws_lambda_logs/mod.rs"]
pub mod aws_lambda_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_mq_activemq_audit_logs/mod.rs"]
pub mod aws_mq_activemq_audit_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_mq_activemq_general_logs/mod.rs"]
pub mod aws_mq_activemq_general_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_mq_activemq_metrics/mod.rs"]
pub mod aws_mq_activemq_metrics;
#[path = "../../dfe-transforms/src/filebeat/aws_mq_rabbitmq_general_logs/mod.rs"]
pub mod aws_mq_rabbitmq_general_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_mq_rabbitmq_metrics/mod.rs"]
pub mod aws_mq_rabbitmq_metrics;
#[path = "../../dfe-transforms/src/filebeat/aws_natgateway/mod.rs"]
pub mod aws_natgateway;
#[path = "../../dfe-transforms/src/filebeat/aws_rds/mod.rs"]
pub mod aws_rds;
#[path = "../../dfe-transforms/src/filebeat/aws_redshift/mod.rs"]
pub mod aws_redshift;
#[path = "../../dfe-transforms/src/filebeat/aws_route53_public_logs/mod.rs"]
pub mod aws_route53_public_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_route53_resolver_logs/mod.rs"]
pub mod aws_route53_resolver_logs;
#[path = "../../dfe-transforms/src/filebeat/aws_s3_daily_storage/mod.rs"]
pub mod aws_s3_daily_storage;
#[path = "../../dfe-transforms/src/filebeat/aws_s3_request/mod.rs"]
pub mod aws_s3_request;
#[path = "../../dfe-transforms/src/filebeat/aws_s3access/mod.rs"]
pub mod aws_s3access;
#[path = "../../dfe-transforms/src/filebeat/aws_securityhub_finding/mod.rs"]
pub mod aws_securityhub_finding;
#[path = "../../dfe-transforms/src/filebeat/aws_securityhub_findings/mod.rs"]
pub mod aws_securityhub_findings;
#[path = "../../dfe-transforms/src/filebeat/aws_securityhub_findings_full_posture/mod.rs"]
pub mod aws_securityhub_findings_full_posture;
#[path = "../../dfe-transforms/src/filebeat/aws_securityhub_insights/mod.rs"]
pub mod aws_securityhub_insights;
#[path = "../../dfe-transforms/src/filebeat/aws_sqs/mod.rs"]
pub mod aws_sqs;
#[path = "../../dfe-transforms/src/filebeat/aws_transitgateway/mod.rs"]
pub mod aws_transitgateway;
#[path = "../../dfe-transforms/src/filebeat/aws_usage/mod.rs"]
pub mod aws_usage;
#[path = "../../dfe-transforms/src/filebeat/aws_vpcflow/mod.rs"]
pub mod aws_vpcflow;
#[path = "../../dfe-transforms/src/filebeat/aws_vpn/mod.rs"]
pub mod aws_vpn;
#[path = "../../dfe-transforms/src/filebeat/aws_waf/mod.rs"]
pub mod aws_waf;
#[path = "../../dfe-transforms/src/filebeat/awsfirehose_logs/mod.rs"]
pub mod awsfirehose_logs;
#[path = "../../dfe-transforms/src/filebeat/awsfirehose_metrics/mod.rs"]
pub mod awsfirehose_metrics;
#[path = "../../dfe-transforms/src/filebeat/axonius_adapter/mod.rs"]
pub mod axonius_adapter;
#[path = "../../dfe-transforms/src/filebeat/axonius_alert_finding/mod.rs"]
pub mod axonius_alert_finding;
#[path = "../../dfe-transforms/src/filebeat/axonius_application/mod.rs"]
pub mod axonius_application;
#[path = "../../dfe-transforms/src/filebeat/axonius_compute/mod.rs"]
pub mod axonius_compute;
#[path = "../../dfe-transforms/src/filebeat/axonius_exposure/mod.rs"]
pub mod axonius_exposure;
#[path = "../../dfe-transforms/src/filebeat/axonius_gateway/mod.rs"]
pub mod axonius_gateway;
#[path = "../../dfe-transforms/src/filebeat/axonius_identity/mod.rs"]
pub mod axonius_identity;
#[path = "../../dfe-transforms/src/filebeat/axonius_incident/mod.rs"]
pub mod axonius_incident;
#[path = "../../dfe-transforms/src/filebeat/axonius_network/mod.rs"]
pub mod axonius_network;
#[path = "../../dfe-transforms/src/filebeat/axonius_storage/mod.rs"]
pub mod axonius_storage;
#[path = "../../dfe-transforms/src/filebeat/axonius_ticket/mod.rs"]
pub mod axonius_ticket;
#[path = "../../dfe-transforms/src/filebeat/axonius_user/mod.rs"]
pub mod axonius_user;
