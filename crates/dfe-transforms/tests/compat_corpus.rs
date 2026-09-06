// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Compare every transform against what Elastic's own engine produced.
//!
//! `scripts/compat.py generate` runs raw source data through the real ingest
//! pipelines in a throwaway Elasticsearch container and writes the confirmed
//! documents to `testdata/compat/<source>/<data stream>/<fixture>/`. This test
//! reads those files. It starts no container and needs no network.
//!
//! The corpus derives from Elastic-Licensed pipelines, so it is not committed.
//! A missing corpus is reported and skipped -- unlike the committed fixtures,
//! where absence is a failure.
//!
//! It carries TWO ratchets, and both are skipped under `DFE_COMPAT_ONLY`
//! because a partial run cannot say whether another source went down:
//!
//! Both live in `tests/compat-baseline.json` and are written by
//! `scripts/raise_baseline.py`: per-source field and event scores, and
//! `never_ran`, how many scripts bound a pattern and never ran it. The static
//! census and the coverage floor both count the CLAIM, so this run is the
//! only thing that sees a pattern which never applies.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use dfe_runtime::event::Event;
use dfe_runtime::testutil::diff::{DiffKind, JsonDiff, MatchMode};
use dfe_runtime::testutil::{flatten_value, policy};
use dfe_runtime::transform::{Transform, TransformResult};
use dfe_transforms::filebeat;
use rayon::prelude::*;
use serde_json::Value;

/// Where `compat.py` writes by default. `DFE_COMPAT_CORPUS` overrides it, the
/// same variable the tool reads.
fn corpus_root() -> PathBuf {
    std::env::var_os("DFE_COMPAT_CORPUS").map_or_else(
        || {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../testdata/compat")
                .to_path_buf()
        },
        PathBuf::from,
    )
}

