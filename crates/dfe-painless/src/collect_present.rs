// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Several candidate fields collected into one list, the absent ones dropped,
//! and a lone survivor unwrapped into the scalar.
//!
//! `trend_micro_vision_one/telemetry` writes eight of these, one per ECS field
//! that several vendor fields could fill:
//!
//! ```painless
//! if (ctx.file == null) ctx.file = [:];
//! ctx.file.size = [
//!   ctx.trend_micro_vision_one?.telemetry?.object_current_file_size,
//!   ctx.trend_micro_vision_one?.telemetry?.object_file_size,
//!   ctx.trend_micro_vision_one?.telemetry?.src_file_size,
//!   ctx.trend_micro_vision_one?.telemetry?.process_file_size,
//!   ctx.trend_micro_vision_one?.telemetry?.parent_file_size
//! ];
//! ctx.file.size.removeIf(v -> v == null);
//! if (ctx.file.size.size() == 1) ctx.file.size = ctx.file.size[0];
//! ```
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// Verbatim from `pipelines/trend_micro_vision_one/telemetry/default.yml`,
    /// the widest of the eight.
    const FILE_SIZE: &str = "if (ctx.file == null) ctx.file = [:];
ctx.file.size = [
  ctx.trend_micro_vision_one?.telemetry?.object_current_file_size,
  ctx.trend_micro_vision_one?.telemetry?.object_file_size,
  ctx.trend_micro_vision_one?.telemetry?.src_file_size,
  ctx.trend_micro_vision_one?.telemetry?.process_file_size,
  ctx.trend_micro_vision_one?.telemetry?.parent_file_size
];
ctx.file.size.removeIf(v -> v == null);
if (ctx.file.size.size() == 1) ctx.file.size = ctx.file.size[0];
";

    /// The two-candidate spelling, from the same file.
    const PROCESS_NAME: &str = "if (ctx.process == null) ctx.process = [:];
