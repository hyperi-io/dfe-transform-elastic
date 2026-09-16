// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Service configuration.
//!
//! Read ONCE at startup and never re-read: the loaded [`Config`] is handed to
//! the batch loop by reference, and scalo's `config-reload` feature is not
//! enabled. Every value here needs a pod restart to change.
//!
//! Two ways in, and they are not equivalent. With no `--config` the whole scalo
//! cascade applies. With `--config` -- which is what the container passes --
//! the named file IS the configuration: scalo has no way to merge an
//! arbitrarily-named file into the cascade as a layer
//! (`ConfigOptions::config_paths` searches for `defaults`/`settings` by name,
//! and `merge_cli` sits above the environment), so the file is read directly.
//! The cascade is still installed either way, because the rest of scalo reads
//! it. Kafka credentials are unaffected: `KafkaConfig::from_env` reads
//! `KAFKA_*` separately.
//!
//! **Two env spellings, and only one of them reaches a `--config` file.** The
//! FLAT, single-underscore form -- `DFE_TRANSFORM_ELASTIC_SOURCE_TOPICS` -- is
//! applied by `loader`'s [`ApplyFlatEnv`](scalo::config::flat_env::ApplyFlatEnv)
//! impls after the configuration is loaded, on both branches. scalo's own
//! double-underscore form is resolved from the cascade, which a named file is
//! not a layer of, so it reaches that deployment never.
//!
//! The consequence is invisible and so is stated outright: a section scalo
//! resolves from the cascade for itself cannot be set from a `--config` file at
//! all. That file yields only what this [`Config`] declares, and scalo sees
//! only what the service then hands it -- which is why `geoip` works from a
//! file. Every other scalo section is listed in [`CASCADE_ONLY_SECTIONS`] and
//! WARNED about here rather than refused: the infra chart renders
//! `.Values.config` verbatim, so a section we do not control can appear in that
//! file, and stopping the pod over it polices something that was never ours.
//!
//! The module is split three ways: the shape lives here, reading a file or the
//! cascade lives in `loader`, and refusing a configuration that cannot work
//! lives in `validate`.

mod loader;
mod validate;

pub use loader::{CASCADE_ONLY_SECTIONS, ENV_PREFIX};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Which transport a side of the service uses.
///
/// `bus` is Kafka between the stages; `direct` is a scalo Push listener inbound
/// and a gRPC client outbound, needing no broker at all. The record and the
/// transform are identical either way -- only who hands the record over changes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    /// Kafka topics.
    #[default]
    Bus,
    /// A scalo Push listener inbound, a gRPC client outbound.
    Direct,
}

impl Transport {
    /// Whether this side is on the direct transport.
    #[must_use]
    pub const fn is_direct(self) -> bool {
        matches!(self, Self::Direct)
    }
}

/// Top-level service configuration.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    /// Inbound side.
    ///
    /// Defaulted so an instance nothing has configured yet still PARSES and
    /// reaches the idle gate. Without it the process dies on a missing
    /// `source` field before `ServiceRuntime::build`, so no `/livez` or
    /// `/readyz` is serving when it exits and the pod crash-loops.
    #[serde(default)]
    pub source: SourceConfig,

    /// Outbound side.
    #[serde(default)]
    pub sink: SinkConfig,

    /// Which MMDB databases the geoip processors read, and how to obtain them.
    ///
    /// scalo's type verbatim: provisioning is shared across the fleet, and the
    /// lookup engine and its cache stay in `dfe-runtime`. Read once at startup,
    /// so a change needs a restart.
    #[serde(default)]
    pub geoip: scalo::geoip_download::GeoIpConfig,
}

