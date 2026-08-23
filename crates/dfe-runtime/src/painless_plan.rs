// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The per-call-site plan for a Painless script.
//!
//! A generated call site's script text is a literal that never changes, yet
//! [`crate::codegen_api::painless_exec`] was re-deciding WHICH matcher runs on
//! every event -- up to 116 substring scans over constant text, at 20k events
//! a batch. The decision is a property of the text alone, so [`PainlessPlan`]
//! makes it once: [`cached_painless`] holds the plan in a site-local
//! `OnceLock`, the same shape as [`crate::cached_grok`], and per event only
//! the matchers the text actually triggers are run.
//!
//! What the plan carries beyond the routing: whatever the trigger's own parse
//! already recovered -- a drop-empty script's [`DropPolicy`], an equality
//! ladder's arms -- so those parses stop running per event too.

use serde_json::Value;
use tracing::debug;

use crate::error::Result;
use crate::event::Event;
use crate::painless_common::{KnownShape, known_shapes, normalise, run_known_shape};
use crate::painless_params::{ParamsShape, params_shape, run_params_shape};

#[cfg(doc)]
use crate::painless_common::DropPolicy;

/// A script's normalised text and the matcher branches its text triggers.
///
/// Built once per call site by [`cached_painless`]; executed per event by
/// [`painless_exec_plan`] / [`painless_exec_plan_params`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PainlessPlan {
    text: String,
    params: Option<ParamsShape>,
    known: Vec<KnownShape>,
}

impl PainlessPlan {
    /// Resolve the script's escapes and decide its dispatch, once.
    #[must_use]
    pub fn new(script: &str) -> Self {
        let text = normalise(script).into_owned();
        Self {
            params: params_shape(&text),
            known: known_shapes(&text),
            text,
        }
    }

