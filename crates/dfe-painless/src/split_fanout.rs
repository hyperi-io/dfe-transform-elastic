// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every element of a list cut on one separator, its parts fanned out to
//! parallel lists, deduped.
//!
//! `panw_cortex_xdr` ships its MITRE mapping as a list of `"<id> - <name>"`
//! strings and declares a helper that appends the two halves to two lists,
//! creating each container on the way and skipping a value already held.
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

use crate::params::{balanced, clean_path};
use dfe_core::Event;

/// One part of the cut, and the list it is appended to.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FanColumn {
    /// Which part of the split this column takes.
    index: usize,
    target: String,
}

/// A list's elements cut on a separator and fanned out to parallel lists.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(clippy::needless_raw_string_hashes)]
#[path = "split_fanout_tests.rs"]
mod tests;