/// Inbound configuration.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SourceConfig {
    /// Beats or Agent source whose transform to apply, e.g. `filebeat.okta`.
    pub name: String,

    /// Which transport the inbound side uses: `bus` consumes `topics`, `direct`
    /// accepts pushes on `listen`.
    #[serde(default)]
    pub transport: Transport,

    /// Address the Push listener binds on the direct transport, ignored on the
    /// bus. The listener speaks plaintext gRPC with no authentication of its
    /// own, so it belongs behind the mesh route dfe-infra provisions and never
    /// on an interface reachable from outside the cluster.
    #[serde(default = "default_listen")]
    pub listen: String,

    /// Which producer wrapped the payload. The transform is the same for all
    /// of them; only the unwrapping differs.
    ///
    /// Defaults to `auto`, which reads it off each event. Naming a family
    /// instead pins it, and validation then rejects one the source cannot
    /// actually arrive in.
    #[serde(default)]
    pub envelope: crate::envelope::EnvelopeSetting,

    /// Topics to consume.
    pub topics: Vec<String>,

    /// Events pulled per batch.
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Ceiling on the BYTES one fetch may bring back, the inbound mirror of
    /// `sink.max_message_bytes`.
    ///
    /// `batch_size` bounds the event count and nothing bounds their size, so a
    /// producer sending large records decides this service's memory. Applied as
    /// librdkafka's `fetch.max.bytes`, which enforces it at the broker rather
    /// than after the bytes have arrived; the pod's memory limit has to cover
    /// this plus the parsed document trees, which are several times larger.
    #[serde(default = "default_max_batch_bytes")]
    pub max_batch_bytes: usize,

    /// Kafka consumer group.
    pub group_id: String,

    /// Broker list.
    pub brokers: Vec<String>,
}

/// Outbound configuration.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SinkConfig {
    /// Topic to produce to.
    pub topic: String,

    /// Which transport the outbound side uses: `bus` produces to `topic`,
    /// `direct` pushes to `endpoint`.
    #[serde(default)]
    pub transport: Transport,

    /// Downstream Push listener on the direct transport, ignored on the bus.
    /// Reached in the clear, so it too stays inside the mesh.
    #[serde(default = "default_endpoint")]
    pub endpoint: String,

    /// Broker list. Defaults to the source brokers when unset.
    #[serde(default)]
    pub brokers: Option<Vec<String>>,

    /// Ceiling on one produced Kafka record. A batch is split into as many
    /// records as it takes to stay under it.
    ///
    /// Must sit below the LOWER of the broker's `message.max.bytes` and the
    /// producer's -- raising the broker limit alone does nothing.
    #[serde(default = "default_max_message_bytes")]
    pub max_message_bytes: usize,
}

/// An unconfigured inbound side: no source, no topics, and the shipped numeric
/// defaults. [`Config::work_state`] reads this as no work and idles.
impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            transport: Transport::default(),
            listen: default_listen(),
            envelope: crate::envelope::EnvelopeSetting::default(),
            topics: Vec::new(),
            batch_size: default_batch_size(),
            max_batch_bytes: default_max_batch_bytes(),
            group_id: String::new(),
            brokers: Vec::new(),
        }
    }
}

/// An unconfigured outbound side, with the shipped record budget.
impl Default for SinkConfig {
    fn default() -> Self {
        Self {
            topic: String::new(),
            transport: Transport::default(),
            endpoint: default_endpoint(),
            brokers: None,
            max_message_bytes: default_max_message_bytes(),
        }
    }
}

const fn default_batch_size() -> usize {
    20_000
}

/// Default bind address for the Push listener, matching the port dfe-infra's
/// chart already advertises for this app.
fn default_listen() -> String {
    "0.0.0.0:6000".to_string()
}

/// Default downstream Push listener, which is dfe-loader on the direct path.
fn default_endpoint() -> String {
    "http://dfe-loader:6000".to_string()
}

/// Default ceiling on one inbound fetch.
///
/// 16 MiB of raw payload parses into several times that as `IndexMap` trees, so
/// this is what the pod's memory request is sized against. Below
/// `MIN_BATCH_BYTES` the per-partition quarter of it falls under librdkafka's
/// own `message.max.bytes` and a full-sized record can never be fetched.
#[must_use]
pub const fn default_max_batch_bytes() -> usize {
    16 * 1024 * 1024
}

