// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A severity that arrives as either a number or a word, normalised to the word
//! and then scored.
//!
//! `beyondtrust_isi` sends `incident.severity` as a long on some events and as
//! the label itself on others, and the conversion ahead of the script is
//! deliberately lenient about which. So the script resolves BOTH spellings to
//! one label, writes that back over the field, and looks the label up again in a
//! second table for the 0-100 `event.severity` score.
//!
//! ```painless
//! def severity = ctx.beyondtrust_isi.incident.severity;
//! String label = null;
//! if (severity instanceof Long) {
//!   label = params.number_to_label.get(String.valueOf(severity));
//! } else if (severity instanceof String) {
//!   label = (String) severity;
//! }
//! if (label == null) {
//!   ctx.beyondtrust_isi.incident.remove('severity');
//!   return;
//! }
//! ctx.beyondtrust_isi.incident.severity = label;
//! def score = params.label_to_score.get(label.toLowerCase());
//! if (score != null) {
//!   ctx.event.severity = score;
//! }
//! ```
//!
//! Two tables and two writes, and BOTH writes are the point: the label table
//! answers a number, and the score table answers a word whichever way the label
//! was reached. A number outside the first table is not a severity at all, and
//! the script deletes the field rather than leaving a digit where a word belongs.

use serde_json::{Map, Value};

use crate::params::clean_path;
use dfe_core::event::Event;

/// The field, the two tables it is read through, and where the score lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelledScore {
    /// The field holding either the number or the label, and where the
    /// resolved label is written back.
    source: String,
    /// The params table naming a label for each number, keyed by its text.
    labels: String,
    /// The params table scoring each label, keyed by its lowercase form.
    scores: String,
    /// Where the score lands.
    target: String,
}

/// `params.<name>` at the head of `text`: the table's key, and what follows it.
fn params_table(text: &str) -> Option<(&str, &str)> {
    let (_, rest) = text.split_once("params.")?;
    let end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
    let (name, tail) = rest.split_at(end);
    (!name.is_empty()).then_some((name, tail))
}

/// Read both tables, the field and the target, or decline.
///
/// Every part is structural: the field comes from the binding the `instanceof`
/// ladder tests, each table from the `params.<name>` its own branch reads, and
/// the target from the guarded write at the end. A script missing any of them --
/// the number branch, the string branch, the remove, the write-back, or the
/// second lookup -- is doing something else and declines rather than binding to
/// a runner that would write half of it.
pub fn parse_labelled_score(script: &str) -> Option<LabelledScore> {
    // `def <local> = ctx.<source>;`
    let (head, rest) = script.split_once(" = ctx.")?;
    let local = head
        .trim_end()
        .rsplit([' ', '\n', '\t'])
        .next()
        .filter(|name| !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))?;
    let source = clean_path(rest.split([';', '\n']).next()?.trim());
    if source.is_empty() || source.contains(['(', ' ', ',']) {
        return None;
    }

    // `if (<local> instanceof Long) { <label> = params.<labels>.get(String.valueOf(<local>)); }`
    let (_, numeric) = script.split_once(&format!("{local} instanceof Long"))?;
    let (labels, after_labels) = params_table(numeric)?;
    if !after_labels.starts_with(".get(String.valueOf(") {
        return None;
    }
    let label = numeric
        .split_once('=')?
        .0
        .trim_end()
        .rsplit([' ', '\n', '\t', '{'])
        .next()?;
    if label.is_empty() || !label.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // `else if (<local> instanceof String) { <label> = (String) <local>; }` --
    // the branch that makes a word its own label.
    if !script.contains(&format!("{local} instanceof String")) {
        return None;
    }

    // A value neither table can name is deleted, not left as a digit.
    let field = source.rsplit('.').next()?;
    if !script.contains(&format!(".remove('{field}')"))
        && !script.contains(&format!(".remove(\"{field}\")"))
    {
        return None;
    }

    // The label written back over the field it came from.
    if !script.contains(&format!("ctx.{source} = {label};")) {
        return None;
    }

    // `def <score> = params.<scores>.get(<label>.toLowerCase());` -- read off the
    // LAST `params.` before the call, so the number table above is not mistaken
    // for it.
    let (before, scoring) = script.split_once(&format!(".get({label}.toLowerCase())"))?;
    let scores = before.rsplit_once("params.")?.1;
    if scores.is_empty() || !scores.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // `if (<score> != null) { ctx.<target> = <score>; }`
    let (_, written) = scoring.split_once("ctx.")?;
    let target = clean_path(written.split_once('=')?.0.trim());
    if target.is_empty() || target.contains(['(', ' ', ',']) {
        return None;
    }

    Some(LabelledScore {
        source,
        labels: labels.to_string(),
        scores: scores.to_string(),
        target,
    })
}

