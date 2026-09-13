// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One member of a list's records collected into whichever of several named
//! lists a LITERAL test on another member chooses.
//!
//! Every `vsphere` data stream ships this script, over its own subtree -- the
//! alarm NAMES sorted into two lists by the alarm's colour:
//!
//! ```painless
//! def alerts = []; def warnings = [];
//! for (alarm in ctx.vsphere.cluster.triggered_alarms) {
//!   if (alarm.status == 'red') { alerts.add(alarm.name); }
//!   if (alarm.status == 'yellow') { warnings.add(alarm.name); }
//! }
//! ctx.alerts = alerts; ctx.warnings = warnings;
//! ```
//!
//! The list the module renames to `vsphere.<stream>.alert.names` is what its
//! own `drop_empty` then removes when nothing was red, so an event with no
//! alarms writing nothing is the parity, not a gap.
//!
//! Distinct from [`crate::common`]'s `CollectFromList`, which reads the member
//! to collect off the GUARD -- one member tested for presence and then
//! collected. Here the guard tests one member and a DIFFERENT one is collected,
//! so reading the guard would put `red` and `yellow` where the names belong.
//! That arm declines these scripts today for an unrelated reason: its reader of
//! the trailing `ctx.<target> = <local>` cannot strip `ctx.` from
//! `} ctx.alerts = alerts`, where the loop's closing brace shares the
//! statement.

use serde_json::Value;

use crate::params::{balanced, clean_path, is_ctx_path};
use dfe_core::Event;

/// One named accumulator and the test that fills it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralArm {
    /// What the tested member has to equal for a record to contribute.
    literal: String,
    /// The member collected from a record that passes.
    member: String,
    /// Where the accumulator lands on the document.
    target: String,
}

/// A list partitioned into named lists by a literal test on one member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectByLiteral {
    /// The list walked.
    list: String,
    /// The member every arm tests.
    tested: String,
    /// The arms, in script order.
    arms: Vec<LiteralArm>,
}

/// `for (<var> in ctx.<list>)`, as the variable, the list, and the loop body.
fn loop_body(script: &str) -> Option<(String, String, String)> {
    let (_, rest) = script.split_once("for (")?;
    let (head, after) = rest.split_once(')')?;
    let (var, source) = head.split_once(" in ")?;
    let var = var.trim().rsplit(' ').next()?.trim();
    let list = clean_path(source.trim().strip_prefix("ctx.")?);
    if var.is_empty() || !is_ctx_path(&list) {
        return None;
    }
    let (body, _) = balanced(after.trim_start(), '{', '}')?;
    Some((var.to_owned(), list, body.to_owned()))
}

/// Whether the script DECLARES `local` as an empty list.
///
/// A declarator, not a bare assignment: `alerts = ...` somewhere in the middle
/// says nothing about what the accumulator started as, and an accumulator that
/// started as something else is not this pattern.
fn declares_empty_list(script: &str, local: &str) -> bool {
    script.split([';', '\n']).any(|statement| {
        let Some((lhs, rhs)) = statement.split_once('=') else {
            return false;
        };
        let mut words = lhs.split_whitespace();
        let Some(name) = words.next_back() else {
            return false;
        };
        // `def alerts` -- the declarator is what separates this from a reassignment.
        if name != local || words.next().is_none() {
            return false;
        }
        let value = rhs.trim();
        value.starts_with("[]") || value.starts_with("new ArrayList()")
    })
}

/// The `ctx.` path a local accumulator is handed to after the loop.
///
/// The statement may open with the loop's own closing brace --
/// `} ctx.alerts = alerts` is one "line" to a split on `;` -- so the subject is
/// the last token past any brace rather than the whole left-hand side.
fn handed_to_ctx(script: &str, local: &str) -> Option<String> {
    script.split([';', '\n']).find_map(|statement| {
        let (lhs, rhs) = statement.split_once('=')?;
        if rhs.trim() != local {
            return None;
        }
        let subject = lhs.rsplit(['{', '}']).next()?.trim();
        let path = clean_path(subject.strip_prefix("ctx.")?);
        is_ctx_path(&path).then_some(path)
    })
}

/// A single- or double-quoted literal's contents.
fn quoted(text: &str) -> Option<&str> {
    let text = text.trim();
    let quote = text.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    text.strip_prefix(quote)?.strip_suffix(quote)
}

