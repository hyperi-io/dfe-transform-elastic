// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The deployment contract: one description of the container, from which
//! scalo generates the Dockerfile, Helm chart, compose fragment and KEDA
//! scaler.
//!
//! The image is just this Rust binary. There is no sidecar and no interpreter
//! -- the transforms are compiled in, which is the whole design.

use scalo::deployment::{
    Capability, DeploymentContract, FieldSpec, HealthContract, ImageProfile, KedaConfig,
    KedaContract, NativeDepsContract, SecretEnvContract, SecretGroupContract,
    base_image_from_cascade,
};

/// Build the deployment contract for dfe-transform-elastic.
#[must_use]
pub fn contract() -> DeploymentContract {
    // One cascade-resolved base image drives both the runtime FROM and the
    // native-deps codename, so librdkafka packages match the base distro.
    let base_image = base_image_from_cascade();
    DeploymentContract {
        app_name: "dfe-transform-elastic".into(),
        binary_name: "dfe-transform-elastic".into(),
        description: "Beats and Elastic Agent JSON in, DFE-normalised events out".into(),
        metrics_port: 9090,
        health: HealthContract {
            liveness_path: "/livez".into(),
            readiness_path: "/readyz".into(),
            metrics_path: "/metrics".into(),
        },
        env_prefix: "DFE_TRANSFORM_ELASTIC".into(),
        metric_prefix: "transform_elastic".into(),
        config_mount_path: "/etc/dfe-transform-elastic/config.yaml".into(),
        image_registry: "ghcr.io/hyperi-io".into(),
        base_image: base_image.clone(),
        // No extra ports. scalo serves /livez, /readyz, /metrics and
        // /scaling/pressure from the ONE metrics listener, so a second
        // declared port would be exposed with nothing behind it.
        extra_ports: vec![],
        entrypoint_args: vec![
            "--config".into(),
            "/etc/dfe-transform-elastic/config.yaml".into(),
        ],
        secrets: vec![SecretGroupContract {
            group_name: "kafka".into(),
            env_vars: vec![
                SecretEnvContract {
                    env_var: "KAFKA_SASL_USERNAME".into(),
                    key_name: "username".into(),
                    secret_key: "kafka-username".into(),
                },
                SecretEnvContract {
                    env_var: "KAFKA_SASL_PASSWORD".into(),
                    key_name: "password".into(),
                    secret_key: "kafka-password".into(),
                },
            ],
        }],
        default_config: Some(serde_json::json!({
            "source": {
                "name": "filebeat.okta.default",
                // `auto` reads the family off each event, which is what the
                // code defaults to. Naming one instead pins it: `receiver`
                // reads dfe-receiver's output and `fetcher` dfe-fetcher's.
                // Which a source accepts is in the capability catalogue, and a
                // wrong one is refused at startup.
                "envelope": "auto",
                "topics": ["raw_events"],
                // Batch-first default: amortises commit, allocation and SIMD setup.
                "batch_size": 20000,
                // The inbound mirror of `sink.max_message_bytes`: 20,000
                // records of unbounded size is unbounded memory, so the fetch
                // is capped in bytes as well as in records.
                "max_batch_bytes": 16_777_216,
                "group_id": "dfe-transform-elastic",
                "brokers": ["kafka:9092"]
            },
            "sink": {
                "topic": "normalised_events",
                "brokers": ["kafka:9092"],
                // A batch is split into as many records as this allows.
                // librdkafka's producer ceiling is 1,000,000; the rest is
                // headroom for the key, headers and framing.
                "max_message_bytes": 900_000
            },
            // scalo provisions the MMDB databases, re-downloading a file older
            // than `max_age_days` and keeping the stale copy when a provider is
            // down. `/var/lib/dfe/geoip` is dfe-loader's directory too, so one
            // volume serves both stages.
            "geoip": {
                "enabled": true,
                "provider": "db_ip_lite",
                "auto_download": {
                    "enabled": true,
                    "data_dir": "/var/lib/dfe/geoip",
                    "max_age_days": 30
                }
            },
            // No `health`, `metrics` or `scaling` section here: a key this
            // service never reads is a knob that silently does nothing.
            // scalo's `--metrics-addr` (env `METRICS_ADDR`) owns the listener,
            // and it resolves `scaling` from its config cascade, which the
            // file this contract ships is not a layer of. Scaling runs on
            // `ScalingPressureConfig::default()` with the components
            // registered by `ServiceApp::scaling_components`; move a gate
            // threshold with `DFE_TRANSFORM_ELASTIC_SCALING__*`.
        })),
        depends_on: vec!["kafka".into()],
        native_deps: NativeDepsContract::for_scalo_features(&["transport-kafka"], &base_image),
        image_profile: ImageProfile::Production,
        // KedaContract is #[non_exhaustive], so it is built from a KedaConfig.
        // The default leaves the scaling_pressure_* trigger off; its Prometheus
        // serverAddress is cluster-specific.
        keda: Some(KedaContract::from_config(&KedaConfig {
            min_replicas: 1,
            max_replicas: 10,
            polling_interval: 15,
            cooldown_period: 300,
            // One batch is 20k events, so the threshold sits an order of
            // magnitude above it -- KEDA reacts to a backlog, not to batching.
            kafka_lag_threshold: 200_000,
            activation_lag_threshold: 0,
            cpu_enabled: true,
            cpu_threshold: 80,
            ..Default::default()
        })),
        schema_version: 3,
        oci_labels: scalo::deployment::OciLabels {
            title: "dfe-transform-elastic".into(),
            description: "Beats and Elastic Agent JSON in, DFE-normalised events out".into(),
            licenses: "BUSL-1.1".into(),
            copyright: "(c) 2026 HYPERI PTY LIMITED".into(),
            ..Default::default()
        },
        config_schema: Some(scalo::deployment::config_schema_json::<crate::config::Config>()),
        capabilities: capabilities(),
    }
}

