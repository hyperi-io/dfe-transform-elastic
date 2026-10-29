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
//! the named file IS the configuration: `loader` reads it directly and installs
//! the cascade with an empty `ConfigOptions::config_paths`, so the file is not
//! one of the cascade's layers. The cascade is still installed either way,
//! because the rest of scalo reads it. Kafka credentials are unaffected:
//! `KafkaConfig::from_env` reads `KAFKA_*` separately.
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
/// `kafka` and `grpc` are read as the same two, the names dfe-engine renders,
/// and a config always writes `bus` and `direct` back.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    /// Kafka topics.
    #[default]
    #[serde(alias = "kafka")]
    Bus,
    /// A scalo Push listener inbound, a gRPC client outbound.
    #[serde(alias = "grpc")]
    Direct,
}

// Written by hand because the derive lists only the canonical names, and a
// validator must take the `grpc` and `kafka` an engine render carries.
impl JsonSchema for Transport {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Transport".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "Which transport a side of the service uses.\n\n`bus` is Kafka between the stages; `direct` is a scalo Push listener inbound and a gRPC client outbound, needing no broker at all. The record and the transform are identical either way -- only who hands the record over changes. `kafka` and `grpc` are read as the same two, the names dfe-engine renders, and a config always writes `bus` and `direct` back.",
            "oneOf": [
                {
                    "description": "Kafka topics.",
                    "type": "string",
                    "enum": ["bus", "kafka"]
                },
                {
                    "description": "A scalo Push listener inbound, a gRPC client outbound.",
                    "type": "string",
                    "enum": ["direct", "grpc"]
                }
            ]
        })
    }
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
    /// so a change needs a restart. A key the section leaves out takes
    /// [`default_geoip`], whose `auto_download.data_dir` is the image's writable
    /// volume rather than scalo's `/var/lib/geoip`.
    #[serde(default = "default_geoip", deserialize_with = "deserialize_geoip")]
    #[schemars(with = "scalo::geoip_download::GeoIpConfig")]
    pub geoip: scalo::geoip_download::GeoIpConfig,
}

/// The directory the downloader writes the databases to, on the `geoip` volume
/// the deployment contract declares. dfe-loader's directory too, so one volume
/// serves both stages. scalo's own default, `/var/lib/geoip`, sits on the
/// image's read-only root.
pub const GEOIP_DATA_DIR: &str = "/var/lib/dfe/geoip";

/// scalo's geoip defaults, with the downloader pointed at [`GEOIP_DATA_DIR`].
#[must_use]
pub fn default_geoip() -> scalo::geoip_download::GeoIpConfig {
    scalo::geoip_download::GeoIpConfig {
        auto_download: scalo::geoip_download::AutoDownloadConfig {
            data_dir: GEOIP_DATA_DIR.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Read the `geoip` section over [`default_geoip`], key by key, so a section
/// that sets only `enabled` still downloads to [`GEOIP_DATA_DIR`].
fn deserialize_geoip<'de, D>(
    deserializer: D,
) -> Result<scalo::geoip_download::GeoIpConfig, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error as _;

    fn overlay(base: &mut serde_json::Value, over: serde_json::Value) {
        match (base, over) {
            (serde_json::Value::Object(base), serde_json::Value::Object(over)) => {
                for (key, value) in over {
                    match base.get_mut(&key) {
                        Some(slot) => overlay(slot, value),
                        None => {
                            base.insert(key, value);
                        }
                    }
                }
            }
            (slot, over) => *slot = over,
        }
    }

    let supplied = serde_json::Value::deserialize(deserializer)?;
    let mut merged = serde_json::to_value(default_geoip()).map_err(D::Error::custom)?;
    overlay(&mut merged, supplied);
    serde_json::from_value(merged).map_err(D::Error::custom)
}

/// Whether one named transform serves the source, or each event picks its own.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// `source.name` names the one transform every event runs.
    #[default]
    Agent,
    /// The source is read off each event's text, and that source's transform
    /// runs. One dfe-receiver syslog stream can then carry many device types.
    Agentless,
}

/// What agentless mode does with an event no marker claims.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Unmatched {
    /// Emit the event unwrapped to the Beats layout with no transform and no
    /// `_source`, so dfe-loader lands it under the sink topic's default table.
    #[default]
    Passthrough,
    /// Run `detect.fallback`'s transform and stamp its label.
    Fallback,
    /// Stop the service with the block unreleased, so a restart reads it again.
    Stop,
}

