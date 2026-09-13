// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A delimited table carried in one field, fanned out into parallel arrays and
//! a list of records.
//!
//! qualys's host detections carry the affected packages as a table inside
//! `RESULTS`, which two `gsub`s ahead of the script turn into one delimited
//! line. The header row is skipped and every later row contributes a cell to
//! each column:
//!
//! ```painless
//! String results = ctx.qualys_vmdr.asset_host_detection.vulnerability.results;
//! if (results.startsWith("Package||")) {
//!   if (ctx.package == null) {
//!     ctx.package = new HashMap();
//!     ctx.qualys_vmdr.asset_host_detection.package_nested = new ArrayList();
//!   }
//!   if (ctx.package.name == null) { ctx.package.name = new ArrayList(); }
//!   def res = results.splitOnToken(";;");
//!   for (int i=1; i < res.length; i++) {
//!     def pkg_nest = [:];
//!     def pkg = res[i].splitOnToken("||");
//!     if (pkg.length < 1) { continue; }
//!     ctx.package.name.add(pkg[0]);
//!     pkg_nest.name = pkg[0];
//!     if (pkg.length < 2) { continue; }
//!     ctx.package.version.add(pkg[1]);
//!     pkg_nest.version = pkg[1];
//!     if (pkg.length == 3) {
//!       ctx.package.fixed_version.add(pkg[2]);
//!       pkg_nest.fixed_version = pkg[2];
//!     }
//!     ctx.qualys_vmdr.asset_host_detection.package_nested.add(pkg_nest);
//!   }
//! }
//! ```
//!
//! **A SHORT ROW is not an error and the arrays go ragged.** The `continue`
//! after each column is the vendor saying so: a row of one cell contributes a
//! name and no record at all, which is why one capture's `package.name` holds
//! two entries beside a `package.version` of one. Padding the arrays or dropping
//! the row would both disagree with Elasticsearch.
//!
//! Unclaimed it costs seven paths on seven events, because `vulnerability.package.*`
//! is a `copy_from` of `package.*` and the nested list is written here too.

use dfe_core::Event;
use serde_json::{Map, Value, json};

use crate::common::{block_statements, if_block, painless_path};
use crate::params::{add_to_list, balanced, clean_path};

