// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every element of a list cut on one separator, its parts fanned out to
//! parallel lists, deduped.
//!
//! panw_cortex_xdr ships its MITRE mapping as a list of `"<id> - <name>"`
//! strings and declares a helper that appends the two halves to two lists,
//! creating each container on the way and skipping a value already held:
//!
//! ```painless
//! void addTechnique(def ctx, def x, def y) {
//!   if (ctx.threat == null) { ctx.threat = new HashMap(); }
//!   if (ctx.threat.technique == null) { ctx.threat.technique = new HashMap(); }
//!   if (ctx.threat.technique.id == null) { ctx.threat.technique.id = new ArrayList(); }
//!   if (ctx.threat.technique.name == null) { ctx.threat.technique.name = new ArrayList(); }
//!   if (!ctx.threat.technique.id.contains(x)) { ctx.threat.technique.id.add(x); }
//!   if (!ctx.threat.technique.name.contains(y)) { ctx.threat.technique.name.add(y); }
//! }
//! for (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {
//!   addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0],
//!                     mitre_technique.splitOnToken(' - ')[1]);
//! }
//! ```
//!
//! The same text appears with `tactic` for `technique` and, on the incidents
//! stream, over the plural field names -- six call sites across three modules
//! and one pattern. The helper's PARAMETER ORDER is what ties a target to a
//! part: `x` names the list `[0]` is appended to only because the loop passes
//! the two in that order, so both halves have to be read together.
//!
//! `CollectFromList` reads the nearest sibling -- a member taken out of every
//! record of a list -- and cannot take this one: there is no member to read,
//! the value is cut out of the element itself, and the two targets come from
//! one walk rather than two arms.

use serde_json::Value;

use crate::Event;
use crate::painless_params::{balanced, clean_path};

/// One part of the cut, and the list it is appended to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanColumn {
    /// Which part of the split this column takes.
    index: usize,
    target: String,
}

/// A list's elements cut on a separator and fanned out to parallel lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitFanOut {
    list: String,
    separator: String,
    columns: Vec<FanColumn>,
}

impl SplitFanOut {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        list: impl Into<String>,
        separator: impl Into<String>,
        columns: Vec<(usize, String)>,
    ) -> Self {
        Self {
            list: list.into(),
            separator: separator.into(),
            columns: columns
                .into_iter()
                .map(|(index, target)| FanColumn { index, target })
                .collect(),
        }
    }
}

/// The helper's name and its value parameters, in order.
///
/// `void <name>(def ctx, def x, def y) {` -- the first parameter is the
/// document and is not a value, so it is dropped.
fn helper_signature(script: &str) -> Option<(&str, Vec<String>, &str)> {
    let (head, rest) = script.split_once("void ")?;
    // A helper declared after anything other than leading trivia is a second
    // statement this reader has not accounted for.
    if !head.trim().is_empty() {
        return None;
    }
    let (name, rest) = rest.split_once('(')?;
    let name = name.trim();
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    // `balanced` wants the opening delimiter, which `split_once` consumed.
    let arguments = format!("({rest}");
    let (arguments, _) = balanced(&arguments, '(', ')')?;
    let mut parameters = arguments.split(',').map(|parameter| {
        parameter
            .trim()
            .trim_start_matches("def ")
            .trim()
            .to_owned()
    });
    if parameters.next()? != "ctx" {
        return None;
    }
    let parameters: Vec<String> = parameters.collect();
    if parameters.is_empty()
        || parameters
            .iter()
            .any(|p| p.is_empty() || !p.chars().all(|c| c.is_alphanumeric() || c == '_'))
    {
        return None;
    }
    let body_at = rest.find(')')?;
    Some((name, parameters, &rest[body_at + 1..]))
}

/// Where each parameter is appended, read off the deduped appends.
///
/// `if (!ctx.<path>.contains(<param>)) { ctx.<path>.add(<param>); }` -- the
/// guard and the append have to name the SAME list, or the script holds a
/// value back from one and adds it to another.
fn append_targets(body: &str, parameters: &[String]) -> Option<Vec<String>> {
    let mut targets = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        let guard = format!(".contains({parameter})");
        let (head, rest) = body.split_once(&guard)?;
        // The guard is a NEGATED membership test: an unnegated one appends a
        // value the list already holds, which is a different script.
        let (_, guarded) = head.rsplit_once("if (!ctx.")?;
        let guarded = clean_path(guarded.trim());
        let appended = rest
            .split_once(&format!(".add({parameter})"))?
            .0
            .rsplit_once("ctx.")?
            .1;
        if guarded.is_empty() || clean_path(appended.trim()) != guarded {
            return None;
        }
        targets.push(guarded);
    }
    Some(targets)
}

