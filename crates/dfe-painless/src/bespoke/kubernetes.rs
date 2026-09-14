// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `kubernetes`'s audit annotations, whose keys are rewritten out of the
//! dotted form Elasticsearch would read as a nesting.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// The annotation the authoriser writes its verdict into.
const DECISION: &str = "authorization.k8s.io/decision";

/// `kubernetes/audit_logs`, the untagged annotations processor:
/// `kubernetes.audit.annotations` re-keyed, and `event.outcome` read off the
/// authoriser's decision.
///
/// A key such as `authorization.k8s.io/decision` is one annotation name, not a
/// path, so every dot in it becomes an underscore before the map goes back into
/// the document.
fn rewrite_annotations(event: &mut Event, _params: &Value) {
    let (updated, outcome) = {
        let Some(annotations) = event.get_object("kubernetes.audit.annotations") else {
            return;
        };
        let mut updated = Map::with_capacity(annotations.len());
        for (key, value) in annotations {
            updated.insert(key.replace('.', "_"), value.clone());
        }
        let outcome = match annotations.get(DECISION).and_then(Value::as_str) {
            Some("allow") => Some("success"),
            Some("forbid") => Some("failure"),
            _ => None,
        };
        (updated, outcome)
    };

    // Painless raises on the assignment where `ctx.event` is absent.
    if let Some(outcome) = outcome
        && event.has("event")
    {
        let _ = event.set("event.outcome", outcome);
    }
    event.update("kubernetes.audit.annotations", Value::Object(updated));
}

/// The transcription this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "15f2a7dc09b2428edf55df7593a862574cee51690d577d9c8164a1fdc5766ef1",
    source: "kubernetes",
    name: "rewrite_annotations",
    run: rewrite_annotations,
}];