/// The transform a capture belongs to.
///
/// The corpus is keyed by Elastic's PACKAGE and data stream, which is not our
/// source name: the azure package holds four of our modules, and fortinet's
/// package is `fortinet_fortigate`. A capture with no transform here is a gap
/// worth failing on rather than skipping.
fn transform_for(package: &str, data_stream: &str) -> Option<&'static dyn Transform> {
    Some(match (package, data_stream) {
        ("azure", "activitylogs") => &filebeat::azure_activitylogs::default::Default,
        ("azure", "auditlogs") => &filebeat::azure_auditlogs::default::Default,
        ("azure", "platformlogs") => &filebeat::azure_platformlogs::default::Default,
        ("azure", "signinlogs") => &filebeat::azure_signinlogs::default::Default,
        // Every arm names its data stream; a package wildcard would swallow the
        // next stream onboarded under it.
        ("cisco_asa", "log") => &filebeat::cisco_asa::default::Default,
        ("cisco_ftd", "log") => &filebeat::cisco_ftd::default::Default,
        ("cisco_ios", "log") => &filebeat::cisco_ios::default::Default,
        ("cisco_meraki", "log") => &filebeat::cisco_meraki::default::Default,
        ("cisco_meraki", "events") => &filebeat::cisco_meraki_events::default::Default,
        ("cisco_nexus", "log") => &filebeat::cisco_nexus::default::Default,
        ("cisco_umbrella", "log") => &filebeat::cisco_umbrella::default::Default,
        ("crowdstrike", "falcon") => &filebeat::crowdstrike::default::Default,
        ("crowdstrike", "alert") => &filebeat::crowdstrike_alert::default::Default,
        ("crowdstrike", "host") => &filebeat::crowdstrike_host::default::Default,
        ("crowdstrike", "identity_protection_assessment") => {
            &filebeat::crowdstrike_identity_protection_assessment::default::Default
        }
        ("crowdstrike", "identity_protection_timeline") => {
            &filebeat::crowdstrike_identity_protection_timeline::default::Default
        }
        ("crowdstrike", "vulnerability") => &filebeat::crowdstrike_vulnerability::default::Default,
        ("fortinet_fortigate", "log") => &filebeat::fortinet::default::Default,
        ("microsoft_dnsserver", "analytical") => {
            &filebeat::microsoft_dnsserver_analytical::default::Default
        }
        ("microsoft_dnsserver", "audit") => &filebeat::microsoft_dnsserver_audit::default::Default,
        ("o365", "audit") => &filebeat::o365::default::Default,
        ("okta", "system") => &filebeat::okta::default::Default,
        ("panw", "panos") => &filebeat::panw::default::Default,
        ("entityanalytics_entra_id", "entity") => {
            &filebeat::entityanalytics_entra_id::default::Default
        }
        ("checkpoint", "firewall") => &filebeat::checkpoint::default::Default,
        ("microsoft_defender_endpoint", "log") => {
            &filebeat::microsoft_defender_endpoint_log::default::Default
        }
        ("microsoft_defender_endpoint", "machine") => {
            &filebeat::microsoft_defender_endpoint_machine::default::Default
        }
        ("microsoft_defender_endpoint", "machine_action") => {
            &filebeat::microsoft_defender_endpoint_machine_action::default::Default
        }
        ("microsoft_defender_endpoint", "vulnerability") => {
            &filebeat::microsoft_defender_endpoint_vulnerability::default::Default
        }
        ("proofpoint_on_demand", "audit") => {
            &filebeat::proofpoint_on_demand_audit::default::Default
        }
        ("proofpoint_on_demand", "mail") => &filebeat::proofpoint_on_demand_mail::default::Default,
        ("proofpoint_on_demand", "message") => {
            &filebeat::proofpoint_on_demand_message::default::Default
        }
        ("mimecast", "archive_search_logs") => {
            &filebeat::mimecast_archive_search_logs::default::Default
        }
        ("mimecast", "audit_events") => &filebeat::mimecast_audit_events::default::Default,
        ("mimecast", "cloud_integrated_logs") => {
            &filebeat::mimecast_cloud_integrated_logs::default::Default
        }
        ("mimecast", "dlp_logs") => &filebeat::mimecast_dlp_logs::default::Default,
        ("mimecast", "message_release_logs") => {
            &filebeat::mimecast_message_release_logs::default::Default
        }
        ("mimecast", "siem_logs") => &filebeat::mimecast_siem_logs::default::Default,
        ("mimecast", "threat_intel_malware_customer") => {
            &filebeat::mimecast_threat_intel_malware_customer::default::Default
        }
        ("mimecast", "threat_intel_malware_grid") => {
            &filebeat::mimecast_threat_intel_malware_grid::default::Default
        }
        ("mimecast", "ttp_ap_logs") => &filebeat::mimecast_ttp_ap_logs::default::Default,
        ("mimecast", "ttp_ip_logs") => &filebeat::mimecast_ttp_ip_logs::default::Default,
        ("mimecast", "ttp_url_logs") => &filebeat::mimecast_ttp_url_logs::default::Default,
        ("gcp", "audit") => &filebeat::gcp_audit::default::Default,
        ("gcp", "billing") => &filebeat::gcp_billing::default::Default,
        ("gcp", "cloudrun_metrics") => &filebeat::gcp_cloudrun_metrics::default::Default,
        ("gcp", "cloudsql_mysql") => &filebeat::gcp_cloudsql_mysql::default::Default,
        ("gcp", "cloudsql_postgresql") => &filebeat::gcp_cloudsql_postgresql::default::Default,
        ("gcp", "cloudsql_sqlserver") => &filebeat::gcp_cloudsql_sqlserver::default::Default,
        ("gcp", "compute") => &filebeat::gcp_compute::default::Default,
        ("gcp", "dataproc") => &filebeat::gcp_dataproc::default::Default,
        ("gcp", "dns") => &filebeat::gcp_dns::default::Default,
        ("gcp", "firestore") => &filebeat::gcp_firestore::default::Default,
        ("gcp", "gke") => &filebeat::gcp_gke::default::Default,
        ("gcp", "loadbalancing_logs") => &filebeat::gcp_loadbalancing_logs::default::Default,
        ("gcp", "loadbalancing_metrics") => &filebeat::gcp_loadbalancing_metrics::default::Default,
        ("gcp", "pubsub") => &filebeat::gcp_pubsub::default::Default,
        ("gcp", "redis") => &filebeat::gcp_redis::default::Default,
        ("gcp", "storage") => &filebeat::gcp_storage::default::Default,
        ("gcp", "firewall") => &filebeat::gcp_firewall::default::Default,
        ("gcp", "vpcflow") => &filebeat::gcp_vpcflow::default::Default,
        ("sentinel_one", "activity") => &filebeat::sentinel_one_activity::default::Default,
        ("sentinel_one", "agent") => &filebeat::sentinel_one_agent::default::Default,
        ("sentinel_one", "alert") => &filebeat::sentinel_one_alert::default::Default,
        ("sentinel_one", "application") => &filebeat::sentinel_one_application::default::Default,
        ("sentinel_one", "application_risk") => {
            &filebeat::sentinel_one_application_risk::default::Default
        }
        ("sentinel_one", "group") => &filebeat::sentinel_one_group::default::Default,
        ("sentinel_one", "threat_event") => &filebeat::sentinel_one_threat_event::default::Default,
        ("sentinel_one", "unified_alert") => {
            &filebeat::sentinel_one_unified_alert::default::Default
        }
        ("sentinel_one", "threat") => &filebeat::sentinel_one_threat::default::Default,
        ("windows", "applocker_exe_and_dll") => {
            &filebeat::windows_applocker_exe_and_dll::default::Default
        }
        ("windows", "applocker_msi_and_script") => {
            &filebeat::windows_applocker_msi_and_script::default::Default
        }
        ("windows", "applocker_packaged_app_deployment") => {
            &filebeat::windows_applocker_packaged_app_deployment::default::Default
        }
        ("windows", "applocker_packaged_app_execution") => {
            &filebeat::windows_applocker_packaged_app_execution::default::Default
        }
        ("windows", "forwarded") => &filebeat::windows_forwarded::default::Default,
        ("windows", "powershell") => &filebeat::windows_powershell::default::Default,
        ("windows", "powershell_operational") => {
            &filebeat::windows_powershell_operational::default::Default
        }
        ("windows", "sysmon_operational") => {
            &filebeat::windows_sysmon_operational::default::Default
        }
        ("windows", "windows_defender") => &filebeat::windows_windows_defender::default::Default,
        ("zscaler_zia", "alerts") => &filebeat::zscaler_zia_alerts::default::Default,
        ("zscaler_zia", "audit") => &filebeat::zscaler_zia_audit::default::Default,
        ("zscaler_zia", "email_dlp") => &filebeat::zscaler_zia_email_dlp::default::Default,
        ("zscaler_zia", "endpoint_dlp") => &filebeat::zscaler_zia_endpoint_dlp::default::Default,
        ("zscaler_zia", "saas_security") => &filebeat::zscaler_zia_saas_security::default::Default,
        ("zscaler_zia", "saas_security_activity") => {
            &filebeat::zscaler_zia_saas_security_activity::default::Default
        }
        ("zscaler_zia", "sandbox_report") => {
            &filebeat::zscaler_zia_sandbox_report::default::Default
        }
        ("zscaler_zia", "sandbox_verdict") => {
            &filebeat::zscaler_zia_sandbox_verdict::default::Default
        }
        ("zscaler_zia", "dns") => &filebeat::zscaler_zia_dns::default::Default,
        ("zscaler_zia", "firewall") => &filebeat::zscaler_zia_firewall::default::Default,
        ("zscaler_zia", "tunnel") => &filebeat::zscaler_zia_tunnel::default::Default,
        ("zscaler_zia", "web") => &filebeat::zscaler_zia_web::default::Default,
        ("m365_defender", "alert") => &filebeat::m365_defender_alert::default::Default,
        ("m365_defender", "incident") => &filebeat::m365_defender_incident::default::Default,
        ("m365_defender", "vulnerability") => {
            &filebeat::m365_defender_vulnerability::default::Default
        }
        ("m365_defender", "event") => &filebeat::m365_defender_event::default::Default,
        ("aws", "apigateway_logs") => &filebeat::aws_apigateway_logs::default::Default,
        ("aws", "awshealth") => &filebeat::aws_awshealth::default::Default,
        ("aws", "billing") => &filebeat::aws_billing::default::Default,
        ("aws", "cloudfront_logs") => &filebeat::aws_cloudfront_logs::default::Default,
        ("aws", "cloudtrail") => &filebeat::aws_cloudtrail::default::Default,
        ("aws", "cloudwatch_logs") => &filebeat::aws_cloudwatch_logs::default::Default,
        ("aws", "cloudwatch_metrics") => &filebeat::aws_cloudwatch_metrics::default::Default,
        ("aws", "config") => &filebeat::aws_config::default::Default,
        ("aws", "dynamodb") => &filebeat::aws_dynamodb::default::Default,
        ("aws", "ec2_logs") => &filebeat::aws_ec2_logs::default::Default,
        ("aws", "ec2_metrics") => &filebeat::aws_ec2_metrics::default::Default,
        ("aws", "elb_logs") => &filebeat::aws_elb_logs::default::Default,
        ("aws", "emr_logs") => &filebeat::aws_emr_logs::default::Default,
        ("aws", "guardduty") => &filebeat::aws_guardduty::default::Default,
        ("aws", "inspector") => &filebeat::aws_inspector::default::Default,
        ("aws", "kafka_metrics") => &filebeat::aws_kafka_metrics::default::Default,
        ("aws", "kinesis") => &filebeat::aws_kinesis::default::Default,
        ("aws", "lambda") => &filebeat::aws_lambda::default::Default,
        ("aws", "lambda_logs") => &filebeat::aws_lambda_logs::default::Default,
        ("aws", "natgateway") => &filebeat::aws_natgateway::default::Default,
        ("aws", "rds") => &filebeat::aws_rds::default::Default,
        ("aws", "redshift") => &filebeat::aws_redshift::default::Default,
        ("aws", "route53_public_logs") => &filebeat::aws_route53_public_logs::default::Default,
        ("aws", "route53_resolver_logs") => &filebeat::aws_route53_resolver_logs::default::Default,
        ("aws", "s3_daily_storage") => &filebeat::aws_s3_daily_storage::default::Default,
        ("aws", "s3_request") => &filebeat::aws_s3_request::default::Default,
        ("aws", "s3access") => &filebeat::aws_s3access::default::Default,
        ("aws", "securityhub_findings") => &filebeat::aws_securityhub_findings::default::Default,
        ("aws", "securityhub_findings_full_posture") => {
            &filebeat::aws_securityhub_findings_full_posture::default::Default
        }
        ("aws", "securityhub_insights") => &filebeat::aws_securityhub_insights::default::Default,
        ("aws", "sqs") => &filebeat::aws_sqs::default::Default,
        ("aws", "transitgateway") => &filebeat::aws_transitgateway::default::Default,
        ("aws", "usage") => &filebeat::aws_usage::default::Default,
        ("aws", "vpcflow") => &filebeat::aws_vpcflow::default::Default,
        ("aws", "vpn") => &filebeat::aws_vpn::default::Default,
        ("aws", "waf") => &filebeat::aws_waf::default::Default,
        ("aws", "firewall_logs") => &filebeat::aws_firewall_logs::default::Default,
        ("azure", "application_gateway") => &filebeat::azure_application_gateway::default::Default,
        ("azure", "eventhub") => &filebeat::azure_eventhub::default::Default,
        ("azure", "events") => &filebeat::azure_events::default::Default,
        ("azure", "firewall_logs") => &filebeat::azure_firewall_logs::default::Default,
        ("azure", "graphactivitylogs") => &filebeat::azure_graphactivitylogs::default::Default,
        ("azure", "identity_protection") => &filebeat::azure_identity_protection::default::Default,
        ("azure", "provisioning") => &filebeat::azure_provisioning::default::Default,
        ("azure", "springcloudlogs") => &filebeat::azure_springcloudlogs::default::Default,
        ("auth0", "logs") => &filebeat::auth0_logs::default::Default,
        ("coredns", "log") => &filebeat::coredns_log::default::Default,
        ("netflow", "log") => &filebeat::netflow_log::default::Default,
        ("amazon_security_lake", "event") => {
            &filebeat::amazon_security_lake_event::default::Default
        }
        ("anthropic", "audit") => &filebeat::anthropic_audit::default::Default,
        ("arista_ngfw", "log") => &filebeat::arista_ngfw_log::default::Default,
        ("aws_securityhub", "finding") => &filebeat::aws_securityhub_finding::default::Default,
        ("azure_network_watcher_nsg", "log") => {
            &filebeat::azure_network_watcher_nsg_log::default::Default
        }
        ("azure_network_watcher_vnet", "log") => {
            &filebeat::azure_network_watcher_vnet_log::default::Default
        }
        ("carbon_black_cloud", "alert_v7") => {
            &filebeat::carbon_black_cloud_alert_v7::default::Default
        }
        ("carbon_black_cloud", "asset_vulnerability_summary") => {
            &filebeat::carbon_black_cloud_asset_vulnerability_summary::default::Default
        }
        ("carbon_black_cloud", "audit") => &filebeat::carbon_black_cloud_audit::default::Default,
        ("carbon_black_cloud", "endpoint_event") => {
            &filebeat::carbon_black_cloud_endpoint_event::default::Default
        }
        ("carbon_black_cloud", "watchlist_hit") => {
            &filebeat::carbon_black_cloud_watchlist_hit::default::Default
        }
        ("cursor", "audit") => &filebeat::cursor_audit::default::Default,
        ("dataminr_pulse", "alerts") => &filebeat::dataminr_pulse_alerts::default::Default,
        ("ece", "adminconsole") => &filebeat::ece_adminconsole::default::Default,
        ("entityanalytics_ad", "entity") => &filebeat::entityanalytics_ad_entity::default::Default,
        ("jumpcloud", "events") => &filebeat::jumpcloud_events::default::Default,
        ("jupiter_one", "asset") => &filebeat::jupiter_one_asset::default::Default,
        ("kolide", "audit") => &filebeat::kolide_audit::default::Default,
        ("kolide", "auth") => &filebeat::kolide_auth::default::Default,
        ("kolide", "deprovisioned_person") => {
            &filebeat::kolide_deprovisioned_person::default::Default
        }
        ("kolide", "device") => &filebeat::kolide_device::default::Default,
        ("kolide", "device_check") => &filebeat::kolide_device_check::default::Default,
        ("kolide", "issues") => &filebeat::kolide_issues::default::Default,
        ("kolide", "osquery_result") => &filebeat::kolide_osquery_result::default::Default,
        ("kolide", "osquery_status") => &filebeat::kolide_osquery_status::default::Default,
        ("kolide", "people") => &filebeat::kolide_people::default::Default,
        ("kolide", "request") => &filebeat::kolide_request::default::Default,
        ("nextron_thor", "thor_forwarding") => {
            &filebeat::nextron_thor_thor_forwarding::default::Default
        }
        ("qualys_vmdr", "asset_host_detection") => {
            &filebeat::qualys_vmdr_asset_host_detection::default::Default
        }
        ("qualys_vmdr", "knowledge_base") => {
            &filebeat::qualys_vmdr_knowledge_base::default::Default
        }
        ("qualys_vmdr", "user_activity") => &filebeat::qualys_vmdr_user_activity::default::Default,
        ("qualys_was", "vulnerability") => &filebeat::qualys_was_vulnerability::default::Default,
        ("symantec_endpoint_security", "event") => {
            &filebeat::symantec_endpoint_security_event::default::Default
        }
        ("symantec_endpoint_security", "incident") => {
            &filebeat::symantec_endpoint_security_incident::default::Default
        }
        ("tenable_sc", "asset") => &filebeat::tenable_sc_asset::default::Default,
        ("tenable_sc", "plugin") => &filebeat::tenable_sc_plugin::default::Default,
        ("tenable_sc", "vulnerability") => &filebeat::tenable_sc_vulnerability::default::Default,
        ("ti_socradar_feeds", "feed") => &filebeat::ti_socradar_feeds_feed::default::Default,
        ("tines", "audit_logs") => &filebeat::tines_audit_logs::default::Default,
        ("tines", "time_saved") => &filebeat::tines_time_saved::default::Default,
        ("barracuda_cloudgen_firewall", "log") => {
            &filebeat::barracuda_cloudgen_firewall_log::default::Default
        }
        ("infoblox_nios", "log") => &filebeat::infoblox_nios_log::default::Default,
        ("microsoft_exchange_online_message_trace", "log") => {
            &filebeat::microsoft_exchange_online_message_trace_log::default::Default
        }
        ("ping_federate", "admin") => &filebeat::ping_federate_admin::default::Default,
        ("ping_federate", "audit") => &filebeat::ping_federate_audit::default::Default,
        ("ping_one", "audit") => &filebeat::ping_one_audit::default::Default,
        ("sonicwall_firewall", "log") => &filebeat::sonicwall_firewall_log::default::Default,
        ("sophos_central", "alert") => &filebeat::sophos_central_alert::default::Default,
        ("sophos_central", "event") => &filebeat::sophos_central_event::default::Default,
        ("squid", "log") => &filebeat::squid_log::default::Default,
        ("suricata", "eve") => &filebeat::suricata_eve::default::Default,
        ("trendmicro", "deep_security") => &filebeat::trendmicro_deep_security::default::Default,
        ("cisco_secure_email_gateway", "log") => {
            &filebeat::cisco_secure_email_gateway_log::default::Default
        }
        ("pfsense", "log") => &filebeat::pfsense_log::default::Default,
        ("abnormal_security", "ai_security_mailbox") => {
            &filebeat::abnormal_security_ai_security_mailbox::default::Default
        }
        ("abnormal_security", "ai_security_mailbox_not_analyzed") => {
            &filebeat::abnormal_security_ai_security_mailbox_not_analyzed::default::Default
        }
        ("abnormal_security", "audit") => &filebeat::abnormal_security_audit::default::Default,
        ("abnormal_security", "case") => &filebeat::abnormal_security_case::default::Default,
        ("abnormal_security", "threat") => &filebeat::abnormal_security_threat::default::Default,
        ("abnormal_security", "vendor_case") => {
            &filebeat::abnormal_security_vendor_case::default::Default
        }
        ("beyondtrust_pra", "access_session") => {
            &filebeat::beyondtrust_pra_access_session::default::Default
        }
        ("cisco_duo", "activity") => &filebeat::cisco_duo_activity::default::Default,
        ("cisco_duo", "admin") => &filebeat::cisco_duo_admin::default::Default,
        ("cisco_duo", "auth") => &filebeat::cisco_duo_auth::default::Default,
        ("cisco_duo", "offline_enrollment") => {
            &filebeat::cisco_duo_offline_enrollment::default::Default
        }
        ("cisco_duo", "summary") => &filebeat::cisco_duo_summary::default::Default,
        ("cisco_duo", "telephony") => &filebeat::cisco_duo_telephony::default::Default,
        ("cisco_duo", "telephony_v2") => &filebeat::cisco_duo_telephony_v2::default::Default,
        ("cisco_duo", "trust_monitor") => &filebeat::cisco_duo_trust_monitor::default::Default,
        ("fim", "event") => &filebeat::fim_event::default::Default,
        ("github", "audit") => &filebeat::github_audit::default::Default,
        ("github", "code_scanning") => &filebeat::github_code_scanning::default::Default,
        ("github", "dependabot") => &filebeat::github_dependabot::default::Default,
        ("github", "issues") => &filebeat::github_issues::default::Default,
        ("github", "secret_scanning") => &filebeat::github_secret_scanning::default::Default,
        ("github", "security_advisories") => {
            &filebeat::github_security_advisories::default::Default
        }
        ("google_workspace", "access_transparency") => {
            &filebeat::google_workspace_access_transparency::default::Default
        }
        ("google_workspace", "admin") => &filebeat::google_workspace_admin::default::Default,
        ("google_workspace", "alert") => &filebeat::google_workspace_alert::default::Default,
        ("google_workspace", "calendar") => &filebeat::google_workspace_calendar::default::Default,
        ("google_workspace", "chat") => &filebeat::google_workspace_chat::default::Default,
        ("google_workspace", "chrome") => &filebeat::google_workspace_chrome::default::Default,
        ("google_workspace", "context_aware_access") => {
            &filebeat::google_workspace_context_aware_access::default::Default
        }
        ("google_workspace", "data_studio") => {
            &filebeat::google_workspace_data_studio::default::Default
        }
        ("google_workspace", "device") => &filebeat::google_workspace_device::default::Default,
        ("google_workspace", "drive") => &filebeat::google_workspace_drive::default::Default,
        ("google_workspace", "gcp") => &filebeat::google_workspace_gcp::default::Default,
        ("google_workspace", "gmail") => &filebeat::google_workspace_gmail::default::Default,
        ("google_workspace", "group_enterprise") => {
            &filebeat::google_workspace_group_enterprise::default::Default
        }
        ("google_workspace", "groups") => &filebeat::google_workspace_groups::default::Default,
        ("google_workspace", "keep") => &filebeat::google_workspace_keep::default::Default,
        ("google_workspace", "login") => &filebeat::google_workspace_login::default::Default,
        ("google_workspace", "meet") => &filebeat::google_workspace_meet::default::Default,
        ("google_workspace", "rules") => &filebeat::google_workspace_rules::default::Default,
        ("google_workspace", "saml") => &filebeat::google_workspace_saml::default::Default,
        ("google_workspace", "token") => &filebeat::google_workspace_token::default::Default,
        ("google_workspace", "user_accounts") => {
            &filebeat::google_workspace_user_accounts::default::Default
        }
        ("google_workspace", "vault") => &filebeat::google_workspace_vault::default::Default,
        ("juniper_srx", "log") => &filebeat::juniper_srx_log::default::Default,
        ("prisma_access", "event") => &filebeat::prisma_access_event::default::Default,
        ("sophos", "utm") => &filebeat::sophos_utm::default::Default,
        ("sophos", "xg") => &filebeat::sophos_xg::default::Default,
        ("syslog_router", "log") => &filebeat::syslog_router_log::default::Default,
        ("watchguard_firebox", "log") => &filebeat::watchguard_firebox_log::default::Default,
        ("zeek", "capture_loss") => &filebeat::zeek_capture_loss::default::Default,
        ("zeek", "connection") => &filebeat::zeek_connection::default::Default,
        ("zeek", "dce_rpc") => &filebeat::zeek_dce_rpc::default::Default,
        ("zeek", "dhcp") => &filebeat::zeek_dhcp::default::Default,
        ("zeek", "dnp3") => &filebeat::zeek_dnp3::default::Default,
        ("zeek", "dns") => &filebeat::zeek_dns::default::Default,
        ("zeek", "dpd") => &filebeat::zeek_dpd::default::Default,
        ("zeek", "files") => &filebeat::zeek_files::default::Default,
        ("zeek", "ftp") => &filebeat::zeek_ftp::default::Default,
        ("zeek", "http") => &filebeat::zeek_http::default::Default,
        ("zeek", "intel") => &filebeat::zeek_intel::default::Default,
        ("zeek", "irc") => &filebeat::zeek_irc::default::Default,
        ("zeek", "kerberos") => &filebeat::zeek_kerberos::default::Default,
        ("zeek", "known_certs") => &filebeat::zeek_known_certs::default::Default,
        ("zeek", "known_hosts") => &filebeat::zeek_known_hosts::default::Default,
        ("zeek", "known_services") => &filebeat::zeek_known_services::default::Default,
        ("zeek", "modbus") => &filebeat::zeek_modbus::default::Default,
        ("zeek", "mysql") => &filebeat::zeek_mysql::default::Default,
        ("zeek", "notice") => &filebeat::zeek_notice::default::Default,
        ("zeek", "ntlm") => &filebeat::zeek_ntlm::default::Default,
        ("zeek", "ntp") => &filebeat::zeek_ntp::default::Default,
        ("zeek", "ocsp") => &filebeat::zeek_ocsp::default::Default,
        ("zeek", "pe") => &filebeat::zeek_pe::default::Default,
        ("zeek", "radius") => &filebeat::zeek_radius::default::Default,
        ("zeek", "rdp") => &filebeat::zeek_rdp::default::Default,
        ("zeek", "rfb") => &filebeat::zeek_rfb::default::Default,
        ("zeek", "signature") => &filebeat::zeek_signature::default::Default,
        ("zeek", "sip") => &filebeat::zeek_sip::default::Default,
        ("zeek", "smb_cmd") => &filebeat::zeek_smb_cmd::default::Default,
        ("zeek", "smb_files") => &filebeat::zeek_smb_files::default::Default,
        ("zeek", "smb_mapping") => &filebeat::zeek_smb_mapping::default::Default,
        ("zeek", "smtp") => &filebeat::zeek_smtp::default::Default,
        ("zeek", "snmp") => &filebeat::zeek_snmp::default::Default,
        ("zeek", "socks") => &filebeat::zeek_socks::default::Default,
        ("zeek", "software") => &filebeat::zeek_software::default::Default,
        ("zeek", "ssh") => &filebeat::zeek_ssh::default::Default,
        ("zeek", "ssl") => &filebeat::zeek_ssl::default::Default,
        ("zeek", "stats") => &filebeat::zeek_stats::default::Default,
        ("zeek", "syslog") => &filebeat::zeek_syslog::default::Default,
        ("zeek", "traceroute") => &filebeat::zeek_traceroute::default::Default,
        ("zeek", "tunnel") => &filebeat::zeek_tunnel::default::Default,
        ("zeek", "weird") => &filebeat::zeek_weird::default::Default,
        ("zeek", "x509") => &filebeat::zeek_x509::default::Default,
        ("agentless_hello_world", "generic") => {
            &filebeat::agentless_hello_world_generic::default::Default
        }
        ("agentless_hello_world", "mock_counter") => {
            &filebeat::agentless_hello_world_mock_counter::default::Default
        }
        ("azure_metrics", "compute_vm") => &filebeat::azure_metrics_compute_vm::default::Default,
        ("azure_metrics", "compute_vm_scaleset") => {
            &filebeat::azure_metrics_compute_vm_scaleset::default::Default
        }
        ("azure_metrics", "container_instance") => {
            &filebeat::azure_metrics_container_instance::default::Default
        }
        ("azure_metrics", "container_registry") => {
            &filebeat::azure_metrics_container_registry::default::Default
        }
        ("azure_metrics", "container_service") => {
            &filebeat::azure_metrics_container_service::default::Default
        }
        ("azure_metrics", "database_account") => {
            &filebeat::azure_metrics_database_account::default::Default
        }
        ("azure_metrics", "monitor") => &filebeat::azure_metrics_monitor::default::Default,
        ("azure_metrics", "storage_account") => {
            &filebeat::azure_metrics_storage_account::default::Default
        }
        ("blacklens", "alerts") => &filebeat::blacklens_alerts::default::Default,
        ("cisco_meraki_metrics", "device_health") => {
            &filebeat::cisco_meraki_metrics_device_health::default::Default
        }
        ("cloud_security_posture", "findings") => {
            &filebeat::cloud_security_posture_findings::default::Default
        }
        ("cloud_security_posture", "vulnerabilities") => {
            &filebeat::cloud_security_posture_vulnerabilities::default::Default
        }
        ("cylance", "protect") => &filebeat::cylance_protect::default::Default,
        ("elastic_agent", "elastic_agent_logs") => {
            &filebeat::elastic_agent_elastic_agent_logs::default::Default
        }
        ("elastic_agent", "status_change_logs") => {
            &filebeat::elastic_agent_status_change_logs::default::Default
        }
        ("elastic_package_registry", "metrics") => {
            &filebeat::elastic_package_registry_metrics::default::Default
        }
        ("entro", "audit") => &filebeat::entro_audit::default::Default,
        ("first_epss", "vulnerability") => &filebeat::first_epss_vulnerability::default::Default,
        ("fortinet_forticlient", "log") => &filebeat::fortinet_forticlient_log::default::Default,
        ("juniper_junos", "log") => &filebeat::juniper_junos_log::default::Default,
        ("juniper_netscreen", "log") => &filebeat::juniper_netscreen_log::default::Default,
        ("keeper_security_siem_integration", "audit") => {
            &filebeat::keeper_security_siem_integration_audit::default::Default
        }
        ("lumos", "activity_logs") => &filebeat::lumos_activity_logs::default::Default,
        ("netscout", "sightline") => &filebeat::netscout_sightline::default::Default,
        ("panw_metrics", "system") => &filebeat::panw_metrics_system::default::Default,
        ("pulse_connect_secure", "log") => &filebeat::pulse_connect_secure_log::default::Default,
        ("rabbitmq", "log") => &filebeat::rabbitmq_log::default::Default,
        ("redis", "log") => &filebeat::redis_log::default::Default,
        ("system_audit", "package") => &filebeat::system_audit_package::default::Default,
        ("tomcat", "log") => &filebeat::tomcat_log::default::Default,
        ("kibana", "audit") => &filebeat::kibana_audit::default::Default,
        ("kibana", "log") => &filebeat::kibana_log::default::Default,
        ("platform_observability", "kibana_audit") => {
            &filebeat::platform_observability_kibana_audit::default::Default
        }
        ("platform_observability", "kibana_log") => {
            &filebeat::platform_observability_kibana_log::default::Default
        }
        ("auditd_manager", "auditd") => &filebeat::auditd_manager_auditd::default::Default,
        ("awsfirehose", "logs") => &filebeat::awsfirehose_logs::default::Default,
        ("awsfirehose", "metrics") => &filebeat::awsfirehose_metrics::default::Default,
        ("azure_functions", "functionapplogs") => {
            &filebeat::azure_functions_functionapplogs::default::Default
        }
        ("azure_functions", "metrics") => &filebeat::azure_functions_metrics::default::Default,
        ("beyondtrust_isi", "incident") => &filebeat::beyondtrust_isi_incident::default::Default,
        ("bitsight", "vulnerability") => &filebeat::bitsight_vulnerability::default::Default,
        ("cassandra", "log") => &filebeat::cassandra_log::default::Default,
        ("cassandra", "metrics") => &filebeat::cassandra_metrics::default::Default,
        ("cisa_kevs", "vulnerability") => &filebeat::cisa_kevs_vulnerability::default::Default,
        ("citrix_waf", "log") => &filebeat::citrix_waf_log::default::Default,
        ("couchdb", "server") => &filebeat::couchdb_server::default::Default,
        ("cyberark_pta", "events") => &filebeat::cyberark_pta_events::default::Default,
        ("elastic_security", "alert") => &filebeat::elastic_security_alert::default::Default,
        ("ess_billing", "credits") => &filebeat::ess_billing_credits::default::Default,
        ("forcepoint_web", "logs") => &filebeat::forcepoint_web_logs::default::Default,
        ("fortinet_fortiedr", "log") => &filebeat::fortinet_fortiedr_log::default::Default,
        ("goflow2", "sflow") => &filebeat::goflow2_sflow::default::Default,
        ("golang", "expvar") => &filebeat::golang_expvar::default::Default,
        ("golang", "heap") => &filebeat::golang_heap::default::Default,
        ("greenhouse", "audit") => &filebeat::greenhouse_audit::default::Default,
        ("haproxy", "log") => &filebeat::haproxy_log::default::Default,
        ("hashicorp_vault", "audit") => &filebeat::hashicorp_vault_audit::default::Default,
        ("hashicorp_vault", "log") => &filebeat::hashicorp_vault_log::default::Default,
        ("hashicorp_vault", "metrics") => &filebeat::hashicorp_vault_metrics::default::Default,
        ("iptables", "log") => &filebeat::iptables_log::default::Default,
        ("kubernetes", "audit_logs") => &filebeat::kubernetes_audit_logs::default::Default,
        ("kubernetes", "container_logs") => &filebeat::kubernetes_container_logs::default::Default,
        ("microsoft_dhcp", "log") => &filebeat::microsoft_dhcp_log::default::Default,
        ("mongodb", "log") => &filebeat::mongodb_log::default::Default,
        ("nats", "log") => &filebeat::nats_log::default::Default,
        ("osquery", "result") => &filebeat::osquery_result::default::Default,
        ("php_fpm", "pool") => &filebeat::php_fpm_pool::default::Default,
        ("php_fpm", "process") => &filebeat::php_fpm_process::default::Default,
        ("pps", "log") => &filebeat::pps_log::default::Default,
        ("qnap_nas", "log") => &filebeat::qnap_nas_log::default::Default,
        ("sailpoint_identity_sc", "events") => {
            &filebeat::sailpoint_identity_sc_events::default::Default
        }
        ("santa", "log") => &filebeat::santa_log::default::Default,
        ("stan", "log") => &filebeat::stan_log::default::Default,
        ("tetragon", "log") => &filebeat::tetragon_log::default::Default,
        ("thycotic_ss", "logs") => &filebeat::thycotic_ss_logs::default::Default,
        ("ti_maltiverse", "indicator") => &filebeat::ti_maltiverse_indicator::default::Default,
        ("ti_strider", "indicator") => &filebeat::ti_strider_indicator::default::Default,
        ("varonis", "logs") => &filebeat::varonis_logs::default::Default,
        ("withsecure_elements", "incidents") => {
            &filebeat::withsecure_elements_incidents::default::Default
        }
        ("withsecure_elements", "security_events") => {
            &filebeat::withsecure_elements_security_events::default::Default
        }
        ("zerofox", "alerts") => &filebeat::zerofox_alerts::default::Default,
        ("zeronetworks", "audit") => &filebeat::zeronetworks_audit::default::Default,
        ("ess_billing", "billing") => &filebeat::ess_billing_billing::default::Default,
        ("lyve_cloud", "audit") => &filebeat::lyve_cloud_audit::default::Default,
        ("1password", "audit_events") => &filebeat::_1password_audit_events::default::Default,
        ("1password", "item_usages") => &filebeat::_1password_item_usages::default::Default,
        ("1password", "signin_attempts") => &filebeat::_1password_signin_attempts::default::Default,
        ("activemq", "audit") => &filebeat::activemq_audit::default::Default,
        ("activemq", "broker") => &filebeat::activemq_broker::default::Default,
        ("activemq", "log") => &filebeat::activemq_log::default::Default,
        ("activemq", "queue") => &filebeat::activemq_queue::default::Default,
        ("activemq", "topic") => &filebeat::activemq_topic::default::Default,
        ("admin_by_request_epm", "auditlog") => {
            &filebeat::admin_by_request_epm_auditlog::default::Default
        }
        ("admin_by_request_epm", "events") => {
            &filebeat::admin_by_request_epm_events::default::Default
        }
        ("anthropic_metrics", "cost") => &filebeat::anthropic_metrics_cost::default::Default,
        ("anthropic_metrics", "rate_limit") => {
            &filebeat::anthropic_metrics_rate_limit::default::Default
        }
        ("anthropic_metrics", "usage") => &filebeat::anthropic_metrics_usage::default::Default,
        ("apache", "access") => &filebeat::apache_access::default::Default,
        ("apache", "error") => &filebeat::apache_error::default::Default,
        ("atlassian_bitbucket", "audit") => &filebeat::atlassian_bitbucket_audit::default::Default,
        ("atlassian_cloud", "audit") => &filebeat::atlassian_cloud_audit::default::Default,
        ("atlassian_confluence", "audit") => {
            &filebeat::atlassian_confluence_audit::default::Default
        }
        ("aws_mq", "activemq_audit_logs") => {
            &filebeat::aws_mq_activemq_audit_logs::default::Default
        }
        ("aws_mq", "activemq_general_logs") => {
            &filebeat::aws_mq_activemq_general_logs::default::Default
        }
        ("aws_mq", "activemq_metrics") => &filebeat::aws_mq_activemq_metrics::default::Default,
        ("aws_mq", "rabbitmq_general_logs") => {
            &filebeat::aws_mq_rabbitmq_general_logs::default::Default
        }
        ("aws_mq", "rabbitmq_metrics") => &filebeat::aws_mq_rabbitmq_metrics::default::Default,
        ("backstage", "logs") => &filebeat::backstage_logs::default::Default,
        ("cef", "log") => &filebeat::cef_log::default::Default,
        ("checkpoint_email", "event") => &filebeat::checkpoint_email_event::default::Default,
        ("cisco_aironet", "log") => &filebeat::cisco_aironet_log::default::Default,
        ("claude_cowork", "events") => &filebeat::claude_cowork_events::default::Default,
        ("contrast_security", "attack_event") => {
            &filebeat::contrast_security_attack_event::default::Default
        }
        ("contrast_security", "incident") => {
            &filebeat::contrast_security_incident::default::Default
        }
        ("contrast_security", "issue") => &filebeat::contrast_security_issue::default::Default,
        ("digital_guardian", "arc") => &filebeat::digital_guardian_arc::default::Default,
        ("forescout", "event") => &filebeat::forescout_event::default::Default,
        ("forescout", "host") => &filebeat::forescout_host::default::Default,
        ("hackerone", "report") => &filebeat::hackerone_report::default::Default,
        ("hid_bravura_monitor", "log") => &filebeat::hid_bravura_monitor_log::default::Default,
        ("iis", "access") => &filebeat::iis_access::default::Default,
        ("iis", "error") => &filebeat::iis_error::default::Default,
        ("imperva", "securesphere") => &filebeat::imperva_securesphere::default::Default,
        ("jamf_pro", "events") => &filebeat::jamf_pro_events::default::Default,
        ("jamf_pro", "inventory") => &filebeat::jamf_pro_inventory::default::Default,
        ("keycloak", "log") => &filebeat::keycloak_log::default::Default,
        ("mattermost", "audit") => &filebeat::mattermost_audit::default::Default,
        ("microsoft_exchange_server", "httpproxy") => {
            &filebeat::microsoft_exchange_server_httpproxy::default::Default
        }
        ("microsoft_exchange_server", "imap4_pop3") => {
            &filebeat::microsoft_exchange_server_imap4_pop3::default::Default
        }
        ("microsoft_exchange_server", "smtp") => {
            &filebeat::microsoft_exchange_server_smtp::default::Default
        }
        ("miniflux", "feed_entry") => &filebeat::miniflux_feed_entry::default::Default,
        ("modsecurity", "auditlog") => &filebeat::modsecurity_auditlog::default::Default,
        ("mysql_enterprise", "audit") => &filebeat::mysql_enterprise_audit::default::Default,
        ("netbox", "devices") => &filebeat::netbox_devices::default::Default,
        ("netbox", "ips") => &filebeat::netbox_ips::default::Default,
        ("nginx", "access") => &filebeat::nginx_access::default::Default,
        ("nginx", "error") => &filebeat::nginx_error::default::Default,
        ("postgresql", "activity") => &filebeat::postgresql_activity::default::Default,
        ("postgresql", "log") => &filebeat::postgresql_log::default::Default,
        ("proofpoint_itm", "report") => &filebeat::proofpoint_itm_report::default::Default,
        ("slack", "audit") => &filebeat::slack_audit::default::Default,
        ("snort", "log") => &filebeat::snort_log::default::Default,
        ("snyk", "audit_logs") => &filebeat::snyk_audit_logs::default::Default,
        ("snyk", "issues") => &filebeat::snyk_issues::default::Default,
        ("spring_boot", "audit_events") => &filebeat::spring_boot_audit_events::default::Default,
        ("spring_boot", "gc") => &filebeat::spring_boot_gc::default::Default,
        ("spring_boot", "http_trace") => &filebeat::spring_boot_http_trace::default::Default,
        ("spring_boot", "memory") => &filebeat::spring_boot_memory::default::Default,
        ("spring_boot", "threading") => &filebeat::spring_boot_threading::default::Default,
        ("ti_cif3", "feed") => &filebeat::ti_cif3_feed::default::Default,
        ("ti_cybersixgill", "threat") => &filebeat::ti_cybersixgill_threat::default::Default,
        ("ti_domaintools", "domaindiscovery_feed") => {
            &filebeat::ti_domaintools_domaindiscovery_feed::default::Default
        }
        ("ti_domaintools", "domainhotlist_feed") => {
            &filebeat::ti_domaintools_domainhotlist_feed::default::Default
        }
        ("ti_domaintools", "domainrdap_feed") => {
            &filebeat::ti_domaintools_domainrdap_feed::default::Default
        }
        ("ti_domaintools", "domainrisk_feed") => {
            &filebeat::ti_domaintools_domainrisk_feed::default::Default
        }
        ("ti_domaintools", "nad_feed") => &filebeat::ti_domaintools_nad_feed::default::Default,
        ("ti_domaintools", "nod_feed") => &filebeat::ti_domaintools_nod_feed::default::Default,
        ("ti_eclecticiq", "threat") => &filebeat::ti_eclecticiq_threat::default::Default,
        ("ti_greynoise", "ip") => &filebeat::ti_greynoise_ip::default::Default,
        ("ti_ticura", "indicator") => &filebeat::ti_ticura_indicator::default::Default,
        ("bbot", "asm_intel") => &filebeat::bbot_asm_intel::default::Default,
        ("beelzebub", "logs") => &filebeat::beelzebub_logs::default::Default,
        ("hid_bravura_monitor", "winlog") => {
            &filebeat::hid_bravura_monitor_winlog::default::Default
        }
        ("macos", "advanced_monitoring") => &filebeat::macos_advanced_monitoring::default::Default,
        ("macos", "authentication") => &filebeat::macos_authentication::default::Default,
        ("macos", "file_read_write") => &filebeat::macos_file_read_write::default::Default,
        ("macos", "network_activity") => &filebeat::macos_network_activity::default::Default,
        ("macos", "process_execution_monitoring") => {
            &filebeat::macos_process_execution_monitoring::default::Default
        }
        ("macos", "system_change") => &filebeat::macos_system_change::default::Default,
        ("macos", "user_and_account_management") => {
            &filebeat::macos_user_and_account_management::default::Default
        }
        ("microsoft_exchange_server", "messagetracking") => {
            &filebeat::microsoft_exchange_server_messagetracking::default::Default
        }
        ("akamai", "siem") => &filebeat::akamai_siem::default::Default,
        ("atlassian_jira", "audit") => &filebeat::atlassian_jira_audit::default::Default,
        ("auditd", "log") => &filebeat::auditd_log::default::Default,
        ("authentik", "event") => &filebeat::authentik_event::default::Default,
        ("authentik", "group") => &filebeat::authentik_group::default::Default,
        ("authentik", "user") => &filebeat::authentik_user::default::Default,
        ("aws_bedrock_agentcore", "gateway_application_logs") => {
            &filebeat::aws_bedrock_agentcore_gateway_application_logs::default::Default
        }
        ("aws_bedrock_agentcore", "memory_application_logs") => {
            &filebeat::aws_bedrock_agentcore_memory_application_logs::default::Default
        }
        ("aws_bedrock_agentcore", "metrics") => {
            &filebeat::aws_bedrock_agentcore_metrics::default::Default
        }
        ("aws_bedrock_agentcore", "runtime_application_logs") => {
            &filebeat::aws_bedrock_agentcore_runtime_application_logs::default::Default
        }
        ("aws_billing", "cur") => &filebeat::aws_billing_cur::default::Default,
        ("azure_ai_foundry", "logs") => &filebeat::azure_ai_foundry_logs::default::Default,
        ("azure_ai_foundry", "metrics") => &filebeat::azure_ai_foundry_metrics::default::Default,
        ("azure_app_service", "app_service_logs") => {
            &filebeat::azure_app_service_app_service_logs::default::Default
        }
        ("azure_openai", "logs") => &filebeat::azure_openai_logs::default::Default,
        ("azure_openai", "metrics") => &filebeat::azure_openai_metrics::default::Default,
        ("barracuda", "waf") => &filebeat::barracuda_waf::default::Default,
        ("box_events", "events") => &filebeat::box_events_events::default::Default,
        ("canva", "audit") => &filebeat::canva_audit::default::Default,
        ("carbonblack_edr", "log") => &filebeat::carbonblack_edr_log::default::Default,
        ("ceph", "cluster_disk") => &filebeat::ceph_cluster_disk::default::Default,
        ("ceph", "cluster_health") => &filebeat::ceph_cluster_health::default::Default,
        ("ceph", "cluster_status") => &filebeat::ceph_cluster_status::default::Default,
        ("ceph", "osd_performance") => &filebeat::ceph_osd_performance::default::Default,
        ("ceph", "osd_pool_stats") => &filebeat::ceph_osd_pool_stats::default::Default,
        ("ceph", "osd_tree") => &filebeat::ceph_osd_tree::default::Default,
        ("ceph", "pool_disk") => &filebeat::ceph_pool_disk::default::Default,
        ("cisco_secure_endpoint", "event") => {
            &filebeat::cisco_secure_endpoint_event::default::Default
        }
        ("claude_code", "events") => &filebeat::claude_code_events::default::Default,
        ("cloudflare", "audit") => &filebeat::cloudflare_audit::default::Default,
        ("cloudflare", "logpull") => &filebeat::cloudflare_logpull::default::Default,
        ("cyberarkpas", "monitor") => &filebeat::cyberarkpas_monitor::default::Default,
        ("elasticsearch", "audit") => &filebeat::elasticsearch_audit::default::Default,
        ("elasticsearch", "deprecation") => &filebeat::elasticsearch_deprecation::default::Default,
        ("elasticsearch", "gc") => &filebeat::elasticsearch_gc::default::Default,
        ("elasticsearch", "ingest_pipeline") => {
            &filebeat::elasticsearch_ingest_pipeline::default::Default
        }
        ("elasticsearch", "querylog") => &filebeat::elasticsearch_querylog::default::Default,
        ("elasticsearch", "server") => &filebeat::elasticsearch_server::default::Default,
        ("elasticsearch", "slowlog") => &filebeat::elasticsearch_slowlog::default::Default,
        ("endace", "flow") => &filebeat::endace_flow::default::Default,
        ("endace", "log") => &filebeat::endace_log::default::Default,
        ("envoyproxy", "log") => &filebeat::envoyproxy_log::default::Default,
        ("envoyproxy", "stats") => &filebeat::envoyproxy_stats::default::Default,
        ("extrahop", "detection") => &filebeat::extrahop_detection::default::Default,
        ("extrahop", "investigation") => &filebeat::extrahop_investigation::default::Default,
        ("falco", "alerts") => &filebeat::falco_alerts::default::Default,
        ("fireeye", "nx") => &filebeat::fireeye_nx::default::Default,
        ("gcp_vertexai", "auditlogs") => &filebeat::gcp_vertexai_auditlogs::default::Default,
        ("gcp_vertexai", "metrics") => &filebeat::gcp_vertexai_metrics::default::Default,
        ("gcp_vertexai", "prompt_response_logs") => {
            &filebeat::gcp_vertexai_prompt_response_logs::default::Default
        }
        ("gdacs", "events") => &filebeat::gdacs_events::default::Default,
        ("ibm_qradar", "offense") => &filebeat::ibm_qradar_offense::default::Default,
        ("ibmmq", "errorlog") => &filebeat::ibmmq_errorlog::default::Default,
        ("ibmmq", "qmgr") => &filebeat::ibmmq_qmgr::default::Default,
        ("imperva_cloud_waf", "event") => &filebeat::imperva_cloud_waf_event::default::Default,
        ("ironscales", "incident") => &filebeat::ironscales_incident::default::Default,
        ("istio", "access_logs") => &filebeat::istio_access_logs::default::Default,
        ("istio", "istiod_metrics") => &filebeat::istio_istiod_metrics::default::Default,
        ("istio", "proxy_metrics") => &filebeat::istio_proxy_metrics::default::Default,
        ("lastpass", "detailed_shared_folder") => {
            &filebeat::lastpass_detailed_shared_folder::default::Default
        }
        ("lastpass", "event_report") => &filebeat::lastpass_event_report::default::Default,
        ("lastpass", "user") => &filebeat::lastpass_user::default::Default,
        ("logstash", "log") => &filebeat::logstash_log::default::Default,
        ("logstash", "pipeline") => &filebeat::logstash_pipeline::default::Default,
        ("logstash", "plugins") => &filebeat::logstash_plugins::default::Default,
        ("logstash", "slowlog") => &filebeat::logstash_slowlog::default::Default,
        ("menlo", "dlp") => &filebeat::menlo_dlp::default::Default,
        ("menlo", "web") => &filebeat::menlo_web::default::Default,
        ("microsoft_intune", "audit") => &filebeat::microsoft_intune_audit::default::Default,
        ("microsoft_intune", "managed_device") => {
            &filebeat::microsoft_intune_managed_device::default::Default
        }
        ("microsoft_sqlserver", "audit") => &filebeat::microsoft_sqlserver_audit::default::Default,
        ("microsoft_sqlserver", "availability_groups") => {
            &filebeat::microsoft_sqlserver_availability_groups::default::Default
        }
        ("microsoft_sqlserver", "log") => &filebeat::microsoft_sqlserver_log::default::Default,
        ("microsoft_sqlserver", "performance") => {
            &filebeat::microsoft_sqlserver_performance::default::Default
        }
        ("microsoft_sqlserver", "transaction_log") => {
            &filebeat::microsoft_sqlserver_transaction_log::default::Default
        }
        ("mysql", "error") => &filebeat::mysql_error::default::Default,
        ("mysql", "performance") => &filebeat::mysql_performance::default::Default,
        ("mysql", "replica_status") => &filebeat::mysql_replica_status::default::Default,
        ("neon_cyber", "detections") => &filebeat::neon_cyber_detections::default::Default,
        ("neon_cyber", "events") => &filebeat::neon_cyber_events::default::Default,
        ("nginx_ingress_controller", "access") => {
            &filebeat::nginx_ingress_controller_access::default::Default
        }
        ("nginx_ingress_controller", "error") => {
            &filebeat::nginx_ingress_controller_error::default::Default
        }
        ("oracle", "database_audit") => &filebeat::oracle_database_audit::default::Default,
        ("oracle", "memory") => &filebeat::oracle_memory::default::Default,
        ("oracle", "performance") => &filebeat::oracle_performance::default::Default,
        ("oracle", "sysmetric") => &filebeat::oracle_sysmetric::default::Default,
        ("oracle", "system_statistics") => &filebeat::oracle_system_statistics::default::Default,
        ("oracle", "tablespace") => &filebeat::oracle_tablespace::default::Default,
        ("oracle_weblogic", "admin_server") => {
            &filebeat::oracle_weblogic_admin_server::default::Default
        }
        ("oracle_weblogic", "deployed_application") => {
            &filebeat::oracle_weblogic_deployed_application::default::Default
        }
        ("oracle_weblogic", "domain") => &filebeat::oracle_weblogic_domain::default::Default,
        ("oracle_weblogic", "managed_server") => {
            &filebeat::oracle_weblogic_managed_server::default::Default
        }
        ("oracle_weblogic", "threadpool") => {
            &filebeat::oracle_weblogic_threadpool::default::Default
        }
        ("proofpoint_365totalprotection", "email") => {
            &filebeat::proofpoint_365totalprotection_email::default::Default
        }
        ("proofpoint_essentials", "threat") => {
            &filebeat::proofpoint_essentials_threat::default::Default
        }
        ("swimlane", "audit_logs") => &filebeat::swimlane_audit_logs::default::Default,
        ("swimlane", "swimlane_api") => &filebeat::swimlane_swimlane_api::default::Default,
        ("swimlane", "tenant_api") => &filebeat::swimlane_tenant_api::default::Default,
        ("swimlane", "turbine_api") => &filebeat::swimlane_turbine_api::default::Default,
        ("sysmon_linux", "log") => &filebeat::sysmon_linux_log::default::Default,
        ("tenable_ot_security", "assets") => {
            &filebeat::tenable_ot_security_assets::default::Default
        }
        ("tenable_ot_security", "events") => {
            &filebeat::tenable_ot_security_events::default::Default
        }
        ("tenable_ot_security", "system_log") => {
            &filebeat::tenable_ot_security_system_log::default::Default
        }
        ("tencent_cloud", "audit") => &filebeat::tencent_cloud_audit::default::Default,
        ("tencent_cloud", "clb") => &filebeat::tencent_cloud_clb::default::Default,
        ("tencent_cloud", "cos") => &filebeat::tencent_cloud_cos::default::Default,
        ("tencent_cloud", "scf") => &filebeat::tencent_cloud_scf::default::Default,
        ("ti_anyrun", "ioc") => &filebeat::ti_anyrun_ioc::default::Default,
        ("ti_crowdstrike", "intel") => &filebeat::ti_crowdstrike_intel::default::Default,
        ("ti_crowdstrike", "ioc") => &filebeat::ti_crowdstrike_ioc::default::Default,
        ("ti_custom", "indicator") => &filebeat::ti_custom_indicator::default::Default,
        ("ti_cyware_intel_exchange", "indicator") => {
            &filebeat::ti_cyware_intel_exchange_indicator::default::Default
        }
        ("ti_mandiant_advantage", "threat_intelligence") => {
            &filebeat::ti_mandiant_advantage_threat_intelligence::default::Default
        }
        ("ti_otx", "pulses_subscribed") => &filebeat::ti_otx_pulses_subscribed::default::Default,
        ("ti_otx", "threat") => &filebeat::ti_otx_threat::default::Default,
        ("traefik", "access") => &filebeat::traefik_access::default::Default,
        ("trellix_edr_cloud", "event") => &filebeat::trellix_edr_cloud_event::default::Default,
        ("websphere_application_server", "jdbc") => {
            &filebeat::websphere_application_server_jdbc::default::Default
        }
        ("websphere_application_server", "servlet") => {
            &filebeat::websphere_application_server_servlet::default::Default
        }
        ("websphere_application_server", "session_manager") => {
            &filebeat::websphere_application_server_session_manager::default::Default
        }
        ("websphere_application_server", "threadpool") => {
            &filebeat::websphere_application_server_threadpool::default::Default
        }
        ("workday", "activity") => &filebeat::workday_activity::default::Default,
        ("workday", "sign_on") => &filebeat::workday_sign_on::default::Default,
        ("cyberarkpas", "audit") => &filebeat::cyberarkpas_audit::default::Default,
        ("opencanary", "events") => &filebeat::opencanary_events::default::Default,
        ("oracle_weblogic", "access") => &filebeat::oracle_weblogic_access::default::Default,
        ("stormshield", "log") => &filebeat::stormshield_log::default::Default,
        ("symantec_endpoint", "log") => &filebeat::symantec_endpoint_log::default::Default,
        ("airflow", "statsd") => &filebeat::airflow_statsd::default::Default,
        ("airlock_digital", "agent") => &filebeat::airlock_digital_agent::default::Default,
        ("airlock_digital", "execution_histories") => &filebeat::airlock_digital_execution_histories::default::Default,
        ("airlock_digital", "server_activities") => &filebeat::airlock_digital_server_activities::default::Default,
        ("apache_spark", "application") => &filebeat::apache_spark_application::default::Default,
        ("apache_spark", "driver") => &filebeat::apache_spark_driver::default::Default,
        ("apache_spark", "executor") => &filebeat::apache_spark_executor::default::Default,
        ("apache_spark", "node") => &filebeat::apache_spark_node::default::Default,
        ("apache_tomcat", "access") => &filebeat::apache_tomcat_access::default::Default,
        ("apache_tomcat", "cache") => &filebeat::apache_tomcat_cache::default::Default,
        ("apache_tomcat", "catalina") => &filebeat::apache_tomcat_catalina::default::Default,
        ("apache_tomcat", "connection_pool") => &filebeat::apache_tomcat_connection_pool::default::Default,
        ("apache_tomcat", "localhost") => &filebeat::apache_tomcat_localhost::default::Default,
        ("apache_tomcat", "memory") => &filebeat::apache_tomcat_memory::default::Default,
        ("apache_tomcat", "request") => &filebeat::apache_tomcat_request::default::Default,
        ("apache_tomcat", "session") => &filebeat::apache_tomcat_session::default::Default,
        ("apache_tomcat", "thread_pool") => &filebeat::apache_tomcat_thread_pool::default::Default,
        ("armis", "alert") => &filebeat::armis_alert::default::Default,
        ("armis", "device") => &filebeat::armis_device::default::Default,
        ("armis", "vulnerability") => &filebeat::armis_vulnerability::default::Default,
        ("aws_bedrock", "guardrails") => &filebeat::aws_bedrock_guardrails::default::Default,
        ("aws_bedrock", "invocation") => &filebeat::aws_bedrock_invocation::default::Default,
        ("aws_bedrock", "runtime") => &filebeat::aws_bedrock_runtime::default::Default,
        ("axonius", "adapter") => &filebeat::axonius_adapter::default::Default,
        ("axonius", "alert_finding") => &filebeat::axonius_alert_finding::default::Default,
        ("axonius", "application") => &filebeat::axonius_application::default::Default,
        ("axonius", "compute") => &filebeat::axonius_compute::default::Default,
        ("axonius", "exposure") => &filebeat::axonius_exposure::default::Default,
        ("axonius", "gateway") => &filebeat::axonius_gateway::default::Default,
        ("axonius", "identity") => &filebeat::axonius_identity::default::Default,
        ("axonius", "incident") => &filebeat::axonius_incident::default::Default,
        ("axonius", "network") => &filebeat::axonius_network::default::Default,
        ("axonius", "storage") => &filebeat::axonius_storage::default::Default,
        ("axonius", "ticket") => &filebeat::axonius_ticket::default::Default,
        ("axonius", "user") => &filebeat::axonius_user::default::Default,
        ("azure_application_insights", "app_insights") => &filebeat::azure_application_insights_app_insights::default::Default,
        ("azure_frontdoor", "access") => &filebeat::azure_frontdoor_access::default::Default,
        ("azure_frontdoor", "health_probe") => &filebeat::azure_frontdoor_health_probe::default::Default,
        ("azure_frontdoor", "waf") => &filebeat::azure_frontdoor_waf::default::Default,
        ("beyondinsight_password_safe", "asset") => &filebeat::beyondinsight_password_safe_asset::default::Default,
        ("beyondinsight_password_safe", "managedaccount") => &filebeat::beyondinsight_password_safe_managedaccount::default::Default,
        ("beyondinsight_password_safe", "managedsystem") => &filebeat::beyondinsight_password_safe_managedsystem::default::Default,
        ("beyondinsight_password_safe", "session") => &filebeat::beyondinsight_password_safe_session::default::Default,
        ("beyondinsight_password_safe", "useraudit") => &filebeat::beyondinsight_password_safe_useraudit::default::Default,
        ("beyondtrust_epm", "audit") => &filebeat::beyondtrust_epm_audit::default::Default,
        ("beyondtrust_epm", "event") => &filebeat::beyondtrust_epm_event::default::Default,
        ("bitdefender", "push_configuration") => &filebeat::bitdefender_push_configuration::default::Default,
        ("bitdefender", "push_notifications") => &filebeat::bitdefender_push_notifications::default::Default,
        ("bitdefender", "push_statistics") => &filebeat::bitdefender_push_statistics::default::Default,
        ("bitwarden", "collection") => &filebeat::bitwarden_collection::default::Default,
        ("bitwarden", "event") => &filebeat::bitwarden_event::default::Default,
        ("bitwarden", "group") => &filebeat::bitwarden_group::default::Default,
        ("bitwarden", "member") => &filebeat::bitwarden_member::default::Default,
        ("bitwarden", "policy") => &filebeat::bitwarden_policy::default::Default,
        ("bluecoat", "director") => &filebeat::bluecoat_director::default::Default,
        ("cato_networks", "audit") => &filebeat::cato_networks_audit::default::Default,
        ("cato_networks", "event") => &filebeat::cato_networks_event::default::Default,
        ("checkpoint_harmony_endpoint", "antibot") => &filebeat::checkpoint_harmony_endpoint_antibot::default::Default,
        ("checkpoint_harmony_endpoint", "antimalware") => &filebeat::checkpoint_harmony_endpoint_antimalware::default::Default,
        ("checkpoint_harmony_endpoint", "forensics") => &filebeat::checkpoint_harmony_endpoint_forensics::default::Default,
        ("checkpoint_harmony_endpoint", "threatemulation") => &filebeat::checkpoint_harmony_endpoint_threatemulation::default::Default,
        ("checkpoint_harmony_endpoint", "threatextraction") => &filebeat::checkpoint_harmony_endpoint_threatextraction::default::Default,
        ("checkpoint_harmony_endpoint", "urlfiltering") => &filebeat::checkpoint_harmony_endpoint_urlfiltering::default::Default,
        ("checkpoint_harmony_endpoint", "zerophishing") => &filebeat::checkpoint_harmony_endpoint_zerophishing::default::Default,
        ("cisco_ise", "log") => &filebeat::cisco_ise_log::default::Default,
        ("citrix_adc", "interface") => &filebeat::citrix_adc_interface::default::Default,
        ("citrix_adc", "lbvserver") => &filebeat::citrix_adc_lbvserver::default::Default,
        ("citrix_adc", "log") => &filebeat::citrix_adc_log::default::Default,
        ("citrix_adc", "service") => &filebeat::citrix_adc_service::default::Default,
        ("citrix_adc", "system") => &filebeat::citrix_adc_system::default::Default,
        ("citrix_adc", "vpn") => &filebeat::citrix_adc_vpn::default::Default,
        ("claroty_ctd", "asset") => &filebeat::claroty_ctd_asset::default::Default,
        ("claroty_ctd", "baseline") => &filebeat::claroty_ctd_baseline::default::Default,
        ("claroty_ctd", "event") => &filebeat::claroty_ctd_event::default::Default,
        ("claroty_xdome", "alert") => &filebeat::claroty_xdome_alert::default::Default,
        ("claroty_xdome", "event") => &filebeat::claroty_xdome_event::default::Default,
        ("claroty_xdome", "vulnerability") => &filebeat::claroty_xdome_vulnerability::default::Default,
        ("cloud_asset_inventory", "asset_inventory") => &filebeat::cloud_asset_inventory_asset_inventory::default::Default,
        ("cloud_defend", "alerts") => &filebeat::cloud_defend_alerts::default::Default,
        ("cloud_defend", "file") => &filebeat::cloud_defend_file::default::Default,
        ("cloud_defend", "process") => &filebeat::cloud_defend_process::default::Default,
        ("cloudflare_logpush", "access_request") => &filebeat::cloudflare_logpush_access_request::default::Default,
        ("cloudflare_logpush", "audit") => &filebeat::cloudflare_logpush_audit::default::Default,
        ("cloudflare_logpush", "casb") => &filebeat::cloudflare_logpush_casb::default::Default,
        ("cloudflare_logpush", "device_posture") => &filebeat::cloudflare_logpush_device_posture::default::Default,
        ("cloudflare_logpush", "dlp_forensic_copies") => &filebeat::cloudflare_logpush_dlp_forensic_copies::default::Default,
        ("cloudflare_logpush", "dns") => &filebeat::cloudflare_logpush_dns::default::Default,
        ("cloudflare_logpush", "dns_firewall") => &filebeat::cloudflare_logpush_dns_firewall::default::Default,
        ("cloudflare_logpush", "email_security_alerts") => &filebeat::cloudflare_logpush_email_security_alerts::default::Default,
        ("cloudflare_logpush", "firewall_event") => &filebeat::cloudflare_logpush_firewall_event::default::Default,
        ("cloudflare_logpush", "gateway_dns") => &filebeat::cloudflare_logpush_gateway_dns::default::Default,
        ("cloudflare_logpush", "gateway_http") => &filebeat::cloudflare_logpush_gateway_http::default::Default,
        ("cloudflare_logpush", "gateway_network") => &filebeat::cloudflare_logpush_gateway_network::default::Default,
        ("cloudflare_logpush", "http_request") => &filebeat::cloudflare_logpush_http_request::default::Default,
        ("cloudflare_logpush", "magic_ids") => &filebeat::cloudflare_logpush_magic_ids::default::Default,
        ("cloudflare_logpush", "nel_report") => &filebeat::cloudflare_logpush_nel_report::default::Default,
        ("cloudflare_logpush", "network_analytics") => &filebeat::cloudflare_logpush_network_analytics::default::Default,
        ("cloudflare_logpush", "network_session") => &filebeat::cloudflare_logpush_network_session::default::Default,
        ("cloudflare_logpush", "page_shield_events") => &filebeat::cloudflare_logpush_page_shield_events::default::Default,
        ("cloudflare_logpush", "sinkhole_http") => &filebeat::cloudflare_logpush_sinkhole_http::default::Default,
        ("cloudflare_logpush", "spectrum_event") => &filebeat::cloudflare_logpush_spectrum_event::default::Default,
        ("cloudflare_logpush", "workers_trace") => &filebeat::cloudflare_logpush_workers_trace::default::Default,
        ("cockroachdb", "status") => &filebeat::cockroachdb_status::default::Default,
        ("couchbase", "bucket") => &filebeat::couchbase_bucket::default::Default,
        ("couchbase", "cache") => &filebeat::couchbase_cache::default::Default,
        ("couchbase", "cbl_replication") => &filebeat::couchbase_cbl_replication::default::Default,
        ("couchbase", "cluster") => &filebeat::couchbase_cluster::default::Default,
        ("couchbase", "database_stats") => &filebeat::couchbase_database_stats::default::Default,
        ("couchbase", "miscellaneous") => &filebeat::couchbase_miscellaneous::default::Default,
        ("couchbase", "node") => &filebeat::couchbase_node::default::Default,
        ("couchbase", "query_index") => &filebeat::couchbase_query_index::default::Default,
        ("couchbase", "resource") => &filebeat::couchbase_resource::default::Default,
        ("couchbase", "xdcr") => &filebeat::couchbase_xdcr::default::Default,
        ("cyberark_epm", "admin_audit") => &filebeat::cyberark_epm_admin_audit::default::Default,
        ("cyberark_epm", "aggregated_event") => &filebeat::cyberark_epm_aggregated_event::default::Default,
        ("cyberark_epm", "policyaudit_aggregated_event") => &filebeat::cyberark_epm_policyaudit_aggregated_event::default::Default,
        ("cyberark_epm", "policyaudit_raw_event") => &filebeat::cyberark_epm_policyaudit_raw_event::default::Default,
        ("cyberark_epm", "raw_event") => &filebeat::cyberark_epm_raw_event::default::Default,
        ("cybereason", "logon_session") => &filebeat::cybereason_logon_session::default::Default,
        ("cybereason", "malop_connection") => &filebeat::cybereason_malop_connection::default::Default,
        ("cybereason", "malop_process") => &filebeat::cybereason_malop_process::default::Default,
        ("cybereason", "malware") => &filebeat::cybereason_malware::default::Default,
        ("cybereason", "poll_malop") => &filebeat::cybereason_poll_malop::default::Default,
        ("cybereason", "suspicions_process") => &filebeat::cybereason_suspicions_process::default::Default,
        ("cyera", "audit") => &filebeat::cyera_audit::default::Default,
        ("cyera", "classification") => &filebeat::cyera_classification::default::Default,
        ("cyera", "datastore") => &filebeat::cyera_datastore::default::Default,
        ("cyera", "event") => &filebeat::cyera_event::default::Default,
        ("cyera", "issue") => &filebeat::cyera_issue::default::Default,
        ("darktrace", "ai_analyst_alert") => &filebeat::darktrace_ai_analyst_alert::default::Default,
        ("darktrace", "model_breach_alert") => &filebeat::darktrace_model_breach_alert::default::Default,
        ("darktrace", "system_status_alert") => &filebeat::darktrace_system_status_alert::default::Default,
        ("doppel", "alerts") => &filebeat::doppel_alerts::default::Default,
        ("doppler", "secret_read") => &filebeat::doppler_secret_read::default::Default,
        ("entityanalytics_okta", "entity") => &filebeat::entityanalytics_okta_entity::default::Default,
        ("eset_protect", "detection") => &filebeat::eset_protect_detection::default::Default,
        ("eset_protect", "device_task") => &filebeat::eset_protect_device_task::default::Default,
        ("eset_protect", "device_vulnerability") => &filebeat::eset_protect_device_vulnerability::default::Default,
        ("eset_protect", "event") => &filebeat::eset_protect_event::default::Default,
        ("etcd", "metrics") => &filebeat::etcd_metrics::default::Default,
        ("etcd", "self") => &filebeat::etcd_self::default::Default,
        ("etcd", "store") => &filebeat::etcd_store::default::Default,
        ("f5_bigip", "log") => &filebeat::f5_bigip_log::default::Default,
        ("forgerock", "am_access") => &filebeat::forgerock_am_access::default::Default,
        ("forgerock", "am_activity") => &filebeat::forgerock_am_activity::default::Default,
        ("forgerock", "am_authentication") => &filebeat::forgerock_am_authentication::default::Default,
        ("forgerock", "am_config") => &filebeat::forgerock_am_config::default::Default,
        ("forgerock", "am_core") => &filebeat::forgerock_am_core::default::Default,
        ("forgerock", "idm_access") => &filebeat::forgerock_idm_access::default::Default,
        ("forgerock", "idm_activity") => &filebeat::forgerock_idm_activity::default::Default,
        ("forgerock", "idm_authentication") => &filebeat::forgerock_idm_authentication::default::Default,
        ("forgerock", "idm_config") => &filebeat::forgerock_idm_config::default::Default,
        ("forgerock", "idm_core") => &filebeat::forgerock_idm_core::default::Default,
        ("forgerock", "idm_sync") => &filebeat::forgerock_idm_sync::default::Default,
        ("fortinet_fortimail", "log") => &filebeat::fortinet_fortimail_log::default::Default,
        ("fortinet_fortimanager", "log") => &filebeat::fortinet_fortimanager_log::default::Default,
        ("fortinet_fortiproxy", "log") => &filebeat::fortinet_fortiproxy_log::default::Default,
        ("gigamon", "ami") => &filebeat::gigamon_ami::default::Default,
        ("gitlab", "api") => &filebeat::gitlab_api::default::Default,
        ("gitlab", "application") => &filebeat::gitlab_application::default::Default,
        ("gitlab", "audit") => &filebeat::gitlab_audit::default::Default,
        ("gitlab", "auth") => &filebeat::gitlab_auth::default::Default,
        ("gitlab", "pages") => &filebeat::gitlab_pages::default::Default,
        ("gitlab", "production") => &filebeat::gitlab_production::default::Default,
        ("gitlab", "sidekiq") => &filebeat::gitlab_sidekiq::default::Default,
        ("google_scc", "asset") => &filebeat::google_scc_asset::default::Default,
        ("google_scc", "audit") => &filebeat::google_scc_audit::default::Default,
        ("google_scc", "finding") => &filebeat::google_scc_finding::default::Default,
        ("google_scc", "source") => &filebeat::google_scc_source::default::Default,
        ("google_secops", "alert") => &filebeat::google_secops_alert::default::Default,
        ("google_secops", "alert_v2") => &filebeat::google_secops_alert_v2::default::Default,
        ("grafana", "logs") => &filebeat::grafana_logs::default::Default,
        ("grafana", "metrics") => &filebeat::grafana_metrics::default::Default,
        ("hadoop", "application") => &filebeat::hadoop_application::default::Default,
        ("hadoop", "cluster") => &filebeat::hadoop_cluster::default::Default,
        ("hadoop", "datanode") => &filebeat::hadoop_datanode::default::Default,
        ("hadoop", "namenode") => &filebeat::hadoop_namenode::default::Default,
        ("hadoop", "node_manager") => &filebeat::hadoop_node_manager::default::Default,
        ("hpe_aruba_cx", "log") => &filebeat::hpe_aruba_cx_log::default::Default,
        ("influxdb", "advstatus") => &filebeat::influxdb_advstatus::default::Default,
        ("influxdb", "status") => &filebeat::influxdb_status::default::Default,
        ("infoblox_bloxone_ddi", "dhcp_lease") => &filebeat::infoblox_bloxone_ddi_dhcp_lease::default::Default,
        ("infoblox_bloxone_ddi", "dns_config") => &filebeat::infoblox_bloxone_ddi_dns_config::default::Default,
        ("infoblox_bloxone_ddi", "dns_data") => &filebeat::infoblox_bloxone_ddi_dns_data::default::Default,
        ("infoblox_threat_defense", "event") => &filebeat::infoblox_threat_defense_event::default::Default,
        ("island_browser", "admin_actions") => &filebeat::island_browser_admin_actions::default::Default,
        ("island_browser", "audit") => &filebeat::island_browser_audit::default::Default,
        ("island_browser", "compromised_credential") => &filebeat::island_browser_compromised_credential::default::Default,
        ("island_browser", "device") => &filebeat::island_browser_device::default::Default,
        ("island_browser", "user") => &filebeat::island_browser_user::default::Default,
        ("jamf_compliance_reporter", "log") => &filebeat::jamf_compliance_reporter_log::default::Default,
        ("jamf_protect", "alerts") => &filebeat::jamf_protect_alerts::default::Default,
        ("jamf_protect", "telemetry") => &filebeat::jamf_protect_telemetry::default::Default,
        ("jamf_protect", "web_threat_events") => &filebeat::jamf_protect_web_threat_events::default::Default,
        ("jamf_protect", "web_traffic_events") => &filebeat::jamf_protect_web_traffic_events::default::Default,
        ("kafka_connect", "client") => &filebeat::kafka_connect_client::default::Default,
        ("kafka_connect", "connector") => &filebeat::kafka_connect_connector::default::Default,
        ("kafka_connect", "task") => &filebeat::kafka_connect_task::default::Default,
        ("kafka_connect", "worker") => &filebeat::kafka_connect_worker::default::Default,
        ("kafka", "consumer") => &filebeat::kafka_consumer::default::Default,
        ("kafka", "controller") => &filebeat::kafka_controller::default::Default,
        ("kafka", "jvm") => &filebeat::kafka_jvm::default::Default,
        ("kafka", "log") => &filebeat::kafka_log::default::Default,
        ("kafka", "log_manager") => &filebeat::kafka_log_manager::default::Default,
        ("kafka", "network") => &filebeat::kafka_network::default::Default,
        ("kafka", "producer") => &filebeat::kafka_producer::default::Default,
        ("kafka", "raft") => &filebeat::kafka_raft::default::Default,
        ("kafka", "replica_manager") => &filebeat::kafka_replica_manager::default::Default,
        ("kafka", "topic") => &filebeat::kafka_topic::default::Default,
        ("memcached", "stats") => &filebeat::memcached_stats::default::Default,
        ("microsoft_defender_cloud", "assessment") => &filebeat::microsoft_defender_cloud_assessment::default::Default,
        ("microsoft_defender_cloud", "event") => &filebeat::microsoft_defender_cloud_event::default::Default,
        ("microsoft_sentinel", "alert") => &filebeat::microsoft_sentinel_alert::default::Default,
        ("microsoft_sentinel", "event") => &filebeat::microsoft_sentinel_event::default::Default,
        ("microsoft_sentinel", "incident") => &filebeat::microsoft_sentinel_incident::default::Default,
        ("mongodb_atlas", "alert") => &filebeat::mongodb_atlas_alert::default::Default,
        ("mongodb_atlas", "disk") => &filebeat::mongodb_atlas_disk::default::Default,
        ("mongodb_atlas", "hardware") => &filebeat::mongodb_atlas_hardware::default::Default,
        ("mongodb_atlas", "mongod_audit") => &filebeat::mongodb_atlas_mongod_audit::default::Default,
        ("mongodb_atlas", "mongod_database") => &filebeat::mongodb_atlas_mongod_database::default::Default,
        ("mongodb_atlas", "organization") => &filebeat::mongodb_atlas_organization::default::Default,
        ("mongodb_atlas", "process") => &filebeat::mongodb_atlas_process::default::Default,
        ("mongodb_atlas", "project") => &filebeat::mongodb_atlas_project::default::Default,
        ("nagios_xi", "events") => &filebeat::nagios_xi_events::default::Default,
        ("nagios_xi", "host") => &filebeat::nagios_xi_host::default::Default,
        ("nagios_xi", "service") => &filebeat::nagios_xi_service::default::Default,
        ("netskope", "alerts") => &filebeat::netskope_alerts::default::Default,
        ("netskope", "alerts_events_v2") => &filebeat::netskope_alerts_events_v2::default::Default,
        ("netskope", "events") => &filebeat::netskope_events::default::Default,
        ("netskope", "transaction") => &filebeat::netskope_transaction::default::Default,
        ("network_traffic", "amqp") => &filebeat::network_traffic_amqp::default::Default,
        ("network_traffic", "cassandra") => &filebeat::network_traffic_cassandra::default::Default,
        ("network_traffic", "dhcpv4") => &filebeat::network_traffic_dhcpv4::default::Default,
        ("network_traffic", "dns") => &filebeat::network_traffic_dns::default::Default,
        ("network_traffic", "flow") => &filebeat::network_traffic_flow::default::Default,
        ("network_traffic", "http") => &filebeat::network_traffic_http::default::Default,
        ("network_traffic", "icmp") => &filebeat::network_traffic_icmp::default::Default,
        ("network_traffic", "memcached") => &filebeat::network_traffic_memcached::default::Default,
        ("network_traffic", "mongodb") => &filebeat::network_traffic_mongodb::default::Default,
        ("network_traffic", "mysql") => &filebeat::network_traffic_mysql::default::Default,
        ("network_traffic", "nfs") => &filebeat::network_traffic_nfs::default::Default,
        ("network_traffic", "pgsql") => &filebeat::network_traffic_pgsql::default::Default,
        ("network_traffic", "redis") => &filebeat::network_traffic_redis::default::Default,
        ("network_traffic", "sip") => &filebeat::network_traffic_sip::default::Default,
        ("network_traffic", "thrift") => &filebeat::network_traffic_thrift::default::Default,
        ("network_traffic", "tls") => &filebeat::network_traffic_tls::default::Default,
        ("nozomi_networks", "alert") => &filebeat::nozomi_networks_alert::default::Default,
        ("nozomi_networks", "asset") => &filebeat::nozomi_networks_asset::default::Default,
        ("nozomi_networks", "audit") => &filebeat::nozomi_networks_audit::default::Default,
        ("nozomi_networks", "health") => &filebeat::nozomi_networks_health::default::Default,
        ("nozomi_networks", "node") => &filebeat::nozomi_networks_node::default::Default,
        ("nozomi_networks", "node_cve") => &filebeat::nozomi_networks_node_cve::default::Default,
        ("nozomi_networks", "session") => &filebeat::nozomi_networks_session::default::Default,
        ("nozomi_networks", "variable") => &filebeat::nozomi_networks_variable::default::Default,
        ("nvidia_gpu", "stats") => &filebeat::nvidia_gpu_stats::default::Default,
        ("o365_metrics", "active_users_services_user_counts") => &filebeat::o365_metrics_active_users_services_user_counts::default::Default,
        ("o365_metrics", "app_registrations") => &filebeat::o365_metrics_app_registrations::default::Default,
        ("o365_metrics", "entra_agent") => &filebeat::o365_metrics_entra_agent::default::Default,
        ("o365_metrics", "entra_alerts") => &filebeat::o365_metrics_entra_alerts::default::Default,
        ("o365_metrics", "entra_features") => &filebeat::o365_metrics_entra_features::default::Default,
        ("o365_metrics", "entra_id_users") => &filebeat::o365_metrics_entra_id_users::default::Default,
        ("o365_metrics", "groups_activity_group_detail") => &filebeat::o365_metrics_groups_activity_group_detail::default::Default,
        ("o365_metrics", "mailbox_usage_detail") => &filebeat::o365_metrics_mailbox_usage_detail::default::Default,
        ("o365_metrics", "mailbox_usage_quota_status") => &filebeat::o365_metrics_mailbox_usage_quota_status::default::Default,
        ("o365_metrics", "onedrive_usage_account_counts") => &filebeat::o365_metrics_onedrive_usage_account_counts::default::Default,
        ("o365_metrics", "onedrive_usage_account_detail") => &filebeat::o365_metrics_onedrive_usage_account_detail::default::Default,
        ("o365_metrics", "onedrive_usage_file_counts") => &filebeat::o365_metrics_onedrive_usage_file_counts::default::Default,
        ("o365_metrics", "onedrive_usage_storage") => &filebeat::o365_metrics_onedrive_usage_storage::default::Default,
        ("o365_metrics", "outlook_activity") => &filebeat::o365_metrics_outlook_activity::default::Default,
        ("o365_metrics", "outlook_app_usage_version_counts") => &filebeat::o365_metrics_outlook_app_usage_version_counts::default::Default,
        ("o365_metrics", "service_health") => &filebeat::o365_metrics_service_health::default::Default,
        ("o365_metrics", "sharepoint_site_usage_detail") => &filebeat::o365_metrics_sharepoint_site_usage_detail::default::Default,
        ("o365_metrics", "sharepoint_site_usage_storage") => &filebeat::o365_metrics_sharepoint_site_usage_storage::default::Default,
        ("o365_metrics", "subscriptions") => &filebeat::o365_metrics_subscriptions::default::Default,
        ("o365_metrics", "teams_call_quality") => &filebeat::o365_metrics_teams_call_quality::default::Default,
        ("o365_metrics", "teams_device_usage_user_counts") => &filebeat::o365_metrics_teams_device_usage_user_counts::default::Default,
        ("o365_metrics", "teams_user_activity_user_counts") => &filebeat::o365_metrics_teams_user_activity_user_counts::default::Default,
        ("o365_metrics", "teams_user_activity_user_detail") => &filebeat::o365_metrics_teams_user_activity_user_detail::default::Default,
        ("o365_metrics", "tenant_settings") => &filebeat::o365_metrics_tenant_settings::default::Default,
        ("o365_metrics", "viva_engage_device_usage_user_counts") => &filebeat::o365_metrics_viva_engage_device_usage_user_counts::default::Default,
        ("o365_metrics", "viva_engage_groups_activity_group_detail") => &filebeat::o365_metrics_viva_engage_groups_activity_group_detail::default::Default,
        ("openai", "audio_speeches") => &filebeat::openai_audio_speeches::default::Default,
        ("openai", "audio_transcriptions") => &filebeat::openai_audio_transcriptions::default::Default,
        ("openai", "audit") => &filebeat::openai_audit::default::Default,
        ("openai", "code_interpreter_sessions") => &filebeat::openai_code_interpreter_sessions::default::Default,
        ("openai", "completions") => &filebeat::openai_completions::default::Default,
        ("openai", "embeddings") => &filebeat::openai_embeddings::default::Default,
        ("openai", "images") => &filebeat::openai_images::default::Default,
        ("openai", "moderations") => &filebeat::openai_moderations::default::Default,
        ("openai", "rate_limits") => &filebeat::openai_rate_limits::default::Default,
        ("openai", "vector_stores") => &filebeat::openai_vector_stores::default::Default,
        ("osquery_manager", "action_responses") => &filebeat::osquery_manager_action_responses::default::Default,
        ("osquery_manager", "query_profile") => &filebeat::osquery_manager_query_profile::default::Default,
        ("osquery_manager", "result") => &filebeat::osquery_manager_result::default::Default,
        ("panw_cortex_xdr", "alerts") => &filebeat::panw_cortex_xdr_alerts::default::Default,
        ("panw_cortex_xdr", "event") => &filebeat::panw_cortex_xdr_event::default::Default,
        ("panw_cortex_xdr", "incidents") => &filebeat::panw_cortex_xdr_incidents::default::Default,
        ("prisma_cloud", "alert") => &filebeat::prisma_cloud_alert::default::Default,
        ("prisma_cloud", "audit") => &filebeat::prisma_cloud_audit::default::Default,
        ("prisma_cloud", "host") => &filebeat::prisma_cloud_host::default::Default,
        ("prisma_cloud", "host_profile") => &filebeat::prisma_cloud_host_profile::default::Default,
        ("prisma_cloud", "incident_audit") => &filebeat::prisma_cloud_incident_audit::default::Default,
        ("prisma_cloud", "misconfiguration") => &filebeat::prisma_cloud_misconfiguration::default::Default,
        ("prisma_cloud", "vulnerability") => &filebeat::prisma_cloud_vulnerability::default::Default,
        ("prometheus", "collector") => &filebeat::prometheus_collector::default::Default,
        ("prometheus", "query") => &filebeat::prometheus_query::default::Default,
        ("proofpoint_tap", "clicks_blocked") => &filebeat::proofpoint_tap_clicks_blocked::default::Default,
        ("proofpoint_tap", "clicks_permitted") => &filebeat::proofpoint_tap_clicks_permitted::default::Default,
        ("proofpoint_tap", "message_blocked") => &filebeat::proofpoint_tap_message_blocked::default::Default,
        ("proofpoint_tap", "message_delivered") => &filebeat::proofpoint_tap_message_delivered::default::Default,
        ("proxysg", "log") => &filebeat::proxysg_log::default::Default,
        ("qualys_gav", "asset") => &filebeat::qualys_gav_asset::default::Default,
        ("radware", "defensepro") => &filebeat::radware_defensepro::default::Default,
        ("rapid7_insightvm", "vulnerability") => &filebeat::rapid7_insightvm_vulnerability::default::Default,
        ("redisenterprise", "node") => &filebeat::redisenterprise_node::default::Default,
        ("redisenterprise", "proxy") => &filebeat::redisenterprise_proxy::default::Default,
        ("rubrik", "drives") => &filebeat::rubrik_drives::default::Default,
        ("rubrik", "filesets") => &filebeat::rubrik_filesets::default::Default,
        ("rubrik", "global_cluster_performance") => &filebeat::rubrik_global_cluster_performance::default::Default,
        ("rubrik", "managed_volumes") => &filebeat::rubrik_managed_volumes::default::Default,
        ("rubrik", "monitoring_jobs") => &filebeat::rubrik_monitoring_jobs::default::Default,
        ("rubrik", "mssql_databases") => &filebeat::rubrik_mssql_databases::default::Default,
        ("rubrik", "node_statistics") => &filebeat::rubrik_node_statistics::default::Default,
        ("rubrik", "physical_hosts") => &filebeat::rubrik_physical_hosts::default::Default,
        ("rubrik", "sla_domains") => &filebeat::rubrik_sla_domains::default::Default,
        ("rubrik", "tasks") => &filebeat::rubrik_tasks::default::Default,
        ("rubrik", "unmanaged_objects") => &filebeat::rubrik_unmanaged_objects::default::Default,
        ("rubrik", "virtual_machines") => &filebeat::rubrik_virtual_machines::default::Default,
        ("salesforce", "apex") => &filebeat::salesforce_apex::default::Default,
        ("salesforce", "logout") => &filebeat::salesforce_logout::default::Default,
        ("salesforce", "setupaudittrail") => &filebeat::salesforce_setupaudittrail::default::Default,
        ("sentinel_one_cloud_funnel", "event") => &filebeat::sentinel_one_cloud_funnel_event::default::Default,
        ("splunk", "alert") => &filebeat::splunk_alert::default::Default,
        ("splunk", "search") => &filebeat::splunk_search::default::Default,
        ("spycloud", "breach_catalog") => &filebeat::spycloud_breach_catalog::default::Default,
        ("spycloud", "breach_record") => &filebeat::spycloud_breach_record::default::Default,
        ("spycloud", "compass") => &filebeat::spycloud_compass::default::Default,
        ("sublime_security", "audit") => &filebeat::sublime_security_audit::default::Default,
        ("sublime_security", "email_message") => &filebeat::sublime_security_email_message::default::Default,
        ("sublime_security", "message_event") => &filebeat::sublime_security_message_event::default::Default,
        ("sysdig", "alerts") => &filebeat::sysdig_alerts::default::Default,
        ("sysdig", "cspm") => &filebeat::sysdig_cspm::default::Default,
        ("sysdig", "event") => &filebeat::sysdig_event::default::Default,
        ("sysdig", "vulnerability") => &filebeat::sysdig_vulnerability::default::Default,
        ("system", "application") => &filebeat::system_application::default::Default,
        ("system", "auth") => &filebeat::system_auth::default::Default,
        ("system", "process") => &filebeat::system_process::default::Default,
        ("system", "security") => &filebeat::system_security::default::Default,
        ("system", "syslog") => &filebeat::system_syslog::default::Default,
        ("system", "system") => &filebeat::system_system::default::Default,
        ("tanium", "action_history") => &filebeat::tanium_action_history::default::Default,
        ("tanium", "client_status") => &filebeat::tanium_client_status::default::Default,
        ("tanium", "discover") => &filebeat::tanium_discover::default::Default,
        ("tanium", "endpoint_config") => &filebeat::tanium_endpoint_config::default::Default,
        ("tanium", "reporting") => &filebeat::tanium_reporting::default::Default,
        ("tanium", "threat_response") => &filebeat::tanium_threat_response::default::Default,
        ("teleport", "audit") => &filebeat::teleport_audit::default::Default,
        ("tenable_io", "asset") => &filebeat::tenable_io_asset::default::Default,
        ("tenable_io", "audit") => &filebeat::tenable_io_audit::default::Default,
        ("tenable_io", "plugin") => &filebeat::tenable_io_plugin::default::Default,
        ("tenable_io", "scan") => &filebeat::tenable_io_scan::default::Default,
        ("tenable_io", "vulnerability") => &filebeat::tenable_io_vulnerability::default::Default,
        ("ti_abusech", "ja3_fingerprints") => &filebeat::ti_abusech_ja3_fingerprints::default::Default,
        ("ti_abusech", "malware") => &filebeat::ti_abusech_malware::default::Default,
        ("ti_abusech", "malwarebazaar") => &filebeat::ti_abusech_malwarebazaar::default::Default,
        ("ti_abusech", "sslblacklist") => &filebeat::ti_abusech_sslblacklist::default::Default,
        ("ti_abusech", "threatfox") => &filebeat::ti_abusech_threatfox::default::Default,
        ("ti_abusech", "url") => &filebeat::ti_abusech_url::default::Default,
        ("ti_anomali", "intelligence") => &filebeat::ti_anomali_intelligence::default::Default,
        ("ti_anomali", "threatstream") => &filebeat::ti_anomali_threatstream::default::Default,
        ("ti_eset", "apt") => &filebeat::ti_eset_apt::default::Default,
        ("ti_eset", "botnet") => &filebeat::ti_eset_botnet::default::Default,
        ("ti_eset", "cc") => &filebeat::ti_eset_cc::default::Default,
        ("ti_eset", "domains") => &filebeat::ti_eset_domains::default::Default,
        ("ti_eset", "files") => &filebeat::ti_eset_files::default::Default,
        ("ti_eset", "ip") => &filebeat::ti_eset_ip::default::Default,
        ("ti_eset", "url") => &filebeat::ti_eset_url::default::Default,
        ("ti_flashpoint", "alert") => &filebeat::ti_flashpoint_alert::default::Default,
        ("ti_flashpoint", "indicator") => &filebeat::ti_flashpoint_indicator::default::Default,
        ("ti_flashpoint", "vulnerability") => &filebeat::ti_flashpoint_vulnerability::default::Default,
        ("ti_google_threat_intelligence", "cryptominer") => &filebeat::ti_google_threat_intelligence_cryptominer::default::Default,
        ("ti_google_threat_intelligence", "first_stage_delivery_vectors") => &filebeat::ti_google_threat_intelligence_first_stage_delivery_vectors::default::Default,
        ("ti_google_threat_intelligence", "infostealer") => &filebeat::ti_google_threat_intelligence_infostealer::default::Default,
        ("ti_google_threat_intelligence", "ioc_stream") => &filebeat::ti_google_threat_intelligence_ioc_stream::default::Default,
        ("ti_google_threat_intelligence", "iot") => &filebeat::ti_google_threat_intelligence_iot::default::Default,
        ("ti_google_threat_intelligence", "linux") => &filebeat::ti_google_threat_intelligence_linux::default::Default,
        ("ti_google_threat_intelligence", "malicious_network_infrastructure") => &filebeat::ti_google_threat_intelligence_malicious_network_infrastructure::default::Default,
        ("ti_google_threat_intelligence", "malware") => &filebeat::ti_google_threat_intelligence_malware::default::Default,
        ("ti_google_threat_intelligence", "mobile") => &filebeat::ti_google_threat_intelligence_mobile::default::Default,
        ("ti_google_threat_intelligence", "osx") => &filebeat::ti_google_threat_intelligence_osx::default::Default,
        ("ti_google_threat_intelligence", "phishing") => &filebeat::ti_google_threat_intelligence_phishing::default::Default,
        ("ti_google_threat_intelligence", "ransomware") => &filebeat::ti_google_threat_intelligence_ransomware::default::Default,
        ("ti_google_threat_intelligence", "threat_actor") => &filebeat::ti_google_threat_intelligence_threat_actor::default::Default,
        ("ti_google_threat_intelligence", "trending") => &filebeat::ti_google_threat_intelligence_trending::default::Default,
        ("ti_google_threat_intelligence", "vulnerability") => &filebeat::ti_google_threat_intelligence_vulnerability::default::Default,
        ("ti_google_threat_intelligence", "vulnerability_weaponization") => &filebeat::ti_google_threat_intelligence_vulnerability_weaponization::default::Default,
        ("ti_misp", "threat") => &filebeat::ti_misp_threat::default::Default,
        ("ti_misp", "threat_attributes") => &filebeat::ti_misp_threat_attributes::default::Default,
        ("ti_opencti", "indicator") => &filebeat::ti_opencti_indicator::default::Default,
        ("ti_rapid7_threat_command", "alert") => &filebeat::ti_rapid7_threat_command_alert::default::Default,
        ("ti_rapid7_threat_command", "ioc") => &filebeat::ti_rapid7_threat_command_ioc::default::Default,
        ("ti_rapid7_threat_command", "vulnerability") => &filebeat::ti_rapid7_threat_command_vulnerability::default::Default,
        ("ti_recordedfuture", "identity_detection") => &filebeat::ti_recordedfuture_identity_detection::default::Default,
        ("ti_recordedfuture", "playbook_alert") => &filebeat::ti_recordedfuture_playbook_alert::default::Default,
        ("ti_recordedfuture", "threat") => &filebeat::ti_recordedfuture_threat::default::Default,
        ("ti_recordedfuture", "triggered_alert") => &filebeat::ti_recordedfuture_triggered_alert::default::Default,
        ("ti_socradar_taxii", "indicator") => &filebeat::ti_socradar_taxii_indicator::default::Default,
        ("ti_threatconnect", "indicator") => &filebeat::ti_threatconnect_indicator::default::Default,
        ("ti_threatq", "threat") => &filebeat::ti_threatq_threat::default::Default,
        ("trellix_epo_cloud", "device") => &filebeat::trellix_epo_cloud_device::default::Default,
        ("trellix_epo_cloud", "event") => &filebeat::trellix_epo_cloud_event::default::Default,
        ("trellix_epo_cloud", "group") => &filebeat::trellix_epo_cloud_group::default::Default,
        ("trend_micro_vision_one", "alert") => &filebeat::trend_micro_vision_one_alert::default::Default,
        ("trend_micro_vision_one", "audit") => &filebeat::trend_micro_vision_one_audit::default::Default,
        ("trend_micro_vision_one", "detection") => &filebeat::trend_micro_vision_one_detection::default::Default,
        ("trend_micro_vision_one", "endpoint_activity") => &filebeat::trend_micro_vision_one_endpoint_activity::default::Default,
        ("trend_micro_vision_one", "network_activity") => &filebeat::trend_micro_vision_one_network_activity::default::Default,
        ("trend_micro_vision_one", "telemetry") => &filebeat::trend_micro_vision_one_telemetry::default::Default,
        ("tychon", "arp") => &filebeat::tychon_arp::default::Default,
        ("tychon", "browser") => &filebeat::tychon_browser::default::Default,
        ("tychon", "ciphers") => &filebeat::tychon_ciphers::default::Default,
        ("tychon", "cmrs") => &filebeat::tychon_cmrs::default::Default,
        ("tychon", "coams") => &filebeat::tychon_coams::default::Default,
        ("tychon", "cpu") => &filebeat::tychon_cpu::default::Default,
        ("tychon", "cve") => &filebeat::tychon_cve::default::Default,
        ("tychon", "epp") => &filebeat::tychon_epp::default::Default,
        ("tychon", "exposedservice") => &filebeat::tychon_exposedservice::default::Default,
        ("tychon", "externaldevicecontrol") => &filebeat::tychon_externaldevicecontrol::default::Default,
        ("tychon", "features") => &filebeat::tychon_features::default::Default,
        ("tychon", "harddrive") => &filebeat::tychon_harddrive::default::Default,
        ("tychon", "hardware") => &filebeat::tychon_hardware::default::Default,
        ("tychon", "host") => &filebeat::tychon_host::default::Default,
        ("tychon", "networkadapter") => &filebeat::tychon_networkadapter::default::Default,
        ("tychon", "softwareinventory") => &filebeat::tychon_softwareinventory::default::Default,
        ("tychon", "stig") => &filebeat::tychon_stig::default::Default,
        ("tychon", "systemcerts") => &filebeat::tychon_systemcerts::default::Default,
        ("tychon", "volume") => &filebeat::tychon_volume::default::Default,
        ("vectra_detect", "log") => &filebeat::vectra_detect_log::default::Default,
        ("vectra_rux", "audit") => &filebeat::vectra_rux_audit::default::Default,
        ("vectra_rux", "detection_event") => &filebeat::vectra_rux_detection_event::default::Default,
        ("vectra_rux", "entity_event") => &filebeat::vectra_rux_entity_event::default::Default,
        ("vectra_rux", "health") => &filebeat::vectra_rux_health::default::Default,
        ("vectra_rux", "lockdown") => &filebeat::vectra_rux_lockdown::default::Default,
        ("vsphere", "cluster") => &filebeat::vsphere_cluster::default::Default,
        ("vsphere", "datastore") => &filebeat::vsphere_datastore::default::Default,
        ("vsphere", "datastorecluster") => &filebeat::vsphere_datastorecluster::default::Default,
        ("vsphere", "host") => &filebeat::vsphere_host::default::Default,
        ("vsphere", "log") => &filebeat::vsphere_log::default::Default,
        ("vsphere", "network") => &filebeat::vsphere_network::default::Default,
        ("vsphere", "resourcepool") => &filebeat::vsphere_resourcepool::default::Default,
        ("vsphere", "virtualmachine") => &filebeat::vsphere_virtualmachine::default::Default,
        ("wiz", "audit") => &filebeat::wiz_audit::default::Default,
        ("wiz", "cloud_configuration_finding") => &filebeat::wiz_cloud_configuration_finding::default::Default,
        ("wiz", "cloud_configuration_finding_full_posture") => &filebeat::wiz_cloud_configuration_finding_full_posture::default::Default,
        ("wiz", "defend") => &filebeat::wiz_defend::default::Default,
        ("wiz", "defend_v2") => &filebeat::wiz_defend_v2::default::Default,
        ("wiz", "issue") => &filebeat::wiz_issue::default::Default,
        ("wiz", "vulnerability") => &filebeat::wiz_vulnerability::default::Default,
        ("xm_cyber", "audit_trail") => &filebeat::xm_cyber_audit_trail::default::Default,
        ("xm_cyber", "device") => &filebeat::xm_cyber_device::default::Default,
        ("xm_cyber", "entity_inventory") => &filebeat::xm_cyber_entity_inventory::default::Default,
        ("xm_cyber", "product") => &filebeat::xm_cyber_product::default::Default,
        ("xm_cyber", "risk_score") => &filebeat::xm_cyber_risk_score::default::Default,
        ("xm_cyber", "vulnerability") => &filebeat::xm_cyber_vulnerability::default::Default,
        ("zoom", "activity") => &filebeat::zoom_activity::default::Default,
        ("zoom", "meeting_activity") => &filebeat::zoom_meeting_activity::default::Default,
        ("zoom", "operation") => &filebeat::zoom_operation::default::Default,
        ("zoom", "webhook") => &filebeat::zoom_webhook::default::Default,
        ("zscaler_zpa", "app_connector_status") => &filebeat::zscaler_zpa_app_connector_status::default::Default,
        ("zscaler_zpa", "audit") => &filebeat::zscaler_zpa_audit::default::Default,
        ("zscaler_zpa", "browser_access") => &filebeat::zscaler_zpa_browser_access::default::Default,
        ("zscaler_zpa", "user_activity") => &filebeat::zscaler_zpa_user_activity::default::Default,
        ("zscaler_zpa", "user_status") => &filebeat::zscaler_zpa_user_status::default::Default,
        ("doppler", "activity") => &filebeat::doppler_activity::default::Default,
        ("rapid7_insightvm", "asset") => &filebeat::rapid7_insightvm_asset::default::Default,
        ("rapid7_insightvm", "asset_vulnerability") => &filebeat::rapid7_insightvm_asset_vulnerability::default::Default,
        ("salesforce", "login") => &filebeat::salesforce_login::default::Default,
        ("servicenow", "event") => &filebeat::servicenow_event::default::Default,
        _ => return None,
    })
}

