// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scores the agentless detector against the compat corpus.
//!
//! Each captured event is detected twice: as the raw line, and as the fields
//! dfe-receiver delivers after `syslog_loose` has split it. An OWN event is one
//! whose source the table can name. It is MISSED when the detector names another
//! source or none. A FOREIGN event belongs to a source the table cannot name and
//! is CLAIMED when the detector names anything, since that sends it to the wrong
//! table. Both counts only fall, held in `tests/ratchets.json`.
//!
//! The corpus is not in a git worktree. Set `DFE_COMPAT_CORPUS` there, or the
//! test reports that it measured nothing.

#![allow(missing_docs, clippy::expect_used, clippy::panic)]

#[path = "detect_corpus/loader.rs"]
mod loader;

use std::collections::BTreeMap;
use std::path::Path;

use dfe_runtime::Event;
use dfe_runtime::testutil::ratchets::Ratchets;
use dfe_transform_elastic::detect::{Detector, detectable};
use serde_json::{Value, json};
use syslog_loose::Variant;

/// Own events in the corpus when the table was written. A load that finds far
/// fewer has read a partial corpus and would pass for the wrong reason.
const MIN_OWN_EVENTS: usize = 8_000;

/// The event dfe-receiver emits for a line, as the receiver sends it.
type Shape = fn(&str) -> Event;

fn raw(text: &str) -> Event {
    Event::new(json!({ "_raw": text }))
}

fn parsed(text: &str) -> Event {
    let m = syslog_loose::parse_message(text, Variant::Either);
    let structured: Vec<Value> = m
        .structured_data
        .iter()
        .map(|e| {
            let params: serde_json::Map<String, Value> =
                e.params().map(|(k, v)| (k.to_string(), json!(v))).collect();
            json!({ "id": e.id, "params": params })
        })
        .collect();
    Event::new(json!({
        "message": m.msg,
        "hostname": m.hostname,
        "appname": m.appname,
        "msgid": m.msgid,
        "structured_data": structured,
    }))
}

#[derive(Default)]
struct Score {
    own: usize,
    right: usize,
    other: usize,
    none: usize,
    foreign: usize,
    claimed: usize,
    missed_by_source: BTreeMap<String, usize>,
}

fn score(lines: &[loader::Line], shape: Shape) -> Score {
    let detector = Detector::new(&[]).expect("no candidates is every source");
    let ours = detectable();
    let mut scratch = String::new();
    let mut s = Score::default();
    for line in lines {
        let got = detector
            .detect_event(&shape(&line.text), &mut scratch)
            .map(|i| ours[i]);
        if ours.contains(&line.source) {
            s.own += 1;
            match got {
                Some(source) if source == line.source => s.right += 1,
                found => {
                    s.other += usize::from(found.is_some());
                    s.none += usize::from(found.is_none());
                    *s.missed_by_source
                        .entry(line.source.to_owned())
                        .or_default() += 1;
                }
            }
        } else {
            s.foreign += 1;
            s.claimed += usize::from(got.is_some());
        }
    }
    s
}

#[test]
fn the_detector_scores_on_raw_and_parsed_input() {
    let Some(lines) = loader::load() else {
        println!(
            "no compat corpus at {}, measured nothing",
            loader::corpus_root().display()
        );
        return;
    };
    let mut ratchets = Ratchets::load(Path::new(env!("CARGO_MANIFEST_DIR")));
    for (name, shape) in [("raw", raw as Shape), ("parsed", parsed as Shape)] {
        let s = score(&lines, shape);
        println!(
            "{name}: own {}/{} right, {} named another source, {} unmatched. \
             foreign {}/{} claimed",
            s.right, s.own, s.other, s.none, s.claimed, s.foreign
        );
        assert!(
            s.own >= MIN_OWN_EVENTS,
            "{name}: only {} own events were read",
            s.own
        );
        ratchets.check(
            &format!("detect_{name}_own_missed"),
            s.own - s.right,
            "own events the detector missed",
        );
        ratchets.check(
            &format!("detect_{name}_foreign_claimed"),
            s.claimed,
            "events of undetectable sources the detector sent to a table",
        );
        ratchets.check_table(
            &format!("detect_{name}_missed_per_source"),
            &s.missed_by_source,
            "a source the detector misses more often",
        );
    }
    ratchets.save();
}
