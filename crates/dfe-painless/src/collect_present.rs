// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Several candidate fields collected into one list, the absent ones dropped,
//! and a lone survivor unwrapped into the scalar.
//!
//! `trend_micro_vision_one/telemetry` writes eight of these, one per ECS field
//! that several vendor fields could fill.
//!
//! `FirstElement` claimed all eight and wrote nothing. Its trigger is the
//! closing `[0];`, so it read the last statement alone -- unwrap `ctx.file.size`
//! if it is a one-element list -- and the list it was told to unwrap is the one
//! the two statements above it build. The script counted as HANDLED throughout,
//! so no unhandled-script count could show it: only the corpus does.
//!
//! Order is not incidental and duplicates are not noise. The captured output
//! carries `[334168, 334168, 57528]` on one event and a three-member
//! `file.created` where the script names four candidates, so this is a plain
//! concatenation in script order with the nulls taken out -- not a set, and not
//! a first-of.
//!
//! An EMPTY result stays an empty list, which is what Painless leaves behind.
//! The pipeline's closing prune -- `DropEmpty` with `empty_collections` -- takes
//! it away again, so the captured documents carry no key at all where nothing
//! was present. Writing nothing here would agree on those events and disagree
//! with any pipeline that has no such prune.

use serde_json::Value;

use crate::params::{balanced, ctx_path_plain, skip_trivia};
use dfe_core::Event;

/// The candidates one script collects, and where the result lands.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CollectPresent {
    /// The `ctx.` path the list is written to.
    target: String,
    /// The `ctx.` paths read as candidates, in the order the script lists them.
    sources: Vec<String>,
}

impl CollectPresent {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(target: impl Into<String>, sources: Vec<String>) -> Self {
        Self {
            target: target.into(),
            sources,
        }
    }
}

/// Collect the present candidates, in script order, and unwrap a lone one.
///
/// A candidate that is absent and one that is explicitly null are the same
/// thing to Painless -- `ctx.a?.b` on a missing `a` evaluates to null, and the
/// `removeIf` drops both.
pub fn collect_present(event: &mut Event, pattern: &CollectPresent) -> bool {
    let mut values: Vec<Value> = Vec::with_capacity(pattern.sources.len());
    for source in &pattern.sources {
        match event.get(source) {
            None | Some(Value::Null) => {}
            Some(value) => values.push(value.clone()),
        }
    }
    // `if (ctx.<t>.size() == 1) ctx.<t> = ctx.<t>[0];` -- one candidate is
    // stored as the scalar, and every other count stays a list. The captured
    // documents carry both forms on the same field.
    if values.len() == 1 {
        let _ = event.set(&pattern.target, values.remove(0));
    } else {
        let _ = event.set(&pattern.target, Value::Array(values));
    }
    true
}

/// Read the whole three-statement pattern off the script, or decline.
///
/// Every statement has to be one of the forms below and they all have to name
/// the SAME target, because the target is what ties a build to the prune and
/// the unwrap that follow it. A script carrying anything else declines whole:
/// half of this pattern is a list left un-pruned or a prune with nothing to
/// prune, and either writes a value Elasticsearch does not.
#[must_use]
pub fn parse_collect_present(script: &str) -> Option<CollectPresent> {
    let rest = skip_trivia(script);
    // The container init is noise -- `Event::set` builds the parents anyway --
    // but it has to be CONSUMED, because the `==` in its test would otherwise
    // be read as the assignment below.
    let rest = strip_container_init(rest).unwrap_or(rest);

    let (target, sources, rest) = parse_build(rest)?;
    let rest = strip_null_prune(rest, &target)?;
    let rest = strip_singleton_unwrap(rest, &target)?;
    skip_trivia(rest)
        .is_empty()
        .then_some(CollectPresent { target, sources })
}

/// `if (ctx.<container> == null) ctx.<container> = [:];`, where both halves
/// name the same container.
fn strip_container_init(script: &str) -> Option<&str> {
    let rest = skip_trivia(script).strip_prefix("if")?;
    let (guard, rest) = balanced(skip_trivia(rest), '(', ')')?;
    let container = ctx_path_plain(guard.trim().strip_suffix("== null")?)?;
    let (statement, rest) = skip_trivia(rest).split_once(';')?;
    let (lhs, rhs) = statement.split_once('=')?;
    (ctx_path_plain(lhs)? == container && rhs.trim() == "[:]").then_some(rest)
}

/// `ctx.<target> = [ ctx.<a>, ctx.<b>, ... ];` -- the target, its candidates in
/// order, and whatever follows.
fn parse_build(script: &str) -> Option<(String, Vec<String>, &str)> {
    let (lhs, rest) = skip_trivia(script).split_once('=')?;
    let target = ctx_path_plain(lhs)?;
    let (body, rest) = balanced(skip_trivia(rest), '[', ']')?;
    let rest = skip_trivia(rest).strip_prefix(';')?;

    // EVERY member has to be a bare `ctx.` read. One that is a call, a literal
    // or a subscript makes this a different script, and taking the members it
    // does understand would write a list short of what Painless builds.
    let sources: Vec<String> = body
        .split(',')
        .map(ctx_path_plain)
        .collect::<Option<Vec<String>>>()?;
    (!sources.is_empty()).then_some((target, sources, rest))
}

/// `ctx.<target>.removeIf(v -> v == null);` and nothing else.
fn strip_null_prune<'a>(script: &'a str, target: &str) -> Option<&'a str> {
    let rest = skip_trivia(script).strip_prefix(&format!("ctx.{target}.removeIf"))?;
    let (lambda, rest) = balanced(skip_trivia(rest), '(', ')')?;
    // Any other predicate is a different prune -- a sentinel removal, or the
    // drop-empty family -- and belongs to the matcher that reads one.
    (without_spaces(lambda) == "v->v==null")
        .then(|| skip_trivia(rest).strip_prefix(';'))
        .flatten()
}

/// `if (ctx.<target>.size() == 1) ctx.<target> = ctx.<target>[0];`, braced or
/// not.
fn strip_singleton_unwrap<'a>(script: &'a str, target: &str) -> Option<&'a str> {
    let rest = skip_trivia(script).strip_prefix("if")?;
    let (guard, rest) = balanced(skip_trivia(rest), '(', ')')?;
    if without_spaces(guard) != format!("ctx.{target}.size()==1") {
        return None;
    }
    let rest = skip_trivia(rest);
    // Braced or bare. The vendored spelling is bare; the braced one means the
    // same thing, and a block holding a SECOND statement does not.
    let (statement, after) = match balanced(rest, '{', '}') {
        Some((body, after)) => {
            let (statement, tail) = body.split_once(';')?;
            if !skip_trivia(tail).is_empty() {
                return None;
            }
            (statement, after)
        }
        None => rest.split_once(';')?,
    };
    (without_spaces(statement) == format!("ctx.{target}=ctx.{target}[0]")).then_some(after)
}

/// An expression with every space removed, so spacing is not part of a match.
fn without_spaces(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "collect_present_tests.rs"]
mod tests;
