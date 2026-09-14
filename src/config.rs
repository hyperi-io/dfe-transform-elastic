// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Service configuration.
//!
//! Read ONCE at startup and never re-read: the loaded [`Config`] is handed to
//! the batch loop by reference, and scalo's `config-reload` feature is not
//! enabled. Every value here needs a pod restart to change.
//!
//! Two ways in, and they are not equivalent. With no `--config` the whole scalo
//! cascade applies, so `DFE_TRANSFORM_ELASTIC_*` overrides the files. With
//! `--config` -- which is what the container passes -- the named file IS the
//! configuration: scalo 2.11.1 has no way to merge an arbitrarily-named file
//! into the cascade as a layer (`ConfigOptions::config_paths` searches for
//! `defaults`/`settings` by name, and `merge_cli` sits above the environment),
//! so the file is read directly and no `DFE_TRANSFORM_ELASTIC_*` variable
//! reaches it. The cascade is still installed either way, because the rest of
//! scalo reads it. Kafka credentials are unaffected: `KafkaConfig::from_env`
//! reads `KAFKA_*` separately.
//!
//! The consequence is invisible and so is stated outright: a section scalo
//! resolves from the cascade for itself cannot be set from a `--config` file at
//! all. That file yields only what this [`Config`] declares, and scalo sees
//! only what the service then hands it -- which is why `geoip` works from a
//! file. Every other scalo section is listed in [`CASCADE_ONLY_SECTIONS`] and
//! refused here, because a block that changes nothing is worse than no block.

