// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Detection against the shapes the producers actually write.
//!
//! Every file under `tests/envelopes/` is one wrapper, read off the
//! producer's own tests or off the compat corpus and carrying the file it came
//! from. Adding a shape is adding a file: this test walks the directory, so a
//! producer that grows a transport cannot be forgotten here.
//!
//! A fixture states two things. `shape` is what the event IS, and `detect` is
//! what detection must return -- they differ exactly where a producer writes no
//! marker, which is how the blind spots stay recorded rather than looking like
//! failures.

// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

use dfe_transform_elastic::envelope::{self, Envelope, EnvelopeSetting};
use dfe_transform_elastic::registry;
use serde_json::Value;

struct Fixture {
    path: PathBuf,
    doc: Value,
}

impl Fixture {
    fn family(&self, section: &str) -> Envelope {
        let name = self.doc[section]["family"]
            .as_str()
            .unwrap_or_else(|| panic!("{}: {section}.family is missing", self.name()));
        match name {
            "beats" => Envelope::Beats,
            "receiver" => Envelope::Receiver,
            "fetcher" => Envelope::Fetcher,
            other => panic!("{}: unknown family `{other}`", self.name()),
        }
    }

    fn variant(&self, section: &str) -> &str {
        self.doc[section]["variant"]
            .as_str()
            .unwrap_or_else(|| panic!("{}: {section}.variant is missing", self.name()))
    }

    fn event(&self) -> dfe_runtime::Event {
        dfe_runtime::Event::new(self.doc["event"].clone())
    }

    fn name(&self) -> String {
        let dir = self.path.parent().and_then(Path::file_name);
        match (dir, self.path.file_name()) {
            (Some(d), Some(f)) => format!("{}/{}", d.to_string_lossy(), f.to_string_lossy()),
            _ => self.path.display().to_string(),
        }
    }
}

fn fixtures() -> Vec<Fixture> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/envelopes");
    let mut found = Vec::new();

    let families = std::fs::read_dir(&root).expect("tests/envelopes exists");
    for family in families {
        let family = family.expect("directory entry reads").path();
        if !family.is_dir() {
            continue;
        }
        let entries = std::fs::read_dir(&family).expect("family directory reads");
        for entry in entries {
            let path = entry.expect("directory entry reads").path();
            if path.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("fixture reads");
            let doc = serde_json::from_str(&text)
                .unwrap_or_else(|e| panic!("{}: not valid JSON: {e}", path.display()));
            found.push(Fixture { path, doc });
        }
    }

    found.sort_by(|a, b| a.path.cmp(&b.path));
    assert!(
        found.len() >= 19,
        "only {} envelope fixtures found -- the directory did not load",
        found.len()
    );
    found
}

/// One fixture by file name, so a transport test names the shape it drives.
fn fixture(name: &str) -> Fixture {
    fixtures()
        .into_iter()
        .find(|f| f.path.file_name().is_some_and(|f| f == name))
        .unwrap_or_else(|| panic!("no fixture named {name}"))
}

/// `name` unwrapped as a source taking the body whole would receive it.
fn unwrapped(name: &str) -> Value {
    let fixture = fixture(name);
    let detected = envelope::detect(&fixture.event());
    let mut event = fixture.event();
    detected
        .family
        .unwrap_into_beats(&mut event, None, &detected.variant)
        .expect("unwraps");
    event.as_value().clone()
}

/// The whole point: every shape a producer writes detects as the fixture says.
#[test]
fn every_recorded_shape_detects_as_its_fixture_says() {
    for fixture in fixtures() {
        let detected = envelope::detect(&fixture.event());
        assert_eq!(
            detected.family,
            fixture.family("detect"),
            "{}: family",
            fixture.name()
        );
        assert_eq!(
            detected.variant,
            fixture.variant("detect"),
            "{}: variant",
            fixture.name()
        );
    }
}

/// A fixture's directory must be the family it declares, or the layout stops
/// telling the truth and a shape can be filed anywhere.
#[test]
fn a_fixture_lives_in_the_directory_its_shape_names() {
    for fixture in fixtures() {
        let dir = fixture
            .path
            .parent()
            .and_then(Path::file_name)
            .expect("fixture has a parent directory")
            .to_string_lossy()
            .into_owned();
        let declared = fixture.doc["shape"]["family"]
            .as_str()
            .expect("shape.family");
        assert_eq!(
            dir,
            declared,
            "{}: filed under the wrong family",
            fixture.name()
        );
    }
}

