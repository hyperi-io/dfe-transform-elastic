// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Whole seconds between two parsed timestamps, written as one map member.
//!
//! github reports how long a finding stayed open by parsing two ISO-8601
//! strings, taking each to epoch seconds and putting the difference under
//! `sec`:
//!
//! ```painless
//! def time_to_resolution = new HashMap();
//! def resolvedAtDt = ctx.github.secret_scanning.resolved_at;
//! def createdAtDt = ctx.github.secret_scanning.created_at;
//! ZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);
//! long createdAtEpoch = zdt.toEpochSecond();
//! zdt = ZonedDateTime.parse(resolvedAtDt);
//! long resolvedAtEpoch = zdt.toEpochSecond();
//! time_to_resolution.put("sec", resolvedAtEpoch - createdAtEpoch);
//! ctx.github.secret_scanning.time_to_resolution = time_to_resolution;
//! ```
//!
//! The code scanning stream writes the same thing over two candidate
//! timestamps, `fixed_at` and then `dismissed_at`, in an `if`/`else` that
//! reads the first as a presence test. Both spellings therefore reduce to one
//! subtrahend and a list of candidate minuends, and the first candidate the
//! event carries decides -- which is what the branch means.
//!
//! Only three files in the generated tree call `toEpochSecond`, all three
//! github's, and unclaimed the pair cost 7 events: 4 on
//! `github.code_scanning.time_to_resolution.sec` and 3 on the secret scanning
//! twin. `github.issues.time_to_close` is the third call site and the same
//! grammar.

use serde_json::{Map, Value};

use crate::Event;
use crate::painless_params::{balanced, clean_path, ctx_path_term as ctx_path};

/// The seconds between one timestamp and whichever candidate is present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondsBetween {
    /// The earlier timestamp, subtracted from the candidate.
    from: String,
    /// The later timestamps, in the order the script tests them.
    to: Vec<String>,
    /// The key the difference is put under -- `sec` at every call site.
    key: String,
    /// Where the one-member map is written.
    target: String,
}

impl SecondsBetween {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        from: impl Into<String>,
        to: Vec<String>,
        key: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            from: from.into(),
            to,
            key: key.into(),
            target: target.into(),
        }
    }
}

/// `<Type>? <name> = <value>`, with the type word dropped.
fn declaration(statement: &str) -> Option<(&str, &str)> {
    let (head, value) = statement.split_once('=')?;
    if value.starts_with('=') || head.ends_with(['!', '<', '>']) {
        return None;
    }
    let head = head.trim();
    if head.contains(['(', ')', '[', ']', '.']) {
        return None;
    }
    let name = head.rsplit(char::is_whitespace).next()?;
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then_some((name, value))
}

/// A `ctx.<path>` term as a dotted path, declining a call or a subscript.
/// The value bound to `local`, from a list of `(local, value)` pairs.
fn bound_to(pairs: &[(String, String)], local: &str) -> Option<String> {
    pairs
        .iter()
        .find(|(name, _)| name == local)
        .map(|(_, value)| value.clone())
}

/// Split a `;`-separated chunk into the branch scaffolding ahead of its
/// statement and the statement itself.
///
/// The scaffolding is read whole rather than searched: a branch this reader
/// skipped over would pick a timestamp it has not seen.
fn chunk_parts(chunk: &str) -> (String, &str) {
    let (head, statement) = chunk
        .rfind(['{', '}'])
        .map_or(("", chunk), |at| (&chunk[..=at], &chunk[at + 1..]));
    let scaffolding = head
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '{' && *c != '}')
        .collect();
    (scaffolding, statement.trim())
}

