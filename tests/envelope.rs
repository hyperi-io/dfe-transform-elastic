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

use dfe_transform_elastic::envelope::{self, Delivery, Envelope};
use dfe_transform_elastic::pipeline::transform_batch_with;
use dfe_transform_elastic::registry;
use serde_json::{Value, json};

/// How `source` arrives under `envelope`, with the variant detected off the
/// event and the framing and dataset the registry declares.
fn delivery(source: &str, envelope: Envelope, event: &dfe_runtime::Event) -> Delivery {
    Delivery {
        envelope,
        variant: envelope::detect(event).variant,
        framing: registry::intake(source).and_then(|i| i.framing),
        dataset: registry::dataset(source).expect("source is registered"),
    }
}

/// Run one event through `source` under `envelope`, returning the output.
fn run(source: &str, envelope: Envelope, event: Value) -> Option<Value> {
    let transform = registry::lookup(source).expect("source is registered");
    let event = dfe_runtime::Event::new(event);
    let delivery = delivery(source, envelope, &event);
    let (out, outcome) = transform_batch_with(transform, &delivery, vec![event]);
    assert_eq!(outcome.total(), 1, "{source}: event vanished");
    out.first().map(|e| e.as_value().clone())
}

/// Every leaf in `value`, keyed by its dotted path.
///
/// Compared leaf by leaf rather than object by object: an envelope that adds a
/// field the Beats path never had -- `event.provider` from a Splunk sourcetype,
/// `log.syslog.*` from a parsed header -- must not read as the whole `event`
/// object differing.
fn leaves(value: &Value) -> std::collections::BTreeMap<String, Value> {
    fn walk(prefix: &str, value: &Value, out: &mut std::collections::BTreeMap<String, Value>) {
        match value {
            Value::Object(map) if !map.is_empty() => {
                for (key, child) in map {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    walk(&path, child, out);
                }
            }
            _ => {
                out.insert(prefix.to_string(), value.clone());
            }
        }
    }

    let mut out = std::collections::BTreeMap::new();
    walk("", value, &mut out);
    out
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
        Envelope::Receiver,
        receiver_syslog(PANW_BODY),
    )
    .expect("syslog path emitted");

    // The syslog path additionally carries the parsed header, which the Beats
    // path never had. Everything the transform itself produced must match.
    let syslog_leaves = leaves(&via_syslog);
    for (path, expected) in leaves(&via_beats) {
        assert_eq!(
            syslog_leaves.get(&path),
            Some(&expected),
            "`{path}` differs between envelopes"
        );
    }
}

/// The header the receiver parsed must survive onto ECS fields, so nothing is
/// lost by having gone through syslog rather than Beats.
#[test]
fn the_syslog_path_keeps_the_parsed_header() {
    let out = run(
        "filebeat.panw.traffic",
        Envelope::Receiver,
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
        Envelope::Receiver,
        receiver_syslog(PANW_BODY),
    )
    .expect("emitted");

    for leaked in dfe_transform_elastic::envelope::RECEIVER_KEYS {
        assert!(
            out.get(*leaked).is_none(),
            "`{leaked}` leaked into the output: {out}"
        );
    }
}

/// `_source` is the one receiver field that must come out the far side: it
/// carries the variant id dfe-loader matches to pick the row's table, so a
/// transform that strips it lands every row in the catch-all
/// (hyperi-io/dfe-transform-elastic#67).
#[test]
fn the_variant_id_survives_a_whole_transform() {
    let out = run(
        "filebeat.panw.traffic",
        Envelope::Receiver,
        receiver_syslog(PANW_BODY),
    )
    .expect("emitted");

    assert_eq!(out.get("_source").and_then(Value::as_str), Some("syslog"));
}

