// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Source name to transform lookup.
//!
//! The name is the module path with dots, e.g. `filebeat.okta.default`.
//! Resolution happens once at startup, not per event.

use dfe_runtime::Transform;
use dfe_transforms::filebeat;

use crate::envelope::Envelope;

/// What a syslog-origin pipeline expects to find in `message`.
///
/// The two families disagree, and getting it wrong is silent: a header handed
/// to a body-only pipeline corrupts its first field, and a body handed to a
/// line pipeline fails its first grok.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framing {
    /// The whole syslog line, header included -- the pipeline groks it out.
    /// `fortinet` wants `<PRI>`; `cisco_ios` and `cisco_nexus` want more.
    Line,
    /// The MSG body alone. `panw` reads it as CSV and `cisco_meraki` as
    /// key-value, so a prefixed header would corrupt the first field.
    Body,
}

/// Every way a source's payload can reach this service.
///
/// The transform is the same whichever it is -- only the wrapper differs, and
/// `envelope` unwraps it -- so this says which wrappers are ACTUALLY available
/// for a given source rather than assuming Elastic's.
///
/// `Beats` is always available: every source here has an Elastic integration.
/// `Receiver` is available when a device pushes the data, which the
/// integration declares by shipping tcp/udp agent streams. `Fetcher` is
/// available when Elastic's agent input is a pure transport -- `httpjson`,
/// `cel`, `aws-s3`, `azure-eventhub`, `streaming` -- because then the ingest
/// pipeline does all the parsing and dfe-fetcher can obtain the same bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Intake {
    /// The wrappers this source accepts, `Beats` always among them.
    pub envelopes: &'static [Envelope],
    /// What a syslog delivery must leave in `message`. `None` unless the
    /// source accepts [`Envelope::Receiver`].
    pub framing: Option<Framing>,
}

impl Intake {
    /// Whether `envelope` is a way this source can be delivered.
    #[must_use]
    pub fn accepts(&self, envelope: Envelope) -> bool {
        self.envelopes.contains(&envelope)
    }
}

/// Beats and nothing else.
///
/// The agent input is neither a pure transport nor something a device pushes:
/// an ETW trace or a Windows event log is read off the host by the agent, and
/// there is no other way to the same bytes.
const fn agent_only() -> Intake {
    Intake {
        envelopes: &[Envelope::Beats],
        framing: None,
    }
}

/// Beats plus a device pushing into dfe-receiver.
const fn pushed(framing: Framing) -> Intake {
    Intake {
        envelopes: &[Envelope::Beats, Envelope::Receiver],
        framing: Some(framing),
    }
}

/// Beats plus us fetching the same bytes the agent would.
const fn fetched() -> Intake {
    Intake {
        envelopes: &[Envelope::Beats, Envelope::Fetcher],
        framing: None,
    }
}

