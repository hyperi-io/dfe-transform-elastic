// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `source.conf` through a real transform.
//!
//! `cisco_nexus` resolves a zone abbreviation such as `AEST` through
//! `_conf.tz_map`. With nothing setting `_conf` the date processors fail and
//! the event leaves as a pipeline error, which is what `source.conf` fixes.
//! The lines are invented, with documentation-range addresses.

// Tests assert by panicking, which the workspace lints ban in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_runtime::Event;
use dfe_transform_elastic::conf::SourceConf;
use dfe_transform_elastic::envelope::{EnvelopeSetting, Resolver};
use dfe_transform_elastic::pipeline::{BatchOutcome, transform_batch_resolved};
use dfe_transform_elastic::registry;
use serde_json::{Map, Value, json};

const NEXUS: &str = "filebeat.cisco_nexus.default";

/// Three of the layouts the nexus pipeline groks, each stamped `AEST`.
const AEST_LINES: [&str; 3] = [
    "<189>Jun 14 15:10:05 192.0.2.10 6031594: Jun 14 15:10:00.123 AEST: %ETHPORT-5-IF_UP: Interface Ethernet1/10 is up in mode access",
    "<189>nx-syn-01: 2026 Jun 14 15:10:00 AEST: %ETHPORT-5-IF_UP: Interface Ethernet1/10 is up in mode access",
    "<190>Jun 14 15:11:05 192.0.2.10 6031595: Jun 14 15:11:00.456 AEST: %AUTHPRIV-6-SYSTEM_MSG: pam_unix(sshd:session): session opened for user admin",
];

/// The same layout stamped `UTC`, which the pipeline reads without any map.
const UTC_LINE: &str = "<189>Jun 14 05:10:05 192.0.2.10 6031594: Jun 14 05:10:00.123 UTC: %ETHPORT-5-IF_UP: Interface Ethernet1/10 is up in mode access";

fn settings(value: Value) -> Map<String, Value> {
    let Value::Object(map) = value else {
        panic!("settings are an object");
    };
    map
}

fn tz_map() -> Map<String, Value> {
    settings(json!({"tz_map": [{"tz_short": "AEST", "tz_long": "Australia/Sydney"}]}))
}

fn run(conf: &SourceConf, envelope: EnvelopeSetting, event: Event) -> (Event, BatchOutcome) {
    let intake = registry::intake(NEXUS).expect("registered");
    let dataset = registry::dataset(NEXUS).expect("registered");
    let transform = registry::lookup(NEXUS).expect("registered");
    let resolver = Resolver::new(envelope, intake, dataset);
    let (mut out, outcome, _) = transform_batch_resolved(transform, &resolver, conf, vec![event]);
    assert_eq!(out.len(), 1, "the event is kept");
    (out.remove(0), outcome)
}

fn beats(line: &str) -> Event {
    Event::new(json!({ "message": line }))
}

/// What the zone map changes, line by line.
#[test]
fn a_tz_map_resolves_an_abbreviation_the_pipeline_cannot() {
    let with = SourceConf::new(&tz_map());
    for line in AEST_LINES {
        let (without, _) = run(&SourceConf::default(), EnvelopeSetting::Beats, beats(line));
        assert_eq!(
            without.get_str("event.kind"),
            Some("pipeline_error"),
            "{line}"
        );
        assert!(!without.has("@timestamp"), "{line}");
        assert!(without.has("error.message"), "{line}");

        let (resolved, outcome) = run(&with, EnvelopeSetting::Beats, beats(line));
        assert_eq!(outcome.errored, 0, "{line}");
        assert_eq!(resolved.get_str("event.kind"), Some("event"), "{line}");
        assert_eq!(
            resolved.get_str("event.timezone"),
            Some("Australia/Sydney"),
            "{line}"
        );
        assert!(!resolved.has("error.message"), "{line}");
        let stamp = resolved.get_str("@timestamp").expect("a parsed timestamp");
        assert!(
            stamp.contains("-06-14T15:1") && stamp.ends_with("+10:00"),
            "{stamp}"
        );
        assert!(
            !resolved.has("_conf"),
            "the pipeline drops its scratch `_conf`"
        );
    }
}

/// An abbreviation the pipeline reads without a map is not changed by one.
#[test]
fn a_tz_map_leaves_a_line_that_needed_none() {
    let none = SourceConf::default();
    let mapped = SourceConf::new(&tz_map());
    let (plain, _) = run(&none, EnvelopeSetting::Beats, beats(UTC_LINE));
    let (with, _) = run(&mapped, EnvelopeSetting::Beats, beats(UTC_LINE));
    assert_eq!(plain.as_value(), with.as_value());
    assert_eq!(with.get_str("event.timezone"), Some("UTC"));
}

/// `tz_offset` is the fallback for a line carrying no zone at all.
#[test]
fn a_tz_offset_sets_the_zone_of_a_line_with_none() {
    let line = "<189>nx-syn-01: 2026 Jun 14 15:10:00: %ETHPORT-5-IF_UP: Interface Ethernet1/10 is up in mode access";
    let offset = SourceConf::new(&settings(json!({"tz_offset": "+10:00"})));
    let (without, _) = run(&SourceConf::default(), EnvelopeSetting::Beats, beats(line));
    let (with, _) = run(&offset, EnvelopeSetting::Beats, beats(line));
    assert_ne!(without.get_str("event.timezone"), Some("+10:00"));
    assert_eq!(with.get_str("event.timezone"), Some("+10:00"));
}

/// The map goes on after the envelope unwraps, so a receiver delivery keeps it.
#[test]
fn the_map_survives_the_receiver_unwrap() {
    let event = Event::new(json!({
        "message": "ignored once the raw line is read",
        "_raw": AEST_LINES[0],
        "facility": "local7",
        "severity": "notice",
        "hostname": "192.0.2.10",
        "_source": "syslog"
    }));
    let (out, outcome) = run(
        &SourceConf::new(&tz_map()),
        EnvelopeSetting::Receiver,
        event,
    );
    assert_eq!(outcome.errored, 0);
    assert_eq!(out.get_str("event.timezone"), Some("Australia/Sydney"));
    assert_eq!(out.get_str("event.kind"), Some("event"));
}

/// Empty settings change nothing: the same output, byte for byte, as no conf.
#[test]
fn empty_settings_are_the_same_as_no_conf() {
    let empty = SourceConf::new(&Map::new());
    for line in AEST_LINES.iter().chain([&UTC_LINE]) {
        let (none, _) = run(&SourceConf::default(), EnvelopeSetting::Beats, beats(line));
        let (same, _) = run(&empty, EnvelopeSetting::Beats, beats(line));
        assert_eq!(none.as_value().to_string(), same.as_value().to_string());
    }
}
