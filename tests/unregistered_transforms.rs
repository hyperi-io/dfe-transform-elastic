// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Transforms the registry cannot reach, swept by hand.
//!
//! `tests/unicode.rs` runs three sweeps over `registry::sources()`, which is
//! every transform a deployment can name -- 1,092 of them. The `crowdstrike`
//! sub-pipelines are not among them: the registry carries
//! `filebeat.crowdstrike.default` and five separate packages, and the fourteen
//! below are reached only by that default routing on the event's own contents.
//!
//! So nothing else instantiates them, and an init panic -- a bad regex in a
//! `OnceLock`, a `cached_grok!` that does not compile -- would first appear in
//! production, where it takes the pod and stalls the partition.
//!
//! This lives in the service crate rather than beside the transforms because
//! the registry does, and the list has to prove its own premise: each name
//! below is asserted ABSENT from the registry, so one that later gains an
//! entry fails here and can be deleted from this file.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_runtime::transform::Transform;
use dfe_transform_elastic::registry;
use serde_json::json;

/// The sub-pipelines, with the registry id they would carry if they were
/// reachable by name.
fn unreachable() -> Vec<(&'static str, &'static dyn Transform)> {
    use dfe_transforms::filebeat::crowdstrike as cs;
    vec![
        (
            "filebeat.crowdstrike.auth_activity_audit",
            &cs::auth_activity_audit::AuthActivityAudit,
        ),
        (
            "filebeat.crowdstrike.detection_summary",
            &cs::detection_summary::DetectionSummary,
        ),
        (
            "filebeat.crowdstrike.firewall_match",
            &cs::firewall_match::FirewallMatch,
        ),
        (
            "filebeat.crowdstrike.incident_summary",
            &cs::incident_summary::IncidentSummary,
        ),
        (
            "filebeat.crowdstrike.ipd_detection_summary",
            &cs::ipd_detection_summary::IpdDetectionSummary,
        ),
        (
            "filebeat.crowdstrike.mobile_detection_summary",
            &cs::mobile_detection_summary::MobileDetectionSummary,
        ),
        (
            "filebeat.crowdstrike.recon_notification_summary",
            &cs::recon_notification_summary::ReconNotificationSummary,
        ),
        (
            "filebeat.crowdstrike.remote_response_session_end",
            &cs::remote_response_session_end::RemoteResponseSessionEnd,
        ),
        (
            "filebeat.crowdstrike.remote_response_session_start",
            &cs::remote_response_session_start::RemoteResponseSessionStart,
        ),
        (
            "filebeat.crowdstrike.user_activity_audit",
            &cs::user_activity_audit::UserActivityAudit,
        ),
        (
            "filebeat.crowdstrike.xdr_detection_summary",
            &cs::xdr_detection_summary::XdrDetectionSummary,
        ),
        (
            "filebeat.crowdstrike.cspm_events",
            &cs::cspm_events::CspmEvents,
        ),
        (
            "filebeat.crowdstrike.scheduled_report_notification_event",
            &cs::scheduled_report_notification_event::ScheduledReportNotificationEvent,
        ),
        (
            "filebeat.crowdstrike.identity_protection_incident",
            &cs::identity_protection_incident::IdentityProtectionIncident,
        ),
    ]
}

/// A transform that gains a registry entry is swept by `unicode.rs` and does
/// not belong here any more.
#[test]
fn every_listed_transform_is_still_unreachable_by_name() {
    let listed = unreachable();
    assert_eq!(listed.len(), 14, "the list changed size without a reason");

    for (name, _) in &listed {
        assert!(
            registry::lookup(name).is_none(),
            "{name} is registered now, so unicode.rs sweeps it -- drop it here"
        );
    }
}

/// Instantiate each one and run it against documents that carry nothing,
/// which is what catches an init panic.
#[test]
fn no_unreachable_transform_panics_on_a_degenerate_document() {
    let documents = [
        ("empty", json!({})),
        ("minimal message", json!({ "message": "{}" })),
        ("null fields", json!({ "message": null, "event": null })),
    ];

    for (name, transform) in unreachable() {
        assert!(!transform.name().is_empty(), "{name}: empty transform name");

        for (label, document) in &documents {
            let mut event = dfe_runtime::event::Event::new(document.clone());
            // Caught rather than left to unwind, so the failure names the
            // transform and the pattern instead of a bare backtrace.
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                transform.transform(&mut event)
            }));
            if let Err(panic) = outcome {
                let message = panic
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic.downcast_ref::<&str>().copied())
                    .unwrap_or("unknown panic");
                panic!("{name} panicked on the {label} document: {message}");
            }
            // An error is expected: these pipelines need their own data. Only
            // a panic is a defect.
        }
    }
}
