// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `FirstElement` must carry EVERY take its script writes, not just the first.
//!
//! checkpoint_harmony_endpoint unwraps two paths in one script, and reading
//! only the first `[0];` left `host.os.version` a one-element list on all 19
//! of the source's events.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_core::Event;
use dfe_painless::plan::{PainlessPlan, painless_exec_plan};
use serde_json::json;

/// Verbatim from `checkpoint_harmony_endpoint_antibot/default.rs`.
const CHECKPOINT_OS: &str = r"if (ctx.host.os.name instanceof List && ctx.host.os.name.size() == 1)\n    ctx.host.os.name = ctx.host.os.name[0];\nif (ctx.host.os.version instanceof List && ctx.host.os.version.size() == 1)\n    ctx.host.os.version = ctx.host.os.version[0];";

/// The single-path spelling the other six checkpoint modules use.
const CHECKPOINT_URL: &str = r"if (ctx.url.original instanceof List && ctx.url.original.size() == 1)\n    ctx.url.original = ctx.url.original[0];";

/// crowdstrike's unguarded take, which must keep selecting the first of many.
const CROWDSTRIKE_TAKE: &str = "ctx.source.ip = ctx.crowdstrike.alert.source_ips[0];";

fn run(script: &str, doc: serde_json::Value) -> serde_json::Value {
    let mut event = Event::new(doc);
    let plan = PainlessPlan::new(script);
    assert!(plan.matches(), "the plan claimed nothing");
    painless_exec_plan(&mut event, &plan).unwrap();
    event.as_value().clone()
}

#[test]
fn both_paths_of_the_checkpoint_script_are_unwrapped() {
    let out = run(
        CHECKPOINT_OS,
        json!({"host": {"os": {"name": ["Windows"], "version": ["10.0-19045-SP0.0-SMP"]}}}),
    );
    assert_eq!(out["host"]["os"]["name"], json!("Windows"));
    assert_eq!(out["host"]["os"]["version"], json!("10.0-19045-SP0.0-SMP"));
}

#[test]
fn the_single_path_spelling_still_unwraps() {
    let out = run(CHECKPOINT_URL, json!({"url": {"original": ["http://a/b"]}}));
    assert_eq!(out["url"]["original"], json!("http://a/b"));
}

#[test]
fn a_size_guarded_take_leaves_a_longer_list_alone() {
    // Painless never enters the branch, so every element survives. Taking the
    // first here would drop the rest and read as a scalar Elasticsearch never
    // wrote.
    let out = run(
        CHECKPOINT_OS,
        json!({"host": {"os": {"name": ["Windows", "Server"], "version": ["10.0"]}}}),
    );
    assert_eq!(out["host"]["os"]["name"], json!(["Windows", "Server"]));
    assert_eq!(out["host"]["os"]["version"], json!("10.0"));
}

#[test]
fn an_unguarded_take_still_selects_the_first_of_many() {
    let out = run(
        CROWDSTRIKE_TAKE,
        json!({"crowdstrike": {"alert": {"source_ips": ["10.0.0.1", "10.0.0.2"]}}}),
    );
    assert_eq!(out["source"]["ip"], json!("10.0.0.1"));
}