/// How agentless mode detects each event's source.
#[derive(Debug, Clone, Default, Deserialize, Serialize, JsonSchema)]
pub struct DetectConfig {
    /// What happens to an event nothing matches.
    #[serde(default)]
    pub unmatched: Unmatched,

    /// The registered source whose transform runs on an unmatched event.
    /// Required when `unmatched` is `fallback`, and refused otherwise.
    #[serde(default)]
    pub fallback: String,

    /// Sources the detector may name, by registered name. Empty means every
    /// source it knows. Narrowing it hands lines two sources share to the one
    /// left, e.g. leaving out `filebeat.cisco_asa.default` lets FTD take them.
    #[serde(default)]
    pub candidates: Vec<String>,
}

/// Inbound configuration.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SourceConfig {
    /// Beats or Agent source whose transform to apply, e.g. `filebeat.okta`.
    /// Left empty in agentless mode, which picks the source per event.
    #[serde(default)]
    pub name: String,

    /// `agent` runs the one transform `name` selects. `agentless` detects the
    /// source of each event instead, and stamps `_source` with its catalogue
    /// label so dfe-loader routes it to that source's table.
    #[serde(default)]
    pub mode: Mode,

    /// Detection settings, read only in agentless mode.
    #[serde(default)]
    pub detect: DetectConfig,

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
    #[serde(default)]
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
    #[serde(default)]
    pub group_id: String,

    /// Broker list.
    #[serde(default)]
    pub brokers: Vec<String>,

    /// When the source is told its records are safe: a Kafka offset commit on
    /// the bus, the answer to a push on direct.
    ///
    /// `enabled: true`, the default, holds it until every event built from a
    /// record is delivered, or dropped and counted, so a crash or a sink outage
    /// redelivers rather than loses. `enabled: false` releases it at receipt,
    /// before the transform runs, and loses what was in flight.
    #[serde(default)]
    pub acknowledgements: scalo::transport::AcknowledgementsConfig,
}

/// Outbound configuration.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SinkConfig {
    /// Where the events land: the topic produced to on the bus, and the
    /// routing key every push carries on direct. Required on both once the
    /// configuration names work.
    #[serde(default)]
    pub topic: String,

    /// Which transport the outbound side uses: `bus` produces to `topic`,
    /// `direct` pushes to `endpoint` with `topic` as the routing key.
    #[serde(default)]
    pub transport: Transport,

    /// Downstream Push listener on the direct transport, ignored on the bus.
    /// Reached in the clear, so it too stays inside the mesh.
    #[serde(default = "default_endpoint")]
    pub endpoint: String,

    /// Broker list. Defaults to the source brokers when unset.
    #[serde(default)]
    pub brokers: Option<Vec<String>>,

    /// Ceiling on one outbound record. An event that serialises larger is
    /// dropped and counted on `events_oversize_total`.
    ///
    /// The default is the DFE record cap, 8,000,000 bytes. Against a broker or
    /// topic whose `message.max.bytes` is lower, set this below that limit, or
    /// the broker refuses those events instead and they count on
    /// `send_filtered_dlq_total`. Brokers that carry more can raise it, up to
    /// 16 MiB.
    #[serde(default = "default_max_message_bytes")]
    pub max_message_bytes: usize,
}

/// An unconfigured inbound side: no source, no topics, and the shipped numeric
/// defaults. [`Config::work_state`] reads this as no work and idles.
impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            mode: Mode::default(),
            detect: DetectConfig::default(),
            transport: Transport::default(),
            listen: default_listen(),
            envelope: crate::envelope::EnvelopeSetting::default(),
            topics: Vec::new(),
            batch_size: default_batch_size(),
            max_batch_bytes: default_max_batch_bytes(),
            group_id: String::new(),
            brokers: Vec::new(),
            acknowledgements: scalo::transport::AcknowledgementsConfig::default(),
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

/// The largest record the producer will put on the wire: the brokers' and
/// topics' `message.max.bytes`, the producer's, and the consumer's fetch all
/// share it. It bounds `sink.max_message_bytes` for an operator whose brokers
/// carry more than the DFE record cap.
///
/// scalo's `MESSAGE_MAX_BYTES`, restated because that constant only exists
/// with the `kafka` feature. A test holds the two equal.
pub const PIPELINE_MESSAGE_MAX_BYTES: usize = 16 * 1024 * 1024;

