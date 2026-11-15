// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A run of null-guarded lookups, each field through its OWN named table.
//!
//! Every other params matcher reads a table `params` either IS or names once --
//! `params.get(ctx.x)`, `params.<table>[key]`, `params[ctx.x]`. gigamon's
//! lookup script does neither: it subscripts the table by a quoted literal and
//! the key after it, `params['end_reason'][ctx.gigamon.ami.end_reason]`, nine
//! times over nine different fields, so no trigger in the ladder above sees it
//! and the whole script is unclaimed.
//!
//! Measured over the corpus it is worth 134 of gigamon's 163 wrong fields and
//! 27 of its 56 unmatched events: `end_reason_value` on 46, `dns_query_type_value`
//! on 29 with `dns.question.type` copied from it, `dns_reply_code_value` and
//! `dns.response_code` on 7 each, and the `ssl_*` and `smb_version` singles.
//! The 29 it does NOT reach are `dns.question.subdomain`, which a different
//! script writes.
//!
//! Two forms, and a script carrying one this cannot read is declined WHOLE.
//! Eight of the nine written and the ninth skipped is the worst outcome
//! available here -- the source then reads as needing polish rather than a
//! different matcher.
//!
//! The second form is `ssl_cipher_suite_id`, whose row is a LIST: a key
//! opening with a digit is looked up and the row's first member written, its
//! second where the row has one, and a key that is NOT a digit is copied
//! through as it stands. The indentation of the vendor's `else` suggests it
//! belongs to the row's null test; the BRACES put it on the digit test, which
//! is what the captured output shows -- the suite NAME
//! `TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384` arrives as the id and comes back out
//! unchanged.
//!
//! **A key with no row writes NOTHING.** Painless assigns the miss as null and
//! the pipeline's closing `painless_remove_null` prunes it, so the captured
//! document carries no key at all: `http_uri_path` is `/_bulk` on one event and
//! `\/BurstingPipe\/adServer.bs` on another, the table holds only `*v1*`, and
//! neither document has an `http_uri_path_value`.

use serde_json::{Map, Value};

use crate::params::{balanced, ctx_path_plain as ctx_path, skip_trivia};
use dfe_core::Event;

/// One field looked up in one named `params` table.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FieldLookup {
    /// The row assigned whole, under the script's own `!= null` guard.
    Row {
        /// The `ctx.` path read as the key.
        source: String,
        /// The `params` member holding the table.
        table: String,
        /// The `ctx.` path the row lands at.
        target: String,
    },
    /// The row assigned whole, keyed by the STRING FORM of a local the block
    /// binds first, under a `containsKey` guard.
    ///
    /// [`FieldLookup::Row`] keys on the field itself, so a NUMBER misses every
    /// row however the table is spelled -- which is correct for gigamon, whose
    /// script subscripts the table with the raw field. This one keys on
    /// `<local>.toString()`, so a number stringifies and HITS, which is what
    /// the `trend_micro` tables want: each is keyed `'0'`..`'8225'` against a field
    /// the pipeline's own `convert` processor has just made a long.
    StringKeyedRow {
        /// The `ctx.` path the local is bound to and the key read from.
        source: String,
        /// The `params` MEMBER holding the table, spelled `params.<table>`.
        table: String,
        /// The `ctx.` path the row lands at.
        target: String,
    },
    /// The row is a LIST, and only a key opening with a digit is looked up at
    /// all.
    ListRow {
        /// The `ctx.` path read as the key.
        source: String,
        /// The `params` member holding the table.
        table: String,
        /// Where the row's first member lands -- and where a non-digit key is
        /// copied to as it stands.
        first: String,
        /// Where the row's second member lands, when it has one.
        second: String,
    },
}

/// Every lookup one script makes, in the order it writes them.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FieldTables {
    /// In script order.
    lookups: Vec<FieldLookup>,
}

impl FieldTables {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(lookups: Vec<FieldLookup>) -> Self {
        Self { lookups }
    }
}

