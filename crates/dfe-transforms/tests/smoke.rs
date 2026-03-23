// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Smoke tests: instantiate every transform, call name(), run against empty/minimal events.
//! Catches init panics (OnceLock, regex compilation), structural issues, and basic sanity.

use dfe_runtime::event::Event;
use dfe_runtime::transform::Transform;
use serde_json::json;

/// Run a transform against a minimal event and verify it doesn't panic.
fn smoke(t: &dyn Transform) {
    // Verify name is non-empty
    let name = t.name();
    assert!(!name.is_empty(), "transform name must not be empty");

    // Empty event — should not panic
    let mut empty = Event::new(json!({}));
    let _ = t.transform(&mut empty);

    // Minimal message event — common pipeline entry point
    let mut msg = Event::new(json!({"message": "{}"}));
    let _ = t.transform(&mut msg);

    // Event with null fields — tests null safety
    let mut nulls = Event::new(json!({"message": null, "event": null}));
    let _ = t.transform(&mut nulls);
}

// --- CrowdStrike ---

#[test]
fn smoke_crowdstrike_default() {
    smoke(&dfe_transforms::filebeat::crowdstrike::default::Default);
}

#[test]
fn smoke_crowdstrike_auth_activity() {
    smoke(&dfe_transforms::filebeat::crowdstrike::auth_activity_audit::AuthActivityAudit);
}

#[test]
fn smoke_crowdstrike_detection_summary() {
    smoke(&dfe_transforms::filebeat::crowdstrike::detection_summary::DetectionSummary);
}

#[test]
fn smoke_crowdstrike_firewall_match() {
    smoke(&dfe_transforms::filebeat::crowdstrike::firewall_match::FirewallMatch);
}

#[test]
fn smoke_crowdstrike_incident_summary() {
    smoke(&dfe_transforms::filebeat::crowdstrike::incident_summary::IncidentSummary);
}

#[test]
fn smoke_crowdstrike_ipd_detection() {
    smoke(&dfe_transforms::filebeat::crowdstrike::ipd_detection_summary::IpdDetectionSummary);
}

#[test]
fn smoke_crowdstrike_mobile_detection() {
    smoke(&dfe_transforms::filebeat::crowdstrike::mobile_detection_summary::MobileDetectionSummary);
}

#[test]
fn smoke_crowdstrike_recon_notification() {
    smoke(
        &dfe_transforms::filebeat::crowdstrike::recon_notification_summary::ReconNotificationSummary,
    );
}

#[test]
fn smoke_crowdstrike_remote_response_end() {
    smoke(
        &dfe_transforms::filebeat::crowdstrike::remote_response_session_end::RemoteResponseSessionEnd,
    );
}

#[test]
fn smoke_crowdstrike_remote_response_start() {
    smoke(
        &dfe_transforms::filebeat::crowdstrike::remote_response_session_start::RemoteResponseSessionStart,
    );
}

#[test]
fn smoke_crowdstrike_user_activity() {
    smoke(&dfe_transforms::filebeat::crowdstrike::user_activity_audit::UserActivityAudit);
}

#[test]
fn smoke_crowdstrike_xdr_detection() {
    smoke(&dfe_transforms::filebeat::crowdstrike::xdr_detection_summary::XdrDetectionSummary);
}

#[test]
fn smoke_crowdstrike_cspm() {
    smoke(&dfe_transforms::filebeat::crowdstrike::cspm_events::CspmEvents);
}

#[test]
fn smoke_crowdstrike_scheduled_report() {
    smoke(
        &dfe_transforms::filebeat::crowdstrike::scheduled_report_notification_event::ScheduledReportNotificationEvent,
    );
}

#[test]
fn smoke_crowdstrike_identity_protection() {
    smoke(
        &dfe_transforms::filebeat::crowdstrike::identity_protection_incident::IdentityProtectionIncident,
    );
}

// --- Okta ---

#[test]
fn smoke_okta_default() {
    smoke(&dfe_transforms::filebeat::okta::default::Default);
}

// --- O365 ---

#[test]
fn smoke_o365_default() {
    smoke(&dfe_transforms::filebeat::o365::default::Default);
}

// --- Azure ---

#[test]
fn smoke_azure_activitylogs() {
    smoke(&dfe_transforms::filebeat::azure_activitylogs::default::Default);
}

#[test]
fn smoke_azure_auditlogs() {
    smoke(&dfe_transforms::filebeat::azure_auditlogs::default::Default);
}

#[test]
fn smoke_azure_signinlogs() {
    smoke(&dfe_transforms::filebeat::azure_signinlogs::default::Default);
}

#[test]
fn smoke_azure_platformlogs() {
    smoke(&dfe_transforms::filebeat::azure_platformlogs::default::Default);
}

// --- Fortinet ---

#[test]
fn smoke_fortinet_default() {
    smoke(&dfe_transforms::filebeat::fortinet::default::Default);
}

// --- Cisco ---

#[test]
fn smoke_cisco_ios_default() {
    smoke(&dfe_transforms::filebeat::cisco_ios::default::Default);
}

#[test]
fn smoke_cisco_meraki_default() {
    smoke(&dfe_transforms::filebeat::cisco_meraki::default::Default);
}

#[test]
fn smoke_cisco_nexus_default() {
    smoke(&dfe_transforms::filebeat::cisco_nexus::default::Default);
}

// --- Panw ---

#[test]
fn smoke_panw_traffic() {
    smoke(&dfe_transforms::filebeat::panw::traffic::Traffic);
}

#[test]
fn smoke_panw_authentication() {
    smoke(&dfe_transforms::filebeat::panw::authentication::Authentication);
}

#[test]
fn smoke_panw_decryption() {
    smoke(&dfe_transforms::filebeat::panw::decryption::Decryption);
}

#[test]
fn smoke_panw_globalprotect() {
    smoke(&dfe_transforms::filebeat::panw::globalprotect::Globalprotect);
}

#[test]
fn smoke_panw_hipmatch() {
    smoke(&dfe_transforms::filebeat::panw::hipmatch::Hipmatch);
}

#[test]
fn smoke_panw_userid() {
    smoke(&dfe_transforms::filebeat::panw::userid::Userid);
}