/// Every transform this service can run, keyed by source name.
///
/// Sorted so the `sources()` listing is stable. The entries are hand-wired
/// because each names a Rust type, and `sources.yaml` is asserted against them
/// so a source declared there and never wired here fails the build.
///
/// The fourth element is the data stream this source's events belong to,
/// `<package>.<data_stream>`. Not derivable from the source name -- fortinet's
/// package is `fortinet_fortigate` and one azure package is four of our
/// modules -- so it is declared alongside the rest.
static TRANSFORMS: &[(&str, &(dyn Transform + Sync), Intake, &str)] = &[
    (
        "filebeat.aws_apigateway_logs.default",
        &filebeat::aws_apigateway_logs::default::Default,
        fetched(),
        "aws.apigateway_logs",
    ),
    (
        "filebeat.aws_awshealth.default",
        &filebeat::aws_awshealth::default::Default,
        fetched(),
        "aws.awshealth",
    ),
    (
        "filebeat.aws_billing.default",
        &filebeat::aws_billing::default::Default,
        fetched(),
        "aws.billing",
    ),
    (
        "filebeat.aws_cloudfront_logs.default",
        &filebeat::aws_cloudfront_logs::default::Default,
        fetched(),
        "aws.cloudfront_logs",
    ),
    (
        "filebeat.aws_cloudtrail.default",
        &filebeat::aws_cloudtrail::default::Default,
        fetched(),
        "aws.cloudtrail",
    ),
    (
        "filebeat.aws_cloudwatch_logs.default",
        &filebeat::aws_cloudwatch_logs::default::Default,
        fetched(),
        "aws.cloudwatch_logs",
    ),
    (
        "filebeat.aws_cloudwatch_metrics.default",
        &filebeat::aws_cloudwatch_metrics::default::Default,
        fetched(),
        "aws.cloudwatch_metrics",
    ),
    (
        "filebeat.aws_config.default",
        &filebeat::aws_config::default::Default,
        fetched(),
        "aws.config",
    ),
    (
        "filebeat.aws_dynamodb.default",
        &filebeat::aws_dynamodb::default::Default,
        fetched(),
        "aws.dynamodb",
    ),
    (
        "filebeat.aws_ec2_logs.default",
        &filebeat::aws_ec2_logs::default::Default,
        fetched(),
        "aws.ec2_logs",
    ),
    (
        "filebeat.aws_ec2_metrics.default",
        &filebeat::aws_ec2_metrics::default::Default,
        fetched(),
        "aws.ec2_metrics",
    ),
    (
        "filebeat.aws_elb_logs.default",
        &filebeat::aws_elb_logs::default::Default,
        fetched(),
        "aws.elb_logs",
    ),
    (
        "filebeat.aws_emr_logs.default",
        &filebeat::aws_emr_logs::default::Default,
        fetched(),
        "aws.emr_logs",
    ),
    (
        "filebeat.aws_firewall_logs.default",
        &filebeat::aws_firewall_logs::default::Default,
        fetched(),
        "aws.firewall_logs",
    ),
    (
        "filebeat.aws_guardduty.default",
        &filebeat::aws_guardduty::default::Default,
        fetched(),
        "aws.guardduty",
    ),
    (
        "filebeat.aws_inspector.default",
        &filebeat::aws_inspector::default::Default,
        fetched(),
        "aws.inspector",
    ),
    (
        "filebeat.aws_kafka_metrics.default",
        &filebeat::aws_kafka_metrics::default::Default,
        fetched(),
        "aws.kafka_metrics",
    ),
    (
        "filebeat.aws_kinesis.default",
        &filebeat::aws_kinesis::default::Default,
        fetched(),
        "aws.kinesis",
    ),
    (
        "filebeat.aws_lambda.default",
        &filebeat::aws_lambda::default::Default,
        fetched(),
        "aws.lambda",
    ),
    (
        "filebeat.aws_lambda_logs.default",
        &filebeat::aws_lambda_logs::default::Default,
        fetched(),
        "aws.lambda_logs",
    ),
    (
        "filebeat.aws_natgateway.default",
        &filebeat::aws_natgateway::default::Default,
        fetched(),
        "aws.natgateway",
    ),
    (
        "filebeat.aws_rds.default",
        &filebeat::aws_rds::default::Default,
        fetched(),
        "aws.rds",
    ),
    (
        "filebeat.aws_redshift.default",
        &filebeat::aws_redshift::default::Default,
        fetched(),
        "aws.redshift",
    ),
    (
        "filebeat.aws_route53_public_logs.default",
        &filebeat::aws_route53_public_logs::default::Default,
        fetched(),
        "aws.route53_public_logs",
    ),
    (
        "filebeat.aws_route53_resolver_logs.default",
        &filebeat::aws_route53_resolver_logs::default::Default,
        fetched(),
        "aws.route53_resolver_logs",
    ),
    (
        "filebeat.aws_s3_daily_storage.default",
        &filebeat::aws_s3_daily_storage::default::Default,
        fetched(),
        "aws.s3_daily_storage",
    ),
    (
        "filebeat.aws_s3_request.default",
        &filebeat::aws_s3_request::default::Default,
        fetched(),
        "aws.s3_request",
    ),
    (
        "filebeat.aws_s3access.default",
        &filebeat::aws_s3access::default::Default,
        fetched(),
        "aws.s3access",
    ),
    (
        "filebeat.aws_securityhub_findings.default",
        &filebeat::aws_securityhub_findings::default::Default,
        fetched(),
        "aws.securityhub_findings",
    ),
    (
        "filebeat.aws_securityhub_findings_full_posture.default",
        &filebeat::aws_securityhub_findings_full_posture::default::Default,
        fetched(),
        "aws.securityhub_findings_full_posture",
    ),
    (
        "filebeat.aws_securityhub_insights.default",
        &filebeat::aws_securityhub_insights::default::Default,
        fetched(),
        "aws.securityhub_insights",
    ),
    (
        "filebeat.aws_sqs.default",
        &filebeat::aws_sqs::default::Default,
        fetched(),
        "aws.sqs",
    ),
    (
        "filebeat.aws_transitgateway.default",
        &filebeat::aws_transitgateway::default::Default,
        fetched(),
        "aws.transitgateway",
    ),
    (
        "filebeat.aws_usage.default",
        &filebeat::aws_usage::default::Default,
        fetched(),
        "aws.usage",
    ),
    (
        "filebeat.aws_vpcflow.default",
        &filebeat::aws_vpcflow::default::Default,
        fetched(),
        "aws.vpcflow",
    ),
    (
        "filebeat.aws_vpn.default",
        &filebeat::aws_vpn::default::Default,
        fetched(),
        "aws.vpn",
    ),
    (
        "filebeat.aws_waf.default",
        &filebeat::aws_waf::default::Default,
        fetched(),
        "aws.waf",
    ),
    (
        "filebeat.azure_activitylogs.default",
        &filebeat::azure_activitylogs::default::Default,
        fetched(),
        "azure.activitylogs",
    ),
    (
        "filebeat.azure_auditlogs.default",
        &filebeat::azure_auditlogs::default::Default,
        fetched(),
        "azure.auditlogs",
    ),
    (
        "filebeat.azure_platformlogs.default",
        &filebeat::azure_platformlogs::default::Default,
        fetched(),
        "azure.platformlogs",
    ),
    (
        "filebeat.azure_signinlogs.default",
        &filebeat::azure_signinlogs::default::Default,
        fetched(),
        "azure.signinlogs",
    ),
    (
        "filebeat.checkpoint.default",
        &filebeat::checkpoint::default::Default,
        pushed(Framing::Line),
        "checkpoint.firewall",
    ),
    (
        "filebeat.cisco_asa.default",
        &filebeat::cisco_asa::default::Default,
        pushed(Framing::Line),
        "cisco_asa.log",
    ),
    (
        "filebeat.cisco_ftd.default",
        &filebeat::cisco_ftd::default::Default,
        pushed(Framing::Line),
        "cisco_ftd.log",
    ),
    (
        "filebeat.cisco_ios.default",
        &filebeat::cisco_ios::default::Default,
        pushed(Framing::Line),
        "cisco_ios.log",
    ),
    (
        "filebeat.cisco_meraki.default",
        &filebeat::cisco_meraki::default::Default,
        pushed(Framing::Body),
        "cisco_meraki.log",
    ),
    (
        "filebeat.cisco_nexus.default",
        &filebeat::cisco_nexus::default::Default,
        pushed(Framing::Line),
        "cisco_nexus.log",
    ),
    (
        "filebeat.cisco_umbrella.default",
        &filebeat::cisco_umbrella::default::Default,
        fetched(),
        "cisco_umbrella.log",
    ),
    (
        "filebeat.crowdstrike.default",
        &filebeat::crowdstrike::default::Default,
        fetched(),
        "crowdstrike.falcon",
    ),
    (
        "filebeat.entityanalytics_entra_id.default",
        &filebeat::entityanalytics_entra_id::default::Default,
        fetched(),
        "entityanalytics_entra_id.entity",
    ),
    (
        "filebeat.entityanalytics_entra_id.device",
        &filebeat::entityanalytics_entra_id::device::Device,
        fetched(),
        "entityanalytics_entra_id.entity",
    ),
    (
        "filebeat.entityanalytics_entra_id.user",
        &filebeat::entityanalytics_entra_id::user::User,
        fetched(),
        "entityanalytics_entra_id.entity",
    ),
    (
        "filebeat.fortinet.default",
        &filebeat::fortinet::default::Default,
        pushed(Framing::Line),
        "fortinet_fortigate.log",
    ),
    (
        "filebeat.gcp_audit.default",
        &filebeat::gcp_audit::default::Default,
        fetched(),
        "gcp.audit",
    ),
    (
        "filebeat.gcp_billing.default",
        &filebeat::gcp_billing::default::Default,
        fetched(),
        "gcp.billing",
    ),
    (
        "filebeat.gcp_cloudrun_metrics.default",
        &filebeat::gcp_cloudrun_metrics::default::Default,
        fetched(),
        "gcp.cloudrun_metrics",
    ),
    (
        "filebeat.gcp_cloudsql_mysql.default",
        &filebeat::gcp_cloudsql_mysql::default::Default,
        fetched(),
        "gcp.cloudsql_mysql",
    ),
    (
        "filebeat.gcp_cloudsql_postgresql.default",
        &filebeat::gcp_cloudsql_postgresql::default::Default,
        fetched(),
        "gcp.cloudsql_postgresql",
    ),
    (
        "filebeat.gcp_cloudsql_sqlserver.default",
        &filebeat::gcp_cloudsql_sqlserver::default::Default,
        fetched(),
        "gcp.cloudsql_sqlserver",
    ),
    (
        "filebeat.gcp_compute.default",
        &filebeat::gcp_compute::default::Default,
        fetched(),
        "gcp.compute",
    ),
    (
        "filebeat.gcp_dataproc.default",
        &filebeat::gcp_dataproc::default::Default,
        fetched(),
        "gcp.dataproc",
    ),
    (
        "filebeat.gcp_dns.default",
        &filebeat::gcp_dns::default::Default,
        fetched(),
        "gcp.dns",
    ),
    (
        "filebeat.gcp_firestore.default",
        &filebeat::gcp_firestore::default::Default,
        fetched(),
        "gcp.firestore",
    ),
    (
        "filebeat.gcp_firewall.default",
        &filebeat::gcp_firewall::default::Default,
        fetched(),
        "gcp.firewall",
    ),
    (
        "filebeat.gcp_gke.default",
        &filebeat::gcp_gke::default::Default,
        fetched(),
        "gcp.gke",
    ),
    (
        "filebeat.gcp_loadbalancing_logs.default",
        &filebeat::gcp_loadbalancing_logs::default::Default,
        fetched(),
        "gcp.loadbalancing_logs",
    ),
    (
        "filebeat.gcp_loadbalancing_metrics.default",
        &filebeat::gcp_loadbalancing_metrics::default::Default,
        fetched(),
        "gcp.loadbalancing_metrics",
    ),
    (
        "filebeat.gcp_pubsub.default",
        &filebeat::gcp_pubsub::default::Default,
        fetched(),
        "gcp.pubsub",
    ),
    (
        "filebeat.gcp_redis.default",
        &filebeat::gcp_redis::default::Default,
        fetched(),
        "gcp.redis",
    ),
    (
        "filebeat.gcp_storage.default",
        &filebeat::gcp_storage::default::Default,
        fetched(),
        "gcp.storage",
    ),
    (
        "filebeat.gcp_vpcflow.default",
        &filebeat::gcp_vpcflow::default::Default,
        fetched(),
        "gcp.vpcflow",
    ),
    (
        "filebeat.m365_defender_alert.default",
        &filebeat::m365_defender_alert::default::Default,
        fetched(),
        "m365_defender.alert",
    ),
    (
        "filebeat.m365_defender_event.default",
        &filebeat::m365_defender_event::default::Default,
        fetched(),
        "m365_defender.event",
    ),
    (
        "filebeat.m365_defender_incident.default",
        &filebeat::m365_defender_incident::default::Default,
        fetched(),
        "m365_defender.incident",
    ),
    (
        "filebeat.m365_defender_vulnerability.default",
        &filebeat::m365_defender_vulnerability::default::Default,
        fetched(),
        "m365_defender.vulnerability",
    ),
    (
        "filebeat.microsoft_defender_endpoint_log.default",
        &filebeat::microsoft_defender_endpoint_log::default::Default,
        fetched(),
        "microsoft_defender_endpoint.log",
    ),
    (
        "filebeat.microsoft_defender_endpoint_machine.default",
        &filebeat::microsoft_defender_endpoint_machine::default::Default,
        fetched(),
        "microsoft_defender_endpoint.machine",
    ),
    (
        "filebeat.microsoft_defender_endpoint_machine_action.default",
        &filebeat::microsoft_defender_endpoint_machine_action::default::Default,
        fetched(),
        "microsoft_defender_endpoint.machine_action",
    ),
    (
        "filebeat.microsoft_defender_endpoint_vulnerability.default",
        &filebeat::microsoft_defender_endpoint_vulnerability::default::Default,
        fetched(),
        "microsoft_defender_endpoint.vulnerability",
    ),
    (
        "filebeat.microsoft_dnsserver_analytical.default",
        &filebeat::microsoft_dnsserver_analytical::default::Default,
        agent_only(),
        "microsoft_dnsserver.analytical",
    ),
    (
        "filebeat.microsoft_dnsserver_audit.default",
        &filebeat::microsoft_dnsserver_audit::default::Default,
        agent_only(),
        "microsoft_dnsserver.audit",
    ),
    (
        "filebeat.mimecast_archive_search_logs.default",
        &filebeat::mimecast_archive_search_logs::default::Default,
        fetched(),
        "mimecast.archive_search_logs",
    ),
    (
        "filebeat.mimecast_audit_events.default",
        &filebeat::mimecast_audit_events::default::Default,
        fetched(),
        "mimecast.audit_events",
    ),
    (
        "filebeat.mimecast_cloud_integrated_logs.default",
        &filebeat::mimecast_cloud_integrated_logs::default::Default,
        fetched(),
        "mimecast.cloud_integrated_logs",
    ),
    (
        "filebeat.mimecast_dlp_logs.default",
        &filebeat::mimecast_dlp_logs::default::Default,
        fetched(),
        "mimecast.dlp_logs",
    ),
    (
        "filebeat.mimecast_message_release_logs.default",
        &filebeat::mimecast_message_release_logs::default::Default,
        fetched(),
        "mimecast.message_release_logs",
    ),
    (
        "filebeat.mimecast_siem_logs.default",
        &filebeat::mimecast_siem_logs::default::Default,
        fetched(),
        "mimecast.siem_logs",
    ),
    (
        "filebeat.mimecast_siem_logs.v1_pipeline",
        &filebeat::mimecast_siem_logs::v1_pipeline::V1Pipeline,
        fetched(),
        "mimecast.siem_logs",
    ),
    (
        "filebeat.mimecast_siem_logs.v2_pipeline",
        &filebeat::mimecast_siem_logs::v2_pipeline::V2Pipeline,
        fetched(),
        "mimecast.siem_logs",
    ),
    (
        "filebeat.mimecast_threat_intel_malware_customer.default",
        &filebeat::mimecast_threat_intel_malware_customer::default::Default,
        fetched(),
        "mimecast.threat_intel_malware_customer",
    ),
    (
        "filebeat.mimecast_threat_intel_malware_grid.default",
        &filebeat::mimecast_threat_intel_malware_grid::default::Default,
        fetched(),
        "mimecast.threat_intel_malware_grid",
    ),
    (
        "filebeat.mimecast_ttp_ap_logs.default",
        &filebeat::mimecast_ttp_ap_logs::default::Default,
        fetched(),
        "mimecast.ttp_ap_logs",
    ),
    (
        "filebeat.mimecast_ttp_ip_logs.default",
        &filebeat::mimecast_ttp_ip_logs::default::Default,
        fetched(),
        "mimecast.ttp_ip_logs",
    ),
    (
        "filebeat.mimecast_ttp_url_logs.default",
        &filebeat::mimecast_ttp_url_logs::default::Default,
        fetched(),
        "mimecast.ttp_url_logs",
    ),
    (
        "filebeat.o365.default",
        &filebeat::o365::default::Default,
        fetched(),
        "o365.audit",
    ),
    (
        "filebeat.okta.default",
        &filebeat::okta::default::Default,
        fetched(),
        "okta.system",
    ),
    // `panw.default` routes on log type and holds the CSV parse and converts
    // the per-type entries depend on. The per-type entries suit a feed already
    // narrowed to one log type.
    (
        "filebeat.panw.audit",
        &filebeat::panw::audit::Audit,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.authentication",
        &filebeat::panw::authentication::Authentication,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.correlated_event",
        &filebeat::panw::correlated_event::CorrelatedEvent,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.decryption",
        &filebeat::panw::decryption::Decryption,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.default",
        &filebeat::panw::default::Default,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.globalprotect",
        &filebeat::panw::globalprotect::Globalprotect,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.gtp",
        &filebeat::panw::gtp::Gtp,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.hipmatch",
        &filebeat::panw::hipmatch::Hipmatch,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.ip_tag",
        &filebeat::panw::ip_tag::IpTag,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.sctp",
        &filebeat::panw::sctp::Sctp,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.system",
        &filebeat::panw::system::System,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.traffic",
        &filebeat::panw::traffic::Traffic,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.tunnel_inspection",
        &filebeat::panw::tunnel_inspection::TunnelInspection,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.panw.userid",
        &filebeat::panw::userid::Userid,
        pushed(Framing::Body),
        "panw.panos",
    ),
    (
        "filebeat.proofpoint_on_demand_audit.default",
        &filebeat::proofpoint_on_demand_audit::default::Default,
        fetched(),
        "proofpoint_on_demand.audit",
    ),
    (
        "filebeat.proofpoint_on_demand_mail.default",
        &filebeat::proofpoint_on_demand_mail::default::Default,
        fetched(),
        "proofpoint_on_demand.mail",
    ),
    (
        "filebeat.proofpoint_on_demand_message.default",
        &filebeat::proofpoint_on_demand_message::default::Default,
        fetched(),
        "proofpoint_on_demand.message",
    ),
    (
        "filebeat.sentinel_one_activity.default",
        &filebeat::sentinel_one_activity::default::Default,
        fetched(),
        "sentinel_one.activity",
    ),
    (
        "filebeat.sentinel_one_agent.default",
        &filebeat::sentinel_one_agent::default::Default,
        fetched(),
        "sentinel_one.agent",
    ),
    (
        "filebeat.sentinel_one_alert.default",
        &filebeat::sentinel_one_alert::default::Default,
        fetched(),
        "sentinel_one.alert",
    ),
    (
        "filebeat.sentinel_one_application.default",
        &filebeat::sentinel_one_application::default::Default,
        fetched(),
        "sentinel_one.application",
    ),
    (
        "filebeat.sentinel_one_application_risk.default",
        &filebeat::sentinel_one_application_risk::default::Default,
        fetched(),
        "sentinel_one.application_risk",
    ),
    (
        "filebeat.sentinel_one_group.default",
        &filebeat::sentinel_one_group::default::Default,
        fetched(),
        "sentinel_one.group",
    ),
    (
        "filebeat.sentinel_one_threat.default",
        &filebeat::sentinel_one_threat::default::Default,
        fetched(),
        "sentinel_one.threat",
    ),
    (
        "filebeat.sentinel_one_threat_event.default",
        &filebeat::sentinel_one_threat_event::default::Default,
        fetched(),
        "sentinel_one.threat_event",
    ),
    (
        "filebeat.sentinel_one_unified_alert.default",
        &filebeat::sentinel_one_unified_alert::default::Default,
        fetched(),
        "sentinel_one.unified_alert",
    ),
    (
        "filebeat.windows_applocker_exe_and_dll.default",
        &filebeat::windows_applocker_exe_and_dll::default::Default,
        agent_only(),
        "windows.applocker_exe_and_dll",
    ),
    (
        "filebeat.windows_applocker_msi_and_script.default",
        &filebeat::windows_applocker_msi_and_script::default::Default,
        agent_only(),
        "windows.applocker_msi_and_script",
    ),
    (
        "filebeat.windows_applocker_packaged_app_deployment.default",
        &filebeat::windows_applocker_packaged_app_deployment::default::Default,
        agent_only(),
        "windows.applocker_packaged_app_deployment",
    ),
    (
        "filebeat.windows_applocker_packaged_app_execution.default",
        &filebeat::windows_applocker_packaged_app_execution::default::Default,
        agent_only(),
        "windows.applocker_packaged_app_execution",
    ),
    (
        "filebeat.windows_forwarded.default",
        &filebeat::windows_forwarded::default::Default,
        agent_only(),
        "windows.forwarded",
    ),
    (
        "filebeat.windows_powershell.default",
        &filebeat::windows_powershell::default::Default,
        agent_only(),
        "windows.powershell",
    ),
    (
        "filebeat.windows_powershell_operational.default",
        &filebeat::windows_powershell_operational::default::Default,
        agent_only(),
        "windows.powershell_operational",
    ),
    (
        "filebeat.windows_sysmon_operational.default",
        &filebeat::windows_sysmon_operational::default::Default,
        agent_only(),
        "windows.sysmon_operational",
    ),
    (
        "filebeat.windows_windows_defender.default",
        &filebeat::windows_windows_defender::default::Default,
        agent_only(),
        "windows.windows_defender",
    ),
    (
        "filebeat.zscaler_zia_alerts.default",
        &filebeat::zscaler_zia_alerts::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.alerts",
    ),
    (
        "filebeat.zscaler_zia_audit.default",
        &filebeat::zscaler_zia_audit::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.audit",
    ),
    (
        "filebeat.zscaler_zia_dns.default",
        &filebeat::zscaler_zia_dns::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.dns",
    ),
    (
        "filebeat.zscaler_zia_email_dlp.default",
        &filebeat::zscaler_zia_email_dlp::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.email_dlp",
    ),
    (
        "filebeat.zscaler_zia_endpoint_dlp.default",
        &filebeat::zscaler_zia_endpoint_dlp::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.endpoint_dlp",
    ),
    (
        "filebeat.zscaler_zia_firewall.default",
        &filebeat::zscaler_zia_firewall::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.firewall",
    ),
    (
        "filebeat.zscaler_zia_saas_security.default",
        &filebeat::zscaler_zia_saas_security::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.saas_security",
    ),
    (
        "filebeat.zscaler_zia_saas_security_activity.default",
        &filebeat::zscaler_zia_saas_security_activity::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.saas_security_activity",
    ),
    (
        "filebeat.zscaler_zia_sandbox_report.default",
        &filebeat::zscaler_zia_sandbox_report::default::Default,
        fetched(),
        "zscaler_zia.sandbox_report",
    ),
    (
        "filebeat.zscaler_zia_sandbox_verdict.default",
        &filebeat::zscaler_zia_sandbox_verdict::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.sandbox_verdict",
    ),
    (
        "filebeat.zscaler_zia_tunnel.default",
        &filebeat::zscaler_zia_tunnel::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.tunnel",
    ),
    (
        "filebeat.zscaler_zia_web.default",
        &filebeat::zscaler_zia_web::default::Default,
        pushed(Framing::Body),
        "zscaler_zia.web",
    ),
];