/// The list walked, and the part index handed to each parameter.
///
/// `for (<var> in ctx.<list>) { <helper>(ctx, <var>.splitOnToken('<sep>')[<i>],
/// ...); }`
fn loop_arguments(after: &str, helper: &str, arity: usize) -> Option<(String, String, Vec<usize>)> {
    let (_, rest) = after.split_once("for (")?;
    let (var, rest) = rest.split_once(" in ctx")?;
    let var = var.trim();
    if var.is_empty() || !var.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (list, rest) = rest.split_once(')')?;
    let list = clean_path(list.trim().trim_start_matches(['?', '.']));
    if list.is_empty() || list.contains(['(', '[', ' ']) {
        return None;
    }

    let call = format!("{helper}(");
    let arguments = rest.split_once(&call)?.1;
    let arguments = format!("({arguments}");
    let (arguments, _) = balanced(&arguments, '(', ')')?;
    let mut arguments = arguments.split(',').map(str::trim);
    if arguments.next()? != "ctx" {
        return None;
    }

    let cut = format!("{var}.splitOnToken(");
    let mut separator = None;
    let mut indices = Vec::with_capacity(arity);
    for argument in arguments {
        let tail = argument.strip_prefix(&cut)?;
        let (quoted, tail) = tail.split_once(')')?;
        let quoted = quoted.trim().trim_matches(['\'', '"']).to_owned();
        if quoted.is_empty() || *separator.get_or_insert(quoted.clone()) != quoted {
            // One call cutting on two separators is not this pattern.
            return None;
        }
        let index = tail.trim().strip_prefix('[')?.split_once(']')?.0;
        indices.push(index.trim().parse::<usize>().ok()?);
    }
    (indices.len() == arity).then_some((list, separator?, indices))
}

/// Read the list, the separator and each part's target, or decline.
#[must_use]
pub fn parse_split_fan_out(script: &str) -> Option<SplitFanOut> {
    let (helper, parameters, body) = helper_signature(script)?;
    let (body, after) = balanced(body.trim_start(), '{', '}')?;
    let targets = append_targets(body, &parameters)?;
    let (list, separator, indices) = loop_arguments(after, helper, parameters.len())?;

    Some(SplitFanOut {
        list,
        separator,
        columns: indices
            .into_iter()
            .zip(targets)
            .map(|(index, target)| FanColumn { index, target })
            .collect(),
    })
}

