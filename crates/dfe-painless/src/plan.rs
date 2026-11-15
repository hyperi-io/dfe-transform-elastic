// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The per-call-site plan for a Painless script.
//!
//! A generated call site's script text is a literal that never changes, yet
//! [`painless_exec`] was re-deciding WHICH matcher runs on every event -- up
//! to 116 substring scans over constant text, at 20k events a batch. The
//! decision is a property of the text alone, so [`PainlessPlan`] makes it
//! once: `cached_painless!` holds the plan in a site-local `OnceLock`, the
//! same pattern as `cached_grok`, and per event only the matchers the text
//! actually triggers are run.
//!
//! What the plan carries beyond the routing: whatever the trigger's own parse
//! already recovered -- a drop-empty script's [`DropPolicy`], an equality
//! ladder's arms -- so those parses stop running per event too.
//!
//! A hand-written script equivalent ([`crate::bespoke`]) is resolved here too, ahead
//! of every matcher: the table is keyed by the script's hash, so the lookup is
//! one hash per call site and nothing per event.

use serde_json::Value;
use tracing::debug;

use crate::bespoke;
use crate::common::{KnownPattern, known_patterns, normalise, run_known_pattern};
use crate::params::{ParamsPattern, params_pattern, run_params_pattern};
use dfe_core::error::Result;
use dfe_core::event::Event;

#[cfg(doc)]
use crate::common::DropPolicy;

/// A script's key, the matcher branches its text triggers, and the text
/// itself only where a runner still reads it.
///
/// Built once per call site by `compiled_painless!` from the plan the generator
/// resolved, or by [`PainlessPlan::new`] from text; executed per event by
/// [`painless_exec_plan`] / [`painless_exec_plan_params`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PainlessPlan {
    hash: String,
    text: Option<String>,
    bespoke: Option<&'static bespoke::Entry>,
    params: Option<ParamsPattern>,
    known: Vec<KnownPattern>,
}

/// The serialisable half of a [`PainlessPlan`]: what dispatch needs, and no text.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompiledPlan {
    params: Option<ParamsPattern>,
    known: Vec<KnownPattern>,
}

impl PainlessPlan {
    /// Resolve the script's escapes and decide its dispatch, once.
    #[must_use]
    pub fn new(script: &str) -> Self {
        let text = normalise(script).into_owned();
        Self {
            hash: bespoke::script_hash(&text),
            bespoke: bespoke::lookup(&text),
            params: params_pattern(&text),
            known: known_patterns(&text),
            text: Some(text),
        }
    }

    /// Rebuild a plan the generator resolved, keyed by its script's hash.
    ///
    /// `text` is the script, passed only where [`Self::reads_text`] held at
    /// generation time, and normalised here as [`Self::new`] does.
    ///
    /// # Panics
    ///
    /// Panics on a plan the generator did not write, which is a build defect.
    #[allow(
        clippy::panic,
        reason = "a generated plan that does not decode is a build defect, caught by the first test that reaches it"
    )]
    #[must_use]
    pub fn compiled(hash: &str, plan: &str, text: Option<&str>) -> Self {
        let decoded: CompiledPlan = serde_json::from_str(plan)
            .unwrap_or_else(|e| panic!("plan for script {hash} does not decode: {e}"));
        Self {
            hash: hash.to_string(),
            text: text.map(|script| normalise(script).into_owned()),
            bespoke: bespoke::lookup_hash(hash),
            params: decoded.params,
            known: decoded.known,
        }
    }

    /// The serialisable half of this plan, for the generator to emit.
    #[must_use]
    pub fn compiled_plan(&self) -> CompiledPlan {
        CompiledPlan {
            params: self.params.clone(),
            known: self.known.clone(),
        }
    }

    /// The key the script is registered and counted under.
    #[must_use]
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// The script with its escapes resolved, where the plan still carries it.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Whether a runner this plan binds still reads the script text per call.
    ///
    /// A registered bespoke runner wins dispatch and never reads it.
    #[must_use]
    pub fn reads_text(&self) -> bool {
        self.bespoke.is_none()
            && (self.params.as_ref().is_some_and(ParamsPattern::reads_text)
                || self.known.iter().any(KnownPattern::reads_text))
    }

    /// Whether any matcher claims this script's text.
    ///
    /// Answers the question STATICALLY, so a script from a package with no
    /// transform can be measured. Triggering is not the same as being right --
    /// the compat corpus in dfe-transform-elastic-dev is what measures that.
    #[must_use]
    pub fn matches(&self) -> bool {
        self.bespoke.is_some() || self.params.is_some() || !self.known.is_empty()
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
    /// be reproduced from extracted parts alone. A bespoke entry declines it
    /// too, so the site keeps its literal and the table serves it at run time.
    #[cfg(feature = "codegen")]
    #[must_use]
    pub fn direct_call(&self) -> Option<Vec<String>> {
        if !self.matches() || self.bespoke.is_some() || self.params.is_some() {
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
        self.bespoke
            .iter()
            .map(|entry| format!("{entry:?}"))
            .chain(self.params.iter().map(|pattern| format!("{pattern:?}")))
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
    let hash = bespoke::script_hash(&normalise(script));
    if let Some(entry) = bespoke::lookup_hash(&hash) {
        (entry.run)(event, params);
        crate::stats::record_handled(&hash);
        return Ok(());
    }
    if crate::params::try_params_painless(event, script, params) {
        crate::stats::record_handled(&hash);
        return Ok(());
    }
    if crate::common::try_known_painless(event, script) {
        crate::stats::record_handled(&hash);
        return Ok(());
    }
    // Counted, because an uncounted skip is indistinguishable from a script
    // that did nothing.
    crate::stats::record_unhandled(&hash);
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
    let text = plan.text.as_deref().unwrap_or_default();
    if let Some(entry) = plan.bespoke {
        (entry.run)(event, params);
        crate::stats::record_handled(&plan.hash);
        return Ok(());
    }
    if let Some(pattern) = &plan.params
        && let Some(map) = params.as_object()
        && run_params_pattern(event, text, map, pattern)
    {
        crate::stats::record_handled(&plan.hash);
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
        .any(|pattern| run_known_pattern(event, text, pattern))
    {
        crate::stats::record_handled(&plan.hash);
        return Ok(());
    }
    // Counted, because an uncounted skip is indistinguishable from a script
    // that did nothing.
    crate::stats::record_unhandled(&plan.hash);
    debug!(
        script = %plan.hash,
        "painless_exec: unrecognised script skipped"
    );
    Ok(())
}

/// A generated call site's [`PainlessPlan`], decoded once per CALL SITE from the
/// plan the generator resolved and keyed by the script's hash.
///
/// The third literal, the normalised text, appears only where a bound runner
/// still reads it.
#[macro_export]
macro_rules! compiled_painless {
    ($hash:literal, $plan:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<$crate::plan::PainlessPlan> =
            ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::plan::PainlessPlan::compiled($hash, $plan, None))
    }};
    ($hash:literal, $plan:literal, $text:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<$crate::plan::PainlessPlan> =
            ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::plan::PainlessPlan::compiled($hash, $plan, Some($text)))
    }};
}

/// A Painless script's [`PainlessPlan`] built from its text, once per call site.
#[macro_export]
macro_rules! cached_painless {
    ($script:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<$crate::plan::PainlessPlan> =
            ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::plan::PainlessPlan::new($script))
    }};
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "plan_tests.rs"]
mod tests;
