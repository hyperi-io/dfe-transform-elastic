// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A map's `<name>Label` / `<name>` pairs re-keyed by what the label SAYS.
//!
//! CEF carries vendor extensions in numbered slots with a parallel label naming
//! each one: `cs1Label: "MailRecipient"` beside `cs1: "test@gmail.com"`. The
//! slot number means nothing on its own, so varonis renames the pair into
//! `MailRecipient: "test@gmail.com"` and drops both originals -- which is what
//! puts the vendor's own field names back on the event
//! (`pipelines/varonis/logs/default.yml:36`).
//!
//! A label whose slot is empty is dropped on its own, leaving the slot under
//! its numbered name: the script's else arm removes the label and nothing
//! else. Running neither arm put 13 `device_custom_*` keys on every varonis
//! event that Elasticsearch does not emit, and left the six fields the labels
//! name absent.
//!
//! The rename is what the `keysToSnakeCase` script downstream then reads, so a
//! label the walk leaves behind is two wrong fields rather than one.

use serde_json::{Map, Value};

use crate::params::{balanced, ctx_path_plain as ctx_path, skip_trivia};
use dfe_core::Event;

/// One map whose label keys are folded into the slots they name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelledKeyRename {
    /// The `ctx.` path of the map walked.
    container: String,
    /// The key suffix that marks a label, read off the script's own test.
    suffix: String,
}

impl LabelledKeyRename {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(container: String, suffix: String) -> Self {
        Self { container, suffix }
    }
}

/// Re-key each label pair, dropping the slot names the rename consumes.
///
/// Returns FALSE where the path holds no map, so the script is counted
/// unhandled rather than claimed and skipped.
pub fn labelled_key_rename(event: &mut Event, pattern: &LabelledKeyRename) -> bool {
    let Some(container) = event.get_object(&pattern.container).cloned() else {
        return false;
    };

    // Both halves of every pair, resolved before the rebuild: a label's own
    // slot can sit anywhere in the map, including behind it.
    let mut dropped: Vec<&str> = Vec::new();
    let mut renamed: Vec<(String, Value)> = Vec::new();
    for (key, label) in &container {
        let Some(slot) = key.strip_suffix(&pattern.suffix).filter(|s| !s.is_empty()) else {
            continue;
        };
        // The label goes whichever way the guard falls, which is the script's
        // else arm: a slot with no value leaves neither key behind.
        dropped.push(key.as_str());
        let named = container.get(slot).filter(|value| !is_empty_value(value));
        let (Some(value), Some(name)) = (named, label.as_str()) else {
            continue;
        };
        dropped.push(slot);
        renamed.push((name.to_owned(), value.clone()));
    }
    if dropped.is_empty() {
        return true;
    }

    // Insertion order is the document's under `preserve_order`, and Painless
    // appends a `put` of a new key the same way -- so the kept entries hold
    // their places and the renames follow them.
    let mut rebuilt = Map::with_capacity(container.len());
    for (key, value) in &container {
        if !dropped.contains(&key.as_str()) {
            rebuilt.insert(key.clone(), value.clone());
        }
    }
    for (name, value) in renamed {
        rebuilt.insert(name, value);
    }
    let _ = event.set(&pattern.container, Value::Object(rebuilt));
    true
}

/// Whether the script's `!= null && !toString().isEmpty()` pair rejects a value.
///
/// A number is kept: `184` renders as `"184"`, which is not empty, and varonis
/// carries a rule id in exactly that slot.
fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(text) => text.is_empty(),
        Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Object(_) => false,
    }
}

