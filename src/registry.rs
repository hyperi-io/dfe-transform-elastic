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
/// Sorted so the `sources()` listing is stable. The entries are hand-wired
/// because each names a Rust type, and `sources.yaml` is asserted against them
/// so a source declared there and never wired here fails the build.
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
    // `panw.default` routes on log type and holds the CSV parse and converts
    // the per-type entries depend on. The per-type entries suit a feed already
    // narrowed to one log type.
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
        "filebeat.panw.default",
        &filebeat::panw::default::Default,
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

    /// Which source is API-origin and what framing a syslog one wants are
    /// declared in `sources.yaml` and checked below; all this adds is that an
    /// unknown name classifies as nothing at all.
    #[test]
    fn every_source_has_exactly_one_origin() {
        let api = TRANSFORMS
            .iter()
            .filter(|(_, _, o)| *o == Origin::Api)
            .count();
        assert_eq!(syslog_sources().count() + api, sources().count());
        assert_eq!(origin("filebeat.nosuchthing"), None);
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
            origin: String,
            framing: Option<String>,
            transforms: Vec<String>,
        }

        const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/sources.yaml");
        let text = std::fs::read_to_string(PATH).expect("read sources.yaml");
        let declaration: Declaration = serde_yaml_ng::from_str(&text).expect("parse sources.yaml");

        let mut expected: Vec<(String, Origin)> = Vec::new();
        for (source, declared) in &declaration.sources {
            let origin = match (declared.origin.as_str(), declared.framing.as_deref()) {
                ("api", None) => Origin::Api,
                ("syslog", Some("line")) => Origin::Syslog(Framing::Line),
                ("syslog", Some("body")) => Origin::Syslog(Framing::Body),
                other => panic!("{source} declares {other:?}, which is not an origin"),
            };
            for transform in &declared.transforms {
                expected.push((format!("filebeat.{source}.{transform}"), origin));
            }
        }
        expected.sort_by(|a, b| a.0.cmp(&b.0));

        let wired: Vec<(String, Origin)> = TRANSFORMS
            .iter()
            .map(|(name, _, origin)| ((*name).to_owned(), *origin))
            .collect();

        assert_eq!(
            wired, expected,
            "src/registry.rs and sources.yaml disagree on the sources or their origins"
        );
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
