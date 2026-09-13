// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A field that arrived as a scalar, rescued into the map the pipeline expects.
//!
//! `contrast_security` delivers its payload under `event`, and a malformed
//! delivery puts a bare string there instead of the object. Everything after it
//! writes into `event.*`, so the first such write raises `type mismatch at
//! 'event': expected object, got string`, the module's own `on_failure` handler
//! raises the same error again, and the whole transform fails. The vendor's
//! FIRST processor exists to rescue exactly that, guarded on
//! `ctx.event != null && !(ctx.event instanceof Map)`:
//!
//! ```painless
//! def original = ctx.event;
//! ctx.event = new HashMap();
//! ctx.event.original = Json.dump(original);
//! ```
//!
//! Unclaimed it costs all three of the source's malformed fixtures, and those
//! were the only three errors left in the whole compat corpus.
//!
//! **`Json.dump` of a scalar is the QUOTED JSON form, not the bare text.**
//! Elasticsearch writes `"\"malformed-event-payload\""` for that
//! 23-character string, quotes included, which is what `serde_json::to_string`
//! gives.
//!
//! **This reads the whole script, not the `Json.dump` keyword.** Thirteen call
//! sites across eleven sources spell `Json.dump` and twelve of them do
//! something else -- build `ctx.event` where it is null, append a
//! `preserve_original_event` tag, serialise one member of a walked list.
//! Claiming those on the keyword would write `event.original`, which
//! `tests/compare-policy.yaml` excludes from comparison, and silently drop the
//! `tags` and the members they do write. So the parse reads three statements in
//! one fixed order and declines anything carrying a fourth.

use serde_json::{Map, Value};

use dfe_core::Event;

use crate::params::clean_path;

/// A field swapped for a map holding the field's own serialised form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DumpIntoMap {
    /// The field read, and then overwritten with the map.
    path: String,
    /// The member of that map the dump is stored under.
    member: String,
}

/// The document path a `ctx.` reference names, or `None` where it is not one.
///
/// A subscript -- quoted or numeric -- fails the character test rather than
/// resolving to something [`Event::set`] cannot walk.
fn ctx_path(reference: &str) -> Option<String> {
    let path = clean_path(reference.trim().strip_prefix("ctx.")?);
    let named = path.split('.').all(|segment| {
        !segment.is_empty() && segment.chars().all(|c| c.is_alphanumeric() || c == '_')
    });
    named.then_some(path)
}

/// Read the rescue, or decline it.
///
/// The three statements have to be the WHOLE script: read a field into a local,
/// replace the field with an empty map, store the local's JSON dump under one
/// member of it. A fourth statement means the script does something this cannot
/// reproduce, and a claim it cannot serve is worse than no claim -- the call
/// site then reports success having written a fraction of the document.
#[must_use]
pub fn parse_dump_into_map(script: &str) -> Option<DumpIntoMap> {
    let mut statements = script
        .split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty());
    let (bind, create, store) = (statements.next()?, statements.next()?, statements.next()?);
    if statements.next().is_some() {
        return None;
    }

    // `def <local> = ctx.<path>`
    let (local, read) = bind.strip_prefix("def ")?.split_once(" = ")?;
    let local = local.trim();
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let path = ctx_path(read)?;

    // `ctx.<path> = new HashMap()`. Painless spells one empty map two ways and
    // means the same by both.
    let (created, empty) = create.split_once(" = ")?;
    if ctx_path(created)? != path || !matches!(empty.trim(), "new HashMap()" | "[:]") {
        return None;
    }

    // `ctx.<path>.<member> = Json.dump(<local>)`, and the local has to be the
    // one the first statement bound -- dumping anything else is a different
    // script that this would write the wrong value for.
    let (target, dumped) = store.split_once(" = ")?;
    let member = ctx_path(target)?
        .strip_prefix(&format!("{path}."))?
        .to_owned();
    if member.contains('.') {
        return None;
    }
    let inner = dumped
        .trim()
        .strip_prefix("Json.dump(")?
        .strip_suffix(')')?;
    if inner.trim() != local {
        return None;
    }

    Some(DumpIntoMap { path, member })
}

