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
/// ```painless
/// def <local> = ctx.<source>;
/// if (<local> != null && params.<table>.containsKey(<local>.toString())) {
///   ctx.<target> = params.<table>[<local>.toString()];
/// }
/// ```
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
        let pattern = crate::params::params_pattern(script());
        assert!(
            matches!(pattern, Some(crate::params::ParamsPattern::FieldTables(_))),
            "the ladder bound {pattern:?} instead"
        );
    }

    /// The `trend_micro` `endpoint_activity` lookups, verbatim from
    /// `pipelines/trend_micro_vision_one/endpoint_activity/default.yml:1018-1041`.
    fn endpoint_script() -> &'static str {
        "def eventId = ctx.trend_micro_vision_one.endpoint_activity.event?.id;\n\
         if (eventId != null && params.eventId.containsKey(eventId.toString())) {\n  \
         ctx.trend_micro_vision_one.endpoint_activity.event.id_value = \
         params.eventId[eventId.toString()];\n}\n\
         def objectTrueType = ctx.trend_micro_vision_one.endpoint_activity.object?.true_type;\n\
         if (objectTrueType != null && params.objectTrueType.containsKey(objectTrueType.toString())) {\n  \
         ctx.trend_micro_vision_one.endpoint_activity.object.true_type_value = \
         params.objectTrueType[objectTrueType.toString()];\n}\n\
         def winEventId = ctx.trend_micro_vision_one.endpoint_activity.win_event_id;\n\
         if (winEventId != null && params.winEventId.containsKey(winEventId.toString())) {\n  \
         ctx.trend_micro_vision_one.endpoint_activity.win_event_id_value = \
         params.winEventId[winEventId.toString()];\n}"
    }

    /// The `network_activity` one-block spelling, which omits the `!= null`
    /// conjunct. Verbatim from
    /// `pipelines/trend_micro_vision_one/network_activity/default.yml:369-372`.
    fn act_script() -> &'static str {
        "def act = ctx.trend_micro_vision_one.network_activity.act;\n\
         if (params.act.containsKey(act.toString())) {\n  \
         ctx.trend_micro_vision_one.network_activity.act_value = params.act[act.toString()];\n}"
    }

    /// The rows those two read, verbatim from the same processors' `params`.
    fn named_params() -> Map<String, Value> {
        json!({
            "eventId": { "1": "EVENT_PROCESS", "4": "EVENT_DNS" },
            "objectTrueType": { "1": "EXE", "127": "ELF" },
            "winEventId": { "4624": "An account was successfully logged on" },
            "act": { "1": "monitor", "2": "block", "4": "override" }
        })
        .as_object()
        .expect("the params block is an object")
        .clone()
    }

    /// Every block is read, with the table and the target taken OFF the script.
    #[test]
    fn every_named_table_lookup_is_read_off_the_script() {
        let pattern = parse_named_table_lookups(endpoint_script()).expect("recognised");
        assert_eq!(pattern.lookups.len(), 3);
        assert_eq!(
            pattern.lookups[0],
            FieldLookup::StringKeyedRow {
                source: "trend_micro_vision_one.endpoint_activity.event.id".into(),
                table: "eventId".into(),
                target: "trend_micro_vision_one.endpoint_activity.event.id_value".into(),
            }
        );
        assert_eq!(
            pattern.lookups[2],
            FieldLookup::StringKeyedRow {
                source: "trend_micro_vision_one.endpoint_activity.win_event_id".into(),
                table: "winEventId".into(),
                target: "trend_micro_vision_one.endpoint_activity.win_event_id_value".into(),
            }
        );
    }

    /// A NUMERIC field hits a string-keyed row, which is the whole point of the
    /// `.toString()` this spelling carries and the raw-key reader misses.
    #[test]
    fn a_numeric_key_hits_a_string_keyed_row() {
        let pattern = parse_named_table_lookups(endpoint_script()).expect("recognised");
        let mut event = Event::new(json!({
            "trend_micro_vision_one": { "endpoint_activity": {
                "event": { "id": 1 },
                "object": { "true_type": 127 },
                "win_event_id": 4624
            } }
        }));
        assert!(field_tables(&mut event, &pattern, &named_params()));
        assert_eq!(
            event.get("trend_micro_vision_one.endpoint_activity.event.id_value"),
            Some(&json!("EVENT_PROCESS"))
        );
        assert_eq!(
            event.get("trend_micro_vision_one.endpoint_activity.object.true_type_value"),
            Some(&json!("ELF"))
        );
        assert_eq!(
            event.get("trend_micro_vision_one.endpoint_activity.win_event_id_value"),
            Some(&json!("An account was successfully logged on"))
        );
    }

    /// A key the table has no row for writes nothing, which is the
    /// `containsKey` guard.
    #[test]
    fn a_named_table_miss_writes_nothing() {
        let pattern = parse_named_table_lookups(act_script()).expect("recognised");
        let mut event = Event::new(json!({
            "trend_micro_vision_one": { "network_activity": { "act": 9 } }
        }));
        assert!(field_tables(&mut event, &pattern, &named_params()));
        assert!(!event.has("trend_micro_vision_one.network_activity.act_value"));
    }

    /// The one-block spelling, without the `!= null` conjunct, reads the same.
    #[test]
    fn the_guard_without_the_null_conjunct_reads_the_same_lookup() {
        let pattern = parse_named_table_lookups(act_script()).expect("recognised");
        assert_eq!(
            pattern.lookups,
            vec![FieldLookup::StringKeyedRow {
                source: "trend_micro_vision_one.network_activity.act".into(),
                table: "act".into(),
                target: "trend_micro_vision_one.network_activity.act_value".into(),
            }]
        );
        let mut event = Event::new(json!({
            "trend_micro_vision_one": { "network_activity": { "act": 4 } }
        }));
        assert!(field_tables(&mut event, &pattern, &named_params()));
        assert_eq!(
            event.get("trend_micro_vision_one.network_activity.act_value"),
            Some(&json!("override"))
        );
    }

    /// An absent source writes nothing rather than keying on an empty string.
    #[test]
    fn an_absent_source_writes_nothing_through_the_named_table() {
        let pattern = parse_named_table_lookups(act_script()).expect("recognised");
        let mut event = Event::new(json!({
            "trend_micro_vision_one": { "network_activity": { "src_ip": "10.0.0.1" } }
        }));
        assert!(field_tables(&mut event, &pattern, &named_params()));
        assert_eq!(
            event.get("trend_micro_vision_one.network_activity"),
            Some(&json!({ "src_ip": "10.0.0.1" })),
            "no lookup fired, so nothing may be added"
        );
    }

    /// A container has no string form, so it misses rather than being written.
    #[test]
    fn a_container_key_writes_nothing() {
        let pattern = parse_named_table_lookups(act_script()).expect("recognised");
        let mut event = Event::new(json!({
            "trend_micro_vision_one": { "network_activity": { "act": ["1"] } }
        }));
        assert!(field_tables(&mut event, &pattern, &named_params()));
        assert!(!event.has("trend_micro_vision_one.network_activity.act_value"));
    }

    /// A guard testing a DIFFERENT local than the block binds is a different
    /// pattern, and is declined.
    #[test]
    fn a_guard_over_another_local_declines() {
        assert!(
            parse_named_table_lookups(
                "def a = ctx.x.a;\nif (params.t.containsKey(b.toString())) {\n  \
                 ctx.x.a_value = params.t[b.toString()];\n}"
            )
            .is_none()
        );
    }

    /// A write through a table the guard did not test is declined.
    #[test]
    fn a_write_through_another_table_declines() {
        assert!(
            parse_named_table_lookups(
                "def a = ctx.x.a;\nif (params.t.containsKey(a.toString())) {\n  \
                 ctx.x.a_value = params.u[a.toString()];\n}"
            )
            .is_none()
        );
    }

    /// A block doing anything besides the lookup is declined.
    #[test]
    fn a_block_that_does_more_than_the_lookup_declines() {
        assert!(
            parse_named_table_lookups(
                "def a = ctx.x.a;\nif (params.t.containsKey(a.toString())) {\n  \
                 ctx.x.a_value = params.t[a.toString()];\n  ctx.x.seen = true;\n}"
            )
            .is_none()
        );
    }

    /// One unreadable block declines the whole script, so a source is never
    /// left part-written.
    #[test]
    fn one_unreadable_block_declines_the_whole_named_script() {
        let broken = endpoint_script().replace(
            "params.winEventId[winEventId.toString()]",
            "describe(winEventId)",
        );
        assert_ne!(broken, endpoint_script(), "the replacement has to bite");
        assert!(parse_named_table_lookups(&broken).is_none());
    }

    /// Text after the last block declines: an unread statement is a different
    /// script, not a longer one.
    #[test]
    fn a_trailing_statement_declines() {
        assert!(
            parse_named_table_lookups(&format!("{}\nctx.x.done = true;", act_script())).is_none()
        );
    }

    /// The neighbouring spellings stay with their own readers.
    #[test]
    fn the_neighbouring_table_spellings_are_left_alone() {
        // gigamon's quoted subscript, which the reader above this one takes.
        assert!(
            parse_named_table_lookups(
                "if (ctx.a.b != null) {\n  ctx.a.b_value = params['t'][ctx.a.b];\n}"
            )
            .is_none()
        );
        // jamf_protect's ternary default, which `TableLookupOrLiteral` takes.
        assert!(
            parse_named_table_lookups(
                "if (ctx.a.b != null) {\n  String k = ctx.a.b.toString();\n  \
                 def v = params.t.containsKey(k) ? params.t[k] : 'Unknown';\n  \
                 ctx.a.b_value = v;\n}"
            )
            .is_none()
        );
        // qualys's `getOrDefault`, which the same matcher takes.
        assert!(
            parse_named_table_lookups(
                "String level = ctx.a.b;\nctx.a.c = params.t.getOrDefault(level, params.t[\"0\"]);"
            )
            .is_none()
        );
    }

    /// The dispatch reaches this reader for all three `trend_micro` spellings.
    #[test]
    fn the_ladder_dispatches_the_named_tables_here() {
        for script in [endpoint_script(), act_script()] {
            let pattern = crate::params::params_pattern(script);
            assert!(
                matches!(pattern, Some(crate::params::ParamsPattern::FieldTables(_))),
                "the ladder bound {pattern:?} instead"
            );
        }
    }
}
