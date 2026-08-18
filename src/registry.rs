// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Source name to transform lookup.
//!
//! The name is the module path with dots, e.g. `filebeat.okta.default`.
//! Resolution happens once at startup, not per event.

use dfe_runtime::Transform;
use dfe_transforms::filebeat;

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

/// Where a source's events can come from.
///
/// This decides whether the syslog envelope may be used: a device that emits
/// over syslog can be fed from dfe-receiver, and an API-only source cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// Pulled from a vendor API. Only the Beats envelope applies.
    Api,
    /// A device can emit this over syslog, so either envelope applies.
    Syslog(Framing),
}

impl Origin {
    /// Whether the syslog envelope applies to this source.
    #[must_use]
    pub const fn is_syslog(self) -> bool {
        matches!(self, Self::Syslog(_))
    }

    /// What this pipeline expects in `message`, if it takes syslog at all.
    #[must_use]
    pub const fn framing(self) -> Option<Framing> {
        match self {
            Self::Syslog(framing) => Some(framing),
            Self::Api => None,
        }
    }
}

/// Every transform this service can run, keyed by source name.
///
/// Sorted so the `sources()` listing is stable.
static TRANSFORMS: &[(&str, &(dyn Transform + Sync), Origin)] = &[
    (
        "filebeat.azure_activitylogs.default",
        &filebeat::azure_activitylogs::default::Default,
        Origin::Api,
    ),
    (
        "filebeat.azure_auditlogs.default",
        &filebeat::azure_auditlogs::default::Default,
        Origin::Api,
    ),
    (
        "filebeat.azure_platformlogs.default",
        &filebeat::azure_platformlogs::default::Default,
        Origin::Api,
    ),
    (
        "filebeat.azure_signinlogs.default",
        &filebeat::azure_signinlogs::default::Default,
        Origin::Api,
    ),
    (
        "filebeat.cisco_ios.default",
        &filebeat::cisco_ios::default::Default,
        Origin::Syslog(Framing::Line),
    ),
    (
        "filebeat.cisco_meraki.default",
        &filebeat::cisco_meraki::default::Default,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.cisco_nexus.default",
        &filebeat::cisco_nexus::default::Default,
        Origin::Syslog(Framing::Line),
    ),
    (
        "filebeat.crowdstrike.default",
        &filebeat::crowdstrike::default::Default,
        Origin::Api,
    ),
    (
        "filebeat.fortinet.default",
        &filebeat::fortinet::default::Default,
        Origin::Syslog(Framing::Line),
    ),
    (
        "filebeat.o365.default",
        &filebeat::o365::default::Default,
        Origin::Api,
    ),
    (
        "filebeat.okta.default",
        &filebeat::okta::default::Default,
        Origin::Api,
    ),
    // panw has no `default` -- the upstream pipeline dispatches per log type.
    (
        "filebeat.panw.authentication",
        &filebeat::panw::authentication::Authentication,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.correlated_event",
        &filebeat::panw::correlated_event::CorrelatedEvent,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.decryption",
        &filebeat::panw::decryption::Decryption,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.globalprotect",
        &filebeat::panw::globalprotect::Globalprotect,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.gtp",
        &filebeat::panw::gtp::Gtp,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.hipmatch",
        &filebeat::panw::hipmatch::Hipmatch,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.ip_tag",
        &filebeat::panw::ip_tag::IpTag,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.sctp",
        &filebeat::panw::sctp::Sctp,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.system",
        &filebeat::panw::system::System,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.traffic",
        &filebeat::panw::traffic::Traffic,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.tunnel_inspection",
        &filebeat::panw::tunnel_inspection::TunnelInspection,
        Origin::Syslog(Framing::Body),
    ),
    (
        "filebeat.panw.userid",
        &filebeat::panw::userid::Userid,
        Origin::Syslog(Framing::Body),
    ),
];

/// Resolve a source name to its transform.
pub fn lookup(name: &str) -> Option<&'static (dyn Transform + Sync)> {
    TRANSFORMS
        .iter()
        .find(|(key, _, _)| *key == name)
        .map(|(_, t, _)| *t)
}

/// Where a source's events can come from.
pub fn origin(name: &str) -> Option<Origin> {
    TRANSFORMS
        .iter()
        .find(|(key, _, _)| *key == name)
        .map(|(_, _, origin)| *origin)
}

/// Every source name this service accepts.
pub fn sources() -> impl Iterator<Item = &'static str> {
    TRANSFORMS.iter().map(|(key, _, _)| *key)
}

/// Every source a device can emit over syslog.
pub fn syslog_sources() -> impl Iterator<Item = &'static str> {
    TRANSFORMS
        .iter()
        .filter(|(_, _, origin)| origin.is_syslog())
        .map(|(key, _, _)| *key)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn resolves_a_known_source() {
        assert!(lookup("filebeat.okta.default").is_some());
    }

    /// The API sources are the ones pulled over HTTP; everything else is a
    /// device that can be pointed at a syslog collector.
    #[test]
    fn origins_are_classified() {
        assert_eq!(origin("filebeat.okta.default"), Some(Origin::Api));
        assert_eq!(origin("filebeat.crowdstrike.default"), Some(Origin::Api));
        assert_eq!(
            origin("filebeat.azure_signinlogs.default"),
            Some(Origin::Api)
        );
        assert_eq!(origin("filebeat.nosuchthing"), None);
        assert!(origin("filebeat.cisco_ios.default").unwrap().is_syslog());
        assert!(origin("filebeat.panw.traffic").unwrap().is_syslog());
    }

    /// The framing split is the one that fails silently when wrong: a header
    /// prefixed onto panw's CSV corrupts its first field, and a body handed to
    /// fortinet fails its `<PRI>` grok.
    #[test]
    fn framing_matches_what_each_pipeline_groks() {
        for line_source in [
            "filebeat.cisco_ios.default",
            "filebeat.cisco_nexus.default",
            "filebeat.fortinet.default",
        ] {
            assert_eq!(
                origin(line_source).and_then(Origin::framing),
                Some(Framing::Line),
                "{line_source} groks the syslog header out of `message`"
            );
        }

        for body_source in [
            "filebeat.cisco_meraki.default",
            "filebeat.panw.traffic",
            "filebeat.panw.userid",
        ] {
            assert_eq!(
                origin(body_source).and_then(Origin::framing),
                Some(Framing::Body),
                "{body_source} reads `message` as a body and a header would corrupt it"
            );
        }
    }

    #[test]
    fn every_source_has_exactly_one_origin() {
        let total = sources().count();
        let syslog = syslog_sources().count();
        let api = TRANSFORMS
            .iter()
            .filter(|(_, _, o)| *o == Origin::Api)
            .count();
        assert_eq!(syslog + api, total);
        assert_eq!(syslog, 16);
        assert_eq!(api, 7);
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