/// Default ceiling on one produced Kafka record.
///
/// librdkafka's producer `message.max.bytes` defaults to 1,000,000 and scalo
/// sets no override, so anything above that is rejected before it leaves the
/// process. 900 KB leaves headroom for the key, headers and record framing,
/// which count against the same limit.
#[must_use]
pub const fn default_max_message_bytes() -> usize {
    900_000
}

impl Config {
    /// Brokers the sink should use, falling back to the source's.
    pub fn sink_brokers(&self) -> &[String] {
        self.sink.brokers.as_deref().unwrap_or(&self.source.brokers)
    }

    /// Does this configuration give the transform work?
    ///
    /// A source name and somewhere for records to arrive are what make this
    /// instance a transform for something. With neither there is nothing to
    /// consume, so the service starts, stays Ready, holds no consumer group,
    /// and picks up the first configuration that names work -- rather than
    /// crash-looping before any probe is serving.
    ///
    /// On the direct transport the LISTENER is the work, so empty topics do not
    /// idle. An unnamed source idles on both, because `validate` resolves the
    /// name through the registry and an empty one raises there.
    #[must_use]
    pub fn work_state(&self) -> scalo::lifecycle::WorkState {
        let no_intake = !self.source.transport.is_direct() && self.source.topics.is_empty();
        scalo::lifecycle::WorkState::idle_if(
            self.source.name.trim().is_empty() || no_intake,
            "no source name, or no topics on the bus transport",
        )
    }
}