/// One fixture's confirmed output, and where it came from.
struct Captured {
    source: String,
    data_stream: String,
    fixture: String,
    engine: String,
    /// The integrations commit the pipelines were taken from. A score is only
    /// comparable to another measured against the same one.
    integrations_sha: String,
    /// The pipeline `compat.py` installed. Anything not prefixed `compat-`
    /// was written by an earlier tool and the capture is stale.
    entry_pipeline: String,
    input: Vec<Value>,
    expected: Vec<Value>,
}

impl Captured {
    /// Whether Elastic's own run failed on every event.
    ///
    /// A uniform `pipeline_error` almost always means the input shape rather
    /// than the pipeline -- a fixture already in the Beats envelope wrapped a
    /// second time. Comparing against it reads as a broken transform.
    fn capture_failed(&self) -> bool {
        !self.expected.is_empty()
            && self
                .expected
                .iter()
                .all(|e| e.pointer("/event/kind").and_then(Value::as_str) == Some("pipeline_error"))
    }

    fn is_stale(&self) -> bool {
        !self.entry_pipeline.starts_with("compat-")
    }
}

fn read_ndjson(path: &Path) -> Vec<Value> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Every captured fixture under the corpus root.
fn captured() -> Vec<Captured> {
    let root = corpus_root();
    let mut out = Vec::new();

    let Ok(sources) = std::fs::read_dir(&root) else {
        return out;
    };
    for source in sources.flatten().filter(|e| e.path().is_dir()) {
        let source_name = source.file_name().to_string_lossy().into_owned();
        let Ok(streams) = std::fs::read_dir(source.path()) else {
            continue;
        };
        for stream in streams.flatten().filter(|e| e.path().is_dir()) {
            let stream_name = stream.file_name().to_string_lossy().into_owned();
            let Ok(fixtures) = std::fs::read_dir(stream.path()) else {
                continue;
            };
            for fixture in fixtures.flatten().filter(|e| e.path().is_dir()) {
                let dir = fixture.path();
                let input = read_ndjson(&dir.join("input.ndjson"));
                let expected = read_ndjson(&dir.join("expected.ndjson"));
                if input.is_empty() || expected.is_empty() {
                    continue;
                }
                let meta: Value = std::fs::read_to_string(dir.join("meta.json"))
                    .ok()
                    .and_then(|t| serde_json::from_str(&t).ok())
                    .unwrap_or(Value::Null);
                out.push(Captured {
                    source: source_name.clone(),
                    data_stream: stream_name.clone(),
                    fixture: fixture.file_name().to_string_lossy().into_owned(),
                    engine: meta
                        .get("elasticsearch_version")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    integrations_sha: meta
                        .get("integrations_sha")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    entry_pipeline: meta
                        .get("entry_pipeline")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    input,
                    expected,
                });
            }
        }
    }
    out.sort_by(|a, b| (&a.source, &a.fixture).cmp(&(&b.source, &b.fixture)));
    out
}

