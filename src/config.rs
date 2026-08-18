// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Service configuration.
//!
//! Loaded through scalo's config cascade under the `DFE_TRANSFORM_ELASTIC`
//! environment prefix. Which keys hot-reload and which need a restart is
//! recorded in CLAUDE.md.

use scalo::config::{self, ConfigOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Environment prefix for the config cascade.
pub const ENV_PREFIX: &str = "DFE_TRANSFORM_ELASTIC";

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
}

/// Inbound configuration.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SourceConfig {
    /// Beats or Agent source whose transform to apply, e.g. `filebeat.okta`.
    pub name: String,

    /// How the payload is wrapped on the way in. The transform is the same
    /// either way; only the unwrapping differs.
    #[serde(default)]
    pub envelope: crate::envelope::Envelope,

    /// Topics to consume.
    pub topics: Vec<String>,

    /// Events pulled per batch.
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

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
        if let Some(path) = config_path {
            let text = std::fs::read_to_string(path).map_err(|e| {
                crate::Error::Config(format!("config file not found at '{path}': {e}"))
            })?;
            return serde_yaml_ng::from_str(&text)
                .map_err(|e| crate::Error::Config(format!("invalid config at '{path}': {e}")));
        }

        config::setup(ConfigOptions {
            env_prefix: ENV_PREFIX.to_string(),
            load_dotenv: true,
            ..Default::default()
        })
        .map_err(|e| crate::Error::Config(format!("cascade setup failed: {e}")))?;

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
        // A budget under one event's worth would drop every event as oversize,
        // and one over librdkafka's producer default would have every record
        // rejected at the client before it reaches a broker.
        if self.sink.max_message_bytes < 4096 || self.sink.max_message_bytes > 1_000_000 {
            return Err(crate::Error::Config(format!(
                "sink.max_message_bytes must be between 4096 and 1000000, got {}",
                self.sink.max_message_bytes
            )));
        }
        let origin = crate::registry::origin(&self.source.name)
            .ok_or_else(|| crate::Error::UnknownSource(self.source.name.clone()))?;

        // An API-only source over the syslog envelope would unwrap a header
        // that is never there, and quietly emit nothing useful. Refuse it at
        // startup rather than at the first batch.
        if self.source.envelope == crate::envelope::Envelope::Syslog && !origin.is_syslog() {
            return Err(crate::Error::Config(format!(
                "source '{}' is pulled from an API and cannot arrive over syslog; \
                 the syslog envelope applies to: {}",
                self.source.name,
                crate::registry::syslog_sources()
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
                envelope: crate::envelope::Envelope::Beats,
                topics: vec!["in".into()],
                batch_size: 100,
                group_id: "g".into(),
                brokers: vec!["localhost:9092".into()],
            },
            sink: SinkConfig {
                topic: "out".into(),
                brokers: None,
                max_message_bytes: default_max_message_bytes(),
            },
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

    #[test]
    fn rejects_an_unregistered_source() {
        let mut c = valid();
        c.source.name = "filebeat.nosuchthing".into();
        assert!(c.validate().is_err());
    }

    #[test]
    fn defaults_to_the_beats_envelope() {
        let parsed: Config = serde_yaml_ng::from_str(
            "source:\n  name: filebeat.okta.default\n  topics: [in]\n  \
             group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
        )
        .expect("config parses without an envelope key");
        assert_eq!(parsed.source.envelope, crate::envelope::Envelope::Beats);
    }

    #[test]
    fn accepts_the_syslog_envelope_on_a_device_source() {
        let mut c = valid();
        c.source.name = "filebeat.cisco_ios.default".into();
        c.source.envelope = crate::envelope::Envelope::Syslog;
        assert!(c.validate().is_ok());
    }

    /// okta is pulled from an API. Asking for it over syslog is a config
    /// error, not a silent no-op at the first batch.
    #[test]
    fn rejects_the_syslog_envelope_on_an_api_source() {
        let mut c = valid();
        c.source.envelope = crate::envelope::Envelope::Syslog;

        let err = c.validate().expect_err("okta over syslog must be rejected");
        let message = err.to_string();
        assert!(message.contains("cannot arrive over syslog"), "{message}");
        // The error must name what WOULD work.
        assert!(message.contains("filebeat.cisco_ios.default"), "{message}");
    }

    #[test]
    fn envelope_round_trips_through_yaml() {
        let parsed: Config = serde_yaml_ng::from_str(
            "source:\n  name: filebeat.fortinet.default\n  envelope: syslog\n  \
             topics: [in]\n  group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
        )
        .expect("config parses");
        assert_eq!(parsed.source.envelope, crate::envelope::Envelope::Syslog);
        assert!(parsed.validate().is_ok());
    }
}