/// The shipped default configuration as YAML, with a header saying where it
/// came from.
///
/// Rendered from the contract rather than hand-written, so the committed
/// `config.example.yaml` cannot drift from what a deployment actually gets.
#[must_use]
pub fn default_config_yaml() -> String {
    let header = "\
# dfe-transform-elastic example configuration.
#
# AUTOGENERATED -- do not edit by hand.
# Rendered from the deployment contract, which is also what the Helm chart
# ships as its ConfigMap.
# Regenerate with: `dfe-transform-elastic emit-config > config.example.yaml`
#
# Every value here is the default. Override what you need; `source.name`,
# `source.topics`, `source.brokers` and `sink.topic` are the ones that
# always change.
#
# `source.envelope` selects which transport delivered the payload. `auto`, the
# default, reads it off each event, so a topic fed by two producers still
# unwraps both. Naming one pins it: `beats`, `receiver` (also spelled `syslog`)
# for dfe-receiver's output, or `fetcher` for dfe-fetcher's. The transform is
# the same for all of them; only the unwrapping differs. Which envelopes a
# source accepts is listed per source in the capability catalogue, and an
# envelope it cannot arrive in is refused at startup rather than at the first
# batch.
#
# `source.max_batch_bytes` caps one inbound fetch, the way
# `sink.max_message_bytes` caps one outbound record. Raise the pod's memory
# limit with it: the parsed documents are several times the raw bytes.
#
# `geoip` provisions the MMDB databases at startup and refreshes them when the
# local copy passes `max_age_days`. Mount `data_dir` on a volume that survives
# a restart, or each new pod downloads again. To supply the files yourself, set
# `geoip.city_db_path` and `geoip.asn_db_path`; to run without enrichment, set
# `geoip.enabled: false`.
";

    let body = contract()
        .default_config
        .as_ref()
        .and_then(|value| serde_yaml_ng::to_string(value).ok())
        .unwrap_or_default();

    format!("{header}\n{body}")
}

/// Point the generated KEDA trigger at this service's config shape.
///
/// scalo's Helm generator hard-codes `.Values.config.kafka.*` for the Kafka
/// scaler, and this service configures its broker list under
/// `.Values.config.source.*`. Without the rewrite the template fails
/// `helm lint` on a nil pointer, so `emit-chart` applies it after generating.
///
/// # Errors
///
/// Returns [`crate::Error::Config`] if the generated template cannot be read
/// or rewritten.
pub fn retarget_keda_trigger(chart_dir: &str) -> crate::Result<()> {
    let path = std::path::Path::new(chart_dir)
        .join("templates")
        .join("keda-scaledobject.yaml");

    let Ok(template) = std::fs::read_to_string(&path) else {
        // No KEDA template means no trigger to retarget.
        return Ok(());
    };

    let patched = template
        .replace(
            "{{ .Values.config.kafka.brokers | quote }}",
            "{{ join \",\" .Values.config.source.brokers | quote }}",
        )
        .replace(
            ".Values.config.kafka.group_id",
            ".Values.config.source.group_id",
        )
        .replace(
            ".Values.config.kafka.topics",
            ".Values.config.source.topics",
        );

    if patched.contains(".Values.config.kafka.") {
        return Err(crate::Error::Config(format!(
            "{} still references .Values.config.kafka after the rewrite; \
             the scalo Helm generator has changed shape",
            path.display()
        )));
    }

    std::fs::write(&path, patched)
        .map_err(|e| crate::Error::Config(format!("failed to write {}: {e}", path.display())))
}

