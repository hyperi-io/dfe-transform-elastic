// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Smoke tests: instantiate every transform, call name(), run against empty/minimal events.
//! Catches init panics (OnceLock, regex compilation), structural issues, and basic sanity.

use dfe_runtime::event::Event;
use dfe_runtime::transform::Transform;
use serde_json::json;

/// Run a transform against minimal events to catch init panics.
///
/// Primary goal: catch panics from OnceLock/regex compilation, not validate
/// transform logic. Transform errors (FieldNotFound etc.) are OK — they mean
/// the transform ran but needs real input data. Panics are NOT OK.
fn smoke(t: &dyn Transform) {
    let name = t.name();
    assert!(!name.is_empty(), "transform name must not be empty");

    // Each event type: catch panics explicitly, allow errors
    for (label, event_json) in [
        ("empty", json!({})),
        ("minimal_message", json!({"message": "{}"})),
        ("null_fields", json!({"message": null, "event": null})),
    ] {
        let mut event = Event::new(event_json);
        let panic_result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| t.transform(&mut event)));

        match panic_result {
            Err(panic) => {
                let msg = panic
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic.downcast_ref::<&str>().copied())
                    .unwrap_or("unknown panic");
                panic!("[{name}] panicked on {label} event: {msg}");
            }
            Ok(Err(_transform_err)) => {
                // Transform errors on empty/null input are expected —
                // pipelines need their data. Not a structural problem.
            }
            Ok(Ok(_)) => {
                // Transform succeeded — good
            }
        }
    }
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
