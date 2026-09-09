// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The per-call-site plan for a Painless script.
//!
//! A generated call site's script text is a literal that never changes, yet
//! [`painless_exec`] was re-deciding WHICH matcher runs on every event -- up
//! to 116 substring scans over constant text, at 20k events a batch. The
//! decision is a property of the text alone, so [`PainlessPlan`] makes it
//! once: [`cached_painless`] holds the plan in a site-local `OnceLock`, the
//! same pattern as `cached_grok`, and per event only the matchers the text
//! actually triggers are run.
//!
//! What the plan carries beyond the routing: whatever the trigger's own parse
//! already recovered -- a drop-empty script's [`DropPolicy`], an equality
//! ladder's arms -- so those parses stop running per event too.

use serde_json::Value;
use tracing::debug;

use crate::common::{KnownPattern, known_patterns, normalise, run_known_pattern};
use crate::params::{ParamsPattern, params_pattern, run_params_pattern};
use dfe_core::error::Result;
use dfe_core::event::Event;

#[cfg(doc)]
use crate::common::DropPolicy;

/// A script's normalised text and the matcher branches its text triggers.
///
/// Built once per call site by [`cached_painless`]; executed per event by
/// [`painless_exec_plan`] / [`painless_exec_plan_params`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PainlessPlan {
    text: String,
    params: Option<ParamsPattern>,
    known: Vec<KnownPattern>,
}

impl PainlessPlan {
    /// Resolve the script's escapes and decide its dispatch, once.
    #[must_use]
    pub fn new(script: &str) -> Self {
        let text = normalise(script).into_owned();
        Self {
            params: params_pattern(&text),
            known: known_patterns(&text),
            text,
        }
    }

    /// The script with its escapes resolved, for callers that log or count it.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Whether any matcher claims this script's text.
    ///
    /// Answers the question STATICALLY, so a script from a package with no
    /// transform can be measured. Triggering is not the same as being right --
    /// `tests/compat_corpus.rs` is what measures that.
    #[must_use]
    pub fn matches(&self) -> bool {
        self.params.is_some() || !self.known.is_empty()
    }

    /// The runner calls that reproduce this script, for a generator emitting
    /// them in place of the script itself.
    ///
    /// `None` unless the WHOLE plan is expressible. Dispatch runs the params
    /// pattern first and skips the text patterns when it succeeds, so emitting a
    /// half-resolved plan would run branches the ladder would not.
    ///
    /// A params pattern declines the whole plan: every params runner still reads
    /// the script text or the `params` block at run time, so none of them can
    /// be reproduced from extracted parts alone.
    #[cfg(feature = "codegen")]
    #[must_use]
    pub fn direct_call(&self) -> Option<Vec<String>> {
        if !self.matches() || self.params.is_some() {
            return None;
        }
        self.known.iter().map(KnownPattern::direct_call).collect()
    }

    /// The matchers this text binds to, in dispatch order, each rendered with
    /// whatever its trigger's parse recovered.
    ///
    /// One list rather than the two fields, because dispatch tries the params
    /// pattern first and the order is the answer.
    #[must_use]
    pub fn binding(&self) -> Vec<String> {
        self.params
            .iter()
            .map(|pattern| format!("{pattern:?}"))
            .chain(self.known.iter().map(|pattern| format!("{pattern:?}")))
            .collect()
    }
}

/// Execute a Painless script against an event, deciding its matcher here.
///
/// The unplanned path: it re-scans the text every call, which is why a
/// generated call site uses [`painless_exec_plan`] instead. Kept because the
/// two must agree, and the equivalence test below is what says they do.
///
/// # Errors
///
/// Never fails today: an unrecognised script is counted and skipped.
pub fn painless_exec(event: &mut Event, script: &str) -> Result<()> {
    painless_exec_params(event, script, &Value::Null)
}

