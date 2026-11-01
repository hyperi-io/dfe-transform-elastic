// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Reading the configuration: an explicit file, or scalo's cascade.
//!
//! An explicit file is read straight into [`Config`] and also handed to scalo's
//! cascade as its settings layer, so the sections scalo resolves for itself
//! apply from that file too.

use scalo::config::flat_env::{
    ApplyFlatEnv, flat_env_bool, flat_env_list, flat_env_parsed, flat_env_string,
};
use scalo::config::{self, ConfigOptions};

use super::{Config, DetectConfig, SinkConfig, SourceConfig};

/// Environment prefix for the config cascade.
pub const ENV_PREFIX: &str = "DFE_TRANSFORM_ELASTIC";

/// Sections scalo resolves for itself, from its cascade or the environment.
///
/// A `--config` file is a layer of that cascade, so each section here applies
/// when written in it. They are not [`Config`] fields, so the parse skips them,
/// and the unread-key warning leaves them out because scalo does read them.
///
/// `batch_processing` configures the runtime's own batch engine, which the
/// service loop does not run -- its engine takes the record cap from
/// `source.batch_size` -- so it stays inert in the file, though scalo still
/// refuses one that does not parse.
///
/// `geoip` is deliberately absent: it is declared on [`Config`] and handed to
/// `scalo::geoip_download` explicitly.
pub const SCALO_SECTIONS: &[&str] = &[
    "batch_processing",
    "http_client",
    "logger",
    "memory",
    "metrics",
    "otel_tracing",
    "scaling",
    "secrets",
    "self_regulation",
    "version_check",
    "worker_pool",
];

/// Warn for every key the parse skipped that the service does not read.
///
/// serde drops an unknown field in silence, so a typo costs the whole setting
/// with no message -- `source.topic` for `source.topics` leaves the instance
/// idle and says nothing about why.
fn warn_ignored_keys(path: &str, unknown_keys: &[String]) {
    for key in ignored_keys(unknown_keys) {
        tracing::warn!(
            key = %key,
            path = path,
            "config key is not read by this service, so setting it does nothing"
        );
    }
}

/// The dotted paths no [`Config`] field took, less the [`SCALO_SECTIONS`]
/// entries, which scalo reads.
fn ignored_keys(unknown_keys: &[String]) -> Vec<&str> {
    unknown_keys
        .iter()
        .map(String::as_str)
        .filter(|key| {
            let section = key.split('.').next().unwrap_or_default();
            !SCALO_SECTIONS.contains(&section)
        })
        .collect()
}

impl Config {
    /// Load from an explicit path, or from scalo's config cascade.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] if the file is missing or unreadable,
    /// or if the cascade cannot be unmarshalled into this shape.
    pub fn load(config_path: Option<&str>) -> crate::Result<Self> {
        // Installed whichever branch runs: `version_check`, `logger`, `scaling`
        // and the rest read the cascade themselves, and they get defaults rather
        // than an error if nothing set it up. An explicit file is its settings
        // layer, so a scalo section written in it applies. Installing twice is
        // a no-op, not a failure -- the cascade is a process-wide `OnceLock`.
        match config::setup(ConfigOptions {
            env_prefix: ENV_PREFIX.to_string(),
            load_dotenv: true,
            config_paths: config_path
                .map(std::path::PathBuf::from)
                .into_iter()
                .collect(),
            ..Default::default()
        }) {
            Ok(()) | Err(config::ConfigError::AlreadyInitialised) => {}
            Err(e) => {
                return Err(crate::Error::Config(format!("cascade setup failed: {e}")));
            }
        }

        // An explicit file is read into `Config` here, as well as being a layer
        // of the cascade above.
        let mut config: Self = if let Some(path) = config_path {
            let text = std::fs::read_to_string(path).map_err(|e| {
                crate::Error::Config(format!("config file not found at '{path}': {e}"))
            })?;
            let loaded =
                scalo::yaml::from_str_reporting::<Self>(&text, path, scalo::yaml::Profile::Config)
                    .map_err(|e| crate::Error::Config(format!("invalid config: {e}")))?;
            warn_ignored_keys(path, &loaded.unknown_keys);
            loaded.value
        } else {
            config::get()
                .unmarshal()
                .map_err(|e| crate::Error::Config(format!("cascade unmarshal failed: {e}")))?
        };

        // Applied on BOTH branches, which is the point: the flat form is the
        // only override that reaches a `--config` deployment.
        config.apply_flat_env(ENV_PREFIX);

        Ok(config)
    }
}

/// Flat, single-underscore env overrides -- `DFE_TRANSFORM_ELASTIC_SOURCE_TOPICS`.
///
/// This is the override that reaches the fields of [`Config`] in a `--config`
/// deployment, whatever the file says. scalo's own double-underscore form
/// (`DFE_TRANSFORM_ELASTIC_SCALING__ENABLED`) is a cascade key and reaches only
/// the sections scalo resolves.
///
/// `geoip` takes none: it is scalo's type and this is scalo's trait, so the
/// orphan rule puts it out of reach. It stays settable from the file and the
/// cascade.
impl ApplyFlatEnv for Config {
    fn apply_flat_env(&mut self, prefix: &str) {
        self.source.apply_flat_env(&format!("{prefix}_SOURCE"));
        self.sink.apply_flat_env(&format!("{prefix}_SINK"));
    }
}