/// Replace the field with the map, or leave the document alone.
///
/// **A field with no value declines**, which is exactly the case every call
/// site's own guard excludes: the vendor gates this on
/// `ctx.event != null && !(ctx.event instanceof Map)`, and Painless reads an
/// absent field and an explicit null as the same null. Nothing in the corpus
/// says what Elasticsearch writes for a dump of null, so this writes nothing
/// rather than inventing a value for a branch that cannot be reached.
pub fn run_dump_into_map(event: &mut Event, pattern: &DumpIntoMap) -> bool {
    let Some(original) = event.get(&pattern.path).filter(|value| !value.is_null()) else {
        return false;
    };
    let Ok(dumped) = serde_json::to_string(original) else {
        return false;
    };
    let mut wrapped = Map::with_capacity(1);
    wrapped.insert(pattern.member.clone(), Value::String(dumped));
    event.set(&pattern.path, Value::Object(wrapped)).is_ok()
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`. Keeping them character-identical is what lets a script
// be copied straight from a module into a test.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from `contrast_security_attack_event/default.rs`, and the same
    /// literal at the `_incident` and `_issue` call sites.
    const CONTRAST: &str = r#"def original = ctx.event; ctx.event = new HashMap(); ctx.event.original = Json.dump(original);"#;

    #[test]
    fn a_scalar_event_becomes_the_map_the_rest_of_the_pipeline_writes_into() {
        let pattern = parse_dump_into_map(CONTRAST).expect("declined contrast's rescue");
        let mut event = Event::new(json!({
            "@timestamp": "2026-01-07T17:46:40.561Z",
            "event": "malformed-event-payload",
            "message": "sql-injection",
        }));
        assert!(run_dump_into_map(&mut event, &pattern));
        // Quotes included: this is the JSON form of the string, which is what
        // `expected.ndjson` carries for all three malformed fixtures.
        assert_eq!(
            event.get("event.original"),
            Some(&json!("\"malformed-event-payload\""))
        );
        // The map holds the dump and nothing else, and `event.*` is now
        // writable.
        assert!(event.set("event.kind", json!("event")).is_ok());
    }

    /// The rescued field keeps its place, because the document order is what
    /// parity rests on.
    #[test]
    fn the_rescued_field_keeps_its_position() {
        let pattern = parse_dump_into_map(CONTRAST).expect("declined contrast's rescue");
        let mut event = Event::new(json!({ "a": 1, "event": "x", "z": 2 }));
        assert!(run_dump_into_map(&mut event, &pattern));
        let keys: Vec<&String> = event
            .as_value()
            .as_object()
            .expect("the document is an object")
            .keys()
            .collect();
        assert_eq!(keys, vec!["a", "event", "z"]);
    }

    #[test]
    fn a_number_dumps_unquoted_and_a_map_dumps_whole() {
        let pattern = parse_dump_into_map(CONTRAST).expect("declined contrast's rescue");
        let mut number = Event::new(json!({ "event": 42 }));
        assert!(run_dump_into_map(&mut number, &pattern));
        assert_eq!(number.get("event.original"), Some(&json!("42")));

        let mut nested = Event::new(json!({ "event": { "b": 1, "a": 2 } }));
        assert!(run_dump_into_map(&mut nested, &pattern));
        assert_eq!(
            nested.get("event.original"),
            Some(&json!(r#"{"b":1,"a":2}"#))
        );
    }

    #[test]
    fn a_field_with_no_value_writes_nothing() {
        let pattern = parse_dump_into_map(CONTRAST).expect("declined contrast's rescue");
        for document in [json!({ "message": "x" }), json!({ "event": null })] {
            let mut event = Event::new(document.clone());
            assert!(!run_dump_into_map(&mut event, &pattern));
            assert_eq!(event.as_value(), &document);
        }
    }

    /// `[:]` and `new HashMap()` are one empty map, spelt Painless's two ways.
    #[test]
    fn the_other_empty_map_spelling_reads_the_same() {
        let script = "def o = ctx.event; ctx.event = [:]; ctx.event.original = Json.dump(o);";
        assert_eq!(parse_dump_into_map(script), parse_dump_into_map(CONTRAST));
    }

    /// The twelve OTHER `Json.dump` scripts in the tree, one per distinct
    /// spelling, each in the form `normalise` hands the ladder. Every one of
    /// them does something this runner does not, so claiming any would report a
    /// success that wrote a fraction of what the vendor writes.
    #[test]
    fn the_other_json_dump_spellings_are_declined() {
        for script in [
            // o365 and jamf_protect_alerts: the field is BUILT where absent,
            // and the dump is of a different path entirely.
            "ctx.event = ctx.event ?: [:];\nctx.event.original = Json.dump(ctx.o365audit)",
            "ctx.event = ctx.event ?: [:];\nctx.event.original = Json.dump(ctx.jamf_protect.alerts);",
            // claude_code_events / claude_cowork_events also append a tag, and
            // `tags` IS compared.
            "ctx.event = ctx.event ?: [:]; ctx.event.original = Json.dump(ctx); \
             ctx.tags = ctx.tags ?: []; if (!ctx.tags.contains('preserve_original_event')) {\n  \
             ctx.tags.add('preserve_original_event');\n}\n",
            // The entityanalytics pair dump FIRST and guard the store.
            "def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  \
             if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  \
             ctx.event.original = stringified_orig;\n}\n",
            "def stringified_orig = Json.dump(ctx);\nif (stringified_orig != null) {\n  \
             ctx.event = ctx.event ?: new HashMap();\n  ctx.event.original = stringified_orig;\n}\n",
            // wiz_defend, the same with a narrower source.
            "def stringified_orig = Json.dump(ctx.json);\nif (stringified_orig != null) {\n  \
             ctx.event = ctx.event ?: [:];\n  ctx.event.original = stringified_orig;\n}\n",
            // gitlab replaces the value with its dump and builds no map at all.
            "def vars = ctx.gitlab.production.params.graphql.variables;\n\
             ctx.gitlab.production.params.graphql.variables = Json.dump(vars);\n",
        ] {
            assert!(
                parse_dump_into_map(script).is_none(),
                "claimed a script it cannot serve: {script}"
            );
        }
    }

    #[test]
    fn a_fourth_statement_is_declined_rather_than_half_run() {
        let script = "def original = ctx.event; ctx.event = new HashMap(); \
                      ctx.event.original = Json.dump(original); ctx.tags = [];";
        assert!(parse_dump_into_map(script).is_none());
    }

    #[test]
    fn the_dump_has_to_be_of_the_local_the_script_bound() {
        let script = "def original = ctx.event; ctx.event = new HashMap(); \
                      ctx.event.original = Json.dump(ctx.message);";
        assert!(parse_dump_into_map(script).is_none());
    }

    #[test]
    fn a_map_built_over_a_different_field_is_declined() {
        let script = "def original = ctx.event; ctx.json = new HashMap(); \
                      ctx.json.original = Json.dump(original);";
        assert!(parse_dump_into_map(script).is_none());
    }

    #[test]
    fn a_subscripted_target_is_declined_rather_than_resolved_wrong() {
        let script = "def original = ctx.event; ctx.event = new HashMap(); \
                      ctx.event['original'] = Json.dump(original);";
        assert!(parse_dump_into_map(script).is_none());
    }
}