/// A bare member name, or `None` where the fragment is anything richer.
fn member(fragment: &str) -> Option<&str> {
    let name = fragment.trim();
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')).then_some(name)
}

/// Read the partition, or decline the script.
///
/// Every arm has to test the SAME member against a quoted literal and append
/// one member of the record to a declared accumulator. One arm this cannot read
/// declines the whole script: a partition missing an arm writes a list the
/// vendor fills and leaves the rest of it empty, which reads as a correct
/// transform over a short list.
#[must_use]
pub fn parse_collect_by_literal(script: &str) -> Option<CollectByLiteral> {
    let (var, list, body) = loop_body(script)?;
    let opener = format!("if ({var}.");

    let mut tested: Option<String> = None;
    let mut arms = Vec::new();
    for chunk in body.split(&opener).skip(1) {
        let (condition, rest) = chunk.split_once(')')?;
        let (name, literal) = condition.split_once("==")?;
        let name = member(name)?;
        match &tested {
            // Two different members tested is a fan-out this cannot express.
            Some(held) if held != name => return None,
            Some(_) => {}
            None => tested = Some(name.to_owned()),
        }
        let literal = quoted(literal)?;

        let (head, added) = rest.split_once(".add(")?;
        // The append has to be the arm's own, not one further down the body.
        if head.contains(&opener) {
            return None;
        }
        let local = head.rsplit(['\n', '\r', ';', '{', '}']).next()?.trim();
        if !declares_empty_list(script, local) {
            return None;
        }
        let collected = member(added.split(')').next()?.strip_prefix(&format!("{var}."))?)?;

        arms.push(LiteralArm {
            literal: literal.to_owned(),
            member: collected.to_owned(),
            target: handed_to_ctx(script, local)?,
        });
    }

    let tested = tested?;
    (!arms.is_empty()).then_some(CollectByLiteral { list, tested, arms })
}

