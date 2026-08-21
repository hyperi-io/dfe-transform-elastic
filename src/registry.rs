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