/// Write each lookup's row, or nothing where the table has no row for the key.
///
/// A key that is present and not a STRING is a miss: the tables are keyed by
/// string, and Painless's `map[key]` is `Map.get`, which an `Integer` key
/// misses however the row is spelled.
pub fn field_tables(event: &mut Event, pattern: &FieldTables, params: &Map<String, Value>) -> bool {
    for lookup in &pattern.lookups {
        match lookup {
            FieldLookup::Row {
                source,
                table,
                target,
            } => {
                if let Some(row) = row_for(event, params, table, source) {
                    let _ = event.set(target, row);
                }
            }
            FieldLookup::StringKeyedRow {
                source,
                table,
                target,
            } => {
                if let Some(row) = string_row_for(event, params, table, source) {
                    let _ = event.set(target, row);
                }
            }
            FieldLookup::ListRow {
                source,
                table,
                first,
                second,
            } => run_list_row(event, params, table, source, first, second),
        }
    }
    true
}

/// The row a field's value selects, or `None` where anything on the way misses.
fn row_for(event: &Event, params: &Map<String, Value>, table: &str, source: &str) -> Option<Value> {
    let key = event.get_str(source)?;
    params.get(table)?.as_object()?.get(key).cloned()
}

/// The row a field's STRING FORM selects, or `None` where anything misses.
///
/// [`row_for`] reads the key with `get_str`, which a number misses. This one
/// stringifies first, because the script does: `params.<table>[k.toString()]`.
/// A container has no string form and is a miss here too, which is where
/// Painless would have thrown.
fn string_row_for(
    event: &Event,
    params: &Map<String, Value>,
    table: &str,
    source: &str,
) -> Option<Value> {
    let key = event.get_as_string(source)?;
    params.get(table)?.as_object()?.get(&key).cloned()
}

/// The list-row form: look a digit-led key up, copy anything else through.
fn run_list_row(
    event: &mut Event,
    params: &Map<String, Value>,
    table: &str,
    source: &str,
    first: &str,
    second: &str,
) {
    let Some(key) = event.get_string(source) else {
        return;
    };
    // `Character.isDigit(s.charAt(0))` in the script. An empty string throws
    // there and the processor fails whole, which writing nothing is the
    // closest reachable behaviour to.
    let Some(opening) = key.chars().next() else {
        return;
    };
    if !opening.is_ascii_digit() {
        let _ = event.set(first, Value::String(key));
        return;
    }
    // `mapping != null && mapping.size() > 0`. A row that is not a list has no
    // `size()` and throws; writing nothing is again the closest reachable
    // behaviour, and no vendored row is spelled that way.
    let Some(members) = params
        .get(table)
        .and_then(Value::as_object)
        .and_then(|rows| rows.get(&key))
        .and_then(Value::as_array)
    else {
        return;
    };
    let Some(head) = members.first().cloned() else {
        return;
    };
    let _ = event.set(first, head);
    if let Some(tail) = members.get(1).cloned() {
        let _ = event.set(second, tail);
    }
}

/// Read a run of guarded table lookups off the script, or decline.
///
/// Every top-level statement has to be one of the two forms, and the guard has
/// to cover the very field the lookup keys on. Anything else declines the whole
/// script rather than the one statement.
#[must_use]
pub fn parse_field_tables(script: &str) -> Option<FieldTables> {
    // Every lookup subscripts the table by a quoted literal. The `params.`
    // and `params[ctx.` spellings belong to the matchers above this one.
    if !script.contains("params['") && !script.contains("params[\"") {
        return None;
    }

    let mut lookups = Vec::new();
    let mut rest = skip_trivia(script);
    while !rest.is_empty() {
        let after = skip_trivia(rest.strip_prefix("if")?);
        let (test, after) = balanced(after, '(', ')')?;
        let subject = guarded_subject(test)?;
        let (body, after) = balanced(skip_trivia(after), '{', '}')?;
        lookups.push(parse_row(&subject, body).or_else(|| parse_list_row(&subject, body))?);
        rest = skip_trivia(after);
    }

    (!lookups.is_empty()).then(|| FieldTables::new(lookups))
}