/// Execute a Painless script that carries a `params` block.
///
/// The recurring params patterns -- sentinel lists, field lists, lookup tables
/// -- read their whole behaviour out of `params`, so the script text alone
/// cannot run them. The generated code passes the pipeline's params block
/// verbatim.
///
/// # Errors
///
/// Never fails today: an unrecognised script is counted and skipped.
pub fn painless_exec_params(event: &mut Event, script: &str, params: &Value) -> Result<()> {
    if crate::params::try_params_painless(event, script, params) {
        crate::stats::record_handled(script);
        return Ok(());
    }
    if crate::common::try_known_painless(event, script) {
        crate::stats::record_handled(script);
        return Ok(());
    }
    // Counted, because an uncounted skip is indistinguishable from a script
    // that did nothing.
    crate::stats::record_unhandled(script);
    debug!(
        script_len = script.len(),
        "painless_exec: unrecognised script skipped"
    );
    Ok(())
}

/// Execute a planned Painless script that carries no `params` block.
///
/// # Errors
///
/// Never fails today: an unrecognised script is counted and skipped, exactly
/// as [`painless_exec`] behaves.
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
    if let Some(pattern) = &plan.params
        && let Some(map) = params.as_object()
        && run_params_pattern(event, &plan.text, map, pattern)
    {
        crate::stats::record_handled(&plan.text);
        return Ok(());
    }
    // FALL-THROUGH, and deliberately so: the listed patterns are ALTERNATIVES,
    // and the first to claim the script wins. Running them all instead was
    // measured and is worse -- fortinet_fortigate went from 0 to 11 fields
    // wrong and symantec_endpoint from 32 matched events to 25, because a
    // later arm that the winner had been shadowing overwrites what it wrote.
    // Do not re-try it; a script needing two patterns needs ONE that does both.
    if plan
        .known
        .iter()
        .any(|pattern| run_known_pattern(event, &plan.text, pattern))
    {
        crate::stats::record_handled(&plan.text);
        return Ok(());
    }
    // Counted, because an uncounted skip is indistinguishable from a script
    // that did nothing.
    crate::stats::record_unhandled(&plan.text);
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
        static SITE: ::std::sync::OnceLock<$crate::plan::PainlessPlan> =
            ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::plan::PainlessPlan::new($script))
    }};
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn painless_exec_unknown_script_noop() {
        let _guard = crate::stats::serialised();
        let mut event = Event::new(json!({"field": "value"}));
        let result = painless_exec(&mut event, "unknown_script_that_does_nothing();");
        assert!(result.is_ok());
        assert_eq!(event.get_str("field"), Some("value"));
    }

    #[test]
    fn painless_exec_drop_empty_known() {
        let _guard = crate::stats::serialised();
        let mut event = Event::new(json!({"a": "", "b": "keep", "c": null}));
        let result = painless_exec(
            &mut event,
            r#"boolean drop(Object o) { if (o == null || o == "") { return true; } }"#,
        );
        assert!(result.is_ok());
    }

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
                // Text, first branch: the drop-empty pattern with its policy.
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
        let _guard = crate::stats::serialised();
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

    /// The patterns a generator can emit today, each rendering to the call
    /// the ladder would have made. The first four are verbatim scripts from
    /// `pipelines/fortinet_fortiproxy/log/default.yml`, which is the source
    /// the direct-emit spike regenerates.
    ///
    /// The drop-empty pair is here twice on purpose: a direct call BAKES the
    /// policy into the shipped module, so the runtime reading and the emitted
    /// literal can drift apart silently. This is what says they have not.
    #[cfg(feature = "codegen")]
    #[test]
    fn an_expressible_plan_renders_the_call_the_ladder_would_make() {
        for (script, expected) in [
            (
                "if (ctx.log?.syslog?.priority == null) {\\n  return;\\n} \
                 def severity = [:]; severity['code'] = ctx.log.syslog.priority&0x7; \
                 ctx.log.syslog['severity'] = severity; def facility = [:]; \
                 facility['code'] = ctx.log.syslog.priority>>3; \
                 ctx.log.syslog['facility'] = facility;",
                "syslog_priority(event, &SyslogPriorityScript::new(None, true, true, false));",
            ),
            (
                "ctx.event['duration'] = ctx.event.duration * 1e9;",
                "scale_field(event, &ScaleField::new(\"event.duration\", \
                 \"event.duration\", Factor::Double(1000000000.0)));",
            ),
            (
                // The commonest pattern in the catalogue, 759 call sites.
                "boolean drop(Object o) { if (o == null || o == '') return true; \
                 if (o instanceof Map) { ((Map) o).values().removeIf(v -> drop(v)); \
                 return ((Map) o).size() == 0; } if (o instanceof List) { \
                 ((List) o).removeIf(v -> drop(v)); return ((List) o).length == 0; } \
                 return false; } drop(ctx);",
                "drop_empty(event, &DropPolicy { nulls: true, empty_strings: true, \
                 empty_collections: true, prune_lists: true, ..DropPolicy::none() }, None);",
            ),
            (
                // tychon gates the copy on ECS's closed vocabulary for
                // host.os.type, 36 call sites.
                "def value = ctx.tychon.host?.os?.family?.toLowerCase();\\n\
                 if (['linux', 'macos', 'unix', 'windows', 'ios', 'android'].contains(value)) {\\n  \
                 if (ctx.host == null) {\\n    ctx.host = [:];\\n  }\\n  \
                 if (ctx.host.os == null) {\\n    ctx.host.os = [:];\\n  }\\n  \
                 ctx.host.os.type = value;\\n}\\n",
                "allowed_value_copy(event, &AllowedValueCopy::new(\"tychon.host.os.family\", \
                 true, vec![\"linux\".into(), \"macos\".into(), \"unix\".into(), \
                 \"windows\".into(), \"ios\".into(), \"android\".into()], \"host.os.type\"));",
            ),
            (
                // The same prune under an `instanceof String` guard means the
                // OTHER axis -- mysql_enterprise and oracle, one call site
                // each. Read as a collection test it inverted both.
                "void handleMap(Map map) {\\n  for (def x : map.values()) {\\n    \
                 if (x instanceof Map) {\\n        handleMap(x);\\n    } \
                 else if (x instanceof List) {\\n        handleList(x);\\n    }\\n  }\\n  \
                 map.values().removeIf(v -> v instanceof String && v.isEmpty() == true);\\n}\\n\
                 void handleList(List list) {\\n  for (def x : list) {\\n      \
                 if (x instanceof Map) {\\n          handleMap(x);\\n      } \
                 else if (x instanceof List) {\\n          handleList(x);\\n      }\\n  }\\n}\\n\
                 handleMap(ctx);\\n",
                "drop_empty(event, &DropPolicy { empty_strings: true, \
                 ..DropPolicy::none() }, None);",
            ),
        ] {
            let plan = PainlessPlan::new(script);
            assert_eq!(
                plan.direct_call().as_deref(),
                Some([expected.to_string()].as_slice()),
                "script rendered wrong: {script}"
            );
        }
    }

    /// A script no matcher claims, and one whose matcher still needs the text,
    /// both decline -- the call site keeps the ladder rather than emitting a
    /// call that would run something else.
    #[cfg(feature = "codegen")]
    #[test]
    fn an_inexpressible_plan_declines() {
        assert!(
            PainlessPlan::new("def splitFancy(String input) { return input; }")
                .direct_call()
                .is_none()
        );
        // A params pattern declines the whole plan, however expressible its
        // text patterns are: dispatch tries params first and skips them.
        assert!(
            PainlessPlan::new(
                "def k = ctx.network.direction.toLowerCase(); def v = params.get(k); \
                 if (v != null) { ctx.network.direction = v; return; } \
                 ctx.network.direction = k;"
            )
            .direct_call()
            .is_none()
        );
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
        assert!(matches!(
            plan.known.as_slice(),
            [KnownPattern::DropEmpty { policy, root: None }] if policy.empty_strings
        ));
    }
}
