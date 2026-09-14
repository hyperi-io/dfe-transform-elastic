// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Startup smoke test: does the service come up with the config it ships?
//!
//! Everything `run` does before it touches a broker, in order -- load the
//! mounted config, validate it, resolve the transform for the configured
//! source, register the metrics, build the envelope resolver, and put a batch
//! through to the bytes a producer would send. A regression in any of those is
//! a pod that starts and dies, which is what a deploy finds out and a test
//! should have.
//!
//! The Kafka transports themselves are not built here; the broker round trip in
//! `tests/broker.rs` covers those.

// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_transform_elastic::config::Config;
use dfe_transform_elastic::envelope::Resolver;
use dfe_transform_elastic::metrics::TransformMetrics;
use dfe_transform_elastic::pipeline::{parse_batch, serialise_chunks, transform_batch_resolved};
use dfe_transform_elastic::registry;

/// The file the chart mounts and the image points `--config` at.
fn shipped_config() -> Config {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/config.example.yaml");
    Config::load(Some(path)).expect("the shipped config loads")
}

/// One event for the shipped source, Beats-wrapped: the raw vendor payload as a
/// string in `message`, which is how Beats delivers it.
fn shipped_source_event() -> Vec<u8> {
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/okta/system/test-okta-system-events.log"
    ))
    .expect("the okta fixture is committed");
    let line = raw
        .lines()
        .find(|l| !l.trim().is_empty())
        .expect("the okta fixture has a line");

    let beat = serde_json::json!({ "message": line });
    let mut out = serde_json::to_vec(&beat).expect("the event serialises");
    out.push(b'\n');
    out
}

/// The whole startup path, up to the first `recv`.
#[test]
fn the_service_starts_on_the_shipped_config() {
    let config = shipped_config();
    config.validate().expect("the shipped config validates");
    assert_eq!(
        config.source.name, "filebeat.okta.default",
        "the fixture below is for this source"
    );

    let source = &config.source.name;
    let transform = registry::lookup(source).expect("the configured source has a transform");
    let intake = registry::intake(source).expect("the configured source has an intake");
    let dataset = registry::dataset(source).expect("the configured source has a dataset");

    let manager = scalo::metrics::MetricsManager::new("transform_elastic_smoke");
    let _metrics = TransformMetrics::register(&manager, "0.0.0-smoke", "smoke");

    let resolver = Resolver::new(config.source.envelope, intake, dataset);

    let (events, parsed) = parse_batch(&shipped_source_event());
    assert_eq!(parsed.parsed, 1, "the seeded line must parse");
    assert_eq!(parsed.bad_lines, 0);

    let (out, outcome, envelopes) = transform_batch_resolved(transform, &resolver, events);
    assert_eq!(outcome.errored, 0, "the transform raised on a clean event");
    assert_eq!(outcome.emitted, 1);
    assert!(
        !envelopes.unaccepted,
        "the shipped source refused its own envelope"
    );

    // `ecs.version` is stamped on the transform's first line, and the `okta.*`
    // namespace only exists once `message` has been unpacked -- together they
    // say the transform ran rather than merely being called.
    let event = out.first().expect("one event out");
    assert_eq!(event.get_str("ecs.version"), Some("8.11.0"));
    assert!(event.has("okta.event_type"), "the payload was not unpacked");

    let (chunks, serialised) = serialise_chunks(&out, config.sink.max_message_bytes);
    assert_eq!(serialised.serialised, 1);
    assert_eq!(serialised.oversize, 0);
    assert_eq!(chunks.len(), 1);
}

/// `GeoIP` is resolved lazily on the first lookup, and a deployment with no
/// database must degrade to empty fields rather than failing to start.
#[test]
fn a_geoip_lookup_without_a_database_is_empty_rather_than_fatal() {
    // Forces the readers open, which is what the service reports at startup.
    // Either answer is a pass; a panic is not.
    let _ = dfe_runtime::enrichment::geoip_global::enabled();

    // A private address has no data in any database, so this is the one lookup
    // whose answer does not depend on which MMDB happens to be on the box.
    let fields = dfe_runtime::enrichment::geoip_global::geoip_lookup("geoip_city", "10.0.0.1");
    assert!(
        fields.is_empty(),
        "a private address must enrich to nothing"
    );
}