/// What dfe-receiver delivers for `line` once a routing rule has relabelled
/// `_source`, with no `_raw`: the tagless PAN-OS line split by `syslog_loose`
/// at its first space, so `appname` holds the first CSV field.
fn relabelled_panw_delivery(label: &str) -> Value {
    let line = format!("<134>Mar  3 10:30:00 fw01 {PANW_BODY}");
    let m = syslog_loose::parse_message(&line, syslog_loose::Variant::Either);
    json!({
        "message": m.msg,
        "hostname": m.hostname,
        "appname": m.appname,
        "facility": "local0",
        "severity": "info",
        "timestamp": "2026-03-03T10:30:00+11:00",
        "_source": label,
    })
}

/// A routing rule turns `_source: "syslog"` into the catalogue label, and the
/// receiver's split of a tagless line puts the first CSV field in `appname`.
/// The panw CSV must still parse, and the label must still reach the loader.
#[test]
fn a_relabelled_panw_delivery_parses_without_raw_capture() {
    let via_beats = run(
        "filebeat.panw.traffic",
        Envelope::Beats,
        json!({ "message": PANW_BODY }),
    )
    .expect("beats path emitted");

    for label in ["demo-syslog", "panw-panos"] {
        let via_relabel = run(
            "filebeat.panw.traffic",
            Envelope::Receiver,
            relabelled_panw_delivery(label),
        )
        .expect("relabelled path emitted");

        let relabelled = leaves(&via_relabel);
        for (path, expected) in leaves(&via_beats) {
            assert_eq!(
                relabelled.get(&path),
                Some(&expected),
                "{label}: `{path}` differs between envelopes"
            );
        }
        assert_eq!(
            via_relabel.get("_source").and_then(Value::as_str),
            Some(label)
        );
        assert!(
            via_relabel.get("error").is_none() && via_relabel.pointer("/event/kind").is_some(),
            "{label}: {via_relabel}"
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
    Envelope::Receiver
        .unwrap_into_beats(&mut event, framing, "syslog")
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
    Envelope::Receiver
        .unwrap_into_beats(&mut event, framing, "syslog")
        .expect("unwraps");

    assert_eq!(event.get_str("message"), Some(raw));
}

/// `cisco_ios` emits under either envelope.
#[test]
fn cisco_ios_emits_under_either_envelope() {
    let transform = registry::lookup("filebeat.cisco_ios.default").expect("registered");
    let line = "<189>29: foo: Mar  3 10:30:00: %SYS-5-CONFIG_I: Configured from console";

    for envelope in [Envelope::Beats, Envelope::Receiver] {
        let mut source = receiver_syslog("%SYS-5-CONFIG_I: Configured from console");
        source["_raw"] = json!(line);
        let event = dfe_runtime::Event::new(source);
        let delivery = delivery("filebeat.cisco_ios.default", envelope, &event);
        let (out, outcome) = transform_batch_with(transform, &delivery, vec![event]);

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

    for source in registry::sources_accepting(Envelope::Receiver) {
        assert!(
            registry::intake(source).is_some_and(|i| i.accepts(Envelope::Receiver)),
            "{source} is listed as syslog but classified otherwise"
        );

        for body in bodies {
            let _ = run(source, Envelope::Receiver, receiver_syslog(body));
            // With _raw as well, which takes the other branch.
            let mut with_raw = receiver_syslog(body);
            with_raw["_raw"] = json!(format!("<134>Mar  3 10:30:00 fw01 app: {body}"));
            let _ = run(source, Envelope::Receiver, with_raw);
        }
    }
}

/// An API source under the syslog envelope is rejected by config validation,
/// but the envelope itself must still not panic if it is ever reached.
#[test]
fn the_envelope_is_safe_even_on_a_source_config_would_reject() {
    let _ = run(
        "filebeat.okta.default",
        Envelope::Receiver,
        receiver_syslog("{\"eventType\":\"user.session.start\"}"),
    );
}

/// Every receiver transport that can carry a device line must deliver the same
/// line to the transform.
///
/// The reason the envelope layer exists: one set of parsers, whatever brought
/// the bytes.
///
/// These wrappers are each transport's shape carrying a PANW line, which is
/// not the same thing as the recorded fixtures in `tests/envelopes/receiver/`
/// -- those hold the body their own producer sent, and `envelope_detection.rs`
/// is what checks them. The two cannot be merged: this test needs a body the
/// panw grok parses, and that is not what a GELF or Fluent sample carries.
///
/// One shape here deliberately differs from its fixture. `fluent.json` puts
/// the body in `log`, because the converter passes the sender's msgpack record
/// through untouched and a Docker driver writes `log`; a sender forwarding a
/// device line writes `message`. Both are real, and the fluent transport lifts
/// neither onto the other -- so a `log`-only record reaches a line-framed
/// transform with no `message` at all. Whether anything forwards device lines
/// that way is open.
#[test]
fn every_transport_that_carries_a_line_produces_the_same_output() {
    let baseline = run(
        "filebeat.panw.traffic",
        Envelope::Beats,
        json!({ "message": PANW_BODY }),
    )
    .expect("beats path emitted");

    let wrapped: [(&str, Value); 4] = [
        ("syslog", receiver_syslog(PANW_BODY)),
        (
            "gelf",
            json!({
                "_source": "gelf",
                "version": "1.1",
                "host": "fw01",
                "short_message": PANW_BODY,
                "message": PANW_BODY,
                "level": 6,
                "severity": "informational",
            }),
        ),
        (
            "fluent",
            json!({
                "_source": "fluent",
                "message": PANW_BODY,
                "tag": "panw.traffic",
                "timestamp": 1_771_459_200.5,
            }),
        ),
        (
            "splunk_hec",
            json!({
                "message": PANW_BODY,
                // `_time` is in the recorded fixture and drives the transport's
                // epoch-seconds lift, which this case did not exercise.
                "_time": 1_771_459_200,
                "host": "fw01",
                "source": "/var/log/panw",
                "sourcetype": "pan:traffic",
                "index": "main",
            }),
        ),
    ];

    let expected = leaves(&baseline);
    for (transport, event) in wrapped {
        let out = run("filebeat.panw.traffic", Envelope::Receiver, event)
            .unwrap_or_else(|| panic!("{transport}: emitted nothing"));
        let got = leaves(&out);

        for (path, value) in &expected {
            assert_eq!(
                got.get(path),
                Some(value),
                "{transport}: `{path}` differs from the beats path"
            );
        }
    }
}

/// Beats stamps the metadata every downstream consumer indexes on -- what
/// shipped it, which data stream it belongs to, how it arrived. The receiver
/// and the fetcher cannot know most of it, so the service supplies it, or the
/// events land unroutable.
#[test]
fn a_receiver_delivery_carries_the_metadata_beats_would_have_stamped() {
    let out = run(
        "filebeat.panw.traffic",
        Envelope::Receiver,
        receiver_syslog(PANW_BODY),
    )
    .expect("emitted");

    assert_eq!(
        out.pointer("/agent/type").and_then(Value::as_str),
        Some("dfe-receiver")
    );
    assert_eq!(
        out.pointer("/data_stream/dataset").and_then(Value::as_str),
        Some("panw.panos")
    );
    assert_eq!(
        out.pointer("/data_stream/type").and_then(Value::as_str),
        Some("logs")
    );
    assert_eq!(
        out.pointer("/input/type").and_then(Value::as_str),
        Some("syslog")
    );
    assert!(
        out.pointer("/agent/version")
            .and_then(Value::as_str)
            .is_some_and(|v| !v.is_empty()),
        "agent.version is the service's own, and must not be blank"
    );
}

/// A Beats delivery already carries its own, and ours would overwrite the
/// truth with a guess.
#[test]
fn a_beats_delivery_is_left_exactly_as_it_arrived() {
    let out = run(
        "filebeat.panw.traffic",
        Envelope::Beats,
        json!({ "message": PANW_BODY }),
    )
    .expect("emitted");

    assert_eq!(out.pointer("/agent/type"), None);
    assert_eq!(out.pointer("/data_stream/dataset"), None);
}

/// The receiver records no receive time, and the transform's own clock is not
/// one: on a replay it would stamp the replay day.
#[test]
fn a_receiver_delivery_claims_no_ingest_time() {
    let out = run(
        "filebeat.panw.traffic",
        Envelope::Receiver,
        receiver_syslog(PANW_BODY),
    )
    .expect("emitted");

    assert_eq!(out.pointer("/event/ingested"), None);
}

/// dfe-fetcher DOES record one, in epoch milliseconds, and it must survive the
/// re-serialisation that moves the payload into `message`.
#[test]
fn a_fetcher_delivery_keeps_the_receive_time_it_was_given() {
    let transform = registry::lookup("filebeat.okta.default").expect("registered");
    let delivery = Delivery {
        envelope: Envelope::Fetcher,
        variant: std::borrow::Cow::Borrowed("okta.system_log"),
        framing: None,
        dataset: registry::dataset("filebeat.okta.default").expect("registered"),
    };
    let event = dfe_runtime::Event::new(json!({
        "eventType": "user.session.start",
        "published": "2026-02-19T00:00:00.000Z",
        "_timestamp_fetcher": 1_771_459_200_000_u64,
        "_timestamp_received": 1_771_459_200_000_u64,
        "_source_fetcher": "okta.system_log",
    }));

    let (out, _) = transform_batch_with(transform, &delivery, vec![event]);
    let out = out.first().map(|e| e.as_value().clone()).expect("emitted");

    assert_eq!(
        out.pointer("/event/ingested").and_then(Value::as_str),
        Some("2026-02-19T00:00:00.000Z")
    );
    assert_eq!(
        out.pointer("/agent/type").and_then(Value::as_str),
        Some("dfe-fetcher")
    );
    assert_eq!(
        out.pointer("/input/type").and_then(Value::as_str),
        Some("okta.system_log")
    );
    assert_eq!(
        out.pointer("/data_stream/dataset").and_then(Value::as_str),
        Some("okta.system")
    );
}

/// The okta record from `tests/envelopes/fetcher/okta_system_log.json`, which
/// was read off dfe-fetcher's own `enrich_record`.
///
/// Beats hands the same provider payload over as a STRING in `message`; the
/// fetcher leaves it at the top level and appends its three delivery keys. The
/// bytes either way are these.
const OKTA_RECORD: &str = r#"{"uuid":"9d5c5a1b-0d3e-4a2f-8a11-2f0c1e6b7d90",
"published":"2026-02-19T00:00:00.000Z","eventType":"user.session.start",
"outcome":{"result":"SUCCESS"},
"actor":{"id":"00u1abcd2efGHIJ3k4l5","type":"User",
"alternateId":"derek@example.com","displayName":"Derek"},
"client":{"ipAddress":"203.0.113.7","userAgent":{"rawUserAgent":"Mozilla/5.0"}}}"#;

/// The fetcher side of the equivalence the receiver side already has: one okta
/// record delivered both ways must parse to the same thing.
///
/// This is the claim the whole fetcher path rests on -- that Elastic's agent is
/// a PURE TRANSPORT for an API-pulled source, so dfe-fetcher can obtain the
/// same bytes and the chain collapses. Asserting the stamping alone never
/// tested it; a transform has to run over both.
#[test]
fn okta_produces_the_same_output_from_beats_or_the_fetcher() {
    let record: Value = serde_json::from_str(OKTA_RECORD).expect("the record parses");

    let via_beats = run(
        "filebeat.okta.default",
        Envelope::Beats,
        json!({ "message": OKTA_RECORD }),
    )
    .expect("beats path emitted");

    // What the fetcher delivers: the provider's own JSON at the top level, plus
    // the three keys `enrich_record` appends.
    let mut delivered = record.as_object().expect("an object").clone();
    delivered.insert("_source_fetcher".into(), json!("okta.system_log"));
    delivered.insert("_timestamp_fetcher".into(), json!(1_771_459_200_000_u64));
    delivered.insert("_timestamp_received".into(), json!(1_771_459_200_000_u64));

    let via_fetcher = run(
        "filebeat.okta.default",
        Envelope::Fetcher,
        Value::Object(delivered),
    )
    .expect("fetcher path emitted");

    // The fetcher path additionally carries what Beats would have stamped --
    // `agent.*`, `data_stream.*`, `input.type`, `event.ingested` -- so the
    // assertion is one-way, exactly as the syslog equivalence is.
    //
    // `event.original` is checked separately, below: the fetcher path
    // re-serialises the payload to move it into `message`, so the two agree as
    // JSON rather than as text.
    let fetcher_leaves = leaves(&via_fetcher);
    for (path, expected) in leaves(&via_beats) {
        if path == "event.original" {
            continue;
        }
        assert_eq!(
            fetcher_leaves.get(&path),
            Some(&expected),
            "`{path}` differs between envelopes"
        );
    }

    // Same document, whatever the whitespace.
    let original_of = |out: &Value| -> Value {
        let text = out
            .pointer("/event/original")
            .and_then(Value::as_str)
            .expect("event.original is set");
        serde_json::from_str(text).expect("event.original is JSON")
    };
    assert_eq!(original_of(&via_beats), original_of(&via_fetcher));

    // A guard against the test passing because BOTH sides parsed nothing.
    assert_eq!(
        via_beats.pointer("/event/action").and_then(Value::as_str),
        Some("user.session.start"),
        "the transform did not parse the record at all"
    );
}

/// A Microsoft Graph `directoryAudit`, as the Graph reference's "Get
/// directoryAudit" example shows it, with `targetResources` spelled as the
/// resource type names it.
const GRAPH_DIRECTORY_AUDIT: &str = r#"{
  "id": "Directory_da159bfb-54fa-4092-8a38-6e1fa7870e30_ABCDE_123456",
  "category": "GroupManagement",
  "correlationId": "da159bfb-54fa-4092-8a38-6e1fa7870e30",
  "result": "success",
  "resultReason": "Successfully added member to group",
  "activityDisplayName": "Add member to group",
  "activityDateTime": "2018-01-09T21:20:02.7215374Z",
  "loggedByService": "Core Directory",
  "operationType": "Add",
  "initiatedBy": {
    "user": {
      "id": "728309ae-1a37-4937-9afe-e35d964db09b",
      "displayName": "Audry Oliver",
      "userPrincipalName": "bob@wingtiptoysonline.com",
      "ipAddress": "127.0.0.1"
    },
    "app": null
  },
  "targetResources": [
    {
      "id": "ef7e527d-6c92-4234-8c6d-cf6fdfb57f95",
      "displayName": "Example.com",
      "type": "Group",
      "modifiedProperties": [
        { "displayName": "Action Client Name", "oldValue": null, "newValue": "DirectorySync" }
      ],
      "groupType": "unifiedGroups"
    },
    {
      "id": "1f0e98f5-3161-4c6b-9b50-d488572f2bb7",
      "displayName": null,
      "type": "User",
      "modifiedProperties": [],
      "userPrincipalName": "example@contoso.com"
    }
  ],
  "additionalDetails": [
    { "key": "Additional Detail Name", "value": "Additional Detail Value" }
  ]
}"#;

/// A Microsoft Graph `signIn`, as the Graph reference's "Get signIn" example
/// shows it.
const GRAPH_SIGN_IN: &str = r#"{
  "id": "66ea54eb-6301-4ee5-be62-ff5a759b0100",
  "createdDateTime": "2023-12-01T16:03:24Z",
  "userDisplayName": "Test Contoso",
  "userPrincipalName": "testaccount1@contoso.com",
  "userId": "26be570a-ae82-4189-b4e2-a37c6808512d",
  "appId": "de8bc8b5-d9f9-48b1-a8ad-b748da725064",
  "appDisplayName": "Graph explorer",
  "ipAddress": "131.107.159.37",
  "clientAppUsed": "Browser",
  "correlationId": "d79f5bee-5860-4832-928f-3133e22ae912",
  "conditionalAccessStatus": "notApplied",
  "isInteractive": true,
  "riskDetail": "none",
  "riskLevelAggregated": "none",
  "riskLevelDuringSignIn": "none",
  "riskState": "none",
  "riskEventTypes": [],
  "resourceDisplayName": "Microsoft Graph",
  "resourceId": "00000003-0000-0000-c000-000000000000",
  "status": { "errorCode": 0, "failureReason": null, "additionalDetails": null },
  "deviceDetail": {
    "deviceId": "",
    "displayName": null,
    "operatingSystem": "Windows 10",
    "browser": "Edge 80.0.361",
    "isCompliant": null,
    "isManaged": null,
    "trustType": null
  },
  "location": {
    "city": "Redmond",
    "state": "Washington",
    "countryOrRegion": "US",
    "geoCoordinates": { "altitude": null, "latitude": 47.68050003051758, "longitude": -122.12094116210938 }
  }
}"#;