/// Fill each named list with the members of the records its literal selects.
///
/// The accumulator is written even when nothing selected it, because the script
/// assigns it unconditionally -- an empty list is what the `drop_empty` beside
/// these call sites then removes, and writing nothing at all would leave a
/// stale list from an earlier processor in its place.
///
/// A record missing the collected member contributes NOTHING. Painless appends
/// a null there, and every one of these call sites prunes nulls out of the list
/// a step later, so the two engines agree on the document.
#[must_use]
pub fn collect_by_literal(event: &mut Event, pattern: &CollectByLiteral) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    for arm in &pattern.arms {
        let mut collected = Vec::with_capacity(records.len());
        for record in &records {
            if record.get(&pattern.tested).and_then(Value::as_str) != Some(arm.literal.as_str()) {
                continue;
            }
            if let Some(value) = record.get(&arm.member).filter(|value| !value.is_null()) {
                collected.push(value.clone());
            }
        }
        let _ = event.set(&arm.target, Value::Array(collected));
    }
    true
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/vsphere_cluster/default.rs`, in the
    /// escaped one-line form a stored script reaches the matcher in.
    const CLUSTER: &str = r"def alerts = []; def warnings = []; for (alarm in ctx.vsphere.cluster.triggered_alarms) {\n    if (alarm.status == 'red') {\n    alerts.add(alarm.name);\n    }\n    if (alarm.status == 'yellow') {\n    warnings.add(alarm.name);\n    }\n} ctx.alerts = alerts; ctx.warnings = warnings;\n";

    /// The other six streams close without the semicolon, which is a different
    /// literal and so a different call site.
    const HOST: &str = r"def alerts = []; def warnings = []; for (alarm in ctx.vsphere.host.triggered_alarms) {\n    if (alarm.status == 'red') {\n    alerts.add(alarm.name);\n    }\n    if (alarm.status == 'yellow') {\n    warnings.add(alarm.name);\n    }\n} ctx.alerts = alerts; ctx.warnings = warnings\n";

    #[test]
    fn the_vsphere_alarms_partition_on_their_colour() {
        assert_eq!(
            parse_collect_by_literal(&crate::common::normalise(CLUSTER)),
            Some(CollectByLiteral {
                list: "vsphere.cluster.triggered_alarms".to_owned(),
                tested: "status".to_owned(),
                arms: vec![
                    LiteralArm {
                        literal: "red".to_owned(),
                        member: "name".to_owned(),
                        target: "alerts".to_owned(),
                    },
                    LiteralArm {
                        literal: "yellow".to_owned(),
                        member: "name".to_owned(),
                        target: "warnings".to_owned(),
                    },
                ],
            })
        );
    }

    /// The same script without the closing semicolon reads the same.
    #[test]
    fn the_semicolon_free_spelling_reads_the_same() {
        let parsed = parse_collect_by_literal(&crate::common::normalise(HOST)).unwrap();
        assert_eq!(parsed.list, "vsphere.host.triggered_alarms");
        assert_eq!(parsed.arms.len(), 2);
    }

    /// The document, not the parse. LIST ORDER, not sorted, and the names --
    /// `Host memory usage` is the one yellow alarm of the four the capture ships.
    #[test]
    fn the_names_land_in_list_order_under_their_colour() {
        let mut event = Event::new(json!({ "vsphere": { "cluster": { "triggered_alarms": [
            { "status": "red", "name": "Host hardware system board status" },
            { "status": "red", "name": "Host storage status" },
            { "status": "yellow", "name": "Host memory usage" },
            { "status": "red", "name": "CPU Utilization" },
        ]}}}));
        assert!(crate::common::try_known_painless(&mut event, CLUSTER));
        assert_eq!(
            event.get("alerts"),
            Some(&json!([
                "Host hardware system board status",
                "Host storage status",
                "CPU Utilization",
            ]))
        );
        assert_eq!(event.get("warnings"), Some(&json!(["Host memory usage"])));
    }

    /// No alarm of a colour leaves that list EMPTY rather than absent, which is
    /// what the vendor's own prune then removes.
    #[test]
    fn a_colour_nothing_matched_writes_an_empty_list() {
        let mut event = Event::new(json!({ "vsphere": { "cluster": { "triggered_alarms": [
            { "status": "yellow", "name": "Host memory usage" },
        ]}}}));
        assert!(crate::common::try_known_painless(&mut event, CLUSTER));
        assert_eq!(event.get("alerts"), Some(&json!([])));
        assert_eq!(event.get("warnings"), Some(&json!(["Host memory usage"])));
    }

    /// The walked list is left where it is: the expected document carries the
    /// alarms and the two name lists together.
    #[test]
    fn the_walked_list_is_left_alone() {
        let mut event = Event::new(json!({ "vsphere": { "cluster": { "triggered_alarms": [
            { "status": "red", "name": "A" },
        ]}}}));
        assert!(crate::common::try_known_painless(&mut event, CLUSTER));
        assert_eq!(
            event.get("vsphere.cluster.triggered_alarms"),
            Some(&json!([{ "status": "red", "name": "A" }]))
        );
    }

    /// A guard testing a member for PRESENCE is `CollectFromList`'s script, and
    /// claiming it here would collect nothing at all.
    #[test]
    fn a_presence_guard_is_declined() {
        let script = r"for (v in ctx.a.list) {\n  if (v.ip != null) {\n    ctx.related.ip.add(v.ip);\n  }\n}";
        assert!(parse_collect_by_literal(&crate::common::normalise(script)).is_none());
    }

    /// Two arms testing DIFFERENT members is a fan-out, not a partition.
    #[test]
    fn two_tested_members_decline_the_script() {
        let script = r"def a = []; def b = []; for (x in ctx.a.list) {\n  if (x.kind == 'one') { a.add(x.name); }\n  if (x.other == 'two') { b.add(x.name); }\n}\nctx.a = a; ctx.b = b;";
        assert!(parse_collect_by_literal(&crate::common::normalise(script)).is_none());
    }

    /// An accumulator nothing hands to a `ctx.` path writes nowhere, so the
    /// script is declined rather than half-applied.
    #[test]
    fn an_accumulator_never_handed_to_ctx_declines_the_script() {
        let script =
            r"def a = []; for (x in ctx.a.list) {\n  if (x.kind == 'one') { a.add(x.name); }\n}";
        assert!(parse_collect_by_literal(&crate::common::normalise(script)).is_none());
    }

    /// An accumulator that was never DECLARED as an empty list started as
    /// something else, and appending to it is a different operation.
    #[test]
    fn an_undeclared_accumulator_declines_the_script() {
        let script =
            r"for (x in ctx.a.list) {\n  if (x.kind == 'one') { a.add(x.name); }\n}\nctx.out = a;";
        assert!(parse_collect_by_literal(&crate::common::normalise(script)).is_none());
    }
}