/// Read the rename off the script, or decline.
///
/// ```painless
/// void <helper>(Map <map>) {
///     List <keys> = new ArrayList(<map>.keySet());
///     for (String <key> : <keys>) {
///         if (<key>.endsWith("<suffix>")) {
///             String <slot> = <key>.substring(0, <key>.length() - <trim>);
///             if (<map>.containsKey(<slot>) && <map>.get(<slot>) != null
///                 && !<map>.get(<slot>).toString().isEmpty()) {
///                 <map>.put(<map>.get(<key>), <map>.get(<slot>));
///                 <map>.remove(<slot>);
///                 <map>.remove(<key>);
///             } else {
///                 <map>.remove(<key>);
///             }
///         }
///     }
/// }
/// <helper>(ctx.<container>);
/// ```
///
/// The two `get` calls inside the `put` are checked in ORDER: reading them the
/// other way round keys the map by its own values, which is a rewrite of the
/// whole extension block rather than a rename.
#[must_use]
pub fn parse_labelled_key_rename(script: &str) -> Option<LabelledKeyRename> {
    let head = skip_trivia(script);
    let rest = head
        .strip_prefix("void")
        .or_else(|| head.strip_prefix("def"))?;
    let opens = rest.find('(')?;
    let helper = identifier(&rest[..opens])?;
    let (parameter, rest) = balanced(&rest[opens..], '(', ')')?;
    let map = declared(parameter, "Map")?;
    let (body, rest) = balanced(skip_trivia(rest), '{', '}')?;

    // The call, and nothing after it: a second statement is work this runner
    // would not do.
    let call = skip_trivia(rest).strip_prefix(helper)?;
    let (argument, after) = balanced(skip_trivia(call), '(', ')')?;
    if !skip_trivia(after.trim_start().trim_start_matches(';')).is_empty() {
        return None;
    }
    let container = ctx_path(argument)?;

    let suffix = parse_body(body, map)?;
    Some(LabelledKeyRename::new(container, suffix))
}