/// A running count of events and of the fields inside them.
///
/// An event score alone cannot separate a transform that is one field short on
/// every event from one that is unrecognisably wrong, and the two need
/// completely different work. 20% of events at 96% of fields is nearly done;
/// 20% at 40% is not started.
#[derive(Default, Clone, Copy)]
struct Score {
    events: usize,
    events_matched: usize,
    events_errored: usize,
    /// Events Elastic's own pipeline failed on, so the capture holds only its
    /// failure and no expectation of correct output.
    events_unanswered: usize,
    fields: usize,
    fields_wrong: usize,
    fields_extra: usize,
}

impl Score {
    fn add(&mut self, other: Self) {
        self.events += other.events;
        self.events_matched += other.events_matched;
        self.events_errored += other.events_errored;
        self.events_unanswered += other.events_unanswered;
        self.fields += other.fields;
        self.fields_wrong += other.fields_wrong;
        self.fields_extra += other.fields_extra;
    }

    fn line(&self) -> String {
        let pct = |n: usize, d: usize| {
            if d == 0 {
                100.0
            } else {
                100.0 * n as f64 / d as f64
            }
        };
        let unanswered = if self.events_unanswered == 0 {
            String::new()
        } else {
            format!(", {} elastic-errored", self.events_unanswered)
        };
        format!(
            "events {}/{} ({:.0}%), fields {}/{} ({:.1}%), {} extra, {} errors{unanswered}",
            self.events_matched,
            self.events,
            pct(self.events_matched, self.events),
            self.fields - self.fields_wrong,
            self.fields,
            pct(self.fields - self.fields_wrong, self.fields),
            self.fields_extra,
            self.events_errored,
        )
    }
}