/// The DFE-wide record cap: dfe-receiver refuses a request body over it, and
/// every broker DFE deploys, Confluent Basic's 8,388,608 included, takes a
/// record this size with room for framing.
pub const DFE_RECORD_CAP_BYTES: usize = 8_000_000;

/// Default ceiling on one outbound record: the DFE record cap. Deployments
/// whose brokers carry more raise `sink.max_message_bytes`, up to
/// [`PIPELINE_MESSAGE_MAX_BYTES`].
#[must_use]
pub const fn default_max_message_bytes() -> usize {
    DFE_RECORD_CAP_BYTES
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
        let no_source = self.source.mode == Mode::Agent && self.source.name.trim().is_empty();
        scalo::lifecycle::WorkState::idle_if(
            no_source || no_intake,
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
            mode: Mode::Agent,
            detect: DetectConfig::default(),
            transport: Transport::Bus,
            listen: default_listen(),
            envelope: crate::envelope::EnvelopeSetting::Beats,
            topics: vec!["in".into()],
            batch_size: 100,
            max_batch_bytes: default_max_batch_bytes(),
            group_id: "g".into(),
            brokers: vec!["localhost:9092".into()],
            acknowledgements: scalo::transport::AcknowledgementsConfig::default(),
        },
        sink: SinkConfig {
            topic: "out".into(),
            transport: Transport::Bus,
            endpoint: default_endpoint(),
            brokers: None,
            max_message_bytes: default_max_message_bytes(),
        },
        geoip: default_geoip(),
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
        let parsed: Config =
            scalo::yaml::from_str("{}", "config.yaml", scalo::yaml::Profile::Config)
                .expect("an empty config parses");
        assert!(parsed.validate().is_ok());
        assert!(parsed.work_state().is_idle());
        assert_eq!(parsed.source.batch_size, 20_000);
        assert_eq!(parsed.sink.max_message_bytes, default_max_message_bytes());
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
        let parsed: Config = scalo::yaml::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("config parses without a transport key");
        assert_eq!(parsed.source.transport, Transport::Bus);
        assert_eq!(parsed.sink.transport, Transport::Bus);
        assert_eq!(parsed.source.listen, "0.0.0.0:6000");
        assert_eq!(parsed.sink.endpoint, "http://dfe-loader:6000");
    }

    /// A direct deployment names no brokers, group or source topics, and is
    /// valid. It still names a sink topic, which is its routing key.
    #[test]
    fn the_direct_transport_round_trips_through_yaml() {
        let parsed: Config = scalo::yaml::from_str(
            "source:\n  name: filebeat.okta.default\n  transport: direct\n  \
             listen: 0.0.0.0:6000\nsink:\n  topic: okta_load\n  transport: direct\n  \
             endpoint: http://loader:6000\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("config parses with the direct transport");
        assert!(parsed.source.transport.is_direct());
        assert!(parsed.sink.transport.is_direct());
        assert!(!parsed.work_state().is_idle());
        assert!(parsed.validate().is_ok());
    }

    /// Every field a deployment may not know yet is optional in the file, so
    /// an idle instance carries only what it sets. A config that names work
    /// without one is still refused, by `validate` rather than by the parse.
    #[test]
    fn source_and_sink_fields_may_be_left_out() {
        let idle: Config = scalo::yaml::from_str(
            "source:\n  brokers: [b:9092]\nsink:\n  brokers: [b:9092]\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("sections naming no source, topics, group or sink topic parse");
        assert!(idle.work_state().is_idle());
        assert!(idle.validate().is_ok());

        let no_group: Config = scalo::yaml::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  brokers: [b:9092]\n\
             sink:\n  topic: out\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("a bus source without a group parses");
        let message = no_group
            .validate()
            .expect_err("the bus needs a consumer group")
            .to_string();
        assert!(message.contains("source.group_id"), "{message}");
    }

    /// Holding the acknowledgement is the default, so a config written before
    /// the key existed gets at-least-once without naming it.
    #[test]
    fn acknowledgements_are_held_unless_turned_off() {
        let base = "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
                    group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n";

        let parsed: Config =
            scalo::yaml::from_str(base, "config.yaml", scalo::yaml::Profile::Config)
                .expect("config parses without the key");
        assert!(parsed.source.acknowledgements.enabled);

        let off: Config = scalo::yaml::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\n  acknowledgements:\n    enabled: false\n\
             sink:\n  topic: out\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("config parses with acknowledgements off");
        assert!(!off.source.acknowledgements.enabled);
        assert!(off.validate().is_ok());
    }

    /// The default budget is the DFE record cap, so an event over a stock
    /// broker's 1 MB is carried and one over 8,000,000 bytes is not.
    #[test]
    fn message_budget_defaults_to_the_dfe_record_cap() {
        let parsed: Config = scalo::yaml::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("config parses without a sink budget");
        assert_eq!(parsed.sink.max_message_bytes, 8_000_000);
        assert_eq!(parsed.sink.max_message_bytes, DFE_RECORD_CAP_BYTES);
        assert!(parsed.sink.max_message_bytes > 1_000_000);
        assert!(parsed.sink.max_message_bytes < PIPELINE_MESSAGE_MAX_BYTES);
    }

    /// The restated ceiling is scalo's, or the budget and the producer it is
    /// paired with disagree about what fits.
    #[cfg(feature = "kafka")]
    #[test]
    fn the_pipeline_ceiling_is_scalos() {
        assert_eq!(
            Ok(PIPELINE_MESSAGE_MAX_BYTES),
            usize::try_from(scalo::transport::kafka::MESSAGE_MAX_BYTES)
        );
    }

    /// The downloader writes to the contract's `geoip` volume unless told
    /// otherwise, for a missing section and for one that sets other keys.
    /// scalo's own default sits on the image's read-only root.
    #[test]
    fn geoip_downloads_to_the_contracts_volume_by_default() {
        let base = "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
                    group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n";
        let dir = |extra: &str| -> std::path::PathBuf {
            let parsed: Config = scalo::yaml::from_str(
                &format!("{base}{extra}"),
                "config.yaml",
                scalo::yaml::Profile::Config,
            )
            .expect("config parses");
            parsed.geoip.auto_download.data_dir
        };

        let volume = std::path::PathBuf::from("/var/lib/dfe/geoip");
        assert_eq!(dir(""), volume, "no geoip section");
        assert_eq!(dir("geoip:\n  enabled: true\n"), volume, "other keys set");
        assert_eq!(
            dir("geoip:\n  auto_download:\n    max_age_days: 3\n"),
            volume,
            "other auto_download keys set"
        );
        assert_eq!(
            dir("geoip:\n  auto_download:\n    data_dir: /data/geo\n"),
            std::path::PathBuf::from("/data/geo"),
            "an explicit data_dir wins"
        );
    }

    /// A config written before the geoip section existed still loads, and
    /// loads with provisioning ON -- the point of the section is that an
    /// operator gets databases without configuring anything.
    #[test]
    fn geoip_provisions_unless_it_is_turned_off() {
        let base = "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
                    group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n";

        let parsed: Config =
            scalo::yaml::from_str(base, "config.yaml", scalo::yaml::Profile::Config)
                .expect("config parses without a geoip section");
        assert!(parsed.geoip.enabled);
        assert!(parsed.geoip.auto_download.enabled);
        assert_eq!(
            parsed.geoip.provider,
            scalo::geoip_download::GeoIpProvider::DbIpLite
        );

        let off: Config = scalo::yaml::from_str(
            &format!("{base}geoip:\n  enabled: false\n"),
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("config parses with geoip disabled");
        assert!(!off.geoip.enabled);
    }

    /// Detection, not a guess: an unset envelope means read it off the batch,
    /// so a deployment that changes producer needs no config change.
    #[test]
    fn defaults_to_detecting_the_envelope() {
        let parsed: Config = scalo::yaml::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
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
            let parsed: Config = scalo::yaml::from_str(
                &format!(
                    "source:\n  name: filebeat.fortinet.default\n  envelope: {written}\n  \
                 topics: [in]\n  group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n"
                ),
                "config.yaml",
                scalo::yaml::Profile::Config,
            )
            .expect("config parses");
            assert_eq!(parsed.source.envelope, expected, "envelope: {written}");
        }
    }

    #[test]
    fn envelope_round_trips_through_yaml() {
        let parsed: Config = scalo::yaml::from_str(
            "source:\n  name: filebeat.fortinet.default\n  envelope: syslog\n  \
             topics: [in]\n  group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("config parses");
        assert_eq!(
            parsed.source.envelope,
            crate::envelope::EnvelopeSetting::Receiver
        );
        assert!(parsed.validate().is_ok());
    }
}
