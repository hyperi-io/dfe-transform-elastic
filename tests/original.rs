// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One copy of the vendor line, never two and never none.
//!
//! The service removes `event.original` from a parsed event and puts it back
//! into `message` for one that parsed nothing. These tests hold both sides to
//! the real transforms: every shell goes back, and no real line does.

// A test asserts by panicking, which the workspace lints ban in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::Path;

use dfe_transform_elastic::pipeline::transform_batch;
use dfe_transform_elastic::registry;
use serde_json::{Value, json};

/// A line no transform can parse.
const GARBAGE: &str = "zzqx-probe-0042 not parseable";

/// The first word of [`GARBAGE`]. A transform whose fallback grok reads it as a
/// hostname has parsed a piece of the line, so it is not a shell.
const TOKEN: &str = "zzqx-probe-0042";

/// How many registered transforms leave a garbage line only in
/// `event.original`. One fewer means a transform now rejects or parses it, one
/// more means a new source that swallows an unparseable line silently.
const SHELLS: usize = 70;

/// The wrapper an Elastic Agent puts around a line, as `tests/broker.rs` sends
/// it, without the tag that keeps `event.original`.
fn wrapped(line: &str) -> dfe_runtime::Event {
    dfe_runtime::Event::new(json!({
        "@timestamp": "2026-02-19T00:00:00.000Z",
        "agent": { "id": "6f1c2d3e", "name": "edge-agent-01", "type": "filebeat", "version": "8.0.0" },
        "data_stream": { "dataset": "x.log", "namespace": "default", "type": "logs" },
        "elastic_agent": { "id": "6f1c2d3e", "snapshot": false, "version": "8.0.0" },
        "event": { "agent_id_status": "verified" },
        "input": { "type": "tcp" },
        "log": { "source": { "address": "192.0.2.10:46792" } },
        "tags": ["forwarded"],
        "message": line,
    }))
}

/// Run `f` on a stack big enough for every generated transform in a debug
/// build.
fn on_big_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(f)
        .expect("spawns")
        .join()
        .expect("the probe ran");
}

/// Every string value in `value`, with its dotted path.
fn strings(value: &Value, path: &str, out: &mut Vec<(String, String)>) {
    match value {
        Value::Object(fields) => {
            for (key, child) in fields {
                let next = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                strings(child, &next, out);
            }
        }
        Value::String(text) => out.push((path.to_owned(), text.clone())),
        _ => {}
    }
}

/// The sources whose transform, run alone, leaves the garbage in
/// `event.original` and nowhere else, with no error to say so.
fn shells() -> BTreeSet<&'static str> {
    registry::sources()
        .filter(|name| {
            let transform = registry::lookup(name).expect("registered");
            let mut event = wrapped(GARBAGE);
            let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                transform.transform(&mut event)
            }));
            if !matches!(ran, Ok(Ok(dfe_runtime::TransformResult::Continue))) {
                return false;
            }
            let mut found = Vec::new();
            strings(event.as_value(), "", &mut found);
            let holders: Vec<&str> = found
                .iter()
                .filter(|(_, text)| text.contains(TOKEN))
                .map(|(path, _)| path.as_str())
                .collect();
            holders == ["event.original"] && !event.has("error.message")
        })
        .collect()
}

/// With the garbage line through every transform, all 70 shells get it back in
/// `message` and no other source does.
#[test]
fn every_shell_gets_its_line_back_and_nothing_else_does() {
    on_big_stack(|| {
        let shells = shells();
        assert_eq!(shells.len(), SHELLS, "{shells:?}");

        let mut restored = BTreeSet::new();
        for name in registry::sources() {
            let transform = registry::lookup(name).expect("registered");
            let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                transform_batch(transform, vec![wrapped(GARBAGE)])
            }));
            let Ok((out, outcome)) = ran else { continue };
            if outcome.restored == 1 {
                restored.insert(name);
                assert_eq!(out[0].get_str("message"), Some(GARBAGE), "{name}");
            }
            for event in out.iter().filter(|e| !tagged(e)) {
                assert!(
                    !event.has("event.original"),
                    "{name} kept event.original untagged"
                );
            }
        }
        let missed: Vec<_> = shells.difference(&restored).collect();
        let extra: Vec<_> = restored.difference(&shells).collect();
        assert!(
            missed.is_empty() && extra.is_empty(),
            "shells not restored: {missed:?}, restored but not shells: {extra:?}"
        );
    });
}

