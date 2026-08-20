// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The syslog envelope against the real transforms.
//!
//! The contract is that a device line produces the SAME output whichever way
//! it arrived: straight from Beats, or via dfe-receiver's syslog JSON. The
//! transform pipeline is one pipeline; only the wrapper differs.
//!
//! Fixtures here are in dfe-receiver's actual output shape
//! (`dfe-receiver/src/server/syslog/convert.rs`): the syslog MSG body in
//! `message`, the header parsed into siblings, `_source: "syslog"`.

// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_transform_elastic::envelope::Envelope;
use dfe_transform_elastic::pipeline::transform_batch_with;
use dfe_transform_elastic::registry;
use serde_json::{Value, json};

/// Run one event through `source` under `envelope`, with the framing the
/// registry declares for that source, returning the output.
fn run(source: &str, envelope: Envelope, event: Value) -> Option<Value> {
    let transform = registry::lookup(source).expect("source is registered");
    let framing = registry::intake(source).and_then(|i| i.framing);
    let events = vec![dfe_runtime::Event::new(event)];
    let (out, outcome) = transform_batch_with(transform, envelope, framing, events);
    assert_eq!(outcome.total(), 1, "{source}: event vanished");
    out.first().map(|e| e.as_value().clone())
}

/// A PAN-OS traffic line. panw reads `message` as CSV and wants no header, so
/// this is the family the receiver's JSON fits without reconstruction.
const PANW_BODY: &str = "1,2026/03/03 10:30:00,001801000000,TRAFFIC,end,2049,\
2026/03/03 10:30:00,10.0.0.1,10.0.0.2,0.0.0.0,0.0.0.0,rule1,,,ssl,vsys1,trust,\
untrust,ethernet1/1,ethernet1/2,fwd,2026/03/03 10:30:00,12345,1,443,54321,0,0,\
0x0,tcp,allow,1024,512,512,4,2026/03/03 10:29:00,10,any,0,987654,0x0,AU,US,0,2,2";

fn receiver_syslog(body: &str) -> Value {
    json!({
        "message": body,
        "facility": "local0",
        "severity": "info",
        "timestamp": "2026-03-03T10:30:00+11:00",
        "hostname": "fw01",
        "_source": "syslog"
    })
}

/// panw takes the body straight, so the receiver's JSON and a Beats event
/// carrying the same body must land in the same place.
#[test]
fn panw_produces_the_same_output_from_either_envelope() {
    let via_beats = run(
        "filebeat.panw.traffic",
        Envelope::Beats,
        json!({ "message": PANW_BODY }),
    )
    .expect("beats path emitted");

    let via_syslog = run(
        "filebeat.panw.traffic",
        Envelope::Syslog,
        receiver_syslog(PANW_BODY),
    )
    .expect("syslog path emitted");

    // The syslog path additionally carries the parsed header, which the Beats
    // path never had. Everything the transform itself produced must match.
    for (key, beats_value) in via_beats.as_object().expect("object") {
        assert_eq!(
            via_syslog.get(key),
            Some(beats_value),
            "`{key}` differs between envelopes"
        );
    }
}

/// The header the receiver parsed must survive onto ECS fields, so nothing is
/// lost by having gone through syslog rather than Beats.
#[test]
fn the_syslog_path_keeps_the_parsed_header() {
    let out = run(
        "filebeat.panw.traffic",
        Envelope::Syslog,
        receiver_syslog(PANW_BODY),
    )
    .expect("emitted");

    assert_eq!(
        out.pointer("/log/syslog/hostname").and_then(Value::as_str),
        Some("fw01")
    );
    assert_eq!(
        out.pointer("/log/syslog/severity/name")
            .and_then(Value::as_str),
        Some("info")
    );
}

/// The receiver's own field names must not reach the output under either
/// envelope -- both must produce the same shape.
#[test]
fn receiver_field_names_never_reach_the_output() {
    let out = run(
        "filebeat.panw.traffic",
        Envelope::Syslog,
        receiver_syslog(PANW_BODY),
    )
    .expect("emitted");

    for leaked in [
        "_source", "_raw", "facility", "severity", "hostname", "appname",
    ] {
        assert!(
            out.get(leaked).is_none(),
            "`{leaked}` leaked into the output: {out}"
        );
    }
}

