// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `prisma_access`'s two repairs of what the CEF decoder handed the pipeline.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// `prisma_access/event`, processor `Script to Rename EM`: take the decoder's
/// own field-parse complaints out of `error.message`.
///
/// The list is REBUILT rather than filtered in place, and the write sits inside
/// the loop -- so an `error.message` that is already an empty list is left
/// exactly as it was rather than being rewritten to another empty one.
fn filter_decoder_errors(event: &mut Event, _params: &Value) {
    let Some(messages) = event.get_array("error.message") else {
        return;
    };
    if messages.is_empty() {
        return;
    }
    // An element that is not a string has no `contains` to answer with, so it
    // survives the filter.
    let kept: Vec<Value> = messages
        .iter()
        .filter(|message| {
            message.as_str().is_none_or(|text| {
                !(text.contains("error in field ") || text.contains("strconv.Parse"))
            })
        })
        .cloned()
        .collect();
    let _ = event.set("error.message", Value::Array(kept));
}

/// `prisma_access/event`, processor `script_set_host_cpu_usage`:
/// `host.cpu.usage` as the fraction ECS wants from the vendor's own reading.
///
/// `host.cpu` is REPLACED with an empty map first, so anything an earlier
/// processor put under it is gone before the usage is written.
fn host_cpu_usage(event: &mut Event, _params: &Value) {
    let Some(usage) = event.get_f64("prisma_access.event.cpu_usage") else {
        return;
    };
    let _ = event.set("host.cpu", Value::Object(Map::new()));
    let _ = event.set("host.cpu.usage", java_round(usage * 10.0) / 1000.0);
}

/// Java's `Math.round`: the closest whole number, with a tie going towards
/// positive infinity.
///
/// Flooring the value plus a half would round 0.49999999999999994 up, which
/// Java does not, so the fraction is compared instead. The result stays an
/// `f64` because the only thing done with it here is the division.
fn java_round(value: f64) -> f64 {
    let floor = value.floor();
    if value - floor >= 0.5 {
        floor + 1.0
    } else {
        floor
    }
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "5aa6e27ecc8384cd1929508c602b8642524dfad68161105d2e42b3ea52b2a882",
        source: "prisma_access",
        name: "filter_decoder_errors",
        run: filter_decoder_errors,
    },
    Entry {
        hash: "5466ce091263e1e55c842786d1e44920959403ecc27823b8c6236ac3865b61dc",
        source: "prisma_access",
        name: "host_cpu_usage",
        run: host_cpu_usage,
    },
];
