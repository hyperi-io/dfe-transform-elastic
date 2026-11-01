// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `Config::load` hands the named file to scalo's cascade as its settings
//! layer, so a `logger.level` and a `scaling` block written in it take effect
//! where scalo reads them. The cascade is set once per process, so this test
//! has a process of its own under `cargo test` as well as nextest.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_transform_elastic::config::Config;

#[test]
fn a_config_file_is_a_layer_of_the_scalo_cascade() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("config.yaml");
    std::fs::write(
        &path,
        "source:\n  name: filebeat.okta.default\n  topics: [in]\n  group_id: g\n  \
         brokers: [b:9092]\nsink:\n  topic: out\nlogger:\n  level: trace\n\
         scaling:\n  enabled: false\n  memory_gate_threshold: 0.5\n",
    )
    .expect("write config");
    Config::load(Some(path.to_str().expect("utf-8 path"))).expect("the file loads");

    let cascade = scalo::config::try_get().expect("load installs the cascade");
    assert!(
        cascade.contains("scaling"),
        "the file did not reach the cascade"
    );

    let logger = scalo::logger::LoggerSettings::from_cascade().unwrap();
    assert_eq!(
        logger.level.as_deref(),
        Some("trace"),
        "logger.level came from the default"
    );

    let scaling = scalo::scaling::ScalingPressureConfig::from_cascade().unwrap();
    assert!(!scaling.enabled, "scaling.enabled came from the default");
    assert!(
        (scaling.memory_gate_threshold - 0.5).abs() < f64::EPSILON,
        "scaling.memory_gate_threshold came from the default"
    );
}