/// Resolve the label, write it back, then score it.
///
/// Each step reproduces one of the script's own outcomes: a number the label
/// table does not carry removes the field, a label the score table does not
/// carry leaves the target alone, and only a resolved label is written back.
pub fn run_labelled_score(
    event: &mut Event,
    pattern: &LabelledScore,
    params: &Map<String, Value>,
) -> bool {
    let label = match event.get(&pattern.source) {
        // `String.valueOf(<long>)` is the digits, so a fractional value is not
        // a `Long` in Painless either and reaches neither branch.
        Some(Value::Number(number)) => number.as_i64().and_then(|held| {
            params
                .get(&pattern.labels)
                .and_then(Value::as_object)
                .and_then(|table| table.get(&held.to_string()))
                .and_then(Value::as_str)
                .map(str::to_string)
        }),
        Some(Value::String(held)) => Some(held.clone()),
        _ => None,
    };

    let Some(label) = label else {
        event.remove(&pattern.source);
        return true;
    };
    let lowered = label.to_lowercase();
    let _ = event.set(&pattern.source, Value::String(label));

    if let Some(score) = params
        .get(&pattern.scores)
        .and_then(Value::as_object)
        .and_then(|table| table.get(&lowered))
    {
        let _ = event.set(&pattern.target, score.clone());
    }
    true
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// `beyondtrust_isi`'s script as the GENERATED call site carries it, one
    /// line with its newlines escaped. Resolving those is the dispatch's first
    /// step, so the parse is exercised on what production actually hands it.
    const RAW: &str = r"// Severity can arrive either as a number (1-4) or as a textual label,\n// so first resolve it to a canonical label before deriving a score.\ndef severity = ctx.beyondtrust_isi.incident.severity;\nString label = null;\n\nif (severity instanceof Long) {\n  label = params.number_to_label.get(String.valueOf(severity));\n} else if (severity instanceof String) {\n  label = (String) severity;\n}\n\n// Drop unrecognized or out-of-range values so only valid labels remain.\nif (label == null) {\n  ctx.beyondtrust_isi.incident.remove('severity');\n  return;\n}\n\nctx.beyondtrust_isi.incident.severity = label;\n\ndef score = params.label_to_score.get(label.toLowerCase());\nif (score != null) {\n  ctx.event.severity = score;\n}";

    fn script() -> String {
        crate::common::normalise(RAW).into_owned()
    }

    fn params() -> Map<String, Value> {
        json!({
            "number_to_label": { "1": "Low", "2": "Medium", "3": "High", "4": "Critical" },
            "label_to_score": { "low": 21, "medium": 47, "high": 73, "critical": 99 }
        })
        .as_object()
        .unwrap()
        .clone()
    }

    #[test]
    fn the_script_names_both_tables_and_both_writes() {
        let pattern = parse_labelled_score(&script()).unwrap();
        assert_eq!(pattern.source, "beyondtrust_isi.incident.severity");
        assert_eq!(pattern.labels, "number_to_label");
        assert_eq!(pattern.scores, "label_to_score");
        assert_eq!(pattern.target, "event.severity");
    }

    #[test]
    fn a_number_becomes_its_label_and_then_its_score() {
        let pattern = parse_labelled_score(&script()).unwrap();
        let mut event = Event::new(json!({"beyondtrust_isi": {"incident": {"severity": 3}}}));
        assert!(run_labelled_score(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("beyondtrust_isi.incident.severity"),
            Some(&json!("High"))
        );
        assert_eq!(event.get("event.severity"), Some(&json!(73)));
    }

    /// A word arrives as its own label, and the score table folds its case.
    #[test]
    fn a_word_keeps_itself_and_still_scores() {
        let pattern = parse_labelled_score(&script()).unwrap();
        let mut event =
            Event::new(json!({"beyondtrust_isi": {"incident": {"severity": "Medium"}}}));
        assert!(run_labelled_score(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("beyondtrust_isi.incident.severity"),
            Some(&json!("Medium"))
        );
        assert_eq!(event.get("event.severity"), Some(&json!(47)));
    }

    /// A number the label table does not carry is not a severity, and the
    /// script deletes it rather than leaving a digit behind.
    #[test]
    fn an_unlisted_number_is_removed_rather_than_kept() {
        let pattern = parse_labelled_score(&script()).unwrap();
        let mut event = Event::new(json!({"beyondtrust_isi": {"incident": {"severity": 9}}}));
        assert!(run_labelled_score(&mut event, &pattern, &params()));
        assert_eq!(event.get("beyondtrust_isi.incident.severity"), None);
        assert_eq!(event.get("event.severity"), None);
    }

    /// A label no score covers keeps the label and leaves the target alone.
    #[test]
    fn an_unscored_label_writes_no_score() {
        let pattern = parse_labelled_score(&script()).unwrap();
        let mut event = Event::new(json!({"beyondtrust_isi": {"incident": {"severity": "Novel"}}}));
        assert!(run_labelled_score(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("beyondtrust_isi.incident.severity"),
            Some(&json!("Novel"))
        );
        assert_eq!(event.get("event.severity"), None);
    }

    /// Every part is demanded: a script missing the number branch, the
    /// write-back or the second lookup is a different script.
    #[test]
    fn a_script_missing_a_part_declines() {
        let script = script();
        for missing in [
            script.replace("severity instanceof Long", "severity instanceof Integer"),
            script.replace("severity instanceof String", "severity instanceof Map"),
            script.replace("ctx.beyondtrust_isi.incident.severity = label;", ""),
            script.replace(".remove('severity')", ".clear()"),
            script.replace("label.toLowerCase()", "label"),
        ] {
            assert!(
                parse_labelled_score(&missing).is_none(),
                "claimed a script it cannot serve: {missing}"
            );
        }
    }
}
