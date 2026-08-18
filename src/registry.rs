// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Source name to transform lookup.
//!
//! The name is the module path with dots, e.g. `filebeat.okta.default`.
//! Resolution happens once at startup, not per event.

use dfe_runtime::Transform;
use dfe_transforms::filebeat;

/// Every transform this service can run, keyed by source name.
///
/// Sorted so the `sources()` listing is stable.
static TRANSFORMS: &[(&str, &(dyn Transform + Sync))] = &[
    (
        "filebeat.azure_activitylogs.default",
        &filebeat::azure_activitylogs::default::Default,
    ),
    (
        "filebeat.azure_auditlogs.default",
        &filebeat::azure_auditlogs::default::Default,
    ),
    (
        "filebeat.azure_platformlogs.default",
        &filebeat::azure_platformlogs::default::Default,
    ),
    (
        "filebeat.azure_signinlogs.default",
        &filebeat::azure_signinlogs::default::Default,
    ),
    (
        "filebeat.cisco_ios.default",
        &filebeat::cisco_ios::default::Default,
    ),
    (
        "filebeat.cisco_meraki.default",
        &filebeat::cisco_meraki::default::Default,
    ),
    (
        "filebeat.cisco_nexus.default",
        &filebeat::cisco_nexus::default::Default,
    ),
    (
        "filebeat.crowdstrike.default",
        &filebeat::crowdstrike::default::Default,
    ),
    (
        "filebeat.fortinet.default",
        &filebeat::fortinet::default::Default,
    ),
    ("filebeat.o365.default", &filebeat::o365::default::Default),
    ("filebeat.okta.default", &filebeat::okta::default::Default),
    // panw has no `default` -- the upstream pipeline dispatches per log type.
    (
        "filebeat.panw.authentication",
        &filebeat::panw::authentication::Authentication,
    ),
    (
        "filebeat.panw.correlated_event",
        &filebeat::panw::correlated_event::CorrelatedEvent,
    ),
    (
        "filebeat.panw.decryption",
        &filebeat::panw::decryption::Decryption,
    ),
    (
        "filebeat.panw.globalprotect",
        &filebeat::panw::globalprotect::Globalprotect,
    ),
    ("filebeat.panw.gtp", &filebeat::panw::gtp::Gtp),
    (
        "filebeat.panw.hipmatch",
        &filebeat::panw::hipmatch::Hipmatch,
    ),
    ("filebeat.panw.ip_tag", &filebeat::panw::ip_tag::IpTag),
    ("filebeat.panw.sctp", &filebeat::panw::sctp::Sctp),
    ("filebeat.panw.system", &filebeat::panw::system::System),
    ("filebeat.panw.traffic", &filebeat::panw::traffic::Traffic),
    (
        "filebeat.panw.tunnel_inspection",
        &filebeat::panw::tunnel_inspection::TunnelInspection,
    ),
    ("filebeat.panw.userid", &filebeat::panw::userid::Userid),
];

/// Resolve a source name to its transform.
pub fn lookup(name: &str) -> Option<&'static (dyn Transform + Sync)> {
    TRANSFORMS
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, t)| *t)
}

/// Every source name this service accepts.
pub fn sources() -> impl Iterator<Item = &'static str> {
    TRANSFORMS.iter().map(|(key, _)| *key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_a_known_source() {
        assert!(lookup("filebeat.okta.default").is_some());
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