/// Capability catalogue: the transform sources this build can run.
///
/// The child list is generated from [`crate::registry`], so it cannot drift
/// from what the binary actually accepts -- adding a transform to the registry
/// adds it here.
fn capabilities() -> Vec<Capability> {
    let sources = crate::registry::sources().map(|name| {
        // The intakes are the operator-facing fact: which transports can carry
        // this source to us, not just the one Elastic ships.
        let envelopes = crate::registry::intake(name).map_or_else(String::new, |intake| {
            intake
                .envelopes
                .iter()
                .map(|e| format!("{e:?}").to_lowercase())
                .collect::<Vec<_>>()
                .join(", ")
        });
        Capability::service(name)
            .description(format!(
                "Compiled transform. Accepts the envelopes: {envelopes}."
            ))
            .maturity("beta")
    });

    vec![
        Capability::new("transform", "elastic")
            .description(
                "Elastic ingest-pipeline logic compiled to native Rust: field extraction, \
                 mutation and enrichment applied per event, selected by source name.",
            )
            .maturity("beta")
            .field(
                FieldSpec::string("source.name")
                    .required()
                    .description("Which compiled transform to apply, e.g. filebeat.okta.default."),
            )
            .field(
                FieldSpec::enumeration(
                    "source.envelope",
                    ["auto", "beats", "receiver", "syslog", "fetcher"],
                )
                .default_value("auto")
                .description(
                    "How the payload is wrapped on the way in. `auto` reads it off each \
                     event; `receiver` (spelled `syslog` before it grew the other \
                     transports) reads dfe-receiver's output and applies only to device \
                     sources; `fetcher` reads dfe-fetcher's and applies only to sources \
                     Elastic pulls from an API.",
                ),
            )
            .children(sources),
        Capability::source("kafka")
            .description("Kafka consumer and producer via the scalo transport.")
            .maturity("stable")
            .field(
                FieldSpec::list("source.topics")
                    .required()
                    .description("Topics to consume."),
            )
            .field(
                FieldSpec::string("sink.topic")
                    .required()
                    .description("Topic the normalised events are produced to."),
            )
            .field(
                FieldSpec::int("source.max_batch_bytes")
                    .default_value(16_777_216)
                    .description(
                        "Ceiling on the bytes one fetch brings back, applied as \
                         librdkafka's fetch.max.bytes. The pod's memory limit has to \
                         cover this plus the parsed documents, which are several times \
                         larger.",
                    ),
            ),
        Capability::new("enrichment", "geoip")
            .description(
                "City and ASN lookups for the geoip processors 14 of the source pipelines \
                 carry. scalo downloads and refreshes the MMDB files; the lookup runs \
                 in-process behind a bounded cache. A database that cannot be obtained \
                 leaves the geo fields empty and never stops the service.",
            )
            .maturity("beta")
            .field(
                FieldSpec::bool("geoip.enabled")
                    .default_value(true)
                    .description("Provision databases at all."),
            )
            .field(
                FieldSpec::enumeration(
                    "geoip.provider",
                    [
                        "db_ip_lite",
                        "max_mind_geo_lite2",
                        "ip_locate",
                        "ip_info_lite",
                        "sapics",
                        "custom",
                    ],
                )
                .default_value("db_ip_lite")
                .description(
                    "Where the databases come from. Only db_ip_lite and max_mind_geo_lite2 \
                     publish both city and ASN.",
                ),
            )
            .field(FieldSpec::string("geoip.city_db_path").description(
                "Mounted city MMDB. Setting either path bypasses the provider and \
                     downloads nothing.",
            ))
            .field(
                FieldSpec::string("geoip.asn_db_path")
                    .description("Mounted ASN MMDB. See geoip.city_db_path."),
            )
            .field(
                FieldSpec::string("geoip.auto_download.data_dir")
                    .default_value("/var/lib/dfe/geoip")
                    .description("Directory the downloaded databases are written to."),
            )
            .field(
                FieldSpec::int("geoip.auto_download.max_age_days")
                    .default_value(30)
                    .description("Age past which a local database is re-downloaded."),
            )
            .field(
                FieldSpec::secret("geoip.auto_download.maxmind_license_key").description(
                    "Required by the max_mind_geo_lite2 provider, with the account id.",
                ),
            ),
    ]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn contract_identity() {
        let c = contract();
        assert_eq!(c.app_name, "dfe-transform-elastic");
        assert_eq!(c.binary_name, "dfe-transform-elastic");
        assert_eq!(c.env_prefix, "DFE_TRANSFORM_ELASTIC");
        assert_eq!(c.metric_prefix, "transform_elastic");
    }

    /// The env prefix in the contract is what the chart stamps into the pod,
    /// and `config::ENV_PREFIX` is what the binary reads. A drift between them
    /// leaves every env-var override silently ignored.
    #[test]
    fn contract_env_prefix_matches_the_config_cascade() {
        assert_eq!(contract().env_prefix, crate::config::ENV_PREFIX);
    }

    /// `--config` must point at the same path the chart mounts, or the pod
    /// starts and dies on "config file not found".
    #[test]
    fn entrypoint_config_arg_matches_the_mount_path() {
        let c = contract();
        assert_eq!(
            c.config_mount_path,
            "/etc/dfe-transform-elastic/config.yaml"
        );
        let idx = c
            .entrypoint_args
            .iter()
            .position(|a| a == "--config")
            .expect("entrypoint_args must carry --config");
        assert_eq!(
            c.entrypoint_args.get(idx + 1).map(String::as_str),
            Some(c.config_mount_path.as_str())
        );
    }

    /// The shipped default config must survive this service's own validation.
    /// A default that cannot start is worse than no default.
    #[test]
    fn default_config_is_a_config_this_service_accepts() {
        let value = contract().default_config.expect("default_config present");
        let config: crate::config::Config =
            serde_json::from_value(value).expect("default_config must deser as Config");
        config.validate().expect("default_config must validate");
        assert_eq!(config.source.batch_size, 20_000);
    }

    /// Nothing this repo ships can set the scaling gate, so scalo's defaults
    /// ARE the deployed values and a change to them moves our gate silently.
    /// Pin them here, where a scalo upgrade has to acknowledge the move.
    #[test]
    fn scaling_runs_on_scalo_defaults() {
        let pressure = scalo::scaling::ScalingPressureConfig::default();
        assert!(pressure.enabled);
        assert!((pressure.memory_gate_threshold - 0.8).abs() < f64::EPSILON);
    }

    /// Everything is served from the one metrics listener, so the contract
    /// must not declare a second port with nothing behind it.
    #[test]
    fn health_is_served_from_the_metrics_port_alone() {
        let c = contract();
        assert_eq!(c.health.liveness_path, "/livez");
        assert_eq!(c.health.readiness_path, "/readyz");
        assert_eq!(c.health.metrics_path, "/metrics");
        assert_eq!(c.metrics_port, 9090);
        assert!(c.extra_ports.is_empty());
    }

    /// The shipped config must not carry keys this service never reads: a
    /// knob that silently does nothing is worse than no knob.
    ///
    /// The contract is delivered as the `--config` file, so every section in
    /// [`crate::config::CASCADE_ONLY_SECTIONS`] is dead here by construction.
    /// Checking that list rather than a hand-typed set means a new scalo
    /// section is covered by naming it in one place.
    #[test]
    fn default_config_has_no_keys_the_service_ignores() {
        let cfg = contract().default_config.expect("default_config present");
        let object = cfg.as_object().expect("default_config is an object");

        for dead in ["health", "retry", "enrichment", "transforms"] {
            assert!(
                !object.contains_key(dead),
                "`{dead}` is in the shipped config but nothing reads it"
            );
        }
        for dead in crate::config::CASCADE_ONLY_SECTIONS {
            assert!(
                !object.contains_key(*dead),
                "`{dead}` is in the shipped config, but scalo reads it from the cascade and \
                 `--config` is not a cascade layer, so it would be ignored"
            );
        }
    }

    /// The shipped config must also be one `Config::load` accepts through the
    /// `--config` path, which is stricter than deserialising it.
    #[test]
    fn the_shipped_config_survives_the_config_file_path() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("config.yaml");
        std::fs::write(&path, default_config_yaml()).expect("write config");
        crate::config::Config::load(Some(path.to_str().expect("utf-8 path")))
            .expect("the shipped config must load through --config");
    }

    /// The catalogue's enumeration is what an operator reads before writing a
    /// value, so a variant missing from it reads as unsupported. `syslog` is in
    /// the list as well because it is a serde alias the old configs still use.
    #[test]
    fn the_envelope_enumeration_names_every_setting() {
        use crate::envelope::EnvelopeSetting;

        let listed = contract()
            .capabilities
            .iter()
            .find(|cap| cap.name == "elastic")
            .and_then(|cap| {
                cap.fields
                    .iter()
                    .find(|field| field.name == "source.envelope")
                    .map(|field| field.enum_values.clone())
            })
            .expect("source.envelope is enumerated");

        for setting in [
            EnvelopeSetting::Auto,
            EnvelopeSetting::Beats,
            EnvelopeSetting::Receiver,
            EnvelopeSetting::Fetcher,
        ] {
            let name = serde_json::to_value(setting).expect("a setting serialises");
            let name = name.as_str().expect("a setting is a string");
            assert!(listed.iter().any(|v| v == name), "{name} is not offered");
        }
    }

    /// Every enumerated value has to be one the config actually accepts, or the
    /// catalogue offers a value that fails at startup.
    #[test]
    fn every_enumerated_envelope_parses() {
        for name in ["auto", "beats", "receiver", "syslog", "fetcher"] {
            let parsed: crate::envelope::EnvelopeSetting =
                serde_json::from_value(serde_json::json!(name)).expect("an offered value parses");
            let _ = parsed.pinned();
        }
    }

    #[test]
    fn keda_is_configured_for_batch_sized_lag() {
        let c = contract();
        let keda = c.keda.as_ref().expect("keda present");
        assert_eq!(keda.min_replicas, 1);
        assert_eq!(keda.max_replicas, 10);
        assert!(
            keda.kafka_lag_threshold > 20_000,
            "lag threshold must sit above one batch, or KEDA scales on normal batching"
        );
        assert!(keda.cpu_enabled);
    }

    /// The capability catalogue is generated from the registry, so it must
    /// list every source the binary accepts -- no more, no less.
    #[test]
    fn capabilities_list_every_registered_source() {
        let c = contract();
        let transform = c
            .capabilities
            .iter()
            .find(|cap| cap.name == "elastic")
            .expect("elastic transform capability");

        let listed: Vec<&str> = transform
            .children
            .iter()
            .map(|child| child.name.as_str())
            .collect();
        let registered: Vec<&str> = crate::registry::sources().collect();

        assert_eq!(listed, registered);
    }

    #[test]
    fn contract_carries_the_reflectable_config() {
        let c = contract();
        assert!(c.config_schema.is_some());
        assert_eq!(c.schema_version, 3);
        assert!(!c.capabilities.is_empty());
    }

    /// The committed `config-schema.*` and `capability-catalog.*` under
    /// `docs/` must match a fresh regen. Refresh with
    /// `dfe-transform-elastic config-schema --dir docs`.
    #[test]
    fn committed_config_artefacts_do_not_drift() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs");
        scalo::deployment::assert_no_config_artifact_drift(&contract(), dir);
    }

    /// The committed `config.example.yaml` must match a fresh emit. Refresh
    /// with `dfe-transform-elastic emit-config > config.example.yaml`.
    #[test]
    fn committed_config_example_does_not_drift() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("config.example.yaml");
        let committed = std::fs::read_to_string(&path).expect("config.example.yaml is committed");
        assert_eq!(committed.trim_end(), default_config_yaml().trim_end());
    }

    /// The example must be a config this service accepts, not just valid YAML.
    #[test]
    fn the_config_example_validates() {
        let yaml = default_config_yaml();
        let config: crate::config::Config =
            serde_yaml_ng::from_str(&yaml).expect("example parses as Config");
        config.validate().expect("example validates");
    }

    /// The committed `Dockerfile` must match a fresh emit. Refresh with
    /// `dfe-transform-elastic emit-dockerfile > Dockerfile`.
    #[test]
    fn committed_dockerfile_does_not_drift() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Dockerfile");
        let committed = std::fs::read_to_string(&path).expect("Dockerfile is committed");
        let fresh = scalo::deployment::generate_dockerfile(&contract(), None);
        assert_eq!(committed.trim_end(), fresh.trim_end());
    }

    /// The generated chart must lint, which means the KEDA trigger has to
    /// reference value paths that exist in `values.yaml`.
    #[test]
    fn keda_retarget_removes_every_config_kafka_reference() {
        let dir = tempfile::tempdir().expect("tempdir");
        let chart = dir.path().to_str().expect("utf-8 path");

        scalo::deployment::generate_chart(&contract(), chart, None).expect("chart generates");
        retarget_keda_trigger(chart).expect("retarget succeeds");

        let template = std::fs::read_to_string(dir.path().join("templates/keda-scaledobject.yaml"))
            .expect("keda template written");

        assert!(!template.contains(".Values.config.kafka."));
        assert!(template.contains("join \",\" .Values.config.source.brokers"));
        assert!(template.contains(".Values.config.source.group_id"));
        assert!(template.contains(".Values.config.source.topics"));
    }

    /// The chart directory, which is committed alongside the generator output.
    fn chart_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("chart/dfe-transform-elastic")
    }

    /// The chart's `config:` block IS the contract's default config, and a
    /// drifted one ships a `ConfigMap` the binary refuses at startup. The
    /// Dockerfile and `config.example.yaml` already have this guard; the chart
    /// did not. Refresh with `dfe-transform-elastic emit-chart`.
    #[test]
    fn the_chart_config_block_matches_the_contract() {
        let text = std::fs::read_to_string(chart_dir().join("values.yaml"))
            .expect("the chart values are committed");
        let values: serde_json::Value =
            serde_yaml_ng::from_str(&text).expect("values.yaml parses as YAML");

        let contract = contract();
        assert_eq!(
            values.get("config"),
            contract.default_config.as_ref(),
            "chart values.yaml has drifted from the contract"
        );
    }

    /// librdkafka only presents SASL credentials when the protocol names a SASL
    /// mechanism, so a chart that never stamps the protocol connects
    /// anonymously in the clear with the secret mounted and unused.
    #[test]
    fn the_chart_stamps_the_kafka_wire_protocol() {
        let values = std::fs::read_to_string(chart_dir().join("values.yaml"))
            .expect("the chart values are committed");
        let deployment = std::fs::read_to_string(chart_dir().join("templates/deployment.yaml"))
            .expect("the deployment template is committed");

        assert!(values.contains("securityProtocol:"), "no securityProtocol");
        assert!(values.contains("saslMechanism:"), "no saslMechanism");
        for stamped in ["KAFKA_SECURITY_PROTOCOL", "KAFKA_SASL_MECHANISM"] {
            assert!(
                deployment.contains(stamped),
                "{stamped} never reaches the pod"
            );
        }
    }

    /// The downloader writes to `geoip.auto_download.data_dir` under a
    /// read-only root filesystem, so that path needs a writable volume or 14 of
    /// the source pipelines enrich to nothing and nothing says so.
    #[test]
    fn the_chart_gives_the_geoip_downloader_somewhere_to_write() {
        let deployment = std::fs::read_to_string(chart_dir().join("templates/deployment.yaml"))
            .expect("the deployment template is committed");

        let data_dir = contract()
            .default_config
            .as_ref()
            .and_then(|c| c.pointer("/geoip/auto_download/data_dir").cloned())
            .and_then(|d| d.as_str().map(str::to_string))
            .expect("the contract names a geoip data_dir");

        assert!(
            deployment.contains(&data_dir) || deployment.contains("auto_download.data_dir"),
            "nothing mounts {data_dir}"
        );
        assert!(deployment.contains("emptyDir"), "no writable volume at all");
    }

    /// A chart directory with no KEDA template is not an error.
    #[test]
    fn keda_retarget_is_a_no_op_without_a_template() {
        let dir = tempfile::tempdir().expect("tempdir");
        let chart = dir.path().to_str().expect("utf-8 path");
        assert!(retarget_keda_trigger(chart).is_ok());
    }

    #[test]
    fn kafka_credentials_come_from_a_secret_not_the_config() {
        let c = contract();
        assert_eq!(c.secrets.len(), 1);
        assert_eq!(c.secrets[0].group_name, "kafka");
        assert_eq!(c.secrets[0].env_vars.len(), 2);
    }
}
