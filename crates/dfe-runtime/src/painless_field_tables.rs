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

use crate::Event;
use crate::painless_params::{balanced, clean_path, skip_trivia};

/// One field looked up in one named `params` table.
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// The list-row form, read whole:
///
/// ```painless
/// String s = ctx.<subject>;
/// if (Character.isDigit(s.charAt(0))) {
///   def mapping = params['<table>'][s];
///   if (mapping != null && mapping.size() > 0) {
///     ctx.<first> = mapping[0];
///     if (mapping.size() > 1) { ctx.<second> = mapping[1]; }
///   }
/// } else {
///   ctx.<first> = s;
/// }
/// ```
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
fn ctx_path(text: &str) -> Option<String> {
    let path = clean_path(text);
    let path = path.strip_prefix("ctx.")?;
    (!path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'))
    .then(|| path.to_string())
}

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// gigamon's lookup script, verbatim from `pipelines/gigamon/ami/default.yml`.
    fn script() -> &'static str {
        r"// end_reason
if (ctx.gigamon.ami.end_reason != null) {
    ctx.gigamon.ami.end_reason_value = params['end_reason'][ctx.gigamon.ami.end_reason];
}
// http_uri_path
if (ctx.gigamon.ami.http_uri_path != null) {
    ctx.gigamon.ami.http_uri_path_value = params['http_uri_path'][ctx.gigamon.ami.http_uri_path];
}
// smb_version
if (ctx.gigamon.ami.smb_version != null) {
    ctx.gigamon.ami.smb_version_value = params['smb_version'][ctx.gigamon.ami.smb_version];
}
// ssl_cipher_suite_id
if (ctx.gigamon.ami.ssl_cipher_suite_id != null) {
  String s = ctx.gigamon.ami.ssl_cipher_suite_id;
  if (Character.isDigit(s.charAt(0))) {
    def mapping = params['ssl_cipher_suite_id'][s];
    if (mapping != null && mapping.size() > 0) {
      ctx.gigamon.ami.ssl_cipher_suite_id_value = mapping[0];
      if (mapping.size() > 1) {
        ctx.gigamon.ami.ssl_cipher_suite_id_protocol = mapping[1];
        }
      }
    }
    else {
        ctx.gigamon.ami.ssl_cipher_suite_id_value = s;
    }
}
// ssl_protocol_version
if (ctx.gigamon.ami.ssl_protocol_version != null) {
    ctx.gigamon.ami.ssl_protocol_version_value = params['ssl_protocol_version'][ctx.gigamon.ami.ssl_protocol_version];
}
// ssl_ext_sig_algorithm_hash
if (ctx.gigamon.ami.ssl_ext_sig_algorithm_hash != null) {
    ctx.gigamon.ami.ssl_ext_sig_algorithm_hash_value = params['ssl_ext_sig_algorithm_hash'][ctx.gigamon.ami.ssl_ext_sig_algorithm_hash];
}
// ssl_ext_sig_algorithm_scheme
if (ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme != null) {
    ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme_value = params['ssl_ext_sig_algorithm_scheme'][ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme];
}
// dns_query_type
if (ctx.gigamon.ami.dns_query_type != null) {
    ctx.gigamon.ami.dns_query_type_value = params['dns_query_type'][ctx.gigamon.ami.dns_query_type];
}
// dns_reply_code
if (ctx.gigamon.ami.dns_reply_code != null) {
    ctx.gigamon.ami.dns_reply_code_value = params['dns_reply_code'][ctx.gigamon.ami.dns_reply_code];
}"
    }

    /// The rows the tests exercise, verbatim from the same processor's `params`.
    fn params() -> Map<String, Value> {
        json!({
            "end_reason": { "1": "Idle Timeout", "2": "Active Timeout", "3": "End of Flow", "0": "None" },
            "http_uri_path": { "*v1*": "V1" },
            "smb_version": { "1": "SMB-V1", "2": "SMB-V2" },
            "ssl_cipher_suite_id": {
                "47": ["TLS_RSA_WITH_AES_128_CBC_SHA", "AES128-SHA"],
                "4866": ["TLS_AES_256_GCM_SHA384"]
            },
            "ssl_protocol_version": { "771": "TLS_1_2" },
            "ssl_ext_sig_algorithm_hash": { "4": "SHA256" },
            "ssl_ext_sig_algorithm_scheme": { "1027": "ecdsa_secp256r1_sha256" },
            "dns_query_type": { "12": "PTR", "255": "*" },
            "dns_reply_code": { "0": "No Error", "3": "Non-Existent Domain" }
        })
        .as_object()
        .expect("the params block is an object")
        .clone()
    }

    /// All nine lookups are read, in script order, with the table names and
    /// the targets taken OFF the script rather than assumed.
    #[test]
    fn every_lookup_is_read_off_the_script() {
        let pattern = parse_field_tables(script()).expect("the lookups are recognised");
        assert_eq!(pattern.lookups.len(), 9);
        assert_eq!(
            pattern.lookups[0],
            FieldLookup::Row {
                source: "gigamon.ami.end_reason".into(),
                table: "end_reason".into(),
                target: "gigamon.ami.end_reason_value".into(),
            }
        );
        assert_eq!(
            pattern.lookups[3],
            FieldLookup::ListRow {
                source: "gigamon.ami.ssl_cipher_suite_id".into(),
                table: "ssl_cipher_suite_id".into(),
                first: "gigamon.ami.ssl_cipher_suite_id_value".into(),
                second: "gigamon.ami.ssl_cipher_suite_id_protocol".into(),
            }
        );
        assert_eq!(
            pattern.lookups[8],
            FieldLookup::Row {
                source: "gigamon.ami.dns_reply_code".into(),
                table: "dns_reply_code".into(),
                target: "gigamon.ami.dns_reply_code_value".into(),
            }
        );
    }

    /// The ordinary lookups, against the captured values.
    #[test]
    fn a_row_is_written_whole() {
        let pattern = parse_field_tables(script()).expect("recognised");
        let mut event = Event::new(json!({
            "gigamon": { "ami": {
                "end_reason": "1",
                "dns_query_type": "255",
                "dns_reply_code": "0",
                "ssl_protocol_version": "771"
            } }
        }));
        assert!(field_tables(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("gigamon.ami.end_reason_value"),
            Some(&json!("Idle Timeout"))
        );
        assert_eq!(
            event.get("gigamon.ami.dns_query_type_value"),
            Some(&json!("*"))
        );
        assert_eq!(
            event.get("gigamon.ami.dns_reply_code_value"),
            Some(&json!("No Error"))
        );
        assert_eq!(
            event.get("gigamon.ami.ssl_protocol_version_value"),
            Some(&json!("TLS_1_2"))
        );
    }

    /// A key the table has no row for writes NOTHING. `http_uri_path` is the
    /// captured case: `/_bulk` against a table holding only `*v1*`, and the
    /// document comes back with no `http_uri_path_value` key at all.
    #[test]
    fn a_key_with_no_row_writes_nothing() {
        let pattern = parse_field_tables(script()).expect("recognised");
        let mut event = Event::new(json!({
            "gigamon": { "ami": { "http_uri_path": "/_bulk" } }
        }));
        assert!(field_tables(&mut event, &pattern, &params()));
        assert!(!event.has("gigamon.ami.http_uri_path_value"));
    }

    /// A row of one member writes the value and no protocol -- the captured
    /// `4866` case.
    #[test]
    fn a_one_member_row_writes_only_the_value() {
        let pattern = parse_field_tables(script()).expect("recognised");
        let mut event = Event::new(json!({
            "gigamon": { "ami": { "ssl_cipher_suite_id": "4866" } }
        }));
        assert!(field_tables(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("gigamon.ami.ssl_cipher_suite_id_value"),
            Some(&json!("TLS_AES_256_GCM_SHA384"))
        );
        assert!(!event.has("gigamon.ami.ssl_cipher_suite_id_protocol"));
    }

    /// A row of two members writes both.
    #[test]
    fn a_two_member_row_writes_the_protocol_as_well() {
        let pattern = parse_field_tables(script()).expect("recognised");
        let mut event = Event::new(json!({
            "gigamon": { "ami": { "ssl_cipher_suite_id": "47" } }
        }));
        assert!(field_tables(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("gigamon.ami.ssl_cipher_suite_id_value"),
            Some(&json!("TLS_RSA_WITH_AES_128_CBC_SHA"))
        );
        assert_eq!(
            event.get("gigamon.ami.ssl_cipher_suite_id_protocol"),
            Some(&json!("AES128-SHA"))
        );
    }

    /// A key that does NOT open with a digit is copied through as it stands.
    /// The captured event carries the suite's own name in the id field, and
    /// Elasticsearch echoes it into the value.
    #[test]
    fn a_non_digit_key_is_copied_through() {
        let pattern = parse_field_tables(script()).expect("recognised");
        let mut event = Event::new(json!({
            "gigamon": { "ami": {
                "ssl_cipher_suite_id": "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384"
            } }
        }));
        assert!(field_tables(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("gigamon.ami.ssl_cipher_suite_id_value"),
            Some(&json!("TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384"))
        );
        assert!(!event.has("gigamon.ami.ssl_cipher_suite_id_protocol"));
    }

    /// A digit-led key with no row writes nothing, where a non-digit key would
    /// have been copied. The `mapping != null` guard is the difference.
    #[test]
    fn a_digit_key_with_no_row_is_not_copied_through() {
        let pattern = parse_field_tables(script()).expect("recognised");
        let mut event = Event::new(json!({
            "gigamon": { "ami": { "ssl_cipher_suite_id": "9999" } }
        }));
        assert!(field_tables(&mut event, &pattern, &params()));
        assert!(!event.has("gigamon.ami.ssl_cipher_suite_id_value"));
    }

    /// An absent field is not looked up, which is the script's own `!= null`.
    #[test]
    fn an_absent_field_writes_nothing() {
        let pattern = parse_field_tables(script()).expect("recognised");
        let mut event = Event::new(json!({ "gigamon": { "ami": { "app_id": "67" } } }));
        assert!(field_tables(&mut event, &pattern, &params()));
        assert_eq!(
            event.get("gigamon.ami"),
            Some(&json!({ "app_id": "67" })),
            "no lookup fired, so nothing may be added"
        );
    }

    /// The whole script is declined when ONE statement cannot be read. Writing
    /// the eight it understands and skipping the ninth would leave the source
    /// looking like it needs polish rather than a different matcher.
    #[test]
    fn one_unreadable_statement_declines_the_whole_script() {
        let broken = script().replace(
            "ctx.gigamon.ami.smb_version_value = params['smb_version']\
             [ctx.gigamon.ami.smb_version];",
            "ctx.gigamon.ami.smb_version_value = describe(ctx.gigamon.ami.smb_version);",
        );
        assert_ne!(broken, script(), "the replacement has to bite");
        assert!(parse_field_tables(&broken).is_none());
    }

    /// A guard that does not cover the field the lookup keys on is a different
    /// pattern, and is declined.
    #[test]
    fn a_guard_over_another_field_declines() {
        assert!(
            parse_field_tables(
                "if (ctx.a.b != null) {\n  ctx.a.b_value = params['t'][ctx.a.c];\n}"
            )
            .is_none()
        );
    }

    /// The `params.` and bare `params[ctx.` spellings belong to the matchers
    /// above this one in the ladder, and are not taken here.
    #[test]
    fn the_other_table_spellings_are_left_alone() {
        assert!(parse_field_tables("ctx.a.b = params.get(ctx.a.c);").is_none());
        assert!(parse_field_tables("ctx.a.b = params[ctx.a.c];").is_none());
    }

    /// The dispatch reaches this matcher: nothing above it in the params
    /// ladder claims the script.
    #[test]
    fn the_ladder_dispatches_the_script_here() {
        let pattern = crate::painless_params::params_pattern(script());
        assert!(
            matches!(
                pattern,
                Some(crate::painless_params::ParamsPattern::FieldTables(_))
            ),
            "the ladder bound {pattern:?} instead"
        );
    }
}