use scalo::config::{self, ConfigOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Environment prefix for the config cascade.
pub const ENV_PREFIX: &str = "DFE_TRANSFORM_ELASTIC";

/// Sections scalo loads from its OWN cascade, which a `--config` file is not a
/// layer of.
///
/// Each is read by a `from_cascade` call inside scalo -- `scaling` by
/// `ServiceRuntime::build`, the rest by whichever subsystem owns them -- and
/// every one of those calls falls back to the type's `Default` when the key is
/// absent. So a block under one of these names in a `--config` file is read by
/// NOTHING, and the operator gets defaults while believing otherwise.
///
/// `geoip` is deliberately absent: it is declared on [`Config`] and handed to
/// `scalo::geoip_download` explicitly, which is what makes it work from a file.
/// Anything added here must be a section scalo resolves for itself.
pub const CASCADE_ONLY_SECTIONS: &[&str] = &[
    "http_client",
    "logger",
    "memory",
    "metrics",
    "otel_tracing",
    "scaling",
    "secrets",
    "self_regulation",
];

/// Refuse a `--config` file carrying a section only the cascade reads.
///
/// The section is not dropped quietly and not warned about: an operator who
/// sets `scaling.memory_gate_threshold` has a reason, and running on a default
/// they did not choose is the failure this exists to stop.
fn reject_cascade_only_sections(path: &str, doc: &serde_yaml_ng::Value) -> crate::Result<()> {
    let Some(map) = doc.as_mapping() else {
        return Ok(());
    };
    for section in CASCADE_ONLY_SECTIONS {
        if map.contains_key(*section) {
            return Err(crate::Error::Config(format!(
                "config file '{path}' sets `{section}`, which nothing reads. scalo resolves \
                 `{section}` from its config cascade -- ./defaults.yaml, ./settings.yaml, \
                 /config/settings.yaml, then {ENV_PREFIX}_* environment variables -- and a file \
                 named with --config is not one of those layers. Set it as \
                 `{ENV_PREFIX}_{upper}__<KEY>` in the environment, or delete the section to take \
                 scalo's defaults",
                upper = section.to_uppercase()
            )));
        }
    }
    Ok(())
}

/// Top-level service configuration.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    /// Name reported in metrics labels.
    #[serde(default = "default_pipeline_name")]
    pub pipeline_name: String,

    /// Inbound side.
    pub source: SourceConfig,

    /// Outbound side.
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

    /// Which producer wrapped the payload. The transform is the same for all
    /// of them; only the unwrapping differs.
    ///
    /// Defaults to `auto`, which reads it off each batch's first event. Naming
    /// a family instead pins it, and validation then rejects one the source
    /// cannot actually arrive in.
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

fn default_pipeline_name() -> String {
    "dfe-transform-elastic".to_string()
}

const fn default_batch_size() -> usize {
    20_000
}

/// Default ceiling on one inbound fetch.
///
/// 16 MiB of raw payload parses into several times that as `IndexMap` trees, so
/// this is what the pod's memory request is sized against. Below
/// [`MIN_BATCH_BYTES`] the per-partition quarter of it falls under librdkafka's
/// own `message.max.bytes` and a full-sized record can never be fetched.
#[must_use]
pub const fn default_max_batch_bytes() -> usize {
    16 * 1024 * 1024
}

/// The smallest inbound fetch budget librdkafka can honour here.
const MIN_BATCH_BYTES: usize = 4 * 1024 * 1024;

/// The largest inbound fetch budget, bounded by what `i32` can carry into
/// librdkafka.
const MAX_BATCH_BYTES: usize = 256 * 1024 * 1024;

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
    /// Load from an explicit path, or from scalo's config cascade.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if the file is missing or unreadable,
    /// or if the cascade cannot be unmarshalled into this shape.
    pub fn load(config_path: Option<&str>) -> crate::Result<Self> {
        // Installed whichever branch runs: `version_check` and the deployment
        // module read the cascade themselves, and they get defaults rather than
        // an error if nothing set it up. Installing twice is a no-op, not a
        // failure -- the cascade is a process-wide `OnceLock`.
        match config::setup(ConfigOptions {
            env_prefix: ENV_PREFIX.to_string(),
            load_dotenv: true,
            ..Default::default()
        }) {
            Ok(()) | Err(config::ConfigError::AlreadyInitialised) => {}
            Err(e) => {
                return Err(crate::Error::Config(format!("cascade setup failed: {e}")));
            }
        }

        // An explicit file is the whole configuration -- see the module docs
        // for why it cannot be a cascade layer on scalo 2.11.1.
        if let Some(path) = config_path {
            let text = std::fs::read_to_string(path).map_err(|e| {
                crate::Error::Config(format!("config file not found at '{path}': {e}"))
            })?;
            let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
                .map_err(|e| crate::Error::Config(format!("invalid config at '{path}': {e}")))?;
            reject_cascade_only_sections(path, &doc)?;
            return serde_yaml_ng::from_value(doc)
                .map_err(|e| crate::Error::Config(format!("invalid config at '{path}': {e}")));
        }

        config::get()
            .unmarshal()
            .map_err(|e| crate::Error::Config(format!("cascade unmarshal failed: {e}")))
    }

    /// Brokers the sink should use, falling back to the source's.
    pub fn sink_brokers(&self) -> &[String] {
        self.sink.brokers.as_deref().unwrap_or(&self.source.brokers)
    }

    /// Reject a configuration that cannot produce a working service.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] when a required value is empty or a
    /// numeric value is out of range.
    pub fn validate(&self) -> crate::Result<()> {
        if self.source.topics.is_empty() {
            return Err(crate::Error::Config("source.topics is empty".into()));
        }
        if self.source.brokers.is_empty() {
            return Err(crate::Error::Config("source.brokers is empty".into()));
        }
        if self.source.group_id.trim().is_empty() {
            return Err(crate::Error::Config("source.group_id is empty".into()));
        }
        if self.sink.topic.trim().is_empty() {
            return Err(crate::Error::Config("sink.topic is empty".into()));
        }
        if self.source.batch_size == 0 {
            return Err(crate::Error::Config("source.batch_size is zero".into()));
        }
        if self.source.max_batch_bytes < MIN_BATCH_BYTES
            || self.source.max_batch_bytes > MAX_BATCH_BYTES
        {
            return Err(crate::Error::Config(format!(
                "source.max_batch_bytes must be between {MIN_BATCH_BYTES} and {MAX_BATCH_BYTES}, \
                 got {}",
                self.source.max_batch_bytes
            )));
        }
        // A budget under one event's worth would drop every event as oversize,
        // and one over librdkafka's producer default would have every record
        // rejected at the client before it reaches a broker.
        if self.sink.max_message_bytes < 4096 || self.sink.max_message_bytes > 1_000_000 {
            return Err(crate::Error::Config(format!(
                "sink.max_message_bytes must be between 4096 and 1000000, got {}",
                self.sink.max_message_bytes
            )));
        }
        let intake = crate::registry::intake(&self.source.name)
            .ok_or_else(|| crate::Error::UnknownSource(self.source.name.clone()))?;

        // An envelope this source never arrives in would unwrap a shape that
        // is not there and quietly emit nothing useful. Refuse it at startup
        // rather than at the first batch. Only a PINNED envelope can be checked
        // here: under `auto` there is no event yet to detect from, so the same
        // mismatch is counted per batch instead.
        if let Some(pinned) = self.source.envelope.pinned()
            && !intake.accepts(pinned)
        {
            let name = |e: &crate::envelope::Envelope| format!("{e:?}").to_lowercase();
            let accepts: Vec<String> = intake.envelopes.iter().map(name).collect();
            return Err(crate::Error::Config(format!(
                "source '{}' cannot arrive over {}; it accepts {}, and the {} \
                 envelope applies to: {}",
                self.source.name,
                name(&pinned),
                accepts.join(" and "),
                name(&pinned),
                crate::registry::sources_accepting(pinned)
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn valid() -> Config {
        Config {
            pipeline_name: "test".into(),
            source: SourceConfig {
                name: "filebeat.okta.default".into(),
                envelope: crate::envelope::EnvelopeSetting::Beats,
                topics: vec!["in".into()],
                batch_size: 100,
                max_batch_bytes: default_max_batch_bytes(),
                group_id: "g".into(),
                brokers: vec!["localhost:9092".into()],
            },
            sink: SinkConfig {
                topic: "out".into(),
                brokers: None,
                max_message_bytes: default_max_message_bytes(),
            },
            geoip: scalo::geoip_download::GeoIpConfig::default(),
        }
    }

    #[test]
    fn accepts_a_complete_config() {
        assert!(valid().validate().is_ok());
    }

    #[test]
    fn sink_falls_back_to_source_brokers() {
        assert_eq!(valid().sink_brokers(), ["localhost:9092".to_string()]);
    }

    #[test]
    fn rejects_empty_topics() {
        let mut c = valid();
        c.source.topics.clear();
        assert!(c.validate().is_err());
    }

    #[test]
    fn rejects_zero_batch_size() {
        let mut c = valid();
        c.source.batch_size = 0;
        assert!(c.validate().is_err());
    }

    /// The budget bounds one Kafka record, so a value above librdkafka's
    /// producer default has every record rejected at the client.
    #[test]
    fn rejects_a_message_budget_the_producer_cannot_honour() {
        let mut c = valid();
        c.sink.max_message_bytes = 4_000_000;
        assert!(c.validate().is_err());

        c.sink.max_message_bytes = 512;
        assert!(c.validate().is_err());
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

    #[test]
    fn rejects_an_unregistered_source() {
        let mut c = valid();
        c.source.name = "filebeat.nosuchthing".into();
        assert!(c.validate().is_err());
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

    /// `auto` cannot be checked against the intake at startup, so a source that
    /// refuses a pinned envelope must still validate when nothing is pinned.
    #[test]
    fn auto_validates_on_a_source_that_refuses_a_pinned_envelope() {
        let mut c = valid();
        c.source.envelope = crate::envelope::EnvelopeSetting::Receiver;
        assert!(c.validate().is_err(), "okta pinned to receiver is refused");

        c.source.envelope = crate::envelope::EnvelopeSetting::Auto;
        assert!(c.validate().is_ok());
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
    fn accepts_the_receiver_envelope_on_a_pushed_source() {
        let mut c = valid();
        c.source.name = "filebeat.cisco_ios.default".into();
        c.source.envelope = crate::envelope::EnvelopeSetting::Receiver;
        assert!(c.validate().is_ok());
    }

    /// okta is pulled from an API. Asking for it over syslog is a config
    /// error, not a silent no-op at the first batch.
    #[test]
    fn rejects_the_receiver_envelope_on_a_fetched_source() {
        let mut c = valid();
        c.source.envelope = crate::envelope::EnvelopeSetting::Receiver;

        let err = c.validate().expect_err("okta over syslog must be rejected");
        let message = err.to_string();
        assert!(message.contains("cannot arrive over receiver"), "{message}");
        // The error must name both what this source DOES take and what would.
        assert!(message.contains("beats and fetcher"), "{message}");
        assert!(message.contains("filebeat.cisco_ios.default"), "{message}");
    }

    /// The other direction: a device pushes `cisco_ios`, so there is nothing for
    /// dfe-fetcher to pull and the fetcher envelope is refused.
    #[test]
    fn rejects_the_fetcher_envelope_on_a_pushed_source() {
        let mut c = valid();
        c.source.name = "filebeat.cisco_ios.default".into();
        c.source.envelope = crate::envelope::EnvelopeSetting::Fetcher;

        let message = c
            .validate()
            .expect_err("cisco_ios over fetcher must be rejected")
            .to_string();
        assert!(message.contains("cannot arrive over fetcher"), "{message}");
        assert!(message.contains("beats and receiver"), "{message}");
        assert!(message.contains("filebeat.okta.default"), "{message}");
    }

    /// dfe-fetcher can obtain what Elastic's `httpjson` input obtains, so okta
    /// accepts it and the config is valid.
    #[test]
    fn accepts_the_fetcher_envelope_on_a_pulled_source() {
        let mut c = valid();
        c.source.envelope = crate::envelope::EnvelopeSetting::Fetcher;
        assert!(c.validate().is_ok());
    }

    /// A minimal config file body, to which a test appends the section under
    /// examination.
    const FILE_BASE: &str = "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
                             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n";

    /// Write `body` to a file and load it the way `--config` does.
    fn load_file(body: &str) -> crate::Result<Config> {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("config.yaml");
        std::fs::write(&path, body).expect("write config");
        Config::load(Some(path.to_str().expect("utf-8 path")))
    }

    /// The trap this refusal exists to close: the chart passes `--config`, and
    /// scalo reads `scaling` from a cascade that file is not a layer of, so a
    /// threshold written there reaches nothing.
    #[test]
    fn a_config_file_carrying_scaling_is_refused() {
        let message = load_file(&format!(
            "{FILE_BASE}scaling:\n  enabled: true\n  memory_gate_threshold: 0.9\n"
        ))
        .expect_err("a --config file carrying `scaling` must be refused")
        .to_string();

        // The error has to name the mechanism and the way out, or it just
        // moves the operator from a silent default to a blocked startup.
        assert!(message.contains("sets `scaling`"), "{message}");
        assert!(message.contains("nothing reads"), "{message}");
        assert!(message.contains("cascade"), "{message}");
        assert!(
            message.contains("DFE_TRANSFORM_ELASTIC_SCALING__<KEY>"),
            "{message}"
        );
    }

    /// Every name on the list is refused, so adding one to the list is the
    /// whole job of covering a new scalo section.
    #[test]
    fn every_cascade_only_section_is_refused() {
        for section in CASCADE_ONLY_SECTIONS {
            let body = format!("{FILE_BASE}{section}:\n  enabled: true\n");
            let message = load_file(&body)
                .expect_err("a cascade-only section must be refused")
                .to_string();
            assert!(message.contains(&format!("sets `{section}`")), "{message}");
        }
    }

    /// The other half of the rule: a section this service declares and hands to
    /// scalo itself DOES work from a file, and must not be caught by the sweep.
    #[test]
    fn geoip_still_loads_from_a_config_file() {
        let loaded = load_file(&format!("{FILE_BASE}geoip:\n  enabled: false\n"))
            .expect("geoip is declared on Config, so a file may set it");
        assert!(!loaded.geoip.enabled);
        assert!(
            !CASCADE_ONLY_SECTIONS.contains(&"geoip"),
            "geoip reaches scalo explicitly, so refusing it would be wrong"
        );
    }

    /// The refusal must not fire on the configuration the service actually
    /// ships, or every deployment fails to start.
    #[test]
    fn the_shipped_example_is_not_refused() {
        let loaded =
            load_file(&crate::deployment::default_config_yaml()).expect("the example loads");
        assert_eq!(loaded.source.batch_size, 20_000);
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