/// Parse a transport name, keeping `current` on anything unrecognised so a typo
/// cannot silently move a deployment off its transport.
///
/// Both spellings are accepted because issue #19 names them `kafka`/`grpc`
/// while this config and dfe-infra name them `bus`/`direct`.
fn parse_transport(value: &str, current: super::Transport) -> super::Transport {
    match value.trim().to_ascii_lowercase().as_str() {
        "bus" | "kafka" => super::Transport::Bus,
        "direct" | "grpc" => super::Transport::Direct,
        other => {
            tracing::warn!(
                value = other,
                "ignoring an unknown transport, keeping the configured one"
            );
            current
        }
    }
}

/// Parse an enum setting by its config-file name, keeping `current` on anything
/// unrecognised so a typo cannot silently change what the service does.
fn parse_setting<T: serde::de::DeserializeOwned>(raw: &str, what: &str, current: T) -> T {
    scalo::yaml::from_str(raw, what, scalo::yaml::Profile::Config).unwrap_or_else(|e| {
        tracing::warn!(value = %raw, error = %e, "ignoring an unknown {what}, keeping the configured one");
        current
    })
}

impl ApplyFlatEnv for DetectConfig {
    fn apply_flat_env(&mut self, prefix: &str) {
        if let Some(raw) = flat_env_string(prefix, "UNMATCHED") {
            self.unmatched = parse_setting(&raw, "unmatched action", self.unmatched);
        }
        if let Some(fallback) = flat_env_string(prefix, "FALLBACK") {
            self.fallback = fallback;
        }
        if let Some(candidates) = flat_env_list(prefix, "CANDIDATES") {
            self.candidates = candidates;
        }
    }
}

impl ApplyFlatEnv for SourceConfig {
    fn apply_flat_env(&mut self, prefix: &str) {
        if let Some(name) = flat_env_string(prefix, "NAME") {
            self.name = name;
        }
        if let Some(raw) = flat_env_string(prefix, "MODE") {
            self.mode = parse_setting(&raw, "source mode", self.mode);
        }
        self.detect.apply_flat_env(&format!("{prefix}_DETECT"));
        if let Some(raw) = flat_env_string(prefix, "TRANSPORT") {
            self.transport = parse_transport(&raw, self.transport);
        }
        if let Some(listen) = flat_env_string(prefix, "LISTEN") {
            self.listen = listen;
        }
        // An unknown name keeps the configured envelope rather than failing the
        // load, because detection is the default and still works.
        if let Some(raw) = flat_env_string(prefix, "ENVELOPE") {
            match scalo::yaml::from_str(&raw, "envelope", scalo::yaml::Profile::Config) {
                Ok(envelope) => self.envelope = envelope,
                Err(e) => tracing::warn!(
                    value = %raw,
                    error = %e,
                    "ignoring an envelope name this service does not know"
                ),
            }
        }
        if let Some(topics) = flat_env_list(prefix, "TOPICS") {
            self.topics = topics;
        }
        if let Some(size) = flat_env_parsed(prefix, "BATCH_SIZE") {
            self.batch_size = size;
        }
        if let Some(bytes) = flat_env_parsed(prefix, "MAX_BATCH_BYTES") {
            self.max_batch_bytes = bytes;
        }
        if let Some(group) = flat_env_string(prefix, "GROUP_ID") {
            self.group_id = group;
        }
        if let Some(brokers) = flat_env_list(prefix, "BROKERS") {
            self.brokers = brokers;
        }
        if let Some(enabled) = flat_env_bool(prefix, "ACKNOWLEDGEMENTS_ENABLED") {
            self.acknowledgements = scalo::transport::AcknowledgementsConfig::new(enabled);
        }
    }
}