/// What dfe-fetcher delivers for an Entra Graph audit unit: the Graph record at
/// the top level, with the three keys `enrich_record` appends.
fn fetched_graph_record(record: &str, unit: &str) -> Value {
    let mut delivered = serde_json::from_str::<Value>(record)
        .expect("the record parses")
        .as_object()
        .expect("an object")
        .clone();
    delivered.insert("_source_fetcher".into(), json!(unit));
    delivered.insert("_timestamp_fetcher".into(), json!(1_771_459_200_000_u64));
    delivered.insert("_timestamp_received".into(), json!(1_771_459_200_000_u64));
    Value::Object(delivered)
}

/// Elastic's `azure` audit-log and sign-in pipelines read the Event Hub
/// diagnostic-log record. The Graph record has a `category` of its own
/// (`GroupManagement`), so before the envelope dressed it the audit-logs
/// pipeline dropped every record and the sign-in pipeline, which keeps only
/// categories ending `SignInLogs`, did the same to a record with none.
#[test]
fn a_graph_audit_record_survives_the_azure_pipelines() {
    for (source, record, unit, expected) in [
        (
            "filebeat.azure_auditlogs.default",
            GRAPH_DIRECTORY_AUDIT,
            "azure.entra_directory_audits",
            [
                ("/event/action", "Add member to group"),
                ("/event/outcome", "success"),
                ("/@timestamp", "2018-01-09T21:20:02.721Z"),
                ("/user/name", "bob@wingtiptoysonline.com"),
                ("/source/ip", "127.0.0.1"),
                ("/azure/auditlogs/category", "AuditLogs"),
                ("/azure/auditlogs/properties/category", "GroupManagement"),
            ],
        ),
        (
            "filebeat.azure_signinlogs.default",
            GRAPH_SIGN_IN,
            "azure.entra_signins",
            [
                ("/event/action", "Sign-in activity"),
                ("/event/outcome", "success"),
                ("/@timestamp", "2023-12-01T16:03:24.000Z"),
                ("/user/name", "testaccount1@contoso.com"),
                ("/source/ip", "131.107.159.37"),
                ("/azure/signinlogs/category", "SignInLogs"),
                (
                    "/azure/signinlogs/properties/app_display_name",
                    "Graph explorer",
                ),
            ],
        ),
    ] {
        let out = run(
            source,
            Envelope::Fetcher,
            fetched_graph_record(record, unit),
        )
        .unwrap_or_else(|| panic!("{source}: the pipeline dropped the Graph record"));
        for (pointer, want) in expected {
            assert_eq!(
                out.pointer(pointer).and_then(Value::as_str),
                Some(want),
                "{source}: {pointer}"
            );
        }
        // The routing marker survives, so the loader still finds the source's table.
        assert_eq!(
            out.get("_source_fetcher").and_then(Value::as_str),
            Some(unit)
        );
    }
}