/// The same run of lookups in the DOTTED spelling, keyed through a local.
///
/// Three differences from [`parse_field_tables`], and each is why that reader
/// cannot see this one: the table is a `params` MEMBER rather than a quoted
/// subscript, the key is a LOCAL the block binds first rather than the path
/// written inline, and the guard is `containsKey` rather than `!= null`.
///
/// The `<local> != null &&` conjunct is OPTIONAL, and its two spellings mean
/// the same thing here. With it, Painless skips an absent field; without it,
/// `null.toString()` throws and the processor fails whole -- and both vendor
/// sites that omit it are guarded by the processor's own
/// `if: ctx.<source> != null`, so the null never arrives. Writing nothing is
/// what either reaches.
///
/// Measured over the corpus this is worth 35 of `trend_micro_vision_one`'s 60
/// wrong fields across THREE streams: six `*_value` fields on every
/// `endpoint_activity` event, `act_value` on every `network_activity` event,
/// and `risk_level_value` on one detection. The last two are the same spelling
/// over a one-block script, which is why one reader covers all three.
///
/// A statement this cannot read declines the WHOLE script, for the reason
/// [`parse_field_tables`] gives: five of the six lookups written and the sixth
/// skipped makes the source read as needing polish.
#[must_use]
pub fn parse_named_table_lookups(script: &str) -> Option<FieldTables> {
    // The quoted-subscript spelling belongs to the matcher above, and a script
    // with no dotted `params.` member cannot be this one.
    if !script.contains("params.") || !script.contains(".containsKey(") {
        return None;
    }

    let mut lookups = Vec::new();
    let mut rest = skip_trivia(script);
    while !rest.is_empty() {
        let (statement, after) = rest.strip_prefix("def ")?.split_once(';')?;
        let (local, bound) = statement.split_once('=')?;
        let local = identifier(local)?;
        let source = ctx_path(bound)?;

        let after = skip_trivia(after).strip_prefix("if")?;
        let (test, after) = balanced(skip_trivia(after), '(', ')')?;
        let table = guard_table(test, local)?;

        let (body, after) = balanced(skip_trivia(after), '{', '}')?;
        let (target, expression) = only_assignment(body)?;
        // The write must read the SAME table through the SAME local, or the
        // guard is not this lookup's own.
        if without_spaces(expression) != format!("params.{table}[{local}.toString()]") {
            return None;
        }

        lookups.push(FieldLookup::StringKeyedRow {
            source,
            table,
            target,
        });
        rest = skip_trivia(after);
    }

    (!lookups.is_empty()).then(|| FieldTables::new(lookups))
}

/// The `params` member a `containsKey` guard tests, keyed on `local`.
///
/// Declines a guard over any OTHER local, so a script binding two and testing
/// one is not read as a lookup of the other.
fn guard_table(test: &str, local: &str) -> Option<String> {
    let squeezed = without_spaces(test);
    let rest = squeezed
        .strip_prefix(&format!("{local}!=null&&"))
        .unwrap_or(squeezed.as_str());
    let table = rest
        .strip_suffix(&format!(".containsKey({local}.toString())"))?
        .strip_prefix("params.")?;
    (!table.is_empty()
        && !table.starts_with(|c: char| c.is_ascii_digit())
        && table.chars().all(|c| c.is_alphanumeric() || c == '_'))
    .then(|| table.to_string())
}

/// The `ctx.` path an `if (<path> != null)` guard covers.
fn guarded_subject(test: &str) -> Option<String> {
    ctx_path(test.trim().strip_suffix("!= null")?)
}

/// `ctx.<target> = params['<table>'][ctx.<subject>];` and nothing else.
fn parse_row(subject: &str, body: &str) -> Option<FieldLookup> {
    let (target, expression) = only_assignment(body)?;
    let (table, key) = table_subscript(expression)?;
    (ctx_path(key)? == subject).then(|| FieldLookup::Row {
        source: subject.to_string(),
        table,
        target,
    })
}