/// fortinet is one of the shells, so an unparseable line comes out in
/// `message`, once.
#[test]
fn an_unparseable_fortinet_line_lands_in_message() {
    on_big_stack(|| {
        let transform = registry::lookup("filebeat.fortinet.default").expect("registered");
        let (out, outcome) = transform_batch(transform, vec![wrapped(GARBAGE)]);

        assert_eq!((outcome.emitted, outcome.restored), (1, 1));
        assert_eq!(out[0].get_str("message"), Some(GARBAGE));
        assert!(!out[0].has("event.original"));
        let mut found = Vec::new();
        strings(out[0].as_value(), "", &mut found);
        let copies = found.iter().filter(|(_, t)| t.contains(TOKEN)).count();
        assert_eq!(copies, 1, "{found:?}");
    });
}

/// The committed vendor samples: the sample directory, the transform it feeds,
/// the text a line must hold to be in the shape that transform reads (empty for
/// every line), and whether the test asserts that nothing is restored.
///
/// okta's sample also holds lines in another schema, and o365's and
/// crowdstrike's samples are not in the shape their transforms read. Those
/// restore, correctly, and are reported rather than asserted.
const SAMPLES: &[(&str, &str, &str, bool)] = &[
    ("azure", "azure_activitylogs", "", true),
    ("cisco_ios", "cisco_ios", "", true),
    ("cisco_meraki", "cisco_meraki", "", true),
    ("cisco_nexus", "cisco_nexus", "", false),
    ("crowdstrike", "crowdstrike", "", false),
    ("fortinet", "fortinet", "", true),
    ("o365", "o365", "", false),
    ("okta", "okta", "\"eventType\"", true),
    ("panw", "panw", "", true),
];

/// Real lines are parsed, so none is restored and every one drops
/// `event.original`.
#[test]
fn real_lines_are_removed_and_never_restored() {
    on_big_stack(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unencumbered");
        let mut totals = Vec::new();
        let mut wrongly_restored = Vec::new();
        for (directory, source, shape, asserted) in SAMPLES {
            let transform =
                registry::lookup(&format!("filebeat.{source}.default")).expect("registered");
            let mut files: Vec<_> = std::fs::read_dir(root.join(directory))
                .expect("sample directory")
                .flatten()
                .map(|entry| entry.path())
                .collect();
            files.sort();

            let (mut emitted, mut restored, mut kept) = (0, 0, 0);
            for file in files {
                let text = std::fs::read_to_string(&file).unwrap_or_default();
                let events: Vec<_> = text
                    .lines()
                    .filter(|line| !line.trim().is_empty() && line.contains(shape))
                    .take(200)
                    .map(wrapped)
                    .collect();
                let (out, outcome) = transform_batch(transform, events);
                emitted += outcome.emitted;
                restored += outcome.restored;
                for event in &out {
                    if tagged(event) {
                        kept += 1;
                    } else {
                        assert!(
                            !event.has("event.original"),
                            "{directory}: {} kept event.original",
                            file.display()
                        );
                    }
                }
            }
            assert!(emitted > 0, "{directory} emitted nothing");
            totals.push((*directory, emitted, kept, restored));
            if *asserted && restored > 0 {
                wrongly_restored.push((*directory, restored));
            }
        }
        println!("(source, emitted, tagged by the transform and kept, restored): {totals:?}");
        assert!(
            wrongly_restored.is_empty(),
            "lines in the shape the transform reads were treated as unparsed: {wrongly_restored:?}"
        );
    });
}

/// Whether a transform tagged the event to keep `event.original`, as its own
/// failure handler does.
fn tagged(event: &dfe_runtime::Event) -> bool {
    event
        .get("tags")
        .is_some_and(|tags| tags.to_string().contains("preserve_original_event"))
}