/// Provenance is the thing that makes these fixtures evidence rather than
/// invention, so an entry without it is not one.
#[test]
fn every_fixture_names_the_file_it_came_from() {
    for fixture in fixtures() {
        let file = fixture.doc["provenance"]["file"].as_str().unwrap_or("");
        assert!(
            !file.trim().is_empty(),
            "{}: provenance.file is empty",
            fixture.name()
        );
    }
}

/// A shape whose `detect` differs from its `shape` is a blind spot, and must
/// fall back to the bare Elastic shape rather than to some other guess.
#[test]
fn an_undetectable_shape_falls_back_to_bare() {
    let mut blind = 0;
    for fixture in fixtures() {
        if fixture.family("shape") == fixture.family("detect")
            && fixture.variant("shape") == fixture.variant("detect")
        {
            continue;
        }
        blind += 1;
        assert_eq!(
            fixture.family("detect"),
            Envelope::Beats,
            "{}: falls back to something other than beats",
            fixture.name()
        );
        assert_eq!(
            fixture.variant("detect"),
            envelope::BARE,
            "{}: falls back to something other than the bare variant",
            fixture.name()
        );
    }
    assert!(
        blind >= 3,
        "the recorded blind spots vanished -- Splunk HEC without metadata, \
         gRPC logs and OTLP's ClickHouse modes all carry no marker"
    );
}

/// Unwrapping a detected shape must not panic on any source that accepts it.
#[test]
fn every_recorded_shape_unwraps_without_panicking() {
    for fixture in fixtures() {
        let detected = envelope::detect(&fixture.event());
        for source in registry::sources_accepting(detected.family) {
            let framing = registry::intake(source).and_then(|i| i.framing);
            let mut event = fixture.event();
            let _ = detected
                .family
                .unwrap_into_beats(&mut event, framing, &detected.variant);
        }
    }
}

/// OTLP's log body is the message and its severity pair is ECS's, so a
/// generic-mode log arrives shaped like every other line.
#[test]
fn otlp_lifts_its_log_onto_ecs() {
    let out = unwrapped("otlp.json");

    assert_eq!(
        out.pointer("/message").and_then(Value::as_str),
        Some("error occurred")
    );
    assert_eq!(
        out.pointer("/log/level").and_then(Value::as_str),
        Some("ERROR")
    );
    assert_eq!(
        out.pointer("/event/severity").and_then(Value::as_i64),
        Some(17)
    );
    assert_eq!(
        out.pointer("/@timestamp").and_then(Value::as_str),
        Some("2026-02-19T00:00:00Z")
    );

    // An empty id and an epoch observed-time are what OTLP writes when it had
    // no value, so neither becomes an ECS field that says otherwise.
    assert_eq!(out.pointer("/trace/id"), None);
    assert_eq!(out.pointer("/span/id"), None);
    assert_eq!(out.pointer("/event/created"), None);

    // The sender's own maps are left where they are.
    assert!(out.pointer("/attributes").is_some());
    assert!(out.pointer("/resource").is_some());
}

/// A flow packet's exporter is an observer and its packet sequence is the
/// event's; the records themselves stay as the collector wrote them.
#[test]
fn a_flow_packet_lifts_its_exporter_and_sequence() {
    let out = unwrapped("netflow.json");

    assert_eq!(
        out.pointer("/observer/ip").and_then(Value::as_str),
        Some("127.0.0.1")
    );
    assert_eq!(
        out.pointer("/event/sequence").and_then(Value::as_i64),
        Some(0)
    );
    assert_eq!(
        out.pointer("/event/created").and_then(Value::as_str),
        Some("2026-05-20T00:00:00Z")
    );
    assert_eq!(
        out.pointer("/flows/0/bytes").and_then(Value::as_i64),
        Some(1500)
    );
    assert_eq!(
        out.pointer("/version").and_then(Value::as_str),
        Some("netflow_v5")
    );
}

/// sflow shares netflow's envelope head, so it must share the lift.
#[test]
fn sflow_lifts_the_same_head_as_netflow() {
    let out = unwrapped("sflow.json");

    assert_eq!(
        out.pointer("/observer/ip").and_then(Value::as_str),
        Some("10.0.0.1")
    );
    assert_eq!(
        out.pointer("/event/sequence").and_then(Value::as_i64),
        Some(42)
    );
}