/// A Graph record missing the fields the envelope derives from still gets the
/// category the pipeline keeps, and a record already in the Event Hub form is
/// not wrapped twice.
#[test]
fn the_graph_envelope_tolerates_sparse_and_already_wrapped_records() {
    let sparse = fetched_graph_record(
        r#"{"id": "Directory_1", "category": "UserManagement", "activityDisplayName": "Add user"}"#,
        "azure.entra_directory_audits",
    );
    let out = run(
        "filebeat.azure_auditlogs.default",
        Envelope::Fetcher,
        sparse,
    )
    .expect("a sparse record is not dropped");
    assert_eq!(
        out.pointer("/event/action").and_then(Value::as_str),
        Some("Add user")
    );

    let mut hub: Value = fetched_graph_record(
        r#"{"time": "2022-01-22T18:15:02.5168093Z", "category": "AuditLogs",
            "operationName": "Add service principal credentials",
            "properties": {"id": "Directory_2", "category": "ApplicationManagement"}}"#,
        "azure.entra_directory_audits",
    );
    assert!(hub.get("properties").is_some());
    let out = run(
        "filebeat.azure_auditlogs.default",
        Envelope::Fetcher,
        std::mem::take(&mut hub),
    )
    .expect("an Event Hub form record is not dropped");
    assert_eq!(
        out.pointer("/azure/auditlogs/properties/id")
            .and_then(Value::as_str),
        Some("Directory_2")
    );
    assert!(
        out.pointer("/azure/auditlogs/properties/properties")
            .is_none()
    );
}

