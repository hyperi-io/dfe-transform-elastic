// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Reading the configuration: an explicit file, or scalo's cascade.
//!
//! The two are not equivalent, and the module docs on the parent say why. What
//! lives here is the consequence: a `--config` file is read directly, so a
//! section scalo resolves for itself reaches nothing when written there and is
//! warned about rather than refused.

use scalo::config::flat_env::{ApplyFlatEnv, flat_env_list, flat_env_parsed, flat_env_string};
use scalo::config::{self, ConfigOptions};

use super::{Config, SinkConfig, SourceConfig};

/// Environment prefix for the config cascade.
pub const ENV_PREFIX: &str = "DFE_TRANSFORM_ELASTIC";

/// Sections that reach NOTHING when written into a `--config` file.
///
/// scalo discovers cascade files by fixed base name -- `defaults.yaml`,
/// `settings.yaml`, `settings.{env}.yaml` -- so a file named with `--config` is
/// not one of its layers. Every section here is resolved by scalo itself, from
/// that cascade or from the environment, and each resolver falls back to the
/// type's `Default` when the key is absent. So the operator gets defaults while
/// believing otherwise.
///
/// The test for membership is "does writing it into the `--config` file reach
/// anything", NOT "does scalo resolve it from the cascade for us". Two entries
/// only make sense under the wider test. `memory` is read by
/// `MemoryGuardConfig::from_env`, not from the cascade, and `batch_processing`
/// is gated on scalo's `worker-batch`, which this binary does not enable --
/// both are inert in that file either way, which is the thing worth saying.
///
/// `geoip` is deliberately absent: it is declared on [`Config`] and handed to
/// `scalo::geoip_download` explicitly, which is what makes it work from a file.
pub const CASCADE_ONLY_SECTIONS: &[&str] = &[
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

/// Warn for every [`CASCADE_ONLY_SECTIONS`] entry a `--config` file carries.
///
/// This used to REFUSE, on the reasoning that an operator who sets
/// `scaling.memory_gate_threshold` has a reason and running on a default they
/// did not choose is worse than not starting. The deployment says otherwise:
/// the infra chart renders `.Values.config` verbatim from whatever dfe-engine
/// publishes, so a section we do not control can appear in that file and a
/// refusal stops the pod over a value that was never ours to police. Warning
/// tells the operator the same thing and leaves the service running, which is
/// what dfe-transform-vrl does.
///
/// Every offending section is reported, not just the first -- a config with
/// three of them should not need three restarts to discover that.
fn warn_unreachable_cascade_sections(path: &str, doc: &serde_yaml_ng::Value) {
    let Some(map) = doc.as_mapping() else {
        return;
    };
    for section in CASCADE_ONLY_SECTIONS {
        if map.contains_key(*section) {
            tracing::warn!(
                section = *section,
                path = path,
                instead = format!("{ENV_PREFIX}_{}__<KEY>", section.to_uppercase()),
                "config section is not applied: scalo resolves it for itself, and a file named \
                 with --config is not one of its cascade layers -- set it in the environment or \
                 delete the section to take scalo's defaults"
            );
        }
    }
}

/// Warn for every key in `doc` that the service does not read.
///
/// serde drops an unknown field in silence, so a typo costs the whole setting
/// with no message -- `source.topic` for `source.topics` leaves the instance
/// idle and says nothing about why.
///
/// The comparison is a ROUND TRIP rather than a list of valid keys: what
/// survives [`Config`] and comes back out is what the service read, so a field
/// added or removed later needs no maintenance here.
///
/// A [`CASCADE_ONLY_SECTIONS`] entry is skipped, having its own warning that
/// names the environment variable which would reach it.
fn warn_ignored_keys(path: &str, doc: &serde_yaml_ng::Value, parsed: &Config) {
    let Ok(kept) = serde_yaml_ng::to_value(parsed) else {
        return;
    };
    let mut ignored = Vec::new();
    collect_ignored(doc, &kept, "", &mut ignored);
    for key in ignored {
        tracing::warn!(
            key = %key,
            path = path,
            "config key is not read by this service, so setting it does nothing"
        );
    }
}

/// Collect dotted paths present in `doc` and absent from `kept`.
fn collect_ignored(
    doc: &serde_yaml_ng::Value,
    kept: &serde_yaml_ng::Value,
    prefix: &str,
    out: &mut Vec<String>,
) {
    let (Some(doc_map), Some(kept_map)) = (doc.as_mapping(), kept.as_mapping()) else {
        return;
    };
    for (key, value) in doc_map {
        let Some(name) = key.as_str() else { continue };
        if prefix.is_empty() && CASCADE_ONLY_SECTIONS.contains(&name) {
            continue;
        }
        let dotted = if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{prefix}.{name}")
        };
        match kept_map.get(key) {
            None => out.push(dotted),
            Some(kept_value) => collect_ignored(value, kept_value, &dotted, out),
        }
    }
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
        let mut config: Self = if let Some(path) = config_path {
            let text = std::fs::read_to_string(path).map_err(|e| {
                crate::Error::Config(format!("config file not found at '{path}': {e}"))
            })?;
            let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
                .map_err(|e| crate::Error::Config(format!("invalid config at '{path}': {e}")))?;
            warn_unreachable_cascade_sections(path, &doc);
            let parsed: Self = serde_yaml_ng::from_value(doc.clone())
                .map_err(|e| crate::Error::Config(format!("invalid config at '{path}': {e}")))?;
            warn_ignored_keys(path, &doc, &parsed);
            parsed
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
/// This is the one override that reaches a `--config` deployment, because scalo
/// resolves its own cascade from the double-underscore form and a named file is
/// not one of those layers.
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

impl ApplyFlatEnv for SourceConfig {
    fn apply_flat_env(&mut self, prefix: &str) {
        if let Some(name) = flat_env_string(prefix, "NAME") {
            self.name = name;
        }
        if let Some(raw) = flat_env_string(prefix, "TRANSPORT") {
            self.transport = parse_transport(&raw, self.transport);
        }
        if let Some(listen) = flat_env_string(prefix, "LISTEN") {
            self.listen = listen;
        }
        // An unknown name keeps the configured envelope rather than failing the
        // load, because detection is the default and still works.
        if let Some(raw) = flat_env_string(prefix, "ENVELOPE") {
            match serde_yaml_ng::from_str(&raw) {
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

    /// The trap this warning exists to name: the chart passes `--config`, and
    /// scalo reads `scaling` from a cascade that file is not a layer of, so a
    /// threshold written there reaches nothing.
    ///
    /// It LOADS. A surplus section is reported and stepped over, because the
    /// chart renders that file from what dfe-engine publishes and refusing
    /// would stop the pod over a value that was never this service's to police.
    #[test]
    fn a_config_file_carrying_scaling_still_loads() {
        let loaded = load_file(&format!(
            "{FILE_BASE}scaling:\n  enabled: true\n  memory_gate_threshold: 0.9\n"
        ))
        .expect("a surplus cascade section is warned about, not refused");

        // Loading past it must not disturb what the file DOES configure.
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
            CASCADE_ONLY_SECTIONS.len() >= 11,
            "the cascade-only list lost entries: {}",
            CASCADE_ONLY_SECTIONS.len()
        );

        for section in CASCADE_ONLY_SECTIONS {
            let body = format!("{FILE_BASE}{section}:\n  enabled: true\n");
            let loaded = load_file(&body)
                .unwrap_or_else(|e| panic!("`{section}` must be warned about, not refused: {e}"));
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
            !CASCADE_ONLY_SECTIONS.contains(&"geoip"),
            "geoip reaches scalo explicitly, so refusing it would be wrong"
        );
    }

    /// Collect the ignored keys a file would be warned about.
    fn ignored_in(body: &str) -> Vec<String> {
        let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(body).expect("the body parses");
        let parsed: Config = serde_yaml_ng::from_value(doc.clone()).expect("it deserialises");
        let kept = serde_yaml_ng::to_value(&parsed).expect("it re-serialises");
        let mut out = Vec::new();
        collect_ignored(&doc, &kept, "", &mut out);
        out
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

    /// The other half, and the one a round trip can get wrong: every key the
    /// service DOES read must stay silent, or the warning trains an operator to
    /// ignore it.
    #[test]
    fn a_config_the_service_reads_warns_about_nothing() {
        assert!(ignored_in(FILE_BASE).is_empty());
        assert!(ignored_in(&crate::deployment::default_config_yaml()).is_empty());
    }

    /// `sink.brokers` is an `Option` that defaults to `None`. A round trip that
    /// omitted it rather than writing null would report a broker list the
    /// service had just read as ignored.
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