/// The list-row form, read whole.
fn parse_list_row(subject: &str, body: &str) -> Option<FieldLookup> {
    let (statement, rest) = skip_trivia(body).strip_prefix("String")?.split_once(';')?;
    let (local, bound) = statement.split_once('=')?;
    let local = identifier(local)?;
    if ctx_path(bound)? != subject {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    if without_spaces(test) != format!("Character.isDigit({local}.charAt(0))") {
        return None;
    }
    let (found, rest) = balanced(skip_trivia(rest), '{', '}')?;
    let rest = skip_trivia(rest).strip_prefix("else")?;
    let (missing, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }

    let (table, first, second) = parse_list_row_body(local, found)?;
    // The else arm copies the key to the SAME field the row's first member
    // would have filled. A different target is a different pattern.
    let (target, copied) = only_assignment(missing)?;
    if target != first || copied.trim() != local {
        return None;
    }

    Some(FieldLookup::ListRow {
        source: subject.to_string(),
        table,
        first,
        second,
    })
}

/// The digit branch: the table, and the two members' targets.
fn parse_list_row_body(local: &str, block: &str) -> Option<(String, String, String)> {
    let (statement, rest) = skip_trivia(block).strip_prefix("def")?.split_once(';')?;
    let (row, expression) = statement.split_once('=')?;
    let row = identifier(row)?;
    let (table, key) = table_subscript(expression)?;
    if key.trim() != local {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    if without_spaces(test) != format!("{row}!=null&&{row}.size()>0") {
        return None;
    }
    let (found, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }

    let (statement, rest) = skip_trivia(found).split_once(';')?;
    let (first, member) = assignment(statement)?;
    if without_spaces(member) != format!("{row}[0]") {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    if without_spaces(test) != format!("{row}.size()>1") {
        return None;
    }
    let (block, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }
    let (second, member) = only_assignment(block)?;
    if without_spaces(member) != format!("{row}[1]") {
        return None;
    }

    Some((table, first, second))
}

/// The one `<lhs> = <expression>;` a block holds, and nothing else.
fn only_assignment(block: &str) -> Option<(String, &str)> {
    let (statement, tail) = skip_trivia(block).split_once(';')?;
    if !skip_trivia(tail).is_empty() {
        return None;
    }
    assignment(statement)
}

/// The `ctx.` path an assignment writes, and the expression it writes.
fn assignment(statement: &str) -> Option<(String, &str)> {
    let (lhs, rhs) = statement.split_once('=')?;
    Some((ctx_path(lhs)?, rhs))
}

/// `params[<literal>][<key>]` and nothing after it: the table's name, and the
/// key expression as the script writes it.
fn table_subscript(expression: &str) -> Option<(String, &str)> {
    let rest = skip_trivia(expression).strip_prefix("params")?;
    let (table, rest) = balanced(skip_trivia(rest), '[', ']')?;
    let (key, rest) = balanced(skip_trivia(rest), '[', ']')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }
    Some((quoted(table)?, key))
}

/// A bare `ctx.<path>`, with null-safe navigation stripped.
/// A single- or double-quoted literal's contents.
fn quoted(text: &str) -> Option<String> {
    let text = text.trim();
    for quote in ['\'', '"'] {
        if let Some(inner) = text
            .strip_prefix(quote)
            .and_then(|rest| rest.strip_suffix(quote))
            && !inner.is_empty()
        {
            return Some(inner.to_string());
        }
    }
    None
}

/// A local's name, or `None` where the text is not one identifier.
fn identifier(text: &str) -> Option<&str> {
    let name = text.trim();
    (!name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
    .then_some(name)
}

/// An expression with every space removed, so spacing is not part of a match.
fn without_spaces(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "field_tables_tests.rs"]
mod tests;
