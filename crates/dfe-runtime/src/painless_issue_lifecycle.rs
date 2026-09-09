// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! An issue classified by its own deadline: pending, blocked, or resolved.
//!
//! kolide's issues stream reads a params table for the base category, adds a
//! security domain keyed by the check id, then decides the event type and the
//! action from two timestamps -- a block deadline that has not yet passed is
//! PENDING, and one that has is a block that already happened:
//!
//! ```painless
//! def action = ctx.event.action;
//! def m = params.exact.get(action);
//! if (m != null) { ctx.event.kind = m.kind; ctx.event.category = new ArrayList(m.category); }
//! else { ctx.event.kind = 'event'; ctx.event.category = ['configuration']; }
//! if (ctx.rule?.id != null) {
//!   def domain = params.check_category.get(ctx.rule.id);
//!   if (domain != null && !ctx.event.category.contains(domain)) { ctx.event.category.add(domain); }
//! }
//! boolean pendingBlock = (ctx.kolide?.issues?.blocks_device_at != null);
//! boolean blocked = false;
//! if (pendingBlock && ctx['@timestamp'] != null) {
//!   ZonedDateTime blockAt = ZonedDateTime.parse(ctx.kolide.issues.blocks_device_at);
//!   ZonedDateTime eventTime = ZonedDateTime.parse(ctx['@timestamp']);
//!   blocked = !blockAt.isAfter(eventTime);
//! }
//! boolean resolved = (ctx.kolide?.issues?.resolved_at != null) || (action == 'issues.resolved');
//! ctx.event.type = (pendingBlock || resolved) ? ['change'] : ['creation'];
//! ctx._tmp = ctx._tmp == null ? [:] : ctx._tmp;
//! ctx._tmp.blocked = blocked;
//! if (action == 'issue') {
//!   if (resolved) { ctx.event.action = 'resolved'; }
//!   else if (blocked) { ctx.event.action = 'blocked'; }
//!   else if (pendingBlock) { ctx.event.action = 'will_be_blocked'; }
//! }
//! ```
//!
//! `RowOrDefaults` reads the lookup at the top and DECLINES this script by
//! design -- its parse requires the branch to be the end of the text, because
//! claiming it would drop everything below. `GuardedCopy` took it instead and
//! resolved every guard over a boolean local to `Never`, so it wrote the base
//! category and never wrote `event.type` at all.
//!
//! The deadline comparison is the whole point: kolide sends the same issue
//! before and after its block takes effect, and only the timestamps tell the
//! two apart.

use serde_json::{Map, Value};

use crate::Event;
use crate::painless_params::{balanced, clean_path, literal_value, skip_trivia, subject_path};

/// A second table keyed by another field, appended to the category list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainAppend {
    /// The params member holding the table.
    table: String,
    /// The `ctx.` path its key is read from.
    key: String,
    target: String,
}

/// The lifecycle decided from a params row and two timestamps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueLifecycle {
    /// The `ctx.` path the action is read from, before anything rewrites it.
    subject: String,
    /// The params table keyed by the action.
    table: String,
    /// Members of the row, and where each is written.
    columns: Vec<(String, String)>,
    /// Literal writes where the action has no row.
    defaults: Vec<(String, Value)>,
    domain: DomainAppend,
    /// The deadline whose presence means a block is pending.
    deadline: String,
    /// The timestamp the deadline is measured against.
    clock: String,
    /// The field whose presence means the issue is resolved.
    resolved: String,
    /// The action value that also means resolved.
    resolved_action: String,
    /// Where the change/creation answer goes, and both literals.
    type_target: String,
    changed: Value,
    created: Value,
    /// Where the computed blocked flag is kept for a later processor.
    flag_target: String,
    /// The action the rewrite applies to, and what it becomes.
    rewrite_when: String,
    rewrite_target: String,
    on_resolved: Value,
    on_blocked: Value,
    on_pending: Value,
}

/// The `ctx.` path a `<something> ctx.<path> != null` test names.
fn path_before_null_test(text: &str) -> Option<String> {
    let (head, _) = text.split_once("!= null")?;
    let path = head.trim_end().rsplit(['(', ' ', '\n']).next()?;
    let path = clean_path(
        path.trim()
            .strip_prefix("ctx")?
            .trim_start_matches(['?', '.']),
    );
    (!path.is_empty()).then_some(path)
}

