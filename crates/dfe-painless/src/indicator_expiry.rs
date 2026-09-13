// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! When a threat indicator stops being worth matching, dated forward from when
//! it was last seen.
//!
//! `ti_cif3` gives each indicator type its own shelf life -- 45 days for an
//! address, 90 for a domain, 365 for a URL or a file hash -- and anything else
//! the lifetime the integration was configured with. The date it lands on
//! becomes `cif3.deleted_at`, and the label becomes `cif3.expiration_duration`:
//!
//! ```painless
//! ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_lasttime_at;
//! if (ctx.threat.indicator.last_seen != null) {
//!     _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);
//! } else {
//!     _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);
//! }
//! if (ctx.threat.indicator.type == 'ipv4-addr' || ctx.threat.indicator.type == 'ipv6-addr') {
//!     _tmp_deleted_at = _tmp_lasttime_at.plusDays(45L);
//!     ctx.cif3.expiration_duration = "45d";
//! } else if ( ... ) { ... } else {
//!     def dur = ctx._conf.ioc_expiration_duration;
//!     ctx.cif3.expiration_duration = ctx._conf.ioc_expiration_duration;
//!     if (dur instanceof String){
//!         String time_unit = dur.substring(dur.length() - 1, dur.length());
//!         String time_value = dur.substring(0, dur.length() - 1);
//!         if (time_unit == 'd') { _tmp_deleted_at = _tmp_lasttime_at.plusDays(Long.parseLong(time_value)); }
//!         ...
//!         else { _tmp_deleted_at = _tmp_lasttime_at.plusDays(90L); ctx.error.message.add('...'); }
//!     }
//! }
//! ctx.cif3.deleted_at = _tmp_deleted_at;
//! ```
//!
//! The equality ladder alone reads the LABELS and nothing else, so
//! `cif3.expiration_duration` was right and `cif3.deleted_at` was absent on
//! every event -- the whole source finished on that one field. A `date`
//! processor normalises what this writes, so the string only has to be
//! ISO-8601; it does not have to be the one Java's `toString` would print.

use chrono::{DateTime, SecondsFormat, TimeDelta};
use serde_json::Value;

use crate::params::{balanced, clean_path, literal_writes, skip_trivia};
use dfe_core::event::Event;

/// The calendar unit a `plus...` call adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Days,
    Hours,
    Minutes,
}

impl Unit {
    /// The call spelling, and the unit it names.
    const CALLS: [(&'static str, Self); 3] = [
        (".plusDays(", Self::Days),
        (".plusHours(", Self::Hours),
        (".plusMinutes(", Self::Minutes),
    ];

    /// `n` of this unit as a span.
    ///
    /// The timestamps carry a fixed offset rather than a named zone, so a day
    /// is 24 hours here -- which is what Java's own `plusDays` works out to in
    /// a zone that never shifts.
    fn span(self, n: i64) -> Option<TimeDelta> {
        match self {
            Self::Days => TimeDelta::try_days(n),
            Self::Hours => TimeDelta::try_hours(n),
            Self::Minutes => TimeDelta::try_minutes(n),
        }
    }
}

/// One indicator type and the shelf life it earns.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpiryArm {
    /// The type values this arm covers.
    types: Vec<String>,
    unit: Unit,
    amount: i64,
    /// The label the arm writes alongside the date.
    label: String,
}

/// The configured fallback, for a type no arm names.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ConfiguredExpiry {
    /// The `ctx.` path holding a `<n><unit>` duration.
    source: String,
    /// The unit letters the script knows, each with what it means.
    units: Vec<(char, Unit)>,
    /// What an unknown unit falls back to.
    default_unit: Unit,
    default_amount: i64,
    /// The complaint appended when it does, and where.
    message: String,
    message_target: String,
}

/// The whole expiry calculation, read once per call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndicatorExpiry {
    /// The timestamps to date from, in the order the script tests them.
    seen: Vec<String>,
    /// The field naming the indicator's type.
    kind: String,
    arms: Vec<ExpiryArm>,
    configured: ConfiguredExpiry,
    /// Where the label is written.
    duration_target: String,
    /// Where the date is written.
    target: String,
}

/// The first quoted run in `text`.
fn quoted(text: &str) -> Option<&str> {
    let start = text.find(['\'', '"'])?;
    let quote = text.as_bytes()[start] as char;
    let end = text[start + 1..].find(quote)?;
    Some(&text[start + 1..=start + end])
}