/// A complete configuration, which the tests in this module and its children
/// mutate one field at a time.
#[cfg(test)]
fn valid() -> Config {
    Config {
        source: SourceConfig {
            name: "filebeat.okta.default".into(),
            transport: Transport::Bus,
            listen: default_listen(),
            envelope: crate::envelope::EnvelopeSetting::Beats,
            topics: vec!["in".into()],
            batch_size: 100,
            max_batch_bytes: default_max_batch_bytes(),
            group_id: "g".into(),
            brokers: vec!["localhost:9092".into()],
        },
        sink: SinkConfig {
            topic: "out".into(),
            transport: Transport::Bus,
            endpoint: default_endpoint(),
            brokers: None,
            max_message_bytes: default_max_message_bytes(),
        },
        geoip: scalo::geoip_download::GeoIpConfig::default(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn sink_falls_back_to_source_brokers() {
        assert_eq!(valid().sink_brokers(), ["localhost:9092".to_string()]);
    }

    #[test]
    fn a_complete_config_has_work() {
        assert!(!valid().work_state().is_idle());
        assert_eq!(valid().work_state().reason(), None);
    }

    /// The whole point of the defaults: an instance nothing has configured must
    /// PARSE. Without that it dies on a missing field before
    /// `ServiceRuntime::build`, so no probe is serving when it exits.
    #[test]
    fn an_empty_document_parses_and_idles() {
        let parsed: Config = serde_yaml_ng::from_str("{}").expect("an empty config parses");
        assert!(parsed.validate().is_ok());
        assert!(parsed.work_state().is_idle());
        assert_eq!(parsed.source.batch_size, 20_000);
        assert_eq!(parsed.sink.max_message_bytes, 900_000);
    }

    /// On the direct transport the listener is the work, so empty topics must
    /// not idle -- an idle direct pod would stay Ready and accept nothing.
    #[test]
    fn a_direct_source_with_no_topics_has_work() {
        let mut c = valid();
        c.source.transport = Transport::Direct;
        c.source.topics.clear();
        assert!(!c.work_state().is_idle());
    }

    /// The name condition holds on BOTH transports, because `validate` resolves
    /// it through the registry and an empty one raises there.
    #[test]
    fn an_unnamed_direct_source_still_idles() {
        let mut c = valid();
        c.source.transport = Transport::Direct;
        c.source.name.clear();
        assert!(c.work_state().is_idle());
    }

    /// Every config written before the transport existed still parses, and
    /// parses onto the bus.
    #[test]
    fn a_config_without_a_transport_is_on_the_bus() {
        let parsed: Config = serde_yaml_ng::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
        )
        .expect("config parses without a transport key");
        assert_eq!(parsed.source.transport, Transport::Bus);
        assert_eq!(parsed.sink.transport, Transport::Bus);
        assert_eq!(parsed.source.listen, "0.0.0.0:6000");
        assert_eq!(parsed.sink.endpoint, "http://dfe-loader:6000");
    }

    /// A direct deployment names no brokers, group or topics, and is valid.
    #[test]
    fn the_direct_transport_round_trips_through_yaml() {
        let parsed: Config = serde_yaml_ng::from_str(
            "source:\n  name: filebeat.okta.default\n  transport: direct\n  \
             listen: 0.0.0.0:6000\n  topics: []\n  group_id: ''\n  brokers: []\n\
             sink:\n  topic: ''\n  transport: direct\n  endpoint: http://loader:6000\n",
        )
        .expect("config parses with the direct transport");
        assert!(parsed.source.transport.is_direct());
        assert!(parsed.sink.transport.is_direct());
        assert!(!parsed.work_state().is_idle());
        assert!(parsed.validate().is_ok());
    }

    #[test]
    fn message_budget_defaults_under_the_librdkafka_ceiling() {
        let parsed: Config = serde_yaml_ng::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
        )
        .expect("config parses without a sink budget");
        assert_eq!(parsed.sink.max_message_bytes, 900_000);
        assert!(parsed.sink.max_message_bytes < 1_000_000);
    }

    /// A config written before the geoip section existed still loads, and
    /// loads with provisioning ON -- the point of the section is that an
    /// operator gets databases without configuring anything.
    #[test]
    fn geoip_provisions_unless_it_is_turned_off() {
        let base = "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
                    group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n";

        let parsed: Config =
            serde_yaml_ng::from_str(base).expect("config parses without a geoip section");
        assert!(parsed.geoip.enabled);
        assert!(parsed.geoip.auto_download.enabled);
        assert_eq!(
            parsed.geoip.provider,
            scalo::geoip_download::GeoIpProvider::DbIpLite
        );

        let off: Config = serde_yaml_ng::from_str(&format!("{base}geoip:\n  enabled: false\n"))
            .expect("config parses with geoip disabled");
        assert!(!off.geoip.enabled);
    }

    /// Detection, not a guess: an unset envelope means read it off the batch,
    /// so a deployment that changes producer needs no config change.
    #[test]
    fn defaults_to_detecting_the_envelope() {
        let parsed: Config = serde_yaml_ng::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
        )
        .expect("config parses without an envelope key");
        assert_eq!(
            parsed.source.envelope,
            crate::envelope::EnvelopeSetting::Auto
        );
        assert_eq!(parsed.source.envelope.pinned(), None);
    }

    /// A config written before detection existed still loads and still pins.
    #[test]
    fn the_old_envelope_names_still_load() {
        for (written, expected) in [
            ("beats", crate::envelope::EnvelopeSetting::Beats),
            ("elastic", crate::envelope::EnvelopeSetting::Beats),
            ("syslog", crate::envelope::EnvelopeSetting::Receiver),
            ("receiver", crate::envelope::EnvelopeSetting::Receiver),
            ("fetcher", crate::envelope::EnvelopeSetting::Fetcher),
            ("auto", crate::envelope::EnvelopeSetting::Auto),
        ] {
            let parsed: Config = serde_yaml_ng::from_str(&format!(
                "source:\n  name: filebeat.fortinet.default\n  envelope: {written}\n  \
                 topics: [in]\n  group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n"
            ))
            .expect("config parses");
            assert_eq!(parsed.source.envelope, expected, "envelope: {written}");
        }
    }

    #[test]
    fn envelope_round_trips_through_yaml() {
        let parsed: Config = serde_yaml_ng::from_str(
            "source:\n  name: filebeat.fortinet.default\n  envelope: syslog\n  \
             topics: [in]\n  group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
        )
        .expect("config parses");
        assert_eq!(
            parsed.source.envelope,
            crate::envelope::EnvelopeSetting::Receiver
        );
        assert!(parsed.validate().is_ok());
    }
}