ctx.process.name = [
  ctx.trend_micro_vision_one?.telemetry?.process_name,
  ctx.trend_micro_vision_one?.telemetry?.object_name
];
ctx.process.name.removeIf(v -> v == null);
if (ctx.process.name.size() == 1) ctx.process.name = ctx.process.name[0];
";

    fn run(script: &str, doc: Value) -> Value {
        let pattern = parse_collect_present(script).expect("the script is recognised");
        let mut event = Event::new(doc);
        assert!(collect_present(&mut event, &pattern));
        event.as_value().clone()
    }

    /// Every candidate is read off the script, in the order it lists them.
    #[test]
    fn every_candidate_is_read_in_script_order() {
        let pattern = parse_collect_present(FILE_SIZE).expect("recognised");
        assert_eq!(pattern.target, "file.size");
        assert_eq!(
            pattern.sources,
            [
                "trend_micro_vision_one.telemetry.object_current_file_size",
                "trend_micro_vision_one.telemetry.object_file_size",
                "trend_micro_vision_one.telemetry.src_file_size",
                "trend_micro_vision_one.telemetry.process_file_size",
                "trend_micro_vision_one.telemetry.parent_file_size",
            ]
        );
    }

    /// Several present candidates stay a LIST, in script order, duplicates
    /// kept. The captured `file.size` on one telemetry event is
    /// `[334168, 334168, 57528]`.
    #[test]
    fn several_candidates_keep_their_order_and_their_duplicates() {
        let out = run(
            FILE_SIZE,
            json!({"trend_micro_vision_one": {"telemetry": {
                "object_current_file_size": 334_168,
                "object_file_size": 334_168,
                "src_file_size": 57_528,
            }}}),
        );
        assert_eq!(out["file"]["size"], json!([334_168, 334_168, 57_528]));
    }

    /// One present candidate is stored as the SCALAR, which is the script's own
    /// `size() == 1` unwrap. Four telemetry events carry `file.size` as 57528
    /// rather than a one-member list.
    #[test]
    fn a_lone_candidate_is_unwrapped_to_the_scalar() {
        let out = run(
            FILE_SIZE,
            json!({"trend_micro_vision_one": {"telemetry": {"src_file_size": 57_528}}}),
        );
        assert_eq!(out["file"]["size"], json!(57_528));
    }

    /// An explicit null is dropped the same way an absent field is: Painless
    /// reads both as null and the `removeIf` takes both.
    #[test]
    fn an_explicit_null_is_dropped_like_an_absent_field() {
        let out = run(
            PROCESS_NAME,
            json!({"trend_micro_vision_one": {"telemetry": {
                "process_name": Value::Null,
                "object_name": "svchost.exe",
            }}}),
        );
        assert_eq!(out["process"]["name"], json!("svchost.exe"));
    }

    /// Nothing present leaves the EMPTY list Painless leaves. The pipeline's
    /// closing prune is what removes it, and writing nothing here would
    /// disagree with a pipeline that has no such prune.
    #[test]
    fn nothing_present_leaves_the_empty_list() {
        let out = run(
            PROCESS_NAME,
            json!({"trend_micro_vision_one": {"telemetry": {}}}),
        );
        assert_eq!(out["process"]["name"], json!([]));
    }

    /// A candidate that is not a bare `ctx.` read declines the WHOLE script.
    /// Collecting the four it understands would write a list short of what
    /// Painless builds, and the event would then read as needing polish.
    #[test]
    fn a_candidate_that_is_not_a_path_declines_the_whole_script() {
        let broken = FILE_SIZE.replace(
            "ctx.trend_micro_vision_one?.telemetry?.src_file_size,",
            "sizeOf(ctx.trend_micro_vision_one?.telemetry?.src_file_size),",
        );
        assert_ne!(broken, FILE_SIZE, "the replacement has to bite");
        assert!(parse_collect_present(&broken).is_none());
    }

    /// A prune over some OTHER field is a different script.
    #[test]
    fn a_prune_naming_another_field_declines() {
        let broken = FILE_SIZE.replace(
            "ctx.file.size.removeIf(v -> v == null);",
            "ctx.file.path.removeIf(v -> v == null);",
        );
        assert!(parse_collect_present(&broken).is_none());
    }

    /// A prune with any other predicate belongs to the matcher that reads one.
    #[test]
    fn a_sentinel_prune_declines() {
        let broken = FILE_SIZE.replace(
            "removeIf(v -> v == null)",
            "removeIf(v -> v == null || v == '-')",
        );
        assert!(parse_collect_present(&broken).is_none());
    }

    /// The build and the prune WITHOUT the unwrap is left unclaimed. No
    /// vendored pipeline spells it, and claiming it here would guess at what
    /// the missing statement meant.
    #[test]
    fn the_build_without_the_unwrap_declines() {
        let (head, _) = FILE_SIZE.split_once("if (ctx.file.size.size()").unwrap();
        assert!(parse_collect_present(head).is_none());
    }

    /// A statement after the unwrap declines: this reader accounts for the
    /// whole script or none of it.
    #[test]
    fn a_trailing_statement_declines() {
        let extra = format!("{FILE_SIZE}ctx.file.owner = 'root';");
        assert!(parse_collect_present(&extra).is_none());
    }

    /// The braced spelling of the unwrap reads the same.
    #[test]
    fn the_braced_unwrap_reads_the_same() {
        let braced = FILE_SIZE.replace(
            "if (ctx.file.size.size() == 1) ctx.file.size = ctx.file.size[0];",
            "if (ctx.file.size.size() == 1) { ctx.file.size = ctx.file.size[0]; }",
        );
        let pattern = parse_collect_present(&braced).expect("recognised");
        assert_eq!(pattern.target, "file.size");
        assert_eq!(pattern.sources.len(), 5);
    }

    /// The two scripts `FirstElement` claimed before this existed are not
    /// taken here: a bare take and a guarded unwrap have no build to read.
    #[test]
    fn a_plain_first_element_take_is_left_alone() {
        assert!(
            parse_collect_present("ctx.source.ip = ctx.crowdstrike.alert.source_ips[0];").is_none()
        );
        assert!(
            parse_collect_present(
                "if (ctx.url.original instanceof List && ctx.url.original.size() == 1)\n    \
                 ctx.url.original = ctx.url.original[0];"
            )
            .is_none()
        );
    }
}