/// How many of an expected document's fields are actually compared.
///
/// The policy's skipped paths are not measured -- counting them would inflate
/// every field score by the same fixed amount and hide movement.
fn compared_field_count(source: &str, expected: &Value) -> usize {
    flatten_value(expected)
        .keys()
        .filter(|path| !policy().skips(Some(source), path))
        .count()
}

/// One entry in the events-unlocked ranking.
struct Blocker {
    path: String,
    /// Failing events this path is wrong in.
    appears: usize,
    /// Events that would pass once this path AND everything above it is fixed.
    unlocks: usize,
}

/// The fields whose repair would unlock the most events.
///
/// Greedy set cover over the failing events: take the path wrong in the most
/// of them, count the events it finishes off, strip it, and repeat. Ranking by
/// raw frequency instead conflates "appears often" with "worth fixing" -- a
/// path wrong in 400 events that are ALSO wrong in four other places unlocks
/// nothing on its own, and `unlocks` is what says so.
fn events_unlocked(failures: &[BTreeSet<String>], top: usize) -> Vec<Blocker> {
    let mut events: Vec<BTreeSet<String>> = failures.to_vec();
    let mut ranked = Vec::new();

    while ranked.len() < top {
        events.retain(|event| !event.is_empty());
        if events.is_empty() {
            break;
        }

        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for event in &events {
            for path in event {
                *counts.entry(path.as_str()).or_default() += 1;
            }
        }

        // Ties break on the lexicographically first path, so the ranking is the
        // same on every run and a diff of two reports means something.
        let Some((path, appears)) = counts
            .iter()
            .max_by_key(|(path, count)| (**count, std::cmp::Reverse(*path)))
            .map(|(path, count)| ((*path).to_owned(), *count))
        else {
            break;
        };

        let unlocks = events
            .iter()
            .filter(|event| event.len() == 1 && event.contains(path.as_str()))
            .count();
        for event in &mut events {
            event.remove(path.as_str());
        }
        ranked.push(Blocker {
            path,
            appears,
            unlocks,
        });
    }
    ranked
}