/// Read the subtraction, or decline it.
///
/// Every statement has to be one this matcher reproduces, and every branch has
/// to be a presence test on one of the candidates -- a guard deciding on
/// anything else picks a timestamp this reader cannot see.
#[must_use]
pub fn parse_seconds_between(script: &str) -> Option<SecondsBetween> {
    let mut accumulator: Option<String> = None;
    let mut clock: Option<String> = None;
    let mut bound: Vec<(String, String)> = Vec::new();
    let mut epochs: Vec<(String, String)> = Vec::new();
    let mut parsed: Option<String> = None;
    let mut guarded: Vec<String> = Vec::new();
    let mut key: Option<String> = None;
    let mut from: Option<String> = None;
    let mut to: Vec<String> = Vec::new();
    let mut target: Option<String> = None;

    for chunk in script.split(';') {
        let (scaffolding, statement) = chunk_parts(chunk);
        // `else`, `if (<local> != null)` and `else if (..)` are the whole of
        // what may sit ahead of a statement; anything else is unread.
        let branch = scaffolding.strip_prefix("else").unwrap_or(&scaffolding);
        if !branch.is_empty() {
            let guard = branch.strip_prefix("if(")?.strip_suffix(')')?;
            guarded.push(guard.strip_suffix("!=null")?.to_owned());
        }
        if statement.is_empty() {
            continue;
        }

        if statement.starts_with("ctx.") {
            let (written, value) = statement.split_once('=')?;
            if Some(value.trim()) != accumulator.as_deref() {
                return None;
            }
            let path = ctx_path(written)?;
            match &target {
                Some(held) if held != &path => return None,
                Some(_) => {}
                None => target = Some(path),
            }
            continue;
        }

        if let Some(holder) = accumulator.as_deref()
            && let Some(rest) = statement.strip_prefix(holder)
            && let Some(rest) = rest.strip_prefix(".put")
        {
            let (arguments, tail) = balanced(rest.trim_start(), '(', ')')?;
            if !tail.trim().is_empty() {
                return None;
            }
            let (name, difference) = arguments.split_once(',')?;
            let name = name.trim().trim_matches(['"', '\'']).to_owned();
            if name.is_empty() {
                return None;
            }
            match &key {
                Some(held) if held != &name => return None,
                Some(_) => {}
                None => key = Some(name),
            }
            let (later, earlier) = difference.split_once('-')?;
            let later = bound_to(&epochs, later.trim())?;
            let earlier = bound_to(&epochs, earlier.trim())?;
            match &from {
                Some(held) if held != &earlier => return None,
                Some(_) => {}
                None => from = Some(earlier),
            }
            to.push(later);
            continue;
        }

        let (name, value) = declaration(statement)?;
        let value = value.trim();
        if value == "new HashMap()" {
            if accumulator.replace(name.to_owned()).is_some() {
                return None;
            }
        } else if let Some(inner) = value
            .strip_prefix("ZonedDateTime.parse(")
            .and_then(|rest| rest.strip_suffix(')'))
        {
            match &clock {
                Some(held) if held != name => return None,
                Some(_) => {}
                None => clock = Some(name.to_owned()),
            }
            parsed = Some(bound_to(&bound, inner.trim())?);
        } else if let Some(receiver) = value.strip_suffix(".toEpochSecond()") {
            if Some(receiver.trim()) != clock.as_deref() {
                return None;
            }
            epochs.push((name.to_owned(), parsed.clone()?));
        } else {
            bound.push((name.to_owned(), ctx_path(value)?));
        }
    }

    let pattern = SecondsBetween::new(from?, to, key?, target?);
    if pattern.to.is_empty() || pattern.to.contains(&pattern.from) {
        return None;
    }
    // A branch that tests something other than a candidate decides on a value
    // this reader has not read.
    for local in &guarded {
        if !pattern.to.contains(&bound_to(&bound, local)?) {
            return None;
        }
    }
    Some(pattern)
}

/// The epoch second of an ISO-8601 timestamp, which is what
/// `ZonedDateTime.parse` takes and `toEpochSecond` returns.
fn epoch_second(text: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(text)
        .ok()
        .map(|held| held.timestamp())
}