impl ApplyFlatEnv for SinkConfig {
    fn apply_flat_env(&mut self, prefix: &str) {
        if let Some(topic) = flat_env_string(prefix, "TOPIC") {
            self.topic = topic;
        }
        if let Some(raw) = flat_env_string(prefix, "TRANSPORT") {
            self.transport = parse_transport(&raw, self.transport);
        }
        if let Some(endpoint) = flat_env_string(prefix, "ENDPOINT") {
            self.endpoint = endpoint;
        }
        if let Some(brokers) = flat_env_list(prefix, "BROKERS") {
            self.brokers = Some(brokers);
        }
        if let Some(bytes) = flat_env_parsed(prefix, "MAX_MESSAGE_BYTES") {
            self.max_message_bytes = bytes;
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

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

    /// A scalo section in a `--config` file loads and leaves what the file
    /// configures alone.
    #[test]
    fn a_config_file_carrying_scaling_still_loads() {
        let loaded = load_file(&format!(
            "{FILE_BASE}scaling:\n  enabled: true\n  memory_gate_threshold: 0.9\n"
        ))
        .expect("a scalo section in the file loads");

        assert_eq!(loaded.source.name, "filebeat.okta.default");
        assert_eq!(loaded.sink.topic, "out");
    }

    /// No name on the list stops a config from loading, so adding one to the
    /// list is the whole job of covering a new scalo section.
    #[test]
    fn no_cascade_only_section_stops_a_config_loading() {
        // A floor, so an emptied list cannot make this pass by looping zero
        // times. It moves only when scalo gains or loses a section.
        assert!(
            SCALO_SECTIONS.len() >= 11,
            "the scalo section list lost entries: {}",
            SCALO_SECTIONS.len()
        );

        for section in SCALO_SECTIONS {
            let body = format!("{FILE_BASE}{section}:\n  enabled: true\n");
            let loaded = load_file(&body)
                .unwrap_or_else(|e| panic!("`{section}` must load from a file: {e}"));
            assert_eq!(loaded.source.name, "filebeat.okta.default", "{section}");
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
            !SCALO_SECTIONS.contains(&"geoip"),
            "geoip reaches scalo explicitly, so refusing it would be wrong"
        );
    }

    /// dfe-engine renders a side's transport by its mechanism, so a config file
    /// naming `grpc` or `kafka` must load through `--config` as the side it
    /// names, not stop the process.
    #[test]
    fn a_config_file_may_name_the_transport_grpc_or_kafka() {
        let direct = load_file(
            "source:\n  name: filebeat.okta.default\n  transport: grpc\n  \
             listen: 0.0.0.0:6000\nsink:\n  topic: okta_load\n  transport: grpc\n  \
             endpoint: http://dfe-loader:6000\n",
        )
        .expect("an engine-rendered grpc config loads");
        assert_eq!(direct.source.transport, crate::config::Transport::Direct);
        assert_eq!(direct.sink.transport, crate::config::Transport::Direct);
        direct
            .validate()
            .expect("and validates as the direct transport");

        let bus = load_file(&FILE_BASE.replace("topics: [in]", "transport: kafka\n  topics: [in]"))
            .expect("an engine-rendered kafka config loads");
        assert_eq!(bus.source.transport, crate::config::Transport::Bus);

        // The file still refuses a name it does not know.
        assert!(
            load_file(&FILE_BASE.replace("topics: [in]", "transport: tcp\n  topics: [in]"))
                .is_err()
        );

        // An alias is a value, not a key, so it is never warned about as unread.
        let aliased = format!(
            "{}  transport: kafka\n",
            FILE_BASE.replace("topics: [in]", "transport: grpc\n  topics: [in]")
        );
        assert!(
            ignored_in(&aliased).is_empty(),
            "{:?}",
            ignored_in(&aliased)
        );

        // Written back, a transport always takes its own name.
        assert_eq!(
            serde_json::to_value(direct.source.transport).unwrap(),
            "direct"
        );
        assert_eq!(serde_json::to_value(bus.source.transport).unwrap(), "bus");
    }

    /// Collect the ignored keys a file would be warned about.
    fn ignored_in(body: &str) -> Vec<String> {
        let loaded = scalo::yaml::from_str_reporting::<Config>(
            body,
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("the body parses");
        ignored_keys(&loaded.unknown_keys)
            .into_iter()
            .map(String::from)
            .collect()
    }

    /// A typo costs the whole setting and serde says nothing, so the warning is
    /// the only thing between a misspelled key and an instance that idles for
    /// no stated reason.
    #[test]
    fn a_misspelled_key_is_reported() {
        // Written out in full rather than appended to FILE_BASE, which ends
        // inside the `sink:` block -- an appended key would nest under the
        // wrong section.
        let ignored = ignored_in(
            "source:\n  name: filebeat.okta.default\n  topic: singular\n  \
             topics: [in]\n  group_id: g\n  brokers: [b:9092]\nsink:\n  topic: out\n",
        );
        assert_eq!(ignored, ["source.topic"], "got {ignored:?}");
    }

    /// The other half: every key the service DOES read must stay silent, or the
    /// warning trains an operator to ignore it.
    #[test]
    fn a_config_the_service_reads_warns_about_nothing() {
        assert_eq!(ignored_in(FILE_BASE), [] as [std::string::String; 0]);
        assert_eq!(
            ignored_in(&crate::deployment::default_config_yaml()),
            [] as [std::string::String; 0]
        );
    }

    /// `sink.brokers` is an `Option` that defaults to `None`, and a broker list a
    /// file sets is read, so it is not reported as ignored.
    #[test]
    fn an_optional_field_a_file_sets_is_not_called_ignored() {
        let ignored = ignored_in(&format!("{FILE_BASE}  brokers: [b:9092]\n"));
        assert!(ignored.is_empty(), "got {ignored:?}");
    }

    /// The refusal must not fire on the configuration the service actually
    /// ships, or every deployment fails to start.
    #[test]
    fn the_shipped_example_is_not_refused() {
        let loaded =
            load_file(&crate::deployment::default_config_yaml()).expect("the example loads");
        assert_eq!(loaded.source.batch_size, 20_000);
    }
}