/// Report how each transform compares against Elastic's confirmed output.
///
/// Reporting, not ratcheting: the corpus is regenerated against whichever
/// engine and integrations commit the operator chose, so a number here is only
/// comparable to another run over the same corpus. `scripts/compat.py audit`
/// is where the tracked figures live.
#[test]
fn transforms_match_elastics_confirmed_output() {
    dfe_runtime::testutil::on_a_deep_stack(score_the_corpus);
}

fn score_the_corpus() {
    let fixtures = captured();
    if fixtures.is_empty() {
        println!(
            "no compat corpus at {} -- run `python3 scripts/compat.py generate --all`",
            corpus_root().display()
        );
        return;
    }

    // The corpus was captured with MaxMind's databases and ours are DB-IP
    // Lite, so every geoip-derived field is excluded from the comparison. The
    // enrichment's SIDE EFFECTS are not excluded, though: an ASN hit MaxMind
    // does not have makes gcp/vpcflow's `source.as.asn` rename land on an
    // occupied `source.as.number`, which fails the document and skips the
    // twenty-nine removes behind it. Comparing against output built from a
    // database we do not have means running without one.
    assert!(
        dfe_runtime::enrichment::geoip_global::disable(),
        "a lookup has already loaded the databases -- disable must come first"
    );

    // Whole-diff output for the named sources, comma-separated. Printing every
    // difference for every source buries the summary the ranking exists to give.
    let detail: BTreeSet<String> = std::env::var("DFE_COMPAT_DETAIL")
        .unwrap_or_default()
        .split(',')
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect();
    let dump = std::env::var("DFE_COMPAT_DUMP").ok();

    // On for every run, because this is the only place the reach number exists
    // and `NEVER_RAN_SCRIPTS` now ratchets it. The lock is taken once per
    // script EXECUTION, which measured inside the run-to-run noise.
    dfe_runtime::painless_stats::reset();
    dfe_runtime::painless_stats::enable_catalogue(true);

    // Written to the named path when asked, for working out WHICH scripts.
    let unhandled_dump = std::env::var("DFE_PAINLESS_UNHANDLED").ok();

    // Score only the named sources, comma-separated. For the inner loop while
    // one source is being worked on; the ratchet is SKIPPED under it, because a
    // partial run cannot say whether another source went down.
    let only: BTreeSet<String> = std::env::var("DFE_COMPAT_ONLY")
        .unwrap_or_default()
        .split(',')
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect();

    // Captures are independent, so they score across cores. Each collects its
    // output rather than printing, and the merge below replays them in corpus
    // order: a gate whose totals moved with scheduling would be no gate.
    let outcomes: Vec<Outcome> = scoring_pool().install(|| {
        fixtures
            .par_iter()
            .map(|capture| score_capture(capture, &only, &detail, dump.as_deref()))
            .collect()
    });

    let mut unmapped = Vec::new();
    let mut by_source: BTreeMap<String, Score> = BTreeMap::new();
    let mut failures: BTreeMap<String, Vec<BTreeSet<String>>> = BTreeMap::new();
    let mut total = Score::default();

    for outcome in outcomes {
        print!("{}", outcome.output);
        if let Some(name) = outcome.unmapped {
            unmapped.push(name);
        }
        let Some(score) = outcome.score else {
            continue;
        };
        if !outcome.failures.is_empty() {
            failures
                .entry(outcome.source.clone())
                .or_default()
                .extend(outcome.failures);
        }
        by_source.entry(outcome.source).or_default().add(score);
        total.add(score);
    }

    print_and_check(
        &fixtures,
        &unmapped,
        &by_source,
        &failures,
        total,
        &only,
        unhandled_dump.as_deref(),
    );
}