/// The literal a `ctx.<path> = <literal>;` statement writes, by target.
fn literal_assignment(text: &str) -> Option<(String, Value)> {
    let (target, value) = text.split_once('=')?;
    let target = clean_path(
        target
            .trim()
            .rsplit(['\n', '{', '}', ';'])
            .next()?
            .trim()
            .strip_prefix("ctx.")?,
    );
    let value = literal_value(value.trim().trim_end_matches(';').trim())?;
    (!target.is_empty()).then_some((target, value))
}

/// Read the tables, the two timestamps and every literal, or decline.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn parse_issue_lifecycle(script: &str) -> Option<IssueLifecycle> {
    // `def <local> = ctx.<subject>;`
    let (head, rest) = script.split_once(" = ctx.")?;
    let local = head.trim().strip_prefix("def ")?.trim();
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let subject = clean_path(rest.split_once(';')?.0.trim());

    // `def <row> = params.<table>.get(<local>);`
    let (head, rest) = script.split_once(&format!(".get({local});"))?;
    let (head, table) = head.rsplit_once("params.")?;
    let row = head
        .trim_end()
        .strip_suffix('=')?
        .trim_end()
        .rsplit([' ', '\n', ';'])
        .next()?
        .trim()
        .to_owned();
    if table.is_empty() || !table.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // `if (<row> != null) { <columns> } else { <defaults> }`
    let branch = rest.find(&format!("if ({row} != null)"))?;
    let (_, after) = balanced(skip_trivia(&rest[branch + "if".len()..]), '(', ')')?;
    let (then, after) = balanced(skip_trivia(after), '{', '}')?;
    let (otherwise, after) = balanced(
        skip_trivia(skip_trivia(after).strip_prefix("else")?),
        '{',
        '}',
    )?;

    // `ctx.<target> = <row>.<member>`, with or without the `new ArrayList(`
    // wrapper the list columns carry.
    let member_of = format!("{row}.");
    let mut columns = Vec::new();
    for statement in then.split(';') {
        let Some((assigned, member)) = statement.split_once(&member_of) else {
            continue;
        };
        let assigned = assigned.trim_end();
        let assigned = assigned.strip_suffix("new ArrayList(").unwrap_or(assigned);
        let Some(target) = assigned
            .trim_end()
            .strip_suffix('=')
            .map(str::trim)
            .and_then(|head| head.rsplit(['\n', '{', '}']).next())
            .map(str::trim)
            .and_then(|line| line.strip_prefix("ctx."))
        else {
            continue;
        };
        let member: String = member
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !member.is_empty() {
            columns.push((clean_path(target), member));
        }
    }
    let defaults: Vec<(String, Value)> = otherwise
        .split(';')
        .filter_map(literal_assignment)
        .collect();
    if columns.is_empty() || defaults.is_empty() {
        return None;
    }

    // `def <d> = params.<table>.get(ctx.<key>); ... ctx.<target>.add(<d>);`
    let (head, rest) = after.split_once(".get(ctx.")?;
    let (_, domain_table) = head.rsplit_once("params.")?;
    let (key, rest) = rest.split_once(')')?;
    let domain_target = rest
        .split_once(".add(")?
        .0
        .rsplit(['\n', ' ', '(', '!'])
        .next()?
        .trim()
        .strip_prefix("ctx.")?;
    if domain_table.is_empty()
        || !domain_table
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }
    let domain = DomainAppend {
        table: domain_table.to_owned(),
        key: clean_path(key.trim()),
        target: clean_path(domain_target.trim()),
    };

    // `boolean <p> = (ctx.<deadline> != null);` and the comparison under it.
    let (_, rest) = script.split_once("boolean ")?;
    let deadline = path_before_null_test(rest.split_once(';')?.0)?;
    let (_, rest) = script.split_once(".isAfter(")?;
    // The clock is the SECOND timestamp parsed, and it is the one spelled as a
    // bracket subscript.
    let clock = {
        let (guarded, _) = script.split_once(".isAfter(")?;
        let (_, second) = guarded.rsplit_once("ZonedDateTime.parse(")?;
        let path = subject_path(second.split_once(')')?.0.trim());
        let stripped = path
            .strip_prefix("ctx")?
            .trim_start_matches(['?', '.'])
            .to_owned();
        clean_path(&stripped)
    };
    if clock.is_empty() || deadline.is_empty() {
        return None;
    }

    // `boolean <r> = (ctx.<resolved> != null) || (<local> == '<value>');`
    let (_, rest) = rest.split_once("boolean ")?;
    let (declaration, rest) = rest.split_once(';')?;
    let resolved = path_before_null_test(declaration)?;
    let resolved_action = declaration
        .rsplit_once("==")?
        .1
        .trim()
        .trim_end_matches(')')
        .trim()
        .trim_matches(['\'', '"'])
        .to_owned();
    if resolved_action.is_empty() {
        return None;
    }

    // `ctx.<target> = (<p> || <r>) ? <changed> : <created>;`
    let (head, ternary) = rest.split_once('?')?;
    let type_target = clean_path(
        head.split_once('=')?
            .0
            .trim()
            .rsplit(['\n', ';'])
            .next()?
            .trim()
            .strip_prefix("ctx.")?,
    );
    let (changed, rest) = ternary.split_once(':')?;
    let (created, rest) = rest.split_once(';')?;
    let changed = literal_value(changed.trim())?;
    let created = literal_value(created.trim())?;

    // `ctx.<flag> = <blocked>;`, the boolean kept for a later processor.
    let flag_target = clean_path(
        rest.rsplit_once("= blocked;")?
            .0
            .trim()
            .rsplit(['\n', ';', '}'])
            .next()?
            .trim()
            .strip_prefix("ctx.")?,
    );

    // `if (<local> == '<when>') { ... }`, the three-step precedence.
    let (_, rest) = script.rsplit_once(&format!("if ({local} == "))?;
    let (when, rest) = rest.split_once(')')?;
    let rewrite_when = when.trim().trim_matches(['\'', '"']).to_owned();
    let writes: Vec<(String, Value)> = rest.split(';').filter_map(literal_assignment).collect();
    if rewrite_when.is_empty() || writes.len() != 3 {
        return None;
    }
    let rewrite_target = writes[0].0.clone();
    if writes.iter().any(|(target, _)| *target != rewrite_target) {
        return None;
    }

    Some(IssueLifecycle {
        subject,
        table: table.to_owned(),
        columns,
        defaults,
        domain,
        deadline,
        clock,
        resolved,
        resolved_action,
        type_target,
        changed,
        created,
        flag_target,
        rewrite_when,
        rewrite_target,
        on_resolved: writes[0].1.clone(),
        on_blocked: writes[1].1.clone(),
        on_pending: writes[2].1.clone(),
    })
}

