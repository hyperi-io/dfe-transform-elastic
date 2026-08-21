// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Filebeat integration transforms.

pub mod azure_activitylogs;
pub mod azure_auditlogs;
pub mod azure_platformlogs;
pub mod azure_signinlogs;
pub mod checkpoint;
pub mod cisco_asa;
pub mod cisco_ftd;
pub mod cisco_ios;
pub mod cisco_meraki;
pub mod cisco_nexus;
pub mod cisco_umbrella;
pub mod crowdstrike;
pub mod entityanalytics_entra_id;
pub mod fortinet;
pub mod microsoft_defender_endpoint_log;
pub mod microsoft_defender_endpoint_machine;
pub mod microsoft_defender_endpoint_machine_action;
pub mod microsoft_defender_endpoint_vulnerability;
pub mod microsoft_dnsserver_analytical;
pub mod microsoft_dnsserver_audit;
pub mod mimecast_archive_search_logs;
pub mod mimecast_audit_events;
pub mod mimecast_cloud_integrated_logs;
pub mod mimecast_dlp_logs;
pub mod mimecast_message_release_logs;
pub mod mimecast_siem_logs;
pub mod mimecast_threat_intel_malware_customer;
pub mod mimecast_threat_intel_malware_grid;
pub mod mimecast_ttp_ap_logs;
pub mod mimecast_ttp_ip_logs;
pub mod mimecast_ttp_url_logs;
pub mod o365;
pub mod okta;
pub mod panw;
pub mod proofpoint_on_demand_audit;
pub mod proofpoint_on_demand_mail;
pub mod proofpoint_on_demand_message;