/// The helper's body: the snapshot, the loop, and the suffix the walk tests for.
fn parse_body(body: &str, map: &str) -> Option<String> {
    let (snapshot, rest) = skip_trivia(body).split_once(';')?;
    // The keys are copied BEFORE the walk, which is what makes removing one
    // safe. A walk over the live key set is a different script.
    let (_, source) = snapshot.split_once('=')?;
    if !same_ignoring_spaces(source, &format!("new ArrayList({map}.keySet())")) {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("for")?;
    let (header, rest) = balanced(skip_trivia(rest), '(', ')')?;
    let (declaration, walked) = header.split_once(':')?;
    let key = declared(declaration, "String")?;
    if walked.trim() != declared(snapshot, "List")? {
        return None;
    }

    let (loop_body, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }

    let rest = skip_trivia(loop_body).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    let suffix = suffix_tested(test, key)?;
    let (block, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }

    parse_pair(block, map, key, suffix.len()).map(|()| suffix)
}

/// The matched branch: the slot name, the guard over it, and the two arms.
fn parse_pair(block: &str, map: &str, key: &str, suffix: usize) -> Option<()> {
    let (declaration, rest) = skip_trivia(block).split_once(';')?;
    let (declaration, cut) = declaration.split_once('=')?;
    let slot = declared(declaration, "String")?;
    // The count the slot name is cut by is the script's own arithmetic, so a
    // suffix and a trim that disagree are cutting a different name.
    if !same_ignoring_spaces(cut, &format!("{key}.substring(0,{key}.length()-{suffix})")) {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    if !same_ignoring_spaces(
        test,
        &format!(
            "{map}.containsKey({slot})&&{map}.get({slot})!=null\
             &&!{map}.get({slot}).toString().isEmpty()"
        ),
    ) {
        return None;
    }

    let (matched, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !same_ignoring_spaces(
        matched,
        &format!(
            "{map}.put({map}.get({key}),{map}.get({slot}));\
             {map}.remove({slot});{map}.remove({key});"
        ),
    ) {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("else")?;
    let (unmatched, rest) = balanced(skip_trivia(rest), '{', '}')?;
    (skip_trivia(rest).is_empty()
        && same_ignoring_spaces(unmatched, &format!("{map}.remove({key});")))
    .then_some(())
}

/// The text a `<key>.endsWith('<suffix>')` test compares against.
fn suffix_tested(test: &str, key: &str) -> Option<String> {
    let rest = test
        .trim()
        .strip_prefix(key)?
        .trim()
        .strip_prefix(".endsWith(")?;
    let quote = rest
        .trim_start()
        .chars()
        .next()
        .filter(|c| *c == '\'' || *c == '"')?;
    let rest = rest.trim_start().strip_prefix(quote)?;
    let (suffix, tail) = rest.split_once(quote)?;
    (!suffix.is_empty() && tail.trim().trim_end_matches(')').trim().is_empty())
        .then(|| suffix.to_owned())
}

/// The name a `<keyword> <name>` declaration binds.
fn declared<'a>(text: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = skip_trivia(text).strip_prefix(keyword)?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    identifier(rest.split('=').next()?)
}

/// A local's name, or `None` where the text is not one identifier.
fn identifier(text: &str) -> Option<&str> {
    let name = text.trim();
    (!name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
    .then_some(name)
}

/// Whether two expressions read the same once every space is removed, so
/// spacing is not part of a match.
fn same_ignoring_spaces(text: &str, expected: &str) -> bool {
    text.chars()
        .filter(|c| !c.is_whitespace())
        .eq(expected.chars().filter(|c| !c.is_whitespace()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// varonis's rename, verbatim from `pipelines/varonis/logs/default.yml:36`
    /// with the YAML fold applied.
    fn script() -> &'static str {
        "void changeLabelsToKeys(Map input) {\n    \
         List keys = new ArrayList(input.keySet()); // To avoid conflicts\n    \
         for (String key : keys) {\n        \
         if (key.endsWith(\"Label\")) {\n            \
         String valueKey = key.substring(0, key.length() - 5);\n            \
         if (input.containsKey(valueKey) && input.get(valueKey) != null \
         && !input.get(valueKey).toString().isEmpty()) {\n                \
         input.put(input.get(key), input.get(valueKey));\n                \
         input.remove(valueKey);\n                \
         input.remove(key);\n            \
         } else {\n                \
         input.remove(key);\n            \
         }\n        \
         }\n    \
         }\n}\nchangeLabelsToKeys(ctx.cef.extensions);\n"
    }

    fn pattern() -> LabelledKeyRename {
        parse_labelled_key_rename(script()).expect("the rename is recognised")
    }

    fn rewrite(extensions: &Value) -> Value {
        let mut event = Event::new(json!({ "cef": { "extensions": extensions } }));
        assert!(labelled_key_rename(&mut event, &pattern()));
        event
            .get("cef.extensions")
            .cloned()
            .expect("the map stands")
    }

    /// Both parts come OFF the script -- the map and the suffix.
    #[test]
    fn every_part_is_read_off_the_script() {
        assert_eq!(
            pattern(),
            LabelledKeyRename {
                container: "cef.extensions".into(),
                suffix: "Label".into(),
            }
        );
    }

    /// The pair is re-keyed by what the label SAYS, and both numbered keys go.
    #[test]
    fn a_pair_is_rekeyed_by_the_labels_value() {
        assert_eq!(
            rewrite(&json!({ "cs1Label": "MailRecipient", "cs1": "test@gmail.com" })),
            json!({ "MailRecipient": "test@gmail.com" })
        );
    }

    /// A number keeps its JSON type: the script only renders it to test for
    /// emptiness, and varonis carries a rule id in that slot.
    #[test]
    fn a_numeric_slot_keeps_its_type() {
        assert_eq!(
            rewrite(&json!({ "cn1Label": "RuleID", "cn1": 184 })),
            json!({ "RuleID": 184 })
        );
    }

    /// An empty, null or absent slot drops the LABEL and renames nothing --
    /// the script's else arm, and what keeps `AttachmentName` off the event.
    ///
    /// That arm removes the label ALONE, so a slot holding an empty value is
    /// left where it was under its own numbered name.
    #[test]
    fn a_label_without_a_value_drops_the_label_alone() {
        assert_eq!(rewrite(&json!({ "cs3Label": "AttachmentName" })), json!({}));
        assert_eq!(
            rewrite(&json!({ "cs3Label": "AttachmentName", "cs3": "" })),
            json!({ "cs3": "" })
        );
        assert_eq!(
            rewrite(&json!({ "cs3Label": "AttachmentName", "cs3": null })),
            json!({ "cs3": null })
        );
    }

    /// Keys the walk does not claim are untouched, and hold their places.
    #[test]
    fn unlabelled_keys_are_untouched() {
        assert_eq!(
            rewrite(&json!({
                "src": "10.0.0.1",
                "cs1Label": "MailRecipient",
                "cs1": "test@gmail.com",
                "dst": "10.0.0.2",
            })),
            json!({ "src": "10.0.0.1", "dst": "10.0.0.2", "MailRecipient": "test@gmail.com" })
        );
    }

    /// A map with no label at all is left exactly as it was.
    #[test]
    fn a_map_with_no_labels_is_unchanged() {
        assert_eq!(
            rewrite(&json!({ "src": "10.0.0.1" })),
            json!({ "src": "10.0.0.1" })
        );
    }

    /// A slot whose label is not text cannot name a key, so the pair is left
    /// alone rather than keyed by a rendering of a number.
    #[test]
    fn a_label_that_is_not_text_renames_nothing() {
        assert_eq!(
            rewrite(&json!({ "cs1Label": 7, "cs1": "value" })),
            json!({ "cs1": "value" })
        );
    }

    /// A path holding no map DECLINES, so the script is counted unhandled
    /// rather than claimed and skipped.
    #[test]
    fn a_missing_map_declines() {
        let mut event = Event::new(json!({ "cef": {} }));
        assert!(!labelled_key_rename(&mut event, &pattern()));

        let mut scalar = Event::new(json!({ "cef": { "extensions": "text" } }));
        assert!(!labelled_key_rename(&mut scalar, &pattern()));
    }

    /// The suffix is the script's own text, and the cut length has to agree
    /// with it.
    #[test]
    fn the_suffix_comes_from_the_script() {
        let tagged = script()
            .replace("endsWith(\"Label\")", "endsWith(\"Name\")")
            .replace("key.length() - 5", "key.length() - 4");
        let pattern = parse_labelled_key_rename(&tagged).expect("recognised");
        assert_eq!(pattern.suffix, "Name");

        // A suffix and a trim that disagree are cutting a different name.
        let mismatched = script().replace("key.length() - 5", "key.length() - 4");
        assert!(parse_labelled_key_rename(&mismatched).is_none());
    }

    /// One statement the reader cannot place declines the WHOLE script, so a
    /// near-miss never rewrites the extension block by the wrong member.
    #[test]
    fn one_unreadable_statement_declines_the_whole_script() {
        for broken in [
            // The two `get` calls the other way round, which would key the map
            // by its own values.
            script().replace(
                "input.put(input.get(key), input.get(valueKey));",
                "input.put(input.get(valueKey), input.get(key));",
            ),
            // The slot kept as well as renamed.
            script().replace("input.remove(valueKey);\n                ", ""),
            // A walk over the LIVE key set, which removes as it iterates.
            script().replace("new ArrayList(input.keySet())", "input.keySet()"),
            // An else arm that keeps the label.
            script().replace("} else {\n                input.remove(key);", "} else {\n"),
            // A second statement after the call.
            script().replace(
                "changeLabelsToKeys(ctx.cef.extensions);",
                "changeLabelsToKeys(ctx.cef.extensions); ctx.a = 1;",
            ),
        ] {
            assert_ne!(broken, script(), "each replacement has to bite");
            assert!(
                parse_labelled_key_rename(&broken).is_none(),
                "read a script it should have declined: {broken}"
            );
        }
    }

    /// The dispatch reaches this matcher, and nothing else is bound beside it.
    #[test]
    fn the_ladder_dispatches_the_script_here() {
        let bound = crate::common::known_patterns(script());
        assert_eq!(
            bound.len(),
            1,
            "the ladder bound {bound:?} instead of this matcher alone"
        );
    }

    /// The production path, end to end. A generated call site hands
    /// `PainlessPlan` the script with its newlines and quotes ESCAPED, so a
    /// matcher that works only on resolved text passes its own tests and does
    /// nothing in the service. This is the literal
    /// `crates/dfe-transforms/src/filebeat/varonis_logs/default.rs:37` holds.
    #[test]
    fn the_escaped_call_site_literal_runs() {
        // Running a plan records into the process-global stats the counting
        // tests read; one lock keeps those honest.
        let _guard = crate::stats::serialised();
        let plan = crate::plan::PainlessPlan::new(
            r#"void changeLabelsToKeys(Map input) {\n    List keys = new ArrayList(input.keySet()); // To avoid conflicts\n    for (String key : keys) {\n        if (key.endsWith(\"Label\")) {\n            String valueKey = key.substring(0, key.length() - 5);\n            if (input.containsKey(valueKey) && input.get(valueKey) != null && !input.get(valueKey).toString().isEmpty()) {\n                input.put(input.get(key), input.get(valueKey));\n                input.remove(valueKey);\n                input.remove(key);\n            } else {\n                input.remove(key);\n            }\n        }\n    }\n}\nchangeLabelsToKeys(ctx.cef.extensions);\n"#,
        );
        assert!(plan.matches(), "the call site's own literal binds nothing");

        let mut event = Event::new(json!({
            "cef": { "extensions": { "cs1Label": "MailRecipient", "cs1": "test@gmail.com" } }
        }));
        crate::plan::painless_exec_plan(&mut event, &plan).expect("the plan runs");
        assert_eq!(
            event.get("cef.extensions"),
            Some(&json!({ "MailRecipient": "test@gmail.com" }))
        );
    }
}