/// The `if (<test>) { <block> }` at the head of `text`, and what follows it.
fn if_block(chunk: &str) -> Option<(&str, &str, &str)> {
    let at = chunk.find("if (")?;
    let (test, tail) = balanced(skip_trivia(&chunk[at + "if".len()..]), '(', ')')?;
    let (block, tail) = balanced(skip_trivia(tail), '{', '}')?;
    Some((test, block, tail))
}

/// The `plus...` call in `block`: which unit, and the LITERAL amount.
///
/// `None` where the amount is an expression -- the configured branch computes
/// it from the document, and reading `Long.parseLong(time_value)` as a number
/// would freeze one event's duration into the pattern.
fn literal_span(block: &str) -> Option<(Unit, i64)> {
    let (call, unit) = Unit::CALLS
        .iter()
        .find_map(|(call, unit)| block.contains(call).then_some((*call, *unit)))?;
    let (_, rest) = block.split_once(call)?;
    let amount = rest.split(')').next()?.trim().trim_end_matches(['L', 'l']);
    amount.parse::<i64>().ok().map(|amount| (unit, amount))
}

/// The unit a `plus...` call names, whatever its argument.
fn span_unit(block: &str) -> Option<Unit> {
    Unit::CALLS
        .iter()
        .find_map(|(call, unit)| block.contains(call).then_some(*unit))
}

/// `ctx.<path> == '<literal>'` terms OR'd together, all naming one path.
fn type_test(test: &str) -> Option<(String, Vec<String>)> {
    let mut path: Option<String> = None;
    let mut literals = Vec::new();
    for term in test.split("||") {
        let (lhs, rhs) = term.split_once("==")?;
        let named = clean_path(lhs.trim().strip_prefix("ctx.")?.trim());
        if named.is_empty() || named.contains(['(', ' ', ',']) {
            return None;
        }
        if path.get_or_insert_with(|| named.clone()) != &named {
            return None;
        }
        literals.push(quoted(rhs)?.to_string());
    }
    let path = path?;
    (!literals.is_empty()).then_some((path, literals))
}

/// Read the timestamps, the per-type arms and the configured fallback, or
/// decline.
///
/// Every part is structural and every part is demanded: the dates come from the
/// script's own `ZonedDateTime.parse` calls, each arm from the types it tests
/// and the span it adds, and the fallback from the unit letters the script
/// spells. A script that computes an expiry some other way declines rather than
/// binding to a runner that would date it wrong -- which is worse than leaving
/// the field absent, because a wrong expiry silently retires a live indicator.
pub fn parse_indicator_expiry(script: &str) -> Option<IndicatorExpiry> {
    let mut seen = Vec::new();
    for chunk in script.split("ZonedDateTime.parse(ctx.").skip(1) {
        let path = clean_path(chunk.split(')').next()?.trim());
        if path.is_empty() || path.contains(['(', ' ', ',']) {
            return None;
        }
        seen.push(path);
    }
    if seen.is_empty() {
        return None;
    }

    // `<expiry> = <base>.plusDays(...)` -- the two locals the script turns on.
    let (head, _) = script.split_once(".plus")?;
    let statement = head.rsplit([';', '\n', '{', '}']).next()?.trim();
    let (expiry_local, base_local) = statement.split_once('=')?;
    let expiry_local = expiry_local.trim();
    let base_local = base_local.trim();
    if expiry_local.is_empty() || base_local.is_empty() || expiry_local == base_local {
        return None;
    }

    // `ctx.<target> = <expiry>;`, which is the script's last statement.
    let assigned = format!("= {expiry_local};");
    let at = script.rfind(&assigned)?;
    let target = clean_path(
        script[..at]
            .trim_end()
            .rsplit([' ', '\n', '\t', '{', '}', ';'])
            .next()?
            .strip_prefix("ctx.")?,
    );
    if target.is_empty() {
        return None;
    }

    // The per-type arms. An `if` whose test is not a type comparison is not an
    // arm: the configured branch's own `instanceof` and unit tests fall through
    // here and are read below.
    let mut kind: Option<String> = None;
    let mut duration_target: Option<String> = None;
    let mut arms = Vec::new();
    let mut rest = script;
    while let Some((test, block, tail)) = if_block(rest) {
        rest = tail;
        let Some((path, types)) = type_test(test) else {
            continue;
        };
        let Some((unit, amount)) = literal_span(block) else {
            continue;
        };
        let writes = literal_writes(block);
        let [(written, Value::String(label))] = writes.as_slice() else {
            return None;
        };
        if kind.get_or_insert_with(|| path.clone()) != &path
            || duration_target.get_or_insert_with(|| written.clone()) != written
        {
            return None;
        }
        arms.push(ExpiryArm {
            types,
            unit,
            amount,
            label: label.clone(),
        });
    }
    if arms.is_empty() {
        return None;
    }

    Some(IndicatorExpiry {
        seen,
        kind: kind?,
        arms,
        configured: parse_configured(script, base_local)?,
        duration_target: duration_target?,
        target,
    })
}