/// The row dfe-fetcher's `cloudtrail_event` builder emits for a `LookupEvents`
/// record: the bare `CloudTrail` event with the three delivery keys appended,
/// recorded in `tests/envelopes/fetcher/aws_cloudtrail.json`.
///
/// The `aws.cloudtrail` pipeline reads the event from `message`, so the same
/// bytes delivered by Beats must produce the same ECS columns.
#[test]
fn cloudtrail_fills_its_columns_from_the_fetcher_row() {
    let fixture: Value =
        serde_json::from_str(include_str!("envelopes/fetcher/aws_cloudtrail.json"))
            .expect("the fixture parses");
    let delivered = fixture["event"].clone();

    let mut bare = delivered.as_object().expect("an object").clone();
    for key in [
        "_timestamp_fetcher",
        "_timestamp_received",
        "_source_fetcher",
    ] {
        bare.remove(key);
    }
    let via_beats = run(
        "filebeat.aws_cloudtrail.default",
        Envelope::Beats,
        json!({ "message": Value::Object(bare).to_string() }),
    )
    .expect("beats path emitted");
    let via_fetcher = run(
        "filebeat.aws_cloudtrail.default",
        Envelope::Fetcher,
        delivered,
    )
    .expect("fetcher path emitted");

    for (column, want) in [
        ("/event/action", "ConsoleLogin"),
        ("/event/provider", "signin.amazonaws.com"),
        ("/cloud/account/id", "123456789012"),
        ("/cloud/region", "ap-southeast-2"),
    ] {
        assert_eq!(
            via_fetcher.pointer(column).and_then(Value::as_str),
            Some(want),
            "{column} from the fetcher row"
        );
    }
    let timestamp = via_fetcher
        .pointer("/@timestamp")
        .and_then(Value::as_str)
        .expect("@timestamp is set");
    assert!(
        timestamp.starts_with("2026-02-19T00:00:00"),
        "@timestamp comes from eventTime, not the fetch time: {timestamp}"
    );

    let fetcher_leaves = leaves(&via_fetcher);
    for (path, expected) in leaves(&via_beats) {
        if path == "event.original" {
            continue;
        }
        assert_eq!(
            fetcher_leaves.get(&path),
            Some(&expected),
            "`{path}` differs between envelopes"
        );
    }
}