/// A delimited table, and where each of its columns lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelimitedTable {
    /// The field holding the whole table.
    source: String,
    /// The table is only read when the text opens with this.
    prefix: String,
    row_separator: String,
    column_separator: String,
    /// Rows the loop starts past -- the vendor's header.
    skip_rows: usize,
    /// The list each row's record is appended to.
    records: Option<String>,
    /// The loop body, in the order the vendor wrote it.
    steps: Vec<RowStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RowStep {
    /// `if (<cells>.length < n) { continue; }` -- a shorter row stops here and
    /// contributes no record.
    RequireColumns(usize),
    Column(Column),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Column {
    /// Which cell of the row.
    index: usize,
    /// The array the cell is appended to.
    target: String,
    /// The record member the cell is also written to.
    member: String,
    /// An `if (<cells>.length == n)` wrapping this column alone.
    only_when_columns: Option<usize>,
}

/// Read the fan-out, or decline.
///
/// Every literal comes off the script -- the prefix, both separators, the row
/// the loop starts at, each column's index and both of its destinations -- so a
/// vendor edit to any of them is followed rather than baked in. A statement the
/// reader has no arm for declines the whole script, because running the rest
/// would write a partial table and leave no error behind.
pub fn parse_delimited_table(script: &str) -> Option<DelimitedTable> {
    let (declaration, rest) = script.split_once(" = ctx.")?;
    let text_local = declaration.trim().rsplit(char::is_whitespace).next()?;
    let (path, after) = rest.split_once(';')?;
    let source = clean_path(path.trim());
    if source.is_empty() || source.contains(char::is_whitespace) {
        return None;
    }

    let (condition, body, _) = if_block(after.trim_start())?;
    let prefix = condition
        .trim()
        .strip_prefix(text_local)?
        .trim()
        .strip_prefix(".startsWith(")?
        .trim()
        .strip_prefix(['\'', '"'])?
        .rsplit_once(['\'', '"'])?
        .0
        .to_owned();
    if prefix.is_empty() {
        return None;
    }

    // The vendor's container allocations, which `Event::set` makes anyway. A
    // guard doing anything else is a script this does not model.
    let mut rest = body.trim_start();
    while let Some((condition, inner, after)) = if_block(rest) {
        if !condition.contains("== null") || !only_creates_containers(inner) {
            return None;
        }
        rest = after
            .trim_start()
            .strip_prefix(';')
            .unwrap_or(after)
            .trim_start();
    }

    // `def <rows> = <text>.splitOnToken("<row separator>");`. Read by hand
    // rather than by statement, because the separator qualys wrote is `;;` and
    // a statement split on `;` cuts it in half.
    let (declaration, tail) = rest.split_once(&format!(" = {text_local}.splitOnToken("))?;
    let rows_local = declaration
        .trim()
        .rsplit(char::is_whitespace)
        .next()?
        .to_owned();
    let (row_separator, tail) = quoted_literal(tail.trim_start())?;
    let tail = tail
        .trim_start()
        .strip_prefix(')')?
        .trim_start()
        .strip_prefix(';')?;

    let (header, tail) = balanced(
        tail.trim_start().strip_prefix("for")?.trim_start(),
        '(',
        ')',
    )?;
    let skip_rows = loop_start(header, &rows_local)?;
    let (loop_body, after_loop) = balanced(tail.trim_start(), '{', '}')?;
    if !after_loop.trim().is_empty() {
        return None;
    }

    let (column_separator, records, steps) = parse_loop(loop_body, &rows_local)?;
    if !steps.iter().any(|step| matches!(step, RowStep::Column(_))) {
        return None;
    }

    Some(DelimitedTable {
        source,
        prefix,
        row_separator,
        column_separator,
        skip_rows,
        records,
        steps,
    })
}

/// The quoted literal `text` opens with, and what follows its closing quote.
fn quoted_literal(text: &str) -> Option<(String, &str)> {
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let (literal, rest) = text[quote.len_utf8()..].split_once(quote)?;
    (!literal.is_empty()).then(|| (literal.to_owned(), rest))
}

/// The row index `for (int <i>=<n>; <i> < <rows>.length; <i>++)` starts at.
fn loop_start(header: &str, rows: &str) -> Option<usize> {
    let (init, tail) = header.split_once(';')?;
    let (name, start) = init.trim().strip_prefix("int ")?.split_once('=')?;
    let name = name.trim();
    if tail.split(';').next()?.trim() != format!("{name} < {rows}.length") {
        return None;
    }
    start.trim().parse().ok()
}

/// `ctx.<a> = new HashMap(); ctx.<b> = new ArrayList();` and nothing else.
fn only_creates_containers(body: &str) -> bool {
    body.split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .all(|part| {
            part.split_once('=').is_some_and(|(lhs, rhs)| {
                painless_path(lhs).is_some()
                    && matches!(
                        rhs.trim(),
                        "new HashMap()" | "new ArrayList()" | "[:]" | "[]"
                    )
            })
        })
}

/// The loop body, as the cell separator, the list the record lands in, and the
/// steps that build it.
///
/// A column is TWO statements -- the array append and the record member -- so
/// the append is held pending until the member that follows it names the same
/// cell. An append with no member, or a member naming another cell, declines:
/// either would write half a column.
fn parse_loop(loop_body: &str, rows: &str) -> Option<(String, Option<String>, Vec<RowStep>)> {
    let mut record_local = None;
    let mut cells_local = None;
    let mut separator = None;
    let mut records = None;
    let mut steps = Vec::new();
    let mut pending: Option<(String, usize)> = None;

    for statement in block_statements(loop_body)? {
        let statement = statement.trim();
        if let Some(bound) = binds_empty_map(statement) {
            record_local = Some(bound);
            continue;
        }
        if let Some((bound, cut)) = splits_a_row(statement, rows) {
            cells_local = Some(bound);
            separator = Some(cut);
            continue;
        }
        let cells = cells_local.as_deref()?;
        let record = record_local.as_deref()?;

        if let Some(minimum) = requires_columns(statement, cells) {
            if pending.is_some() {
                return None;
            }
            steps.push(RowStep::RequireColumns(minimum));
            continue;
        }
        if let Some((exactly, inner)) = guarded_by_length(statement, cells) {
            if pending.is_some() {
                return None;
            }
            steps.push(RowStep::Column(parse_column(
                inner,
                cells,
                record,
                Some(exactly),
            )?));
            continue;
        }
        if let Some(appended) = appends_record(statement, record) {
            if pending.is_some() {
                return None;
            }
            records = Some(appended);
            continue;
        }
        if let Some((head, rest)) = statement.split_once(".add(") {
            if pending.is_some() {
                return None;
            }
            let index = cell_index(rest.trim().trim_end_matches(')'), cells)?;
            pending = Some((painless_path(head)?, index));
            continue;
        }

        let (target, index) = pending.take()?;
        let (member, named) = record_member(statement, cells, record)?;
        if named != index {
            return None;
        }
        steps.push(RowStep::Column(Column {
            index,
            target,
            member,
            only_when_columns: None,
        }));
    }

    if pending.is_some() {
        return None;
    }
    Some((separator?, records, steps))
}

/// `def <bound> = <rows>[<i>].splitOnToken('<separator>')`, as its two parts.
fn splits_a_row(statement: &str, rows: &str) -> Option<(String, String)> {
    let (declaration, rest) = statement.split_once(&format!(" = {rows}["))?;
    let bound = declaration
        .trim()
        .rsplit(char::is_whitespace)
        .next()?
        .to_owned();
    let (_, rest) = rest.split_once("].splitOnToken(")?;
    let (separator, _) = quoted_literal(rest.trim_start())?;
    (!bound.is_empty()).then_some((bound, separator))
}

/// `<record>.<member> = <cells>[<n>]`, as the member and the cell it names.
fn record_member(statement: &str, cells: &str, record: &str) -> Option<(String, usize)> {
    let (lhs, rhs) = statement.split_once('=')?;
    let named = lhs.trim().strip_prefix(record)?.strip_prefix('.')?.trim();
    if named.is_empty() || !named.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    Some((named.to_owned(), cell_index(rhs.trim(), cells)?))
}

/// `def <bound> = [:];`, the record each row builds.
fn binds_empty_map(statement: &str) -> Option<String> {
    let (declaration, rest) = statement.split_once(" = ")?;
    matches!(rest.trim(), "[:]" | "new HashMap()")
        .then(|| declaration.trim().rsplit(char::is_whitespace).next())
        .flatten()
        .map(str::to_owned)
}

/// `if (<cells>.length < n) { continue; }`, as the minimum it demands.
fn requires_columns(statement: &str, cells: &str) -> Option<usize> {
    let (condition, body, _) = if_block(statement)?;
    if body.trim().trim_end_matches(';').trim() != "continue" {
        return None;
    }
    condition
        .trim()
        .strip_prefix(cells)?
        .trim()
        .strip_prefix(".length")?
        .trim()
        .strip_prefix('<')?
        .trim()
        .parse()
        .ok()
}

/// `if (<cells>.length == n) { <inner> }`, as the count and the body.
fn guarded_by_length<'a>(statement: &'a str, cells: &str) -> Option<(usize, &'a str)> {
    let (condition, body, _) = if_block(statement)?;
    let exactly = condition
        .trim()
        .strip_prefix(cells)?
        .trim()
        .strip_prefix(".length")?
        .trim()
        .strip_prefix("==")?
        .trim()
        .parse()
        .ok()?;
    Some((exactly, body))
}