/// The `else` branch: the configured duration, the units it accepts, and what
/// an unknown unit falls back to.
fn parse_configured(script: &str, base_local: &str) -> Option<ConfiguredExpiry> {
    // `def <dur> = ctx.<source>; ... if (<dur> instanceof String)`
    let (before, branch) = script.split_once(" instanceof String")?;
    let local = before.trim_end().rsplit([' ', '\n', '\t', '(']).next()?;
    let source = crate::common::ctx_path_bound_to(script, local)?;

    // Each `if (<unit local> == '<letter>') { ... .plus<Unit>(Long.parseLong(...)) }`.
    let mut units = Vec::new();
    let mut fallback = None;
    let mut rest = branch;
    while let Some((test, block, tail)) = if_block(rest) {
        rest = tail;
        if !block.contains(&format!("{base_local}.plus")) {
            continue;
        }
        let Some(letter) = quoted(test).and_then(|held| {
            let mut chars = held.chars();
            chars.next().filter(|_| chars.next().is_none())
        }) else {
            continue;
        };
        let unit = span_unit(block)?;
        units.push((letter, unit));
        // The arm after the last unit is the `else`, which the walk reaches as
        // the same block's tail -- its span is a literal where the units' are
        // parsed from the document.
        if let Some((unit, amount)) = literal_span(tail) {
            fallback = Some((unit, amount, tail));
        }
    }
    if units.is_empty() {
        return None;
    }
    let (default_unit, default_amount, unknown) = fallback?;

    // `ctx.<path>.add('<message>')` -- the complaint that goes with the default.
    let (before, after) = unknown.split_once(".add(")?;
    let message = quoted(after)?.to_string();
    let message_target = clean_path(
        before
            .trim_end()
            .rsplit([' ', '\n', '\t', '{', '}', ';'])
            .next()?
            .strip_prefix("ctx.")?,
    );
    if message.is_empty() || message_target.is_empty() {
        return None;
    }

    Some(ConfiguredExpiry {
        source,
        units,
        default_unit,
        default_amount,
        message,
        message_target,
    })
}

/// Date the indicator forward and write both fields.
///
/// A timestamp neither candidate carries, or one that will not parse, writes
/// nothing: Painless throws there and the call site's own `on_failure` is what
/// answers, so inventing a date would be worse than the absence.
pub fn indicator_expiry(event: &mut Event, pattern: &IndicatorExpiry) -> bool {
    let Some(base) = pattern
        .seen
        .iter()
        .find_map(|path| event.get_str(path))
        .and_then(|text| DateTime::parse_from_rfc3339(text).ok())
    else {
        return true;
    };

    let kind = event.get_string(&pattern.kind).unwrap_or_default();
    let matched = pattern
        .arms
        .iter()
        .find(|arm| arm.types.iter().any(|held| held == &kind));

    let (unit, amount) = if let Some(arm) = matched {
        let _ = event.set(&pattern.duration_target, Value::String(arm.label.clone()));
        (arm.unit, arm.amount)
    } else {
        // The label is the configured value itself, written through whatever
        // type it arrives as, and only read as a duration when it is text.
        let Some(held) = event.get(&pattern.configured.source).cloned() else {
            return true;
        };
        let _ = event.set(&pattern.duration_target, held.clone());
        let Value::String(text) = held else {
            return true;
        };
        let Some(span) = configured_span(event, &pattern.configured, &text) else {
            return true;
        };
        span
    };

    let Some(span) = unit.span(amount) else {
        return true;
    };
    let Some(expiry) = base.checked_add_signed(span) else {
        return true;
    };
    let _ = event.set(
        &pattern.target,
        Value::String(expiry.to_rfc3339_opts(SecondsFormat::Millis, true)),
    );
    true
}