/// What an Elastic Agent hands the `cisco_ios` pipeline, from the Elastic
/// data's `tests/envelopes/beats/agent_cisco_ios.json`.
fn agent_wrapped() -> Value {
    let path = dfe_runtime::testutil::require_elastic_root()
        .join("tests/envelopes/beats/agent_cisco_ios.json");
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{} reads: {e}", path.display()));
    let doc: Value = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{}: not valid JSON: {e}", path.display()));
    doc["event"].clone()
}

/// The Agent path through a whole transform, which detection alone never
/// tested: `data_stream` and `elastic_agent` appear ZERO times in the corpus,
/// so every parity number was measured on the module shape.
#[test]
#[ignore = "reads Elastic-licensed test data kept outside this repository: set DFE_ELASTIC_FIXTURES to its fixtures/elastic directory and run with --ignored"]
fn cisco_ios_produces_the_same_output_from_beats_or_the_agent() {
    let wrapped = agent_wrapped();
    let line = wrapped["message"]
        .as_str()
        .expect("the Agent document carries a message")
        .to_string();

    let via_beats = run(
        "filebeat.cisco_ios.default",
        Envelope::Beats,
        json!({ "message": line, "tags": wrapped["tags"] }),
    )
    .expect("beats path emitted");

    let via_agent = run(
        "filebeat.cisco_ios.default",
        Envelope::Beats,
        wrapped.clone(),
    )
    .expect("agent path emitted");

    let agent_leaves = leaves(&via_agent);
    for (path, expected) in leaves(&via_beats) {
        assert_eq!(
            agent_leaves.get(&path),
            Some(&expected),
            "`{path}` differs between envelopes"
        );
    }

    // A guard against both sides parsing nothing: the mnemonic is read out of
    // the line's `%FACILITY-SEVERITY-MNEMONIC:` tag.
    let code = via_beats
        .pointer("/event/code")
        .and_then(Value::as_str)
        .expect("the transform did not parse the line at all");
    assert!(
        !code.is_empty() && line.contains(&format!("-{code}:")),
        "event.code `{code}` is not the mnemonic in `{line}`"
    );

    // The Agent's own wrapper survives, because it is the truth about the
    // delivery and nothing downstream can recover it.
    for pointer in ["/elastic_agent/version", "/data_stream/dataset"] {
        let sent = wrapped.pointer(pointer).expect("the Agent sent it");
        assert_eq!(via_agent.pointer(pointer), Some(sent), "{pointer}");
    }
}