/// Resolve a source name to its transform.
pub fn lookup(name: &str) -> Option<&'static (dyn Transform + Sync)> {
    TRANSFORMS
        .iter()
        .find(|(key, ..)| *key == name)
        .map(|(_, t, ..)| *t)
}

/// Every way a source's payload can reach this service.
pub fn intake(name: &str) -> Option<Intake> {
    TRANSFORMS
        .iter()
        .find(|(key, ..)| *key == name)
        .map(|(_, _, intake, _)| *intake)
}

/// The data stream this source's events belong to, `<package>.<data_stream>`.
///
/// Beats and the Agent stamp this on every event; the receiver and the fetcher
/// have no way to know it, so the service supplies it from here.
pub fn dataset(name: &str) -> Option<&'static str> {
    TRANSFORMS
        .iter()
        .find(|(key, ..)| *key == name)
        .map(|(.., dataset)| *dataset)
}

/// Every source name this service accepts.
pub fn sources() -> impl Iterator<Item = &'static str> {
    TRANSFORMS.iter().map(|(key, ..)| *key)
}

/// Every source that can be delivered in `envelope`.
pub fn sources_accepting(envelope: Envelope) -> impl Iterator<Item = &'static str> {
    TRANSFORMS
        .iter()
        .filter(move |(_, _, intake, _)| intake.accepts(envelope))
        .map(|(key, ..)| *key)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn resolves_a_known_source() {
        assert!(lookup("filebeat.okta.default").is_some());
    }

    /// Which intakes each source has are declared in `sources.yaml` and checked
    /// below. All this adds is the two invariants the declaration cannot state:
    /// Beats always works, and an unknown name has no intake at all.
    ///
    /// A source may be pushed by a device or fetchable or NEITHER: an ETW trace
    /// and a Windows event log are read off the host by the agent, and there is
    /// no other route to the same bytes. What no source may be is both, which
    /// would mean one payload arriving in two different wrappers.
    #[test]
    fn every_source_accepts_beats_and_at_most_one_other() {
        assert_eq!(
            sources_accepting(Envelope::Beats).count(),
            sources().count(),
            "every source has an Elastic integration, so Beats always applies"
        );
        for (name, _, intake, _) in TRANSFORMS {
            assert!(
                !(intake.accepts(Envelope::Receiver) && intake.accepts(Envelope::Fetcher)),
                "{name} claims both a device push and a fetch"
            );
        }
        assert_eq!(intake("filebeat.nosuchthing"), None);
    }

    /// Framing describes a syslog delivery, so it exists exactly when one is
    /// possible. A fetched source carrying one would be read as a device.
    #[test]
    fn framing_is_present_exactly_when_the_receiver_is() {
        for (name, _, intake, _) in TRANSFORMS {
            assert_eq!(
                intake.accepts(Envelope::Receiver),
                intake.framing.is_some(),
                "{name} disagrees with itself about the receiver"
            );
        }
    }

    #[test]
    fn rejects_an_unknown_source() {
        assert!(lookup("filebeat.nosuchthing").is_none());
    }

    #[test]
    fn every_registered_source_resolves() {
        for name in sources() {
            assert!(lookup(name).is_some(), "{name} did not resolve");
        }
    }

    /// `sources.yaml` is the one declaration per source; the four tools that
    /// used to carry their own copy all read it. This registry cannot, because
    /// every entry names a Rust type, so it is checked against it instead --
    /// a source declared and never wired here fails, and so does the reverse.
    #[test]
    fn the_registry_matches_the_source_declaration() {
        use std::collections::BTreeMap;

        #[derive(serde::Deserialize)]
        struct Declaration {
            sources: BTreeMap<String, Declared>,
        }

        #[derive(serde::Deserialize)]
        struct Declared {
            package: String,
            data_stream: String,
            intakes: Vec<String>,
            framing: Option<String>,
            transforms: Vec<String>,
        }

        const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/sources.yaml");
        let text = std::fs::read_to_string(PATH).expect("read sources.yaml");
        let declaration: Declaration = serde_yaml_ng::from_str(&text).expect("parse sources.yaml");

        // Compared as text: the declaration is the readable form, and a
        // mismatch has to name what it saw rather than a struct's Debug.
        let mut expected: Vec<(String, String)> = Vec::new();
        for (source, declared) in &declaration.sources {
            for name in &declared.intakes {
                assert!(
                    ["beats", "receiver", "fetcher"].contains(&name.as_str()),
                    "{source} declares intake {name:?}, which is not one"
                );
            }
            let described = describe(
                &declared.intakes,
                declared.framing.as_deref(),
                &format!("{}.{}", declared.package, declared.data_stream),
            );
            for transform in &declared.transforms {
                expected.push((format!("filebeat.{source}.{transform}"), described.clone()));
            }
        }
        expected.sort_by(|a, b| a.0.cmp(&b.0));

        let wired: Vec<(String, String)> = TRANSFORMS
            .iter()
            .map(|(name, _, intake, dataset)| {
                let names: Vec<String> = intake
                    .envelopes
                    .iter()
                    .map(|e| format!("{e:?}").to_lowercase())
                    .collect();
                let framing = intake.framing.map(|f| format!("{f:?}").to_lowercase());
                (
                    (*name).to_owned(),
                    describe(&names, framing.as_deref(), dataset),
                )
            })
            .collect();

        assert_eq!(
            wired, expected,
            "src/registry.rs and sources.yaml disagree on the sources or their intakes"
        );
    }

    /// One string per source, so a mismatch reads as what was declared.
    fn describe(intakes: &[String], framing: Option<&str>, dataset: &str) -> String {
        let mut names: Vec<&str> = intakes.iter().map(String::as_str).collect();
        names.sort_unstable();
        match framing {
            Some(f) => format!("{dataset} {}/{f}", names.join("+")),
            None => format!("{dataset} {}", names.join("+")),
        }
    }

    /// Every source has a dataset, and an unknown name has none.
    #[test]
    fn every_source_has_a_dataset() {
        for name in sources() {
            let dataset = dataset(name).unwrap_or_else(|| panic!("{name} has no dataset"));
            assert!(
                dataset.contains('.') && !dataset.starts_with('.') && !dataset.ends_with('.'),
                "{name}: `{dataset}` is not <package>.<data_stream>"
            );
        }
        assert_eq!(dataset("filebeat.nosuchthing"), None);
    }

    #[test]
    fn names_are_unique_and_sorted() {
        let names: Vec<&str> = sources().collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            names, sorted,
            "registry must be sorted and free of duplicates"
        );
    }
}