/// `ctx.<records>.add(<record>);`, the list every complete row lands in.
fn appends_record(statement: &str, record: &str) -> Option<String> {
    let (head, _) = statement.split_once(&format!(".add({record})"))?;
    let path = painless_path(head)?;
    (!path.is_empty()).then_some(path)
}

/// One column: the array append and the record member, both off the same cell.
///
/// Both halves are required. A cell written to only one of them is a shape this
/// does not model, and taking it would silently drop the other destination.
fn parse_column(
    text: &str,
    cells: &str,
    record: &str,
    only_when_columns: Option<usize>,
) -> Option<Column> {
    let mut target = None;
    let mut member = None;
    let mut index = None;

    for statement in text.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        let at = if let Some((head, rest)) = statement.split_once(".add(") {
            target = Some(painless_path(head)?);
            cell_index(rest.trim().trim_end_matches(')'), cells)?
        } else {
            let (named, at) = record_member(statement, cells, record)?;
            member = Some(named);
            at
        };
        if index.get_or_insert(at) != &at {
            return None;
        }
    }

    Some(Column {
        index: index?,
        target: target?,
        member: member?,
        only_when_columns,
    })
}

/// `<cells>[<n>]`, as the cell it names.
fn cell_index(term: &str, cells: &str) -> Option<usize> {
    term.trim()
        .strip_prefix(cells)?
        .trim()
        .strip_prefix('[')?
        .strip_suffix(']')?
        .trim()
        .parse()
        .ok()
}