/// What one capture produced: its output, its score, and the fields that
/// disagreed. Collected rather than emitted so a parallel run reads exactly
/// like a serial one.
#[derive(Default)]
struct Outcome {
    source: String,
    output: String,
    score: Option<Score>,
    failures: Vec<BTreeSet<String>>,
    unmapped: Option<String>,
}

/// One pool for the whole run, sized to the host and given the deep stack the
/// transforms need.
///
/// Rayon's default 2 MB worker stack overflows on the recursive shapes, which
/// is why the test body itself already runs on a 64 MB thread. The stacks are
/// reserved address space rather than resident pages, so a wide host pays
/// nothing for them.
///
/// `DFE_COMPAT_THREADS` overrides the width, for pinning a measurement or for
/// bisecting a failure that only appears in parallel.
fn scoring_pool() -> rayon::ThreadPool {
    let threads = std::env::var("DFE_COMPAT_THREADS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|threads| *threads > 0)
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(8, std::num::NonZero::get));

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(64 * 1024 * 1024)
        .build()
        .expect("build the corpus scoring pool")
}

fn score_capture(
    capture: &Captured,
    only: &BTreeSet<String>,
    detail: &BTreeSet<String>,
    dump: Option<&str>,
) -> Outcome {
    use std::fmt::Write as _;

    let mut out = Outcome {
        source: capture.source.clone(),
        ..Outcome::default()
    };

    if !only.is_empty() && !only.contains(&capture.source) {
        return out;
    }
    let Some(transform) = transform_for(&capture.source, &capture.data_stream) else {
        out.unmapped = Some(format!("{}/{}", capture.source, capture.data_stream));
        return out;
    };

    // Say why a capture is not worth comparing BEFORE printing a score
    // against it, or a zero reads as a broken transform.
    if capture.is_stale() {
        let _ = writeln!(
            out.output,
            "[{}/{}] SKIPPED: written by an earlier tool ({}), regenerate it",
            capture.source, capture.fixture, capture.entry_pipeline,
        );
        return out;
    }
    if capture.capture_failed() {
        let _ = writeln!(
            out.output,
            "[{}/{}] SKIPPED: Elastic errored on all {} events, so the capture \
             carries no expectation -- check the input shape",
            capture.source,
            capture.fixture,
            capture.expected.len(),
        );
        return out;
    }

    let mut score = Score::default();
    {
        let failures = &mut out.failures;
        for (i, raw) in capture.input.iter().enumerate() {
            let Some(expected) = capture.expected.get(i) else {
                continue;
            };
            // Elastic's own pipeline failed on this one, so the capture holds
            // its failure and no expectation of correct output. Scoring
            // against it counts our CORRECT output as a miss. The bare
            // `_compat_error` OBJECT is the same thing one layer down --
            // simulate errored with no on_failure to shape a document.
            if expected.pointer("/event/kind").and_then(Value::as_str) == Some("pipeline_error")
                || expected.get("_compat_error").is_some_and(Value::is_object)
            {
                score.events_unanswered += 1;
                continue;
            }

            // Elasticsearch returned NOTHING for this document -- its
            // pipeline dropped it, and the capture records the marker so the
            // corpus stays aligned. The matching outcome is our transform
            // dropping it too, scored as one whole-event field.
            let expected_drop =
                expected.get("_compat_error").and_then(Value::as_str) == Some("no result");

            score.events += 1;
            score.fields += if expected_drop {
                1
            } else {
                compared_field_count(&capture.source, expected)
            };

            let mut event = Event::new(raw.clone());
            match transform.transform(&mut event) {
                Err(err) => {
                    // The count alone is a dead end. An errored event scores
                    // zero however right its fields are -- google_workspace's
                    // chrome and data_studio captures were 300/300 and 487/487
                    // correct and still 0/6 and 0/12 events -- and the reason
                    // lived only inside a discarded `Err`. Printed under the
                    // same switch that dumps a document.
                    if dump.as_deref() == Some(capture.fixture.as_str()) {
                        // The document AS IT STANDS when the error is raised,
                        // not the input: a type mismatch names the path it
                        // tripped on and says nothing about what put the wrong
                        // value there.
                        let _ = writeln!(
                            out.output,
                            "  {}[{i}] ERRORED: {err}\n    PARTIAL: {}",
                            capture.fixture,
                            serde_json::to_string(event.as_value()).unwrap_or_default()
                        );
                    }
                    score.events_errored += 1;
                    continue;
                }
                Ok(TransformResult::Drop) if expected_drop => {
                    score.events_matched += 1;
                    continue;
                }
                Ok(TransformResult::Drop) => {
                    // Dropped an event Elastic kept: every expected field is
                    // gone, and the ranking hears about it under one name.
                    score.fields_wrong += compared_field_count(&capture.source, expected);
                    failures.push(BTreeSet::from(["_dropped".to_string()]));
                    continue;
                }
                Ok(_) if expected_drop => {
                    // Kept an event Elastic dropped.
                    score.fields_wrong += 1;
                    failures.push(BTreeSet::from(["_not_dropped".to_string()]));
                    continue;
                }
                Ok(_) => {}
            }

            let diff = JsonDiff::compare_for(
                Some(&capture.source),
                expected,
                event.as_value(),
                MatchMode::Semantic,
            );
            if diff.is_match() {
                score.events_matched += 1;
                continue;
            }

            if detail.contains(&capture.source) {
                let _ = writeln!(out.output, "  {}[{i}]: {diff}", capture.fixture);
            }
            // A diff names the fields that disagree; it does not say what ELSE
            // the transform wrote, which is where a stray value's real source
            // shows up. `DFE_COMPAT_DUMP=<fixture>` prints the whole document.
            if dump == Some(capture.fixture.as_str()) {
                let _ = writeln!(
                    out.output,
                    "  {}[{i}] GOT: {}",
                    capture.fixture,
                    serde_json::to_string(event.as_value()).unwrap_or_default()
                );
            }

            let mut paths = BTreeSet::new();
            for field in &diff.diffs {
                match field.kind {
                    DiffKind::Extra { .. } => score.fields_extra += 1,
                    DiffKind::Missing { .. } | DiffKind::Mismatch { .. } => score.fields_wrong += 1,
                }
                paths.insert(field.path.clone());
            }
            failures.push(paths);
        }
    }

    let _ = writeln!(
        out.output,
        "[{}/{}] {} (es {})",
        capture.source,
        capture.fixture,
        score.line(),
        capture.engine,
    );
    out.score = Some(score);
    out
}