/// Whether the deadline has already passed, which is what `!isAfter` asks.
fn deadline_passed(event: &Event, deadline: &str, clock: &str) -> bool {
    let parse = |path: &str| {
        event
            .get(path)
            .and_then(Value::as_str)
            .and_then(|text| chrono::DateTime::parse_from_rfc3339(text).ok())
    };
    match (parse(deadline), parse(clock)) {
        (Some(at), Some(now)) => at <= now,
        _ => false,
    }
}

/// Classify the issue and rewrite its action.
#[must_use]
pub fn issue_lifecycle(
    event: &mut Event,
    pattern: &IssueLifecycle,
    params: &Map<String, Value>,
) -> bool {
    let action = event.get_as_string(&pattern.subject).unwrap_or_default();

    // The base category, from the table or from the literals.
    match params
        .get(&pattern.table)
        .and_then(|table| table.get(&action))
        .cloned()
    {
        Some(row) => {
            for (target, member) in &pattern.columns {
                if let Some(value) = row.get(member).filter(|value| !value.is_null()) {
                    let _ = event.set(target, value.clone());
                }
            }
        }
        None => {
            for (target, value) in &pattern.defaults {
                let _ = event.set(target, value.clone());
            }
        }
    }

    // The security domain for this check, added to the base category.
    if let Some(key) = event.get_as_string(&pattern.domain.key)
        && let Some(domain) = params
            .get(&pattern.domain.table)
            .and_then(|table| table.get(&key))
            .filter(|value| !value.is_null())
            .cloned()
        && let Some(Value::Array(mut held)) = event.get(&pattern.domain.target).cloned()
        && !held.contains(&domain)
    {
        held.push(domain);
        let _ = event.set(&pattern.domain.target, Value::Array(held));
    }

    let pending = event.has_value(&pattern.deadline);
    let blocked = pending && deadline_passed(event, &pattern.deadline, &pattern.clock);
    let resolved = event.has_value(&pattern.resolved) || action == pattern.resolved_action;

    let _ = event.set(
        &pattern.type_target,
        if pending || resolved {
            pattern.changed.clone()
        } else {
            pattern.created.clone()
        },
    );
    let _ = event.set(&pattern.flag_target, Value::Bool(blocked));

    if action == pattern.rewrite_when {
        let written = if resolved {
            Some(&pattern.on_resolved)
        } else if blocked {
            Some(&pattern.on_blocked)
        } else if pending {
            Some(&pattern.on_pending)
        } else {
            None
        };
        if let Some(written) = written {
            let _ = event.set(&pattern.rewrite_target, written.clone());
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
    use crate::painless_plan::{PainlessPlan, painless_exec_plan_params};
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/kolide_issues/default.rs`.
    const KOLIDE_ISSUES: &str = r#"def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['configuration'];\n}\n\n// Add a security-domain category (malware/vulnerability) for specific\n// checks, keyed by check id; the base 'configuration' category is kept.\nif (ctx.rule?.id != null) {\n  def domain = params.check_category.get(ctx.rule.id);\n  if (domain != null && !ctx.event.category.contains(domain)) {\n    ctx.event.category.add(domain);\n  }\n}\n\n// blocked/resolved issues are a change; a newly detected issue is a creation.\n// blocked_device_at is a future deadline, not an immediate state: the device\n// is only actually blocked once that deadline has passed relative to this event.\nboolean pendingBlock = (ctx.kolide?.issues?.blocks_device_at != null);\nboolean blocked = false;\nif (pendingBlock && ctx['@timestamp'] != null) {\n  ZonedDateTime blockAt = ZonedDateTime.parse(ctx.kolide.issues.blocks_device_at);\n  ZonedDateTime eventTime = ZonedDateTime.parse(ctx['@timestamp']);\n  blocked = !blockAt.isAfter(eventTime);\n}\nboolean resolved = (ctx.kolide?.issues?.resolved_at != null) || (action == 'issues.resolved');\nctx.event.type = (pendingBlock || resolved) ? ['change'] : ['creation'];\n\n// Persist the computed blocked state for the fingerprint processor in\n// default.yml, so pending/blocked/resolved each fingerprint to a distinct _id.\nctx._tmp = ctx._tmp == null ? [:] : ctx._tmp;\nctx._tmp.blocked = blocked;\n\n// Precedence: a resolved issue takes priority over a still-blocking one;\n// a pending future block takes priority over a plain open issue.\nif (action == 'issue') {\n  if (resolved) {\n    ctx.event.action = 'resolved';\n  } else if (blocked) {\n    ctx.event.action = 'blocked';\n  } else if (pendingBlock) {\n    ctx.event.action = 'will_be_blocked';\n  }\n}"#;

    /// kolide's auth stream, which reads the same `params.exact` table and is
    /// `RowOrDefaults`. It must not be claimed here.
    const KOLIDE_AUTH: &str = "def action = ctx.event.action;\nctx.event.kind = 'event';\n\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['authentication'];\n  ctx.event.type = ['info'];\n}";

    fn params() -> Value {
        json!({
            "exact": {},
            "check_category": { "20": "malware", "41": "vulnerability" },
        })
    }

    /// Run one of the stream's own documents through the plan.
    fn classify(document: Value) -> Event {
        let mut event = Event::new(document);
        let plan = PainlessPlan::new(KOLIDE_ISSUES);
        assert!(painless_exec_plan_params(&mut event, &plan, &params()).is_ok());
        event
    }

    #[test]
    fn the_kolide_issue_lifecycle_reads_both_tables_and_both_timestamps() {
        let held = format!(
            "{:?}",
            parse_issue_lifecycle(&crate::painless_common::normalise(KOLIDE_ISSUES))
        );
        for part in [
            r#"subject: "event.action""#,
            r#"table: "exact""#,
            r#"columns: [("event.kind", "kind"), ("event.category", "category")]"#,
            r#"defaults: [("event.kind", String("event")), ("event.category", Array [String("configuration")])]"#,
            r#"table: "check_category", key: "rule.id", target: "event.category""#,
            r#"deadline: "kolide.issues.blocks_device_at""#,
            r#"clock: "@timestamp""#,
            r#"resolved: "kolide.issues.resolved_at""#,
            r#"resolved_action: "issues.resolved""#,
            r#"type_target: "event.type""#,
            r#"changed: Array [String("change")]"#,
            r#"created: Array [String("creation")]"#,
            r#"flag_target: "_tmp.blocked""#,
            r#"rewrite_when: "issue""#,
            r#"rewrite_target: "event.action""#,
            r#"on_resolved: String("resolved")"#,
            r#"on_blocked: String("blocked")"#,
            r#"on_pending: String("will_be_blocked")"#,
        ] {
            assert!(held.contains(part), "lost {part}: {held}");
        }
    }

    /// The WRITTEN values, over the stream's own documents. A deadline that has
    /// not yet passed is PENDING and one that has is a block that happened, and
    /// only the two timestamps separate them.
    #[test]
    fn the_deadline_decides_pending_from_blocked() {
        // A plain open issue.
        let event = classify(json!({
            "event": { "action": "issue" },
            "@timestamp": "2026-06-14T20:31:58.869Z",
            "rule": { "id": "32" },
        }));
        assert_eq!(event.get("event.kind"), Some(&json!("event")));
        assert_eq!(event.get("event.category"), Some(&json!(["configuration"])));
        assert_eq!(event.get("event.type"), Some(&json!(["creation"])));
        assert_eq!(event.get("event.action"), Some(&json!("issue")));
        assert_eq!(event.get("_tmp.blocked"), Some(&json!(false)));

        // A deadline in 2099 has not passed: pending, not blocked.
        let event = classify(json!({
            "event": { "action": "issue" },
            "@timestamp": "2026-08-27T06:01:02.066187847Z",
            "rule": { "id": "32" },
            "kolide": { "issues": { "blocks_device_at": "2099-01-01T00:00:00.000Z" } },
        }));
        assert_eq!(event.get("event.type"), Some(&json!(["change"])));
        assert_eq!(event.get("event.action"), Some(&json!("will_be_blocked")));
        assert_eq!(event.get("_tmp.blocked"), Some(&json!(false)));

        // The same deadline, now reached to the millisecond.
        let event = classify(json!({
            "event": { "action": "issue" },
            "@timestamp": "2026-06-16T00:00:00Z",
            "rule": { "id": "32" },
            "kolide": { "issues": { "blocks_device_at": "2026-06-16T00:00:00.000Z" } },
        }));
        assert_eq!(event.get("event.type"), Some(&json!(["change"])));
        assert_eq!(event.get("event.action"), Some(&json!("blocked")));
        assert_eq!(event.get("_tmp.blocked"), Some(&json!(true)));

        // Resolved outranks a block that already happened.
        let event = classify(json!({
            "event": { "action": "issue" },
            "@timestamp": "2026-06-20T00:00:00Z",
            "rule": { "id": "32" },
            "kolide": { "issues": {
                "blocks_device_at": "2026-06-16T00:00:00.000Z",
                "resolved_at": "2026-06-20T00:00:00.000Z",
            } },
        }));
        assert_eq!(event.get("event.type"), Some(&json!(["change"])));
        assert_eq!(event.get("event.action"), Some(&json!("resolved")));
    }

    /// The check id adds a security domain beside the base category, and the
    /// webhook actions are classified without being rewritten.
    #[test]
    fn the_check_id_adds_a_domain_and_the_webhook_actions_stand() {
        let event = classify(json!({
            "event": { "action": "issue" },
            "@timestamp": "2024-03-11T09:00:00Z",
            "rule": { "id": "20" },
        }));
        assert_eq!(
            event.get("event.category"),
            Some(&json!(["configuration", "malware"]))
        );

        let event = classify(json!({
            "event": { "action": "issue" },
            "@timestamp": "2024-03-11T10:00:00Z",
            "rule": { "id": "41" },
        }));
        assert_eq!(
            event.get("event.category"),
            Some(&json!(["configuration", "vulnerability"]))
        );

        // `issues.resolved` means resolved, and keeps its own action.
        let event = classify(json!({
            "event": { "action": "issues.resolved" },
            "@timestamp": "2023-10-28T21:00:58.000Z",
            "rule": { "id": "71" },
            "kolide": { "issues": { "resolved_at": "2023-10-28T21:00:58.000Z" } },
        }));
        assert_eq!(event.get("event.type"), Some(&json!(["change"])));
        assert_eq!(event.get("event.action"), Some(&json!("issues.resolved")));

        let event = classify(json!({
            "event": { "action": "issues.new" },
            "@timestamp": "2023-10-28T20:50:15.000Z",
            "rule": { "id": "71" },
        }));
        assert_eq!(event.get("event.type"), Some(&json!(["creation"])));
        assert_eq!(event.get("event.action"), Some(&json!("issues.new")));
    }

    /// kolide's auth stream reads the same table and belongs to
    /// `RowOrDefaults`, which writes `event.type` off the row rather than off a
    /// deadline this script does not have.
    #[test]
    fn the_sibling_auth_stream_is_declined() {
        assert!(parse_issue_lifecycle(&crate::painless_common::normalise(KOLIDE_AUTH)).is_none());
    }
}