/// Cut every element and append its parts, skipping what a target already
/// holds.
///
/// An element the cut cannot fill EVERY column from is skipped whole. Painless
/// throws on the out-of-range subscript and abandons the script, so no partial
/// row is a reading of this text -- writing one half of a pair would put an id
/// on the document with no name beside it.
#[must_use]
pub fn split_fan_out(event: &mut Event, pattern: &SplitFanOut) -> bool {
    let Some(Value::Array(elements)) = event.get(&pattern.list).cloned() else {
        // The call site is guarded on the list being present.
        return true;
    };

    // The containers are created inside the helper, so a walk that appends
    // nothing leaves the document untouched rather than holding empty lists.
    let mut gathered: Vec<Vec<Value>> = pattern
        .columns
        .iter()
        .map(|column| match event.get(&column.target) {
            Some(Value::Array(held)) => held.clone(),
            _ => Vec::new(),
        })
        .collect();
    let held = gathered.clone();

    for element in &elements {
        let Some(text) = element.as_str() else {
            continue;
        };
        let parts: Vec<&str> = text.split(pattern.separator.as_str()).collect();
        if pattern
            .columns
            .iter()
            .any(|column| parts.len() <= column.index)
        {
            continue;
        }
        for (column, into) in pattern.columns.iter().zip(gathered.iter_mut()) {
            let value = Value::String(parts[column.index].to_owned());
            if !into.contains(&value) {
                into.push(value);
            }
        }
    }

    for ((column, values), before) in pattern.columns.iter().zip(gathered).zip(held) {
        if values != before {
            let _ = event.set(&column.target, Value::Array(values));
        }
    }
    true
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(clippy::needless_raw_string_hashes)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/panw_cortex_xdr_alerts/v2_pipeline.rs`,
    /// in the escaped one-line form the call site holds.
    const PANW_TECHNIQUE: &str = r#"void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}"#;

    /// The incidents stream's tactic spelling: plural field names, a different
    /// indent inside the helper, and the call indented further.
    const PANW_INCIDENT_TACTIC: &str = r#"void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n  ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n  ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n  ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n  ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n  ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n  ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactics_ids_and_names) {\n    addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}"#;

    #[test]
    fn the_panw_technique_list_is_cut_into_two_deduped_lists() {
        assert_eq!(
            parse_split_fan_out(&crate::painless_common::normalise(PANW_TECHNIQUE)),
            Some(SplitFanOut::new(
                "panw_cortex.xdr.mitre_technique_id_and_name",
                " - ",
                vec![
                    (0, "threat.technique.id".to_owned()),
                    (1, "threat.technique.name".to_owned()),
                ],
            ))
        );

        // The WRITTEN value, over the stream's own data: four techniques and a
        // repeated tactic, which the vendor's `contains` guard collapses.
        let mut event = Event::new(json!({ "panw_cortex": { "xdr": {
            "mitre_technique_id_and_name": [
                "T1018 - Remote System Discovery",
                "T1082 - System Information Discovery",
                "T1016 - System Network Configuration Discovery",
                "T1007 - System Service Discovery",
                "T1018 - Remote System Discovery",
            ]
        }}}));
        assert!(crate::painless_common::try_known_painless(
            &mut event,
            PANW_TECHNIQUE
        ));
        assert_eq!(
            event.get("threat.technique.id"),
            Some(&json!(["T1018", "T1082", "T1016", "T1007"]))
        );
        assert_eq!(
            event.get("threat.technique.name"),
            Some(&json!([
                "Remote System Discovery",
                "System Information Discovery",
                "System Network Configuration Discovery",
                "System Service Discovery",
            ]))
        );
    }

    /// A name carrying the separator keeps only the part the subscript names,
    /// which is what `splitOnToken(...)[1]` says.
    #[test]
    fn a_sub_technique_name_is_cut_the_way_painless_cuts_it() {
        let mut event = Event::new(json!({ "panw_cortex": { "xdr": {
            "mitre_technique_id_and_name": [
                "T1059.001 - Command and Scripting Interpreter: PowerShell",
            ]
        }}}));
        assert!(crate::painless_common::try_known_painless(
            &mut event,
            PANW_TECHNIQUE
        ));
        assert_eq!(
            event.get("threat.technique.id"),
            Some(&json!(["T1059.001"]))
        );
        assert_eq!(
            event.get("threat.technique.name"),
            Some(&json!(["Command and Scripting Interpreter: PowerShell"]))
        );
    }

    #[test]
    fn the_incident_tactic_spelling_reads_the_same_pattern() {
        assert_eq!(
            parse_split_fan_out(&crate::painless_common::normalise(PANW_INCIDENT_TACTIC)),
            Some(SplitFanOut::new(
                "panw_cortex.xdr.mitre_tactics_ids_and_names",
                " - ",
                vec![
                    (0, "threat.tactic.id".to_owned()),
                    (1, "threat.tactic.name".to_owned()),
                ],
            ))
        );

        let mut event = Event::new(json!({ "panw_cortex": { "xdr": {
            "mitre_tactics_ids_and_names": ["TA0010 - Exfiltration"]
        }}}));
        assert!(crate::painless_common::try_known_painless(
            &mut event,
            PANW_INCIDENT_TACTIC
        ));
        assert_eq!(event.get("threat.tactic.id"), Some(&json!(["TA0010"])));
        assert_eq!(
            event.get("threat.tactic.name"),
            Some(&json!(["Exfiltration"]))
        );
    }

    /// An absent list writes nothing, and an element the cut cannot fill both
    /// columns from is skipped rather than half-written.
    #[test]
    fn a_list_the_cut_cannot_fill_writes_nothing() {
        let mut quiet = Event::new(json!({ "panw_cortex": { "xdr": {} } }));
        assert!(crate::painless_common::try_known_painless(
            &mut quiet,
            PANW_TECHNIQUE
        ));
        assert_eq!(quiet.get("threat.technique.id"), None);

        let mut short = Event::new(json!({ "panw_cortex": { "xdr": {
            "mitre_technique_id_and_name": ["T1018"]
        }}}));
        assert!(crate::painless_common::try_known_painless(
            &mut short,
            PANW_TECHNIQUE
        ));
        assert_eq!(short.get("threat.technique.id"), None);
        assert_eq!(short.get("threat.technique.name"), None);
    }

    /// A guard and an append naming DIFFERENT lists hold one value back and
    /// add it to another, so the pair is declined.
    #[test]
    fn a_fan_out_guarding_another_list_is_declined() {
        let script = r"void add(def ctx, def x) {\n  if (!ctx.a.one.contains(x)) {\n    ctx.a.two.add(x);\n  }\n}\nfor (v in ctx.a.list) {\n  add(ctx, v.splitOnToken(' - ')[0]);\n}";
        assert!(parse_split_fan_out(&crate::painless_common::normalise(script)).is_none());
    }

    /// Two separators in one call is a different cut for each half.
    #[test]
    fn a_fan_out_cutting_on_two_separators_is_declined() {
        let script = r"void add(def ctx, def x, def y) {\n  if (!ctx.a.one.contains(x)) {\n    ctx.a.one.add(x);\n  }\n  if (!ctx.a.two.contains(y)) {\n    ctx.a.two.add(y);\n  }\n}\nfor (v in ctx.a.list) {\n  add(ctx, v.splitOnToken(' - ')[0], v.splitOnToken(':')[1]);\n}";
        assert!(parse_split_fan_out(&crate::painless_common::normalise(script)).is_none());
    }
}