/// The span a `<n><unit>` duration names, complaining where the unit is not one
/// the script knows.
fn configured_span(
    event: &mut Event,
    configured: &ConfiguredExpiry,
    text: &str,
) -> Option<(Unit, i64)> {
    let letter = text.chars().next_back()?;
    let Some(unit) = configured
        .units
        .iter()
        .find_map(|(spelling, unit)| (*spelling == letter).then_some(*unit))
    else {
        // The unknown-unit branch ignores the number entirely and says so.
        let _ = event.append(
            &configured.message_target,
            Value::String(configured.message.clone()),
        );
        return Some((configured.default_unit, configured.default_amount));
    };
    // A number Java's `parseLong` would throw on kills the script before it
    // writes the date, so nothing is written here either.
    let amount = text[..text.len() - letter.len_utf8()].parse::<i64>().ok()?;
    Some((unit, amount))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// `ti_cif3`'s script as the generated call site carries it: one line with
    /// its newlines escaped, which is what dispatch resolves before parsing.
    const RAW: &str = r#"ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_lasttime_at; if (ctx.threat.indicator.last_seen != null) {\n    _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n    _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n} if (ctx.threat.indicator.type == 'ipv4-addr' || ctx.threat.indicator.type == 'ipv6-addr') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(45L);\n    ctx.cif3.expiration_duration = \"45d\";\n} else if (ctx.threat.indicator.type == 'domain-name') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(90L);\n    ctx.cif3.expiration_duration = \"90d\";\n} else if (ctx.threat.indicator.type == 'url' || ctx.threat.indicator.type == 'file') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(365L);\n    ctx.cif3.expiration_duration = \"365d\";\n} else {\n    def dur = ctx._conf.ioc_expiration_duration;\n    ctx.cif3.expiration_duration = ctx._conf.ioc_expiration_duration;\n    if (dur instanceof String){\n        String time_unit = dur.substring(dur.length() -  1, dur.length());\n        String time_value = dur.substring(0, dur.length() - 1);\n        if (time_unit == 'd') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusDays(Long.parseLong(time_value));\n        } else if (time_unit == 'h') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusHours(Long.parseLong(time_value));\n        } else if (time_unit == 'm') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusMinutes(Long.parseLong(time_value));\n        } else {\n            _tmp_deleted_at = _tmp_lasttime_at.plusDays(90L);\n            if (ctx.error == null) {\n            ctx.error = new HashMap();\n            }\n            if (ctx.error.message == null) {\n            ctx.error.message = new ArrayList();\n            }\n            ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n        }\n    }\n} ctx.cif3.deleted_at = _tmp_deleted_at;\n"#;

    fn script() -> String {
        crate::common::normalise(RAW).into_owned()
    }

    fn pattern() -> IndicatorExpiry {
        parse_indicator_expiry(&script()).unwrap()
    }

    fn indicator(kind: &str) -> Value {
        json!({
            "_conf": { "ioc_expiration_duration": "5d" },
            "cif3": {},
            "threat": { "indicator": {
                "last_seen": "2022-07-19T08:35:05.971Z",
                "first_seen": "2022-07-19T07:40:41.000Z",
                "type": kind,
            } }
        })
    }

    #[test]
    fn the_script_names_both_timestamps_every_arm_and_the_fallback() {
        let pattern = pattern();
        assert_eq!(
            pattern.seen,
            ["threat.indicator.last_seen", "threat.indicator.first_seen"]
        );
        assert_eq!(pattern.kind, "threat.indicator.type");
        assert_eq!(pattern.target, "cif3.deleted_at");
        assert_eq!(pattern.duration_target, "cif3.expiration_duration");
        assert_eq!(pattern.arms.len(), 3);
        assert_eq!(pattern.arms[0].types, ["ipv4-addr", "ipv6-addr"]);
        assert_eq!(pattern.arms[0].amount, 45);
        assert_eq!(pattern.arms[0].unit, Unit::Days);
        assert_eq!(pattern.arms[0].label, "45d");
        assert_eq!(pattern.configured.source, "_conf.ioc_expiration_duration");
        assert_eq!(
            pattern.configured.units,
            [('d', Unit::Days), ('h', Unit::Hours), ('m', Unit::Minutes)]
        );
        assert_eq!(pattern.configured.default_amount, 90);
        assert_eq!(pattern.configured.message_target, "error.message");
    }

    /// The capture's own event: an address, last seen on 19 July, expiring 45
    /// days later.
    #[test]
    fn an_address_expires_forty_five_days_after_it_was_last_seen() {
        let pattern = pattern();
        let mut event = Event::new(indicator("ipv4-addr"));
        assert!(indicator_expiry(&mut event, &pattern));
        assert_eq!(
            event.get("cif3.deleted_at"),
            Some(&json!("2022-09-02T08:35:05.971Z"))
        );
        assert_eq!(event.get("cif3.expiration_duration"), Some(&json!("45d")));
    }

    /// No `last_seen` means the first sighting dates it instead.
    #[test]
    fn the_first_sighting_dates_it_when_the_last_is_absent() {
        let pattern = pattern();
        let mut input = indicator("domain-name");
        input["threat"]["indicator"]
            .as_object_mut()
            .unwrap()
            .remove("last_seen");
        let mut event = Event::new(input);
        assert!(indicator_expiry(&mut event, &pattern));
        assert_eq!(
            event.get("cif3.deleted_at"),
            Some(&json!("2022-10-17T07:40:41.000Z"))
        );
        assert_eq!(event.get("cif3.expiration_duration"), Some(&json!("90d")));
    }

    /// A type no arm names takes the configured duration, and its label is the
    /// configured value rather than one of the script's own.
    #[test]
    fn an_unnamed_type_takes_the_configured_duration() {
        let pattern = pattern();
        let mut event = Event::new(indicator("email-addr"));
        assert!(indicator_expiry(&mut event, &pattern));
        assert_eq!(
            event.get("cif3.deleted_at"),
            Some(&json!("2022-07-24T08:35:05.971Z"))
        );
        assert_eq!(event.get("cif3.expiration_duration"), Some(&json!("5d")));
    }

    /// Hours and minutes are units too.
    #[test]
    fn the_configured_unit_can_be_hours_or_minutes() {
        let pattern = pattern();
        for (configured, expected) in [
            ("6h", "2022-07-19T14:35:05.971Z"),
            ("90m", "2022-07-19T10:05:05.971Z"),
        ] {
            let mut input = indicator("email-addr");
            input["_conf"]["ioc_expiration_duration"] = json!(configured);
            let mut event = Event::new(input);
            assert!(indicator_expiry(&mut event, &pattern));
            assert_eq!(event.get("cif3.deleted_at"), Some(&json!(expected)));
        }
    }

    /// An unknown unit takes the default and says so, which is the only path
    /// that writes to `error.message`.
    #[test]
    fn an_unknown_unit_falls_back_ninety_days_and_complains() {
        let pattern = pattern();
        let mut input = indicator("email-addr");
        input["_conf"]["ioc_expiration_duration"] = json!("12w");
        let mut event = Event::new(input);
        assert!(indicator_expiry(&mut event, &pattern));
        assert_eq!(
            event.get("cif3.deleted_at"),
            Some(&json!("2022-10-17T08:35:05.971Z"))
        );
        // A list, because the script builds an `ArrayList` before it adds.
        assert_eq!(
            event.get("error.message"),
            Some(&json!([
                "invalid ioc_expiration_duration: using default 90 days"
            ]))
        );
    }

    /// Neither timestamp present writes nothing at all -- Painless throws there
    /// and the call site's `on_failure` is what answers.
    #[test]
    fn no_timestamp_writes_nothing() {
        let pattern = pattern();
        let mut event = Event::new(json!({"threat": {"indicator": {"type": "ipv4-addr"}}}));
        assert!(indicator_expiry(&mut event, &pattern));
        assert_eq!(event.get("cif3.deleted_at"), None);
        assert_eq!(event.get("cif3.expiration_duration"), None);
    }

    /// Every part is demanded: a script that dates some other way, names no
    /// types, or never writes the result declines rather than binding.
    #[test]
    fn a_script_missing_a_part_declines() {
        let script = script();
        for missing in [
            script.replace("ZonedDateTime.parse(ctx.", "LocalDate.parse(ctx."),
            script.replace(
                "ctx.threat.indicator.type == ",
                "ctx.threat.indicator.type != ",
            ),
            script.replace(" ctx.cif3.deleted_at = _tmp_deleted_at;", ""),
            script.replace("dur instanceof String", "dur instanceof Map"),
            script.replace(".add('invalid ioc_expiration_duration", ".push('invalid"),
        ] {
            assert!(
                parse_indicator_expiry(&missing).is_none(),
                "claimed a script it cannot serve: {missing}"
            );
        }
    }
}