/// Fan the table out, or leave the document untouched.
pub fn delimited_table(event: &mut Event, pattern: &DelimitedTable) -> bool {
    let Some(text) = event.get_str(&pattern.source) else {
        return true;
    };
    if !text.starts_with(&pattern.prefix) {
        return true;
    }

    let rows: Vec<Vec<String>> = text
        .split(&pattern.row_separator)
        .skip(pattern.skip_rows)
        .map(|row| {
            row.split(&pattern.column_separator)
                .map(str::to_owned)
                .collect()
        })
        .collect();

    for cells in rows {
        let mut record = Map::new();
        let mut short = false;
        for step in &pattern.steps {
            match step {
                RowStep::RequireColumns(minimum) => {
                    if cells.len() < *minimum {
                        short = true;
                        break;
                    }
                }
                RowStep::Column(column) => {
                    if column.only_when_columns.is_some_and(|n| cells.len() != n) {
                        continue;
                    }
                    let Some(cell) = cells.get(column.index) else {
                        continue;
                    };
                    add_to_list(event, &column.target, json!(cell));
                    record.insert(column.member.clone(), json!(cell));
                }
            }
        }
        if short {
            continue;
        }
        if let Some(records) = &pattern.records {
            add_to_list(event, records, Value::Object(record));
        }
    }
    true
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::common::{normalise, try_known_painless};

    /// Verbatim from `qualys_vmdr_asset_host_detection/default.rs`, in the
    /// escaped one-line form the call site holds.
    const QUALYS_PACKAGES: &str = r#"String results = ctx.qualys_vmdr.asset_host_detection.vulnerability.results;\nif (results.startsWith(\"Package||\")) {\n  if (ctx.package == null) {\n    ctx.package = new HashMap();\n    ctx.qualys_vmdr.asset_host_detection.package_nested = new ArrayList();\n  }\n  if (ctx.package.name == null) {\n    ctx.package.name = new ArrayList();\n  }\n  if (ctx.package.version == null) {\n    ctx.package.version = new ArrayList();\n  }\n  if (ctx.package.fixed_version == null) {\n    ctx.package.fixed_version = new ArrayList();\n  }\n  def res = results.splitOnToken(\";;\");\n  for (int i=1; i < res.length; i++) {\n    def pkg_nest = [:];\n    def pkg = res[i].splitOnToken(\"||\");\n    if (pkg.length < 1) {\n      continue;\n    }\n    ctx.package.name.add(pkg[0]);\n    pkg_nest.name = pkg[0];\n    if (pkg.length < 2) {\n      continue;\n    }\n    ctx.package.version.add(pkg[1]);\n    pkg_nest.version = pkg[1];\n    if (pkg.length == 3) {\n      ctx.package.fixed_version.add(pkg[2]);\n      pkg_nest.fixed_version = pkg[2];\n    }\n    ctx.qualys_vmdr.asset_host_detection.package_nested.add(pkg_nest);\n  }\n}"#;

    fn with_results(results: &str) -> Event {
        Event::new(json!({ "qualys_vmdr": { "asset_host_detection": {
            "vulnerability": { "results": results }
        }}}))
    }

    const NESTED: &str = "qualys_vmdr.asset_host_detection.package_nested";

    #[test]
    fn every_full_row_fills_both_destinations() {
        let mut event = with_results(
            "Package||Installed Version||Required Version\
             ;;kernel-devel||3.10.0-1062.1.1||3.10.0-1062.1.2\
             ;;kernel||3.10.0-1062.1.1||3.10.0-1062.1.2",
        );
        assert!(try_known_painless(&mut event, QUALYS_PACKAGES));
        assert_eq!(
            event.get("package.name"),
            Some(&json!(["kernel-devel", "kernel"]))
        );
        assert_eq!(
            event.get("package.version"),
            Some(&json!(["3.10.0-1062.1.1", "3.10.0-1062.1.1"]))
        );
        assert_eq!(
            event.get("package.fixed_version"),
            Some(&json!(["3.10.0-1062.1.2", "3.10.0-1062.1.2"]))
        );
        assert_eq!(
            event.get(NESTED),
            Some(&json!([
                {"name": "kernel-devel", "version": "3.10.0-1062.1.1", "fixed_version": "3.10.0-1062.1.2"},
                {"name": "kernel", "version": "3.10.0-1062.1.1", "fixed_version": "3.10.0-1062.1.2"},
            ]))
        );
    }

    /// A one-cell row contributes a name and stops, so the arrays go ragged and
    /// no record is written for it. Padding them would disagree with the
    /// capture, which holds two names beside one version.
    #[test]
    fn a_short_row_contributes_a_name_and_no_record() {
        let mut event = with_results(
            "Package||Installed Version||Required Version\
             ;;rsync||3.1.2-12.els1||3.1.2-12.els2\
             ;;CentOS Linux release 7.9",
        );
        assert!(try_known_painless(&mut event, QUALYS_PACKAGES));
        assert_eq!(
            event.get("package.name"),
            Some(&json!(["rsync", "CentOS Linux release 7.9"]))
        );
        assert_eq!(
            event.get("package.version"),
            Some(&json!(["3.1.2-12.els1"]))
        );
        assert_eq!(
            event.get(NESTED),
            Some(&json!([
                {"name": "rsync", "version": "3.1.2-12.els1", "fixed_version": "3.1.2-12.els2"},
            ]))
        );
    }

    /// The last column is guarded on EXACTLY three cells, so a wider row takes
    /// the record without a `fixed_version`.
    #[test]
    fn a_row_wider_than_the_table_skips_the_guarded_column() {
        let mut event = with_results("Package||A||B;;curl||8.5.0||8.5.1||extra");
        assert!(try_known_painless(&mut event, QUALYS_PACKAGES));
        assert_eq!(event.get("package.fixed_version"), None);
        assert_eq!(
            event.get(NESTED),
            Some(&json!([{"name": "curl", "version": "8.5.0"}]))
        );
    }

    /// The vendor's own guard: text that is not the package table is left alone.
    #[test]
    fn text_without_the_prefix_writes_nothing() {
        let mut event = with_results("No packages found;;at all");
        assert!(try_known_painless(&mut event, QUALYS_PACKAGES));
        assert_eq!(event.get("package.name"), None);
        assert_eq!(event.get(NESTED), None);
    }

    /// The reader names the literals it found rather than assuming qualys's.
    #[test]
    fn the_reader_names_the_separators_and_the_header_row() {
        let pattern =
            parse_delimited_table(&normalise(QUALYS_PACKAGES)).expect("the fan-out is read");
        assert_eq!(pattern.prefix, "Package||");
        assert_eq!(pattern.row_separator, ";;");
        assert_eq!(pattern.column_separator, "||");
        assert_eq!(pattern.skip_rows, 1);
        assert_eq!(pattern.records.as_deref(), Some(NESTED));
        assert_eq!(pattern.steps.len(), 5);
    }

    /// A statement the reader has no arm for declines the whole script rather
    /// than writing a partial table.
    #[test]
    fn an_unrecognised_statement_declines() {
        let script = normalise(QUALYS_PACKAGES).replace(
            "ctx.qualys_vmdr.asset_host_detection.package_nested.add(pkg_nest);",
            "ctx.qualys_vmdr.asset_host_detection.package_nested.add(pkg_nest);\n    ctx.event.kind = pkg[9];",
        );
        assert!(parse_delimited_table(&script).is_none());
    }
}