/// The ranking, the totals, the optional reach dump, and the ratchet.
fn print_and_check(
    fixtures: &[Captured],
    unmapped: &[String],
    by_source: &BTreeMap<String, Score>,
    failures: &BTreeMap<String, Vec<BTreeSet<String>>>,
    total: Score,
    only: &BTreeSet<String>,
    unhandled_dump: Option<&str>,
) {
    println!("\n=== per source ===");
    for (source, score) in by_source {
        println!("{source:<20} {}", score.line());
        for blocker in events_unlocked(failures.get(source).map_or(&[], Vec::as_slice), 6) {
            println!(
                "      wrong in {:>5}, unlocks {:>5}   {}",
                blocker.appears, blocker.unlocks, blocker.path
            );
        }
    }
    println!("\n{:<20} {}", "TOTAL", total.line());

    dfe_runtime::painless_stats::enable_catalogue(false);
    let reach = dfe_runtime::painless_stats::reach();
    let never = reach.iter().filter(|(_, ran, _)| *ran == 0).count();
    let handled = dfe_runtime::painless_stats::handled();
    let skipped = dfe_runtime::painless_stats::unhandled();
    println!(
        "\npainless runtime reach: {handled} handled, {skipped} skipped across {} \
         distinct scripts, {never} of which NEVER ran",
        reach.len(),
    );

    if let Some(path) = unhandled_dump {
        let rows: Vec<serde_json::Value> = reach
            .iter()
            .map(|(script, ran, skipped)| {
                serde_json::json!({ "ran": ran, "skipped": skipped, "script": script })
            })
            .collect();
        println!("  the per-script catalogue is at {path}");
        std::fs::write(
            path,
            serde_json::to_string_pretty(&serde_json::json!({
                "handled": handled,
                "unhandled": skipped,
                "distinct": rows.len(),
                "never_ran": never,
                "scripts": rows,
            }))
            .expect("the catalogue is plain strings and counts"),
        )
        .expect("DFE_PAINLESS_UNHANDLED names a writable path");
    }

    assert!(
        unmapped.is_empty(),
        "the corpus holds sources with no transform mapped in this test: {unmapped:?}"
    );

    // A partial run cannot say whether another source went down, so it reports
    // and never ratchets. The whole-corpus run stays the only gate.
    if only.is_empty() {
        check_baseline(by_source, &provenance(fixtures), never);
    } else {
        println!("\nDFE_COMPAT_ONLY is set, so neither ratchet was checked");
    }
}

/// The integrations commit and engine version the whole corpus was taken at.
///
/// `None` when the captures disagree, which means the corpus was regenerated
/// piecemeal and no single baseline describes it.
fn provenance(fixtures: &[Captured]) -> Option<(String, String)> {
    let mut seen: Option<(String, String)> = None;
    for capture in fixtures {
        let here = (capture.integrations_sha.clone(), capture.engine.clone());
        match &seen {
            None => seen = Some(here),
            Some(first) if *first == here => {}
            Some(_) => return None,
        }
    }
    seen
}

/// Per-source scores this corpus is expected to reach.
#[derive(serde::Deserialize)]
struct Baseline {
    integrations_sha: String,
    elasticsearch_version: String,
    /// Scripts that bound a pattern and never once ran it.
    ///
    /// A pattern whose runner declines every event still MATCHES, so the static
    /// census counts it as covered and `painless_coverage.rs` reads 100% --
    /// both measure the CLAIM. Only this run sees whether it applied.
    ///
    /// Not a defect count: a script can legitimately never run because the
    /// corpus holds no event carrying its source field. Read alongside
    /// `scripts/pattern_reach.py`, never on its own.
    ///
    /// Lives here rather than in a constant because it moves on every
    /// improvement, and `scripts/raise_baseline.py` writes it. A number a
    /// human retypes is a number that grows a changelog beside it.
    never_ran: usize,
    sources: BTreeMap<String, Expected>,
}

#[derive(serde::Deserialize)]
struct Expected {
    events: usize,
    events_total: usize,
    fields_wrong: usize,
}

/// The baseline entry a measured score is written as.
fn entry(score: Score) -> String {
    format!(
        "{{ \"events\": {}, \"events_total\": {}, \"fields_wrong\": {} }}",
        score.events_matched, score.events, score.fields_wrong
    )
}

/// Set to acknowledge that this run scores a corpus the baseline cannot
/// ratchet against, and to let it pass anyway.
///
/// The legitimate case is a regeneration in progress: the corpus is being
/// recaptured at a new `integrations_sha` and the baseline has not caught up.
/// Whoever is doing that knows it; nobody else does.
const ALLOW_UNRATCHETED: &str = "DFE_COMPAT_ALLOW_UNRATCHETED";

/// Fail if a source scores below what it scored when the baseline was written.
///
/// Only asserted when the corpus on disk was captured at the same integrations
/// commit and engine version the baseline names. Anything else is a different
/// measurement -- ratcheting one against the other would fail for the wrong
/// reason -- so this REFUSES to run rather than scoring it.
///
/// Refusing loudly is the point. Each of these was a `println!` and a `return`,
/// which turned the whole parity gate into a printer while the run still
/// exited 0, among thousands of per-fixture lines. Regenerate the corpus at a
/// new sha, forget line 4 of the baseline, and parity is unenforced with no
/// signal for as long as the drift lasts.
///
/// The corpus being ABSENT is a different thing and still skips, up in
/// `score_the_corpus` -- it is gitignored, so a fresh clone has none.
fn check_baseline(
    measured: &BTreeMap<String, Score>,
    provenance: &Option<(String, String)>,
    never: usize,
) {
    const PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/compat-baseline.json"
    );
    let acknowledged = std::env::var_os(ALLOW_UNRATCHETED).is_some();
    let refuse = |reason: String| {
        assert!(
            acknowledged,
            "{reason}\n  The corpus is present, so this run COULD have ratcheted and did not. \
             Fix the mismatch, or set {ALLOW_UNRATCHETED}=1 to say you know -- \
             which leaves parity unenforced for this run."
        );
        println!("\nbaseline NOT asserted ({ALLOW_UNRATCHETED} is set): {reason}");
    };

    let Ok(text) = std::fs::read_to_string(PATH) else {
        refuse(format!("no readable baseline at {PATH}"));
        return;
    };
    let baseline: Baseline = serde_json::from_str(&text).expect("parse compat-baseline.json");

    let Some((sha, engine)) = provenance else {
        refuse("the corpus mixes provenances, so regenerate it whole".to_string());
        return;
    };
    if *sha != baseline.integrations_sha || *engine != baseline.elasticsearch_version {
        refuse(format!(
            "corpus is {}/{engine}, baseline is {}/{}",
            &sha[..12.min(sha.len())],
            &baseline.integrations_sha[..12.min(baseline.integrations_sha.len())],
            baseline.elasticsearch_version,
        ));
        return;
    }

    assert!(
        never <= baseline.never_ran,
        "{never} scripts bound a pattern and never ran it, up from {}. A pattern \
         claiming a script it cannot apply reads as covered everywhere except \
         here -- the painless coverage floor counts the CLAIM. Run with \
         DFE_PAINLESS_UNHANDLED=<path> to see which, and \
         `scripts/pattern_reach.py` to join them to call sites.",
        baseline.never_ran
    );

    let mut failures = Vec::new();
    let mut improved = Vec::new();
    let mut resized = Vec::new();
    for (source, expected) in &baseline.sources {
        let Some(score) = measured.get(source) else {
            failures.push(format!(
                "{source}: in the baseline and absent from the corpus"
            ));
            continue;
        };
        // A different set of events is a different measurement: neither the
        // event floor nor the field count carries across it, so the entry
        // constrains nothing until it is rewritten -- the same consequence as
        // a source nobody wrote down, and so the same failure.
        if score.events != expected.events_total {
            failures.push(format!(
                "{source}: {} events in the corpus, baseline was written against {} -- \
                 nothing constrains it until the entry is rewritten",
                score.events, expected.events_total
            ));
            resized.push(format!("RESIZE  \"{source}\": {},", entry(*score)));
            continue;
        }
        if score.events_matched < expected.events {
            failures.push(format!(
                "{source}: {}/{} events, baseline {}",
                score.events_matched, score.events, expected.events
            ));
        }
        if score.fields_wrong > expected.fields_wrong {
            failures.push(format!(
                "{source}: {} fields wrong, baseline {}",
                score.fields_wrong, expected.fields_wrong
            ));
        }
        if score.events_matched > expected.events || score.fields_wrong < expected.fields_wrong {
            improved.push(format!("  \"{source}\": {},", entry(*score)));
        }
    }

    // A source scored and never written down is measured by nobody: it may
    // rot to zero without failing anything. Capturing one is not finished
    // until its score is a floor, so this is a failure with the line to paste.
    let mut fresh = Vec::new();
    for (source, score) in measured {
        if !baseline.sources.contains_key(source) {
            failures.push(format!(
                "{source}: scored and not in the baseline -- add {}",
                entry(*score)
            ));
            fresh.push(format!("NEW     \"{source}\": {},", entry(*score)));
        }
    }

    if !improved.is_empty() {
        println!("\nbaseline can be raised -- paste into tests/compat-baseline.json:");
        for line in &improved {
            println!("{line}");
        }
    }

    // Marked rather than paste-ready: rewriting one of these accepts a score
    // nothing was holding, so it goes through `raise_baseline.py --resize`,
    // which the plain raise run will not do for you.
    if !resized.is_empty() {
        println!(
            "\nthese hold a different set of events than the baseline was written \
             against -- rewrite with scripts/raise_baseline.py --resize once the new \
             total is the intended one:"
        );
        for line in &resized {
            println!("{line}");
        }
    }

    // Marked the same way and for the same reason: a first score is whatever
    // the transform happens to do, so writing it down is a deliberate act.
    if !fresh.is_empty() {
        println!(
            "\nthese are scored and unwritten, so nothing holds them -- record \
             their first score with scripts/raise_baseline.py --new:"
        );
        for line in &fresh {
            println!("{line}");
        }
    }

    assert!(
        failures.is_empty(),
        "the corpus regressed against tests/compat-baseline.json:\n  {}",
        failures.join("\n  ")
    );
}

/// Every package and data stream `compat.py` captures must reach a transform.
#[test]
fn every_captured_source_is_a_registered_transform() {
    for capture in captured() {
        assert!(
            transform_for(&capture.source, &capture.data_stream).is_some(),
            "{}/{} is in the corpus with no transform",
            capture.source,
            capture.data_stream
        );
    }
}

/// The same check with no corpus on disk. The corpus is gitignored, so the
/// test above is vacuous on a fresh clone and a newly declared source would be
/// silently skipped rather than scored.
#[test]
fn every_declared_source_reaches_a_transform() {
    #[derive(serde::Deserialize)]
    struct Declaration {
        sources: std::collections::BTreeMap<String, Declared>,
    }

    #[derive(serde::Deserialize)]
    struct Declared {
        package: String,
        data_stream: String,
    }

    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../sources.yaml");
    let text = std::fs::read_to_string(PATH).expect("read sources.yaml");
    let declaration: Declaration = serde_yaml_ng::from_str(&text).expect("parse sources.yaml");

    for (name, declared) in &declaration.sources {
        assert!(
            transform_for(&declared.package, &declared.data_stream).is_some(),
            "{name} is declared as {}/{} and `transform_for` does not map it",
            declared.package,
            declared.data_stream
        );
    }
}

/// No arm of `transform_for` may match a data stream by package alone.
///
/// A wildcard answers for streams that do not exist yet, so a newly onboarded
/// one is scored against the wrong transform while the checks above stay green.
#[test]
fn no_package_answers_for_a_stream_it_was_not_declared_with() {
    #[derive(serde::Deserialize)]
    struct Declaration {
        sources: std::collections::BTreeMap<String, Declared>,
    }

    #[derive(serde::Deserialize)]
    struct Declared {
        package: String,
    }

    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../sources.yaml");
    let text = std::fs::read_to_string(PATH).expect("read sources.yaml");
    let declaration: Declaration = serde_yaml_ng::from_str(&text).expect("parse sources.yaml");

    let packages: std::collections::BTreeSet<&str> = declaration
        .sources
        .values()
        .map(|declared| declared.package.as_str())
        .collect();

    // Every offender is reported, because narrowing one arm at a time costs a
    // build per package.
    let wildcards: Vec<&str> = packages
        .into_iter()
        .filter(|package| transform_for(package, "a_data_stream_that_does_not_exist").is_some())
        .collect();

    assert!(
        wildcards.is_empty(),
        "these packages match ANY data stream -- narrow each arm to the streams \
         it is really for: {wildcards:?}"
    );
}
