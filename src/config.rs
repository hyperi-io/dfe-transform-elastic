// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Service configuration.
//!
//! Loaded through scalo's config cascade under the `DFE_TRANSFORM_ELASTIC`
//! environment prefix. Which keys hot-reload and which need a restart is
//! recorded in CLAUDE.md.

use scalo::config::{self, ConfigOptions};
use serde::{Deserialize, Serialize};

/// Environment prefix for the config cascade.
pub const ENV_PREFIX: &str = "DFE_TRANSFORM_ELASTIC";

/// Top-level service configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceConfig {
    /// Beats or Agent source whose transform to apply, e.g. `filebeat.okta`.
    pub name: String,

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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SinkConfig {
    /// Topic to produce to.
    pub topic: String,

    /// Broker list. Defaults to the source brokers when unset.
    #[serde(default)]
    pub brokers: Option<Vec<String>>,
}

fn default_pipeline_name() -> String {
    "dfe-transform-elastic".to_string()
}

const fn default_batch_size() -> usize {
    20_000
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
        crate::registry::lookup(&self.source.name)
            .map(|_| ())
            .ok_or_else(|| crate::Error::UnknownSource(self.source.name.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> Config {
        Config {
            pipeline_name: "test".into(),
            source: SourceConfig {
                name: "filebeat.okta.default".into(),
                topics: vec!["in".into()],
                batch_size: 100,
                group_id: "g".into(),
                brokers: vec!["localhost:9092".into()],
            },
            sink: SinkConfig {
                topic: "out".into(),
                brokers: None,
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

    #[test]
    fn rejects_an_unregistered_source() {
        let mut c = valid();
        c.source.name = "filebeat.nosuchthing".into();
        assert!(c.validate().is_err());
    }
}