    /// The script with its escapes resolved, for callers that log or count it.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Execute a planned Painless script that carries no `params` block.
///
/// # Errors
///
/// Never fails today: an unrecognised script is counted and skipped, exactly
/// as [`crate::codegen_api::painless_exec`] behaves.
pub fn painless_exec_plan(event: &mut Event, plan: &PainlessPlan) -> Result<()> {
    painless_exec_plan_params(event, plan, &Value::Null)
}

/// Execute a planned Painless script against its `params` block.
///
/// The params matcher the text triggers runs first, then the text-only
/// matchers the plan lists, in dispatch order -- the same fall-through the
/// string-typed entry points walk, minus the per-event trigger scans.
///
/// # Errors
///
/// Never fails today; see [`painless_exec_plan`].
pub fn painless_exec_plan_params(
    event: &mut Event,
    plan: &PainlessPlan,
    params: &Value,
) -> Result<()> {
    if let Some(shape) = &plan.params
        && let Some(map) = params.as_object()
        && run_params_shape(event, &plan.text, map, shape)
    {
        crate::painless_stats::record_handled();
        return Ok(());
    }
    if plan
        .known
        .iter()
        .any(|shape| run_known_shape(event, &plan.text, shape))
    {
        crate::painless_stats::record_handled();
        return Ok(());
    }
    // Counted, because an uncounted skip is indistinguishable from a script
    // that did nothing.
    crate::painless_stats::record_unhandled(&plan.text);
    debug!(
        script_len = plan.text.len(),
        "painless_exec: unrecognised script skipped"
    );
    Ok(())
}

/// A Painless script's [`PainlessPlan`], built once per CALL SITE.
///
/// Replaces `cached_script!` at generated sites: where that macro cached only
/// the escape resolution, this also caches the dispatch decision and the
/// trigger-parses' results, so per event nothing constant is re-derived.
#[macro_export]
macro_rules! cached_painless {
    ($script:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<$crate::painless_plan::PainlessPlan> =
            ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::painless_plan::PainlessPlan::new($script))
    }};
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::codegen_api::painless_exec_params;

    /// Scripts that between them route through a params matcher, an early
    /// text matcher, a guarded fall-through, the named tail, and no matcher
    /// at all -- with the events that make each branch fire.
    fn corpus() -> Vec<(&'static str, Value, Value)> {
        vec![
            (
                // Params: sentinel removal, verbatim from crowdstrike.
                "ctx.crowdstrike.event.entrySet().removeIf(entry -> \
                 params.values.contains(entry.getValue()));\\n",
                json!({ "values": [null, "", "-", "N/A", "NA", 0] }),
                json!({ "crowdstrike": { "event": { "keep": "v", "dash": "-" } } }),
            ),
            (
                // Text, first branch: the drop-empty shape with its policy.
                "boolean drop(Object o) { if (o == null || o == '') return true; \
                 if (o instanceof Map) { ((Map) o).values().removeIf(v -> drop(v)); \
                 return ((Map) o).size() == 0; } if (o instanceof List) { \
                 ((List) o).removeIf(v -> drop(v)); return ((List) o).length == 0; } \
                 return false; } drop(ctx);",
                Value::Null,
                json!({ "a": "", "b": null, "c": { "d": null }, "keep": "x" }),
            ),
            (
                // Text, guarded fall-through: sum-of-fields declines when a
                // side is absent, and nothing later claims the script.
                "ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes;",
                Value::Null,
                json!({ "source": { "bytes": 7 } }),
            ),
            (
                // Named tail: the okta.target rename.
                "for (item in ctx.okta.target) {\\n  if (item.alternateId != null) \
                 {\\n    item.alternate_id = item.alternateId;\\n  }\\n}",
                Value::Null,
                json!({ "okta": { "target": [ { "alternateId": "a@b.c" } ] } }),
            ),
            (
                // Unhandled: nothing matches, the event must come back intact.
                "def splitFancy(String input) { return input; }",
                Value::Null,
                json!({ "message": "x" }),
            ),
        ]
    }

    /// The plan path and the string path are the same dispatch: same event
    /// afterwards, whichever way the script went in.
    #[test]
    fn plan_path_matches_string_path() {
        // Executing scripts records into the process-global stats, which the
        // stats tests read -- one lock keeps the counting tests honest.
        let _guard = crate::painless_stats::serialised();
        for (script, params, input) in corpus() {
            let plan = PainlessPlan::new(script);

            let mut via_plan = Event::new(input.clone());
            painless_exec_plan_params(&mut via_plan, &plan, &params).unwrap();

            let mut via_string = Event::new(input);
            // The string path expects the escapes already resolved, which is
            // what `cached_script!` hands it.
            let normalised = normalise(script).into_owned();
            painless_exec_params(&mut via_string, &normalised, &params).unwrap();

            assert_eq!(
                via_plan.as_value(),
                via_string.as_value(),
                "script diverged: {script}"
            );
        }
    }

    /// The macro caches: two expansions of one literal are one plan.
    #[test]
    fn cached_painless_returns_the_same_plan() {
        fn site() -> &'static PainlessPlan {
            cached_painless!("ctx.a = ctx.b + ctx.c;")
        }
        assert!(std::ptr::eq(site(), site()));
    }

    /// A drop-empty plan carries its policy, parsed once.
    #[test]
    fn the_plan_carries_the_trigger_parse() {
        let plan = PainlessPlan::new(
            "boolean drop(Object o) { if (o == null || o == '') return true; \
             if (o instanceof Map) { ((Map) o).values().removeIf(v -> drop(v)); \
             return ((Map) o).size() == 0; } if (o instanceof List) { \
             ((List) o).removeIf(v -> drop(v)); return ((List) o).length == 0; } \
             return false; } drop(ctx);",
        );
        assert!(
            matches!(
                plan.known.as_slice(),
                [KnownShape::DropEmpty { policy, root: None }] if policy.empty_strings
            )
        );
    }
}