/// What a `Framing::Line` source is handed: a `<PRI>`-prefixed line, rebuilt
/// when the receiver kept no raw one.
///
/// Asserted on the envelope rather than through a transform, because the
/// line-framed pipelines delete the evidence -- fortinet removes
/// `event.original` unconditionally, and `cisco_ios` errors before it gets
/// there (see `cisco_ios_errors_on_every_event`).
#[test]
fn a_line_framed_source_is_handed_a_pri_prefixed_line() {
    let body = "date=2026-03-03 time=10:30:00 devname=\"fg01\" type=\"traffic\"";
    let mut event = dfe_runtime::Event::new(receiver_syslog(body));

    let framing = registry::intake("filebeat.fortinet.default").and_then(|i| i.framing);
    assert_eq!(framing, Some(registry::Framing::Line));
    Envelope::Syslog
        .unwrap_into_beats(&mut event, framing)
        .expect("unwraps");

    // local0(16)*8 + info(6) = 134.
    let message = event.get_str("message").expect("message set");
    assert!(message.starts_with("<134>"), "{message}");
    assert!(message.ends_with(body), "{message}");
}

/// `_raw` is the device's own spelling and must reach the transform untouched,
/// because `cisco_ios` and `cisco_nexus` need header fields the receiver does
/// not keep -- a source IP and a sequence number respectively.
#[test]
fn raw_reaches_a_line_framed_source_untouched() {
    let raw = "<189>29: foo: Mar  3 10:30:00: %SYS-5-CONFIG_I: Configured from console by vty0";
    let mut source = receiver_syslog("%SYS-5-CONFIG_I: Configured from console by vty0");
    source["_raw"] = json!(raw);
    let mut event = dfe_runtime::Event::new(source);

    let framing = registry::intake("filebeat.cisco_ios.default").and_then(|i| i.framing);
    Envelope::Syslog
        .unwrap_into_beats(&mut event, framing)
        .expect("unwraps");

    assert_eq!(event.get_str("message"), Some(raw));
}

/// `cisco_ios` emits under either envelope.
#[test]
fn cisco_ios_emits_under_either_envelope() {
    let transform = registry::lookup("filebeat.cisco_ios.default").expect("registered");
    let line = "<189>29: foo: Mar  3 10:30:00: %SYS-5-CONFIG_I: Configured from console";

    for envelope in [Envelope::Beats, Envelope::Syslog] {
        let mut source = receiver_syslog("%SYS-5-CONFIG_I: Configured from console");
        source["_raw"] = json!(line);
        let framing = registry::intake("filebeat.cisco_ios.default").and_then(|i| i.framing);
        let (out, outcome) = transform_batch_with(
            transform,
            envelope,
            framing,
            vec![dfe_runtime::Event::new(source)],
        );

        assert_eq!(outcome.errored, 0, "{envelope:?}: cisco_ios errored");
        assert!(!out.is_empty(), "{envelope:?}: cisco_ios emitted nothing");
    }
}

/// Every syslog-origin source must survive the syslog envelope without
/// panicking, whatever the body. A transform may legitimately drop or error;
/// only a panic is a failure.
#[test]
fn no_syslog_source_panics_on_the_syslog_envelope() {
    let bodies = [
        "",
        "a plain message",
        "日本語のログ",
        "key=value key2=\"quoted value\"",
        "1,2,3,4,5",
        "%ASA-6-302013: Built connection",
    ];

    for source in registry::sources_accepting(Envelope::Syslog) {
        assert!(
            registry::intake(source).is_some_and(|i| i.accepts(Envelope::Syslog)),
            "{source} is listed as syslog but classified otherwise"
        );

        for body in bodies {
            let _ = run(source, Envelope::Syslog, receiver_syslog(body));
            // With _raw as well, which takes the other branch.
            let mut with_raw = receiver_syslog(body);
            with_raw["_raw"] = json!(format!("<134>Mar  3 10:30:00 fw01 app: {body}"));
            let _ = run(source, Envelope::Syslog, with_raw);
        }
    }
}

/// An API source under the syslog envelope is rejected by config validation,
/// but the envelope itself must still not panic if it is ever reached.
#[test]
fn the_envelope_is_safe_even_on_a_source_config_would_reject() {
    let _ = run(
        "filebeat.okta.default",
        Envelope::Syslog,
        receiver_syslog("{\"eventType\":\"user.session.start\"}"),
    );
}