/// Write the difference, or leave the target alone.
///
/// A timestamp the event does not carry, or one that will not parse, writes
/// nothing -- Painless throws there and a guessed span is worse than none.
pub fn seconds_between(event: &mut Event, pattern: &SecondsBetween) -> bool {
    let Some(from) = event.get_str(&pattern.from).and_then(epoch_second) else {
        return true;
    };
    let Some(to) = pattern
        .to
        .iter()
        .find_map(|path| event.get_str(path).and_then(epoch_second))
    else {
        return true;
    };
    let mut record = Map::new();
    record.insert(pattern.key.clone(), Value::from(to - from));
    let _ = event.set(&pattern.target, Value::Object(record));
    true
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::painless_common::{normalise, try_known_painless};
    use serde_json::json;

    /// Verbatim from `github_secret_scanning/default.rs`, in the escaped
    /// one-line form the call site holds -- a stored script arrives with its
    /// newlines escaped, so a test written with real newlines passes over a
    /// defect in the resolution step.
    const SECRET_SCANNING: &str = r#"def time_to_resolution = new HashMap();\ndef resolvedAtDt = ctx.github.secret_scanning.resolved_at;\ndef createdAtDt = ctx.github.secret_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(resolvedAtDt);\nlong resolvedAtEpoch = zdt.toEpochSecond();\ntime_to_resolution.put(\"sec\", resolvedAtEpoch - createdAtEpoch);\nctx.github.secret_scanning.time_to_resolution = time_to_resolution;\n"#;

    /// Verbatim from `github_code_scanning/default.rs`, the branched twin.
    const CODE_SCANNING: &str = r#"def time_to_resolution = new HashMap();\ndef fixedAtDt = ctx.github.code_scanning.fixed_at;\ndef dismissedAtDt = ctx.github.code_scanning.dismissed_at;\ndef createdAtDt = ctx.github.code_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nif (fixedAtDt != null) {\n    zdt = ZonedDateTime.parse(fixedAtDt);\n    long fixedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", fixedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\nelse {\n    zdt = ZonedDateTime.parse(dismissedAtDt);\n    long dismissedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", dismissedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\n"#;

    /// Verbatim from `github_issues/default.rs`, the third call site.
    const ISSUES_TIME_TO_CLOSE: &str = r#"def time_to_close = new HashMap();\ndef closedAtDt = ctx.github.issues.closed_at;\ndef createdAtDt = ctx.github.issues.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(closedAtDt);\nlong closedAtEpoch = zdt.toEpochSecond();\ntime_to_close.put(\"sec\", closedAtEpoch - createdAtEpoch);\nctx.github.issues.time_to_close = time_to_close;\n"#;

    #[test]
    fn the_straight_line_span_is_seconds_under_its_own_key() {
        let parsed = parse_seconds_between(&normalise(SECRET_SCANNING)).expect("recognised");
        assert_eq!(parsed.from, "github.secret_scanning.created_at");
        assert_eq!(parsed.to, ["github.secret_scanning.resolved_at"]);
        assert_eq!(parsed.key, "sec");

        let mut event = Event::new(json!({ "github": { "secret_scanning": {
            "created_at": "2020-11-06T18:48:51Z",
            "resolved_at": "2020-11-07T02:47:13Z",
        }}}));
        assert!(try_known_painless(&mut event, SECRET_SCANNING));
        assert_eq!(
            event.get("github.secret_scanning.time_to_resolution"),
            Some(&json!({ "sec": 28702 }))
        );

        // No resolution writes nothing at all.
        let mut open = Event::new(json!({ "github": { "secret_scanning": {
            "created_at": "2020-11-06T18:18:30Z",
        }}}));
        assert!(try_known_painless(&mut open, SECRET_SCANNING));
        assert_eq!(open.get("github.secret_scanning.time_to_resolution"), None);
    }

    /// The branch reduces to two candidates, and the second is reached only
    /// where the first is absent.
    #[test]
    fn the_branched_span_takes_the_first_candidate_the_event_carries() {
        let parsed = parse_seconds_between(&normalise(CODE_SCANNING)).expect("recognised");
        assert_eq!(parsed.from, "github.code_scanning.created_at");
        assert_eq!(
            parsed.to,
            [
                "github.code_scanning.fixed_at",
                "github.code_scanning.dismissed_at",
            ]
        );

        // Dismissed only, which is every resolved capture github ships.
        let mut dismissed = Event::new(json!({ "github": { "code_scanning": {
            "created_at": "2020-02-13T12:29:18Z",
            "dismissed_at": "2020-02-14T12:29:18.000Z",
        }}}));
        assert!(try_known_painless(&mut dismissed, CODE_SCANNING));
        assert_eq!(
            dismissed.get("github.code_scanning.time_to_resolution"),
            Some(&json!({ "sec": 86400 }))
        );

        // Both present, and the fixed date is the one the branch reaches
        // first.
        let mut both = Event::new(json!({ "github": { "code_scanning": {
            "created_at": "2020-02-13T12:29:18Z",
            "fixed_at": "2020-02-13T13:29:18Z",
            "dismissed_at": "2020-02-14T12:29:18.000Z",
        }}}));
        assert!(try_known_painless(&mut both, CODE_SCANNING));
        assert_eq!(
            both.get("github.code_scanning.time_to_resolution"),
            Some(&json!({ "sec": 3600 }))
        );
    }

    #[test]
    fn the_third_call_site_reads_the_same_way() {
        let parsed = parse_seconds_between(&normalise(ISSUES_TIME_TO_CLOSE)).expect("recognised");
        assert_eq!(parsed.target, "github.issues.time_to_close");
        assert_eq!(parsed.to, ["github.issues.closed_at"]);
    }

    /// A statement this reader does not reproduce declines the whole script,
    /// rather than writing the span and dropping the rest.
    #[test]
    fn a_script_that_does_more_than_the_subtraction_is_declined() {
        // A second field written beside the span.
        let extra = r#"def m = new HashMap();\ndef a = ctx.x.a;\ndef b = ctx.x.b;\nZonedDateTime zdt = ZonedDateTime.parse(b);\nlong be = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(a);\nlong ae = zdt.toEpochSecond();\nm.put(\"sec\", ae - be);\nctx.x.c = m;\nctx.x.d = 1;\n"#;
        assert!(parse_seconds_between(&normalise(extra)).is_none());

        // A branch deciding on something that is not a candidate.
        let elsewhere = r#"def m = new HashMap();\ndef a = ctx.x.a;\ndef b = ctx.x.b;\ndef flag = ctx.x.flag;\nZonedDateTime zdt = ZonedDateTime.parse(b);\nlong be = zdt.toEpochSecond();\nif (flag != null) {\n  zdt = ZonedDateTime.parse(a);\n  long ae = zdt.toEpochSecond();\n  m.put(\"sec\", ae - be);\n  ctx.x.c = m;\n}\n"#;
        assert!(parse_seconds_between(&normalise(elsewhere)).is_none());

        // A branch that is not a presence test at all.
        let width = r#"def m = new HashMap();\ndef a = ctx.x.a;\ndef b = ctx.x.b;\nZonedDateTime zdt = ZonedDateTime.parse(b);\nlong be = zdt.toEpochSecond();\nif (a.length() > 3) {\n  zdt = ZonedDateTime.parse(a);\n  long ae = zdt.toEpochSecond();\n  m.put(\"sec\", ae - be);\n  ctx.x.c = m;\n}\n"#;
        assert!(parse_seconds_between(&normalise(width)).is_none());

        // A loop around the subtraction, which this reader never enters.
        let looped = r#"def m = new HashMap();\ndef a = ctx.x.a;\ndef b = ctx.x.b;\nZonedDateTime zdt = ZonedDateTime.parse(b);\nlong be = zdt.toEpochSecond();\nfor (def i: ctx.x.list) {\n  zdt = ZonedDateTime.parse(a);\n  long ae = zdt.toEpochSecond();\n  m.put(\"sec\", ae - be);\n  ctx.x.c = m;\n}\n"#;
        assert!(parse_seconds_between(&normalise(looped)).is_none());
    }
}