/// The delivery keys are the fetcher's own bookkeeping and must not reach the
/// payload the transform parses, under any spelling.
///
/// `_source_fetcher` is the exception on the EVENT: dfe-engine compiles a
/// fetched source's routing rule against that field, so it has to come out the
/// far side the way `_source` does on the receiver path.
///
/// The keys are inserted with the types the fetcher writes rather than looped
/// out of `FETCHER_KEYS`, because `_timestamp_received` is read before it is
/// stripped. The completeness check below is what stops the list drifting: a
/// fourth key this test never delivers would otherwise "pass" by never being
/// there in the first place.
#[test]
fn the_fetchers_own_keys_never_reach_the_output() {
    let record: Value = serde_json::from_str(OKTA_RECORD).expect("the record parses");
    let mut delivered = record.as_object().expect("an object").clone();
    delivered.insert("_source_fetcher".into(), json!("okta.system_log"));
    delivered.insert("_timestamp_fetcher".into(), json!(1_771_459_200_000_u64));
    delivered.insert("_timestamp_received".into(), json!(1_771_459_200_000_u64));

    for key in dfe_transform_elastic::envelope::FETCHER_KEYS {
        assert!(
            delivered.contains_key(*key),
            "`{key}` is a fetcher key this test never delivers, so it proves nothing about it"
        );
    }

    let out = run(
        "filebeat.okta.default",
        Envelope::Fetcher,
        Value::Object(delivered),
    )
    .expect("emitted");

    // The payload the transform parsed, which is what `event.original` holds.
    let payload = out
        .pointer("/event/original")
        .and_then(Value::as_str)
        .expect("event.original is set");
    for key in dfe_transform_elastic::envelope::FETCHER_KEYS {
        assert!(
            !payload.contains(&format!("\"{key}\"")),
            "`{key}` reached the payload"
        );
    }

    let rendered = serde_json::to_string(&out).expect("serialises");
    for key in ["_timestamp_fetcher", "_timestamp_received"] {
        assert!(
            !rendered.contains(&format!("\"{key}\"")),
            "`{key}` reached the output"
        );
    }
    assert_eq!(
        out.get("_source_fetcher").and_then(Value::as_str),
        Some("okta.system_log"),
        "the routing field the loader matches on was stripped"
    );
}