/// Prometheus sends its labels as top-level fields. Two have an exact ECS home
/// and the rest are the sender's own naming, which is left alone.
#[test]
fn prometheus_lifts_the_two_labels_ecs_has_a_home_for() {
    let out = unwrapped("prometheus.json");

    assert_eq!(
        out.pointer("/service/name").and_then(Value::as_str),
        Some("api")
    );
    assert_eq!(
        out.pointer("/service/address").and_then(Value::as_str),
        Some("10.0.0.5:9090")
    );
    assert_eq!(
        out.pointer("/@timestamp").and_then(Value::as_str),
        Some("2026-02-19T00:00:00.000Z")
    );
    assert_eq!(
        out.pointer("/__name__").and_then(Value::as_str),
        Some("http_requests_total")
    );
}

/// `timestamp` is on the list of receiver keys stripped before the transform,
/// so a Vector metric lost its stamp entirely until the arm lifted it first.
#[test]
fn a_vector_metric_keeps_its_stamp() {
    let out = unwrapped("grpc_metric.json");

    assert_eq!(
        out.pointer("/@timestamp").and_then(Value::as_str),
        Some("2026-02-19T00:00:00Z")
    );
    assert_eq!(
        out.pointer("/name").and_then(Value::as_str),
        Some("http_requests_total")
    );
}

/// A producer's delivery keys describe the delivery, so none may reach a
/// transform under any transport.
#[test]
fn no_producer_key_survives_unwrapping() {
    const KEYS: &[&str] = &[
        "_source",
        "_raw",
        "_signal",
        "_vector_type",
        "_timestamp",
        "_observed_timestamp",
    ];

    for fixture in fixtures() {
        if fixture.family("shape") != Envelope::Receiver {
            continue;
        }
        let detected = envelope::detect(&fixture.event());
        let mut event = fixture.event();
        detected
            .family
            .unwrap_into_beats(&mut event, None, &detected.variant)
            .expect("unwraps");

        let rendered = event.as_value().to_string();
        for key in KEYS {
            assert!(
                !rendered.contains(&format!("\"{key}\"")),
                "{}: `{key}` reached the transform",
                fixture.name()
            );
        }
    }
}

/// A pinned envelope wins over what the events look like, because pinning is
/// the way through for a shape detection cannot see.
#[test]
fn a_pinned_envelope_beats_detection_and_is_counted() {
    let intake = registry::intake("filebeat.cisco_ios.default").expect("registered");
    let event = dfe_runtime::Event::new(serde_json::json!({
        "message": "a line",
        "_source": "syslog",
    }));

    let auto = envelope::resolve(EnvelopeSetting::Auto, Some(&event), intake, "test.dataset");
    assert_eq!(auto.delivery.envelope, Envelope::Receiver);
    assert!(!auto.contradicted);

    let pinned = envelope::resolve(EnvelopeSetting::Beats, Some(&event), intake, "test.dataset");
    assert_eq!(pinned.delivery.envelope, Envelope::Beats);
    assert!(pinned.contradicted, "the mismatch must be reported");
}

/// Detecting a family the source cannot arrive in must not unwrap a shape that
/// is not there: fall back to beats, which every source accepts.
#[test]
fn an_envelope_the_source_cannot_take_falls_back_to_beats() {
    let intake = registry::intake("filebeat.okta.default").expect("registered");
    assert!(!intake.accepts(Envelope::Receiver), "okta is API-pulled");

    let event = dfe_runtime::Event::new(serde_json::json!({
        "message": "a line",
        "_source": "syslog",
    }));
    let resolved = envelope::resolve(EnvelopeSetting::Auto, Some(&event), intake, "test.dataset");

    assert_eq!(resolved.delivery.envelope, Envelope::Beats);
    assert!(resolved.unaccepted);
}

/// An empty batch has nothing to detect from, and must not be an error.
#[test]
fn an_empty_batch_resolves_to_beats() {
    let intake = registry::intake("filebeat.okta.default").expect("registered");
    let resolved = envelope::resolve(EnvelopeSetting::Auto, None, intake, "test.dataset");

    assert_eq!(resolved.delivery.envelope, Envelope::Beats);
    assert_eq!(resolved.detected, None);
    assert!(!resolved.contradicted);
    assert!(!resolved.unaccepted);
}
