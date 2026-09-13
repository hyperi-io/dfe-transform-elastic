// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Named members gathered out of every record of a list, each into a list of
//! its own, written only where the walk came to something.
//!
//! `ti_threatq` walks its sources once and takes two members out of them, the
//! second filtered against a literal allow-list:
//!
//! ```painless
//! def ecsTlps = ['WHITE', 'GREEN', 'AMBER', 'RED', 'CLEAR', 'AMBER+STRICT'];
//! def providers = new ArrayList();
//! def tlps = new ArrayList();
//! for (source in ctx.threatq.sources) {
//!   if (source == null) { return; }
//!   if (source.containsKey("name") && source["name"] != null) {
//!     providers.add(source["name"]);
//!   }
//!   if (source.containsKey("tlp_name") && source["tlp_name"] != null
//!       && ecsTlps.contains(source["tlp_name"])) {
//!     tlps.add(source["tlp_name"]);
//!   }
//! }
//! if (tlps.size() > 0) { ... ctx.threat.indicator.marking.tlp = tlps; }
//! if (providers.size() > 0) { ... ctx.threat.indicator.provider = providers; }
//! ```
//!
//! `CollectFromList` claims the same loop and reads the member name off the
//! GUARD, so both columns came back named `containsKey`, the allow-list was
//! dropped and each target was written as an empty list. The member is read
//! here off the `add` instead, which is the statement that says what is
//! gathered.
//!
//! The reader is held to the SUBSCRIPT spelling. gdacs walks its countries the
//! same way over `c.countryname` and writes its three lists unguarded, and that
//! script is `CollectFromList`'s -- taking it here would drop the empty list it
//! is meant to leave behind.
//!
//! # What one walk can produce besides a plain list
//!
//! `ti_mandiant_advantage` ships two scripts over this grammar that between
//! them use every part of it, and reading only the plain list cost the source
//! 19 of its 29 wrong fields while this matcher reported a clean binding for
//! both. Each is a property of the script, not a new pattern:
//!
//! - **A member that is itself a LIST**, flattened in with `addAll` rather than
//!   appended whole with `add`. `categories.addAll(source["category"])` was
//!   read by neither spelling, so the column was skipped in silence and the
//!   indicator's categories never written.
//! - **A dedupe**, `!providers.contains(...)` in the arm's own guard. Without
//!   it a source listed five times came back as `["Mandiant","Mandiant", ...]`.
//! - **A selector**, `association["type"] == "threat-actor"`, choosing WHICH
//!   records give a column its value. One list of associations feeds both the
//!   threat group and the software column and the `type` member is what splits
//!   them; ignoring it wrote every name into both.
//! - **More than one target**, `ctx.threat.group.id` and
//!   `ctx.threat.group.name` off the same accumulator. Reading only the last
//!   assignment dropped the id.
//! - **A literal written beside the list** under the same `size() > 0` guard --
//!   `ctx.threat.software.type = "Malware"`, which is part of what the walk
//!   produces rather than a separate processor.
//! - **A boolean folded across the walk**, opened true and cleared by the first
//!   record that carries a false member, choosing between two literals at the
//!   end. mandiant's `is_osint` picks the indicator's TLP that way.
//!
//! All six are declined by every other script this matcher claims, which is
//! what the neighbouring tests below hold.

use serde_json::Value;

use crate::params::{balanced, clean_path};
use dfe_core::Event;

/// Which records of a walk give a column its value.
///
/// mandiant sorts one list of associations into two columns by a `type` member
/// the columns do not themselves gather.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selector {
    key: String,
    value: String,
}

/// A boolean folded across the walk and written as one of two literals.
///
/// mandiant's `is_osint` opens true, goes false at the first source whose
/// `osint` member is present and false, and decides the indicator's TLP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatheredFlag {
    /// The member whose presence-and-falsehood clears the flag.
    member: String,
    target: String,
    /// Written when no record cleared the flag, and when one did.
    when_set: String,
    when_clear: String,
}

/// One member gathered, and where it lands.
///
/// The three booleans are independent modifiers of the same gather rather than
/// correlated states -- how the value enters the list, whether it is folded on
/// the way in, and whether a repeat is kept -- so they stay separate flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatheredColumn {
    /// The record key gathered.
    member: String,
    /// Every path the gathered list is written to, in the order the script
    /// writes them.
    targets: Vec<String>,
    /// The literal values the script admits, empty where it filters nothing.
    allowed: Vec<String>,
    /// Whether the value is folded on the way in, which is also the spelling
    /// the allow-list is tested in.
    lowercase: bool,
    /// `addAll` -- the member is a list flattened in, not a value appended.
    flatten: bool,
    /// `!<acc>.contains(<value>)` -- a value already gathered is not gathered
    /// again.
    dedupe: bool,
    /// The member and literal that choose which records feed this column.
    selector: Option<Selector>,
    /// Literals written beside the list under the same `size() > 0` guard.
    literals: Vec<(String, String)>,
}

impl GatheredColumn {
    /// The plain column: one member gathered into one target, unfiltered.
    #[must_use]
    pub fn new(member: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            member: member.into(),
            targets: vec![target.into()],
            allowed: Vec::new(),
            lowercase: false,
            flatten: false,
            dedupe: false,
            selector: None,
            literals: Vec::new(),
        }
    }

    /// Admit only these literal values.
    #[must_use]
    pub fn allowing(mut self, allowed: &[&str]) -> Self {
        self.allowed = allowed.iter().map(|held| (*held).to_owned()).collect();
        self
    }

    /// Fold the value before testing and gathering it.
    #[must_use]
    pub fn folded(mut self) -> Self {
        self.lowercase = true;
        self
    }

    /// The member is a list flattened into the accumulator.
    #[must_use]
    pub fn flattened(mut self) -> Self {
        self.flatten = true;
        self
    }

    /// A value already gathered is not gathered again.
    #[must_use]
    pub fn deduped(mut self) -> Self {
        self.dedupe = true;
        self
    }

    /// Take only records whose `key` member carries `value`.
    #[must_use]
    pub fn selected_on(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.selector = Some(Selector {
            key: key.into(),
            value: value.into(),
        });
        self
    }

    /// Write the same list to a further target.
    #[must_use]
    pub fn also_into(mut self, target: impl Into<String>) -> Self {
        self.targets.push(target.into());
        self
    }

    /// Stamp a literal beside the list when the walk came to something.
    #[must_use]
    pub fn with_literal(mut self, path: impl Into<String>, value: impl Into<String>) -> Self {
        self.literals.push((path.into(), value.into()));
        self
    }
}

/// Members gathered out of a list's records into parallel lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatherMembers {
    list: String,
    /// Whether a null element abandons the whole script.
    stop_on_null_element: bool,
    columns: Vec<GatheredColumn>,
    flag: Option<GatheredFlag>,
}

impl GatherMembers {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        list: impl Into<String>,
        stop_on_null_element: bool,
        columns: Vec<GatheredColumn>,
    ) -> Self {
        Self {
            list: list.into(),
            stop_on_null_element,
            columns,
            flag: None,
        }
    }

    /// Add the boolean this walk also folds.
    #[must_use]
    pub fn with_flag(
        mut self,
        member: impl Into<String>,
        target: impl Into<String>,
        when_set: impl Into<String>,
        when_clear: impl Into<String>,
    ) -> Self {
        self.flag = Some(GatheredFlag {
            member: member.into(),
            target: target.into(),
            when_set: when_set.into(),
            when_clear: when_clear.into(),
        });
        self
    }
}

/// The literal string lists a script binds to locals.
///
/// `def ecsTlps = ['WHITE', 'GREEN'];` -- a list holding anything but quoted
/// literals is not an allow-list this can check against.
fn literal_lists(script: &str) -> Vec<(String, Vec<String>)> {
    let mut lists = Vec::new();
    for statement in script.split(';') {
        let Some((name, value)) = statement
            .trim()
            .strip_prefix("def ")
            .and_then(|s| s.split_once('='))
        else {
            continue;
        };
        let value = value.trim();
        let Some((inner, after)) = balanced(value, '[', ']') else {
            continue;
        };
        if !after.trim().is_empty() || inner.trim().is_empty() {
            continue;
        }
        let members: Vec<String> = inner
            .split(',')
            .map(|member| member.trim().trim_matches(['\'', '"']).to_owned())
            .collect();
        if members.iter().any(String::is_empty) || inner.contains(['(', ':']) {
            continue;
        }
        lists.push((name.trim().to_owned(), members));
    }
    lists
}

/// The locals a script opens as empty accumulators.
///
/// `def names = new ArrayList()` and `ArrayList names = new ArrayList()` open
/// the same accumulator, and reading only the `def` spelling cost
/// `jamf_protect` its `related.user`: the walk below found no accumulator, this
/// reader declined, and `CollectFromList` claimed the script and named the
/// member off the guard. The name is the declaration's last word either way.
/// `ctx.a = new ArrayList()` is not a local and is refused on the dot.
///
/// The split is on the ALLOCATION rather than the statement's first `=`: a
/// declaration can open inside the guard that precedes it, and jamf's
/// `if (... != null) { ArrayList names = new ArrayList()` is one statement whose
/// first `=` belongs to the guard's own `!=`.
fn accumulators(script: &str) -> Vec<String> {
    script
        .split(';')
        .filter_map(|statement| {
            let (declaration, after) = statement.split_once("= new ArrayList()")?;
            if !after.trim().is_empty() {
                return None;
            }
            let name = declaration
                .trim_end()
                .rsplit([' ', '\t', '\n', '\r'])
                .next()?;
            (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
                .then(|| name.to_owned())
        })
        .collect()
}

/// The allow-list a guard writes INLINE at the test it makes.
///
/// `["linux", "macos"].contains(...)` is the same filter `ti_threatq` binds to a
/// local first, and cybereason spells it at the point of use.
fn inline_allow_list(arm: &str) -> Option<Vec<String>> {
    let (head, _) = arm.split_once("].contains(")?;
    let at = head.rfind('[')?;
    let members: Vec<String> = head[at + 1..]
        .split(',')
        .map(|member| member.trim().trim_matches(['\'', '"']).to_owned())
        .collect();
    // A list holding anything but quoted literals is not a filter this can
    // check a value against.
    (!members.is_empty()
        && !members.iter().any(String::is_empty)
        && !head[at + 1..].contains(['(', ':', '[']))
    .then_some(members)
}

/// The loop variable and the `ctx.` list it walks.
///
/// Painless spells the same for-each two ways -- `for (v in ctx.x)` and
/// `for (def v : ctx.x)` -- and both are read, because which one a vendor wrote
/// says nothing about what the loop does.
fn walked_list(script: &str) -> Option<(String, String, &str)> {
    let (_, rest) = script.split_once("for (")?;
    let (var, rest) = rest
        .split_once(" in ctx")
        .or_else(|| rest.split_once(" : ctx"))?;
    let var = var.trim().trim_start_matches("def ").trim();
    if var.is_empty() || !var.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (list, rest) = rest.split_once(')')?;
    let list = clean_path(list.trim().trim_start_matches(['?', '.']));
    if list.is_empty() || list.contains(['(', '[', ' ']) {
        return None;
    }
    Some((var.to_owned(), list, rest))
}

/// The `{ ... }` block that opens next, and the text after it.
fn block(text: &str) -> Option<(&str, &str)> {
    let at = text.find('{')?;
    balanced(&text[at..], '{', '}')
}

/// The `ctx.` path a statement assigns to, read back from the `=`.
fn assigned_path(head: &str) -> Option<String> {
    let path = head
        .trim_end()
        .rsplit(['\n', '\r', '\t', ' ', '{', '}', ';', '('])
        .next()?
        .trim()
        .strip_prefix("ctx")?;
    let path = clean_path(path.trim_start_matches(['?', '.']));
    (!path.is_empty() && !path.contains(['(', '['])).then_some(path)
}

/// Every `ctx.<path> = "<literal>";` in a block.
///
/// The `= ` is spelled with its surrounding spaces on purpose: `== null` and
/// `!= null` both hold an `=` next to a space and neither is an assignment.
fn literal_assignments(text: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = text[cursor..].find(" = ") {
        let at = cursor + rel;
        cursor = at + " = ".len();
        let rest = &text[cursor..];
        let Some(quote @ ('"' | '\'')) = rest.chars().next() else {
            continue;
        };
        let Some((value, _)) = rest[quote.len_utf8()..].split_once(quote) else {
            continue;
        };
        let Some(path) = assigned_path(&text[..at]) else {
            continue;
        };
        found.push((path, value.to_owned()));
    }
    found
}

/// Every `ctx.<path> = <accumulator>;`, with where the script writes it.
///
/// The leading space is what keeps `= groups;` out of `= subgroups;`.
fn assigned_targets(script: &str, accumulator: &str) -> Vec<(usize, String)> {
    let assignment = format!("= {accumulator};");
    let mut targets: Vec<(usize, String)> = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = script[cursor..].find(&assignment) {
        let at = cursor + rel;
        cursor = at + assignment.len();
        let Some(path) = assigned_path(&script[..at]) else {
            continue;
        };
        if !targets.iter().any(|(_, held)| held == &path) {
            targets.push((at, path));
        }
    }
    targets
}

/// `<var>["<key>"] == "<literal>"` -- the member a column's records are chosen
/// by.
///
/// Every subscript in the arm is tried, because the guard that tests a member
/// for presence and the one that tests another for a value appear in either
/// order. Only a `==` against a quoted literal is a selector: threatq and jamf
/// both subscript the member they are about to gather and compare it to
/// `null`, which chooses no records at all.
fn selector(arm: &str, var: &str) -> Option<Selector> {
    for opening in [format!("{var}[\""), format!("{var}['")] {
        let mut cursor = 0usize;
        while let Some(rel) = arm[cursor..].find(opening.as_str()) {
            let at = cursor + rel + opening.len();
            cursor = at;
            let Some((key, after)) = arm[at..].split_once(']') else {
                break;
            };
            let key = key.trim().trim_matches(['"', '\'']);
            let after = after.trim_start();
            let Some(tail) = after.strip_prefix("==") else {
                continue;
            };
            let tail = tail.trim_start();
            let Some(quote @ ('"' | '\'')) = tail.chars().next() else {
                continue;
            };
            let Some((value, _)) = tail[quote.len_utf8()..].split_once(quote) else {
                continue;
            };
            if !key.is_empty() && !value.is_empty() {
                return Some(Selector {
                    key: key.to_owned(),
                    value: value.to_owned(),
                });
            }
        }
    }
    None
}

/// What reading one accumulator came to.
enum Column {
    /// The walk never adds to it, so it is a local doing some other job.
    NotGathered,
    /// The gather, and where in the script its list is written.
    Read(usize, Box<GatheredColumn>),
    /// An add this reader cannot resolve, which declines the WHOLE script --
    /// writing some of a walk and not the rest is what makes a source read as
    /// needing polish rather than a different matcher.
    Unreadable,
}

/// Where the walk appends to this accumulator, and how.
///
/// `<acc>.add(<var>["<member>"])` -- the add says what is gathered, and an add
/// of anything but the walked record's own member is a value this reader cannot
/// resolve.
///
/// `.get("<member>")` is the same read written as a call, which is what
/// `ti_threatconnect` spells. Reading only the subscript declined it, and
/// `CollectFromList` then named the member off the guard and gathered one
/// called `get`. The DOTTED spelling stays out: gdacs walks `c.countryname` and
/// writes its lists whether or not they hold anything, which is that matcher's
/// script and not this one.
///
/// `addAll` is the same statement for a member that is itself a list. The two
/// spellings cannot collide -- `add` is only `addAll` with a `(` where the `A`
/// is.
fn appended<'a>(
    body: &'a str,
    var: &str,
    accumulator: &str,
) -> Option<(&'a str, &'a str, char, bool)> {
    [
        (format!("{accumulator}.add({var}["), ']', false),
        (format!("{accumulator}.add({var}.get("), ')', false),
        (format!("{accumulator}.addAll({var}["), ']', true),
        (format!("{accumulator}.addAll({var}.get("), ')', true),
    ]
    .iter()
    .find_map(|(needle, closing, flatten)| {
        let (guard, rest) = body.split_once(needle.as_str())?;
        Some((guard, rest, *closing, *flatten))
    })
}

/// Read one accumulator's column.
fn column(
    script: &str,
    body: &str,
    var: &str,
    accumulator: &str,
    lists: &[(String, Vec<String>)],
) -> Column {
    let Some((guard, rest, closing, flatten)) = appended(body, var, accumulator) else {
        return Column::NotGathered;
    };
    match read_column(
        script,
        guard,
        rest,
        closing,
        flatten,
        var,
        accumulator,
        lists,
    ) {
        Some((at, read)) => Column::Read(at, Box::new(read)),
        None => Column::Unreadable,
    }
}

/// The gather itself, once the append has been found.
#[allow(clippy::too_many_arguments)] // The parts of one append, passed on rather than re-found.
fn read_column(
    script: &str,
    guard: &str,
    rest: &str,
    closing: char,
    flatten: bool,
    var: &str,
    accumulator: &str,
    lists: &[(String, Vec<String>)],
) -> Option<(usize, GatheredColumn)> {
    let (member, after) = rest.split_once(closing)?;
    let member = member.trim().trim_matches(['\'', '"']).to_owned();
    if member.is_empty() || !rest.starts_with(['"', '\'']) {
        return None;
    }
    // `.toLowerCase()` on the way into the list, which cybereason applies to
    // the value AND to the allow-list test that admits it.
    let lowercase = after.trim_start().starts_with(".toLowerCase()");

    // `ctx.<target> = <acc>;`, guarded on the walk having come to something.
    // Without the guard the script writes an empty list, which is a different
    // answer and a different matcher.
    if !script.contains(&format!("{accumulator}.size() > 0")) {
        return None;
    }
    let targets = assigned_targets(script, accumulator);
    let (at, _) = *targets.first()?;

    // The allow-list is whichever list the arm's own guard tests: a local the
    // script bound, or one written inline at the test.
    let arm = guard.rsplit_once("if (").map_or(guard, |(_, arm)| arm);
    let allowed = lists
        .iter()
        .find(|(name, _)| arm.contains(&format!("{name}.contains(")))
        .map(|(_, members)| members.clone())
        .or_else(|| inline_allow_list(arm))
        .unwrap_or_default();

    // The literals the terminal block stamps alongside the list.
    let literals = script
        .find(&format!("{accumulator}.size() > 0"))
        .and_then(|at| block(&script[at..]))
        .map(|(terminal, _)| literal_assignments(terminal))
        .unwrap_or_default();

    Some((
        at,
        GatheredColumn {
            member,
            targets: targets.into_iter().map(|(_, path)| path).collect(),
            allowed,
            lowercase,
            flatten,
            dedupe: arm.contains(&format!("!{accumulator}.contains(")),
            selector: selector(arm, var),
            literals,
        },
    ))
}

/// The boolean this walk folds, where it folds one.
///
/// Held to the shape mandiant spells: opened `true` before the loop, cleared by
/// a guard that reads the record's own member, and read once at the end to
/// choose between two literals landing on ONE field. Two different fields is
/// two writes and not a flag.
fn parse_flag(script: &str, body: &str, var: &str, tail: &str) -> Option<GatheredFlag> {
    let (head, _) = script.split_once(" = true;")?;
    let name = head.trim_end().rsplit([' ', '\t', '\n', '\r']).next()?;
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    let (guard, _) = body.split_once(&format!("{name} = false;"))?;
    let arm = guard.rsplit_once("if (").map_or(guard, |(_, arm)| arm);
    let (_, rest) = arm.split_once(&format!("!{var}["))?;
    let (member, _) = rest.split_once(']')?;
    let member = member.trim().trim_matches(['"', '\'']).to_owned();
    if member.is_empty() {
        return None;
    }

    let (_, rest) = tail.split_once(&format!("if ({name})"))?;
    let (set_body, after) = block(rest)?;
    let (set_target, when_set) = literal_assignments(set_body).into_iter().next()?;
    let (_, otherwise) = after.split_once("else")?;
    let (clear_body, _) = block(otherwise)?;
    let (clear_target, when_clear) = literal_assignments(clear_body).into_iter().next()?;

    (set_target == clear_target).then_some(GatheredFlag {
        member,
        target: set_target,
        when_set,
        when_clear,
    })
}

/// Read the list, the members gathered and where each is written, or decline.
#[must_use]
pub fn parse_gather_members(script: &str) -> Option<GatherMembers> {
    let (var, list, after) = walked_list(script)?;
    let (body, tail) = balanced(after.trim_start(), '{', '}')?;
    let lists = literal_lists(script);

    let mut columns = Vec::new();
    for accumulator in accumulators(script) {
        match column(script, body, &var, &accumulator, &lists) {
            Column::NotGathered => {}
            Column::Read(at, read) => columns.push((at, *read)),
            Column::Unreadable => return None,
        }
    }
    if columns.is_empty() || !tail.contains(".size() > 0") {
        return None;
    }

    // Written in the order the script writes them, which is the order its
    // terminal guards appear rather than the order the loop gathers.
    columns.sort_by_key(|(at, _)| *at);

    Some(GatherMembers {
        list,
        stop_on_null_element: body.contains(&format!("{var} == null")) && body.contains("return"),
        columns: columns.into_iter().map(|(_, column)| column).collect(),
        flag: parse_flag(script, body, &var, tail),
    })
}

/// Gather each member across the records, writing only a non-empty list.
#[must_use]
pub fn gather_members(event: &mut Event, pattern: &GatherMembers) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.list).cloned() else {
        // The call site is guarded on the list being present.
        return true;
    };

    // A null element returns out of the script, so nothing at all is written --
    // including whatever the walk had gathered before reaching it.
    if pattern.stop_on_null_element && records.iter().any(Value::is_null) {
        return true;
    }

    for column in &pattern.columns {
        let mut gathered: Vec<Value> = Vec::with_capacity(records.len());
        for record in &records {
            // One list of associations feeds both the group column and the
            // software column, and the selector's member is what splits them.
            if let Some(selector) = &column.selector
                && record.get(&selector.key).and_then(Value::as_str)
                    != Some(selector.value.as_str())
            {
                continue;
            }
            let Some(value) = record.get(&column.member).filter(|v| !v.is_null()) else {
                continue;
            };
            // `addAll` of anything but a list throws, so the walk gives up that
            // record rather than gathering the value whole.
            let flattened = match (column.flatten, value) {
                (true, Value::Array(items)) => items.as_slice(),
                (true, _) => continue,
                (false, value) => std::slice::from_ref(value),
            };
            for value in flattened {
                // The fold happens BEFORE the test, because the script tests the
                // folded spelling and gathers the folded value.
                let value = match (column.lowercase, value.as_str()) {
                    (true, Some(text)) => Value::String(text.to_lowercase()),
                    _ => value.clone(),
                };
                if !column.allowed.is_empty()
                    && !value
                        .as_str()
                        .is_some_and(|text| column.allowed.iter().any(|held| held == text))
                {
                    continue;
                }
                if column.dedupe && gathered.contains(&value) {
                    continue;
                }
                gathered.push(value);
            }
        }
        let Some((last, rest)) = column.targets.split_last() else {
            continue;
        };
        if gathered.is_empty() {
            continue;
        }
        for target in rest {
            let _ = event.set(target, Value::Array(gathered.clone()));
        }
        let _ = event.set(last, Value::Array(gathered));
        for (path, literal) in &column.literals {
            let _ = event.set(path, Value::String(literal.clone()));
        }
    }

    if let Some(flag) = &pattern.flag {
        // The flag opens true and the first record carrying the member as false
        // clears it.
        let set = !records
            .iter()
            .any(|record| record.get(&flag.member) == Some(&Value::Bool(false)));
        let literal = if set {
            &flag.when_set
        } else {
            &flag.when_clear
        };
        let _ = event.set(&flag.target, Value::String(literal.clone()));
    }
    true
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(clippy::needless_raw_string_hashes)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/ti_threatq_threat/default.rs`, in
    /// the escaped one-line form the call site holds.
    const THREATQ_SOURCES: &str = r#"def ecsTlps = ['WHITE', 'GREEN', 'AMBER', 'RED', 'CLEAR', 'AMBER+STRICT'];\ndef providers = new ArrayList();\ndef tlps = new ArrayList();\nfor (source in ctx.threatq.sources) {\n  if (source == null) {\n    return;\n  }\n  if (source.containsKey(\"name\") && source[\"name\"] != null) {\n    providers.add(source[\"name\"]);\n  }\n  if (source.containsKey(\"tlp_name\") && source[\"tlp_name\"] != null && ecsTlps.contains(source[\"tlp_name\"])) {\n    tlps.add(source[\"tlp_name\"]);\n  }\n}\nif (tlps.size() > 0) {\n  if (ctx.threat.indicator.marking == null) {\n    ctx.threat.indicator.marking = new HashMap();\n  }\n  ctx.threat.indicator.marking.tlp = tlps;\n}\nif (providers.size() > 0) {\n  if (ctx.threat.indicator.provider == null) {\n    ctx.threat.indicator.provider = new HashMap();\n  }\n  ctx.threat.indicator.provider = providers;\n}"#;

    /// gdacs's walk over the same grammar, which stays with `CollectFromList`:
    /// dotted members, and three lists written whether or not they hold
    /// anything.
    const GDACS_COUNTRIES: &str = r#"def countries = ctx.gdacs?.affected_countries;\nif (countries == null || countries.size() == 0) { return; }\n\ndef names = new ArrayList();\ndef iso2_codes = new ArrayList();\ndef iso3_codes = new ArrayList();\n\nfor (def c : countries) {\n  if (c.countryname != null) { names.add(c.countryname); }\n  if (c.iso2 != null) { iso2_codes.add(c.iso2); }\n  if (c.iso3 != null) { iso3_codes.add(c.iso3); }\n}\n\nctx.gdacs.affected_country_names = names;\nctx.gdacs.affected_country_iso2 = iso2_codes;\nctx.gdacs.affected_country_iso3 = iso3_codes;\n"#;

    /// `ti_threatconnect`'s neighbour, which cuts a label before filtering it
    /// against the same allow-list and binds its add to a LOCAL.
    const THREATCONNECT_LABELS: &str = r#"def ecsTlps = ['WHITE','CLEAR','GREEN','AMBER','AMBER+STRICT','RED']; def tlps = new ArrayList(); for (def obj : ctx.json.securityLabels.data) {\n  if (obj.containsKey('name')) {\n    if (obj.get('name').contains(':')){\n       def name = obj.get('name').splitOnToken(':')[1];\n       if (ecsTlps.contains(name)) {\n          tlps.add(name)\n       }\n    }\n  }\n} if (tlps.size() > 0){\n  if (ctx.threat.indicator.marking == null) {\n    ctx.threat.indicator.marking = new HashMap();\n  }\n  ctx.threat.indicator.marking.tlp = tlps;\n}"#;

    #[test]
    fn the_threatq_sources_give_up_two_members_in_one_walk() {
        assert_eq!(
            parse_gather_members(&crate::common::normalise(THREATQ_SOURCES)),
            Some(GatherMembers::new(
                "threatq.sources",
                true,
                vec![
                    GatheredColumn::new("tlp_name", "threat.indicator.marking.tlp").allowing(&[
                        "WHITE",
                        "GREEN",
                        "AMBER",
                        "RED",
                        "CLEAR",
                        "AMBER+STRICT"
                    ]),
                    GatheredColumn::new("name", "threat.indicator.provider"),
                ],
            ))
        );

        // The WRITTEN value: both members gathered, and a tlp the allow-list
        // does not hold left out of the marking.
        let mut event = Event::new(json!({ "threatq": { "sources": [
            { "name": "TAXII Feed", "tlp_name": "AMBER" },
            { "name": "Internal Research" },
            { "name": "Partner Feed", "tlp_name": "PURPLE" },
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            THREATQ_SOURCES
        ));
        assert_eq!(
            event.get("threat.indicator.provider"),
            Some(&json!(["TAXII Feed", "Internal Research", "Partner Feed"]))
        );
        assert_eq!(
            event.get("threat.indicator.marking.tlp"),
            Some(&json!(["AMBER"]))
        );
    }

    /// threatq subscripts the member it gathers and compares it to `null`, and
    /// reading that as a selector would choose no records at all.
    #[test]
    fn a_null_comparison_on_the_gathered_member_is_not_a_selector() {
        let parsed = parse_gather_members(&crate::common::normalise(THREATQ_SOURCES));
        let columns: &[GatheredColumn] = parsed.as_ref().map_or(&[], |read| &read.columns);
        assert_eq!(columns.len(), 2, "threatq's sources were declined");
        assert!(columns.iter().all(|column| column.selector.is_none()));
        assert!(columns.iter().all(|column| !column.dedupe));
        assert!(columns.iter().all(|column| column.literals.is_empty()));
        assert!(parsed.is_some_and(|read| read.flag.is_none()));
    }

    /// The `size() > 0` guards: a walk that gathers nothing writes nothing,
    /// where the empty list was what made these fields EXTRA.
    #[test]
    fn a_walk_that_gathers_nothing_writes_neither_field() {
        let mut event = Event::new(json!({ "threatq": { "sources": [ { "id": 4 } ] } }));
        assert!(crate::common::try_known_painless(
            &mut event,
            THREATQ_SOURCES
        ));
        assert_eq!(event.get("threat.indicator.provider"), None);
        assert_eq!(event.get("threat.indicator.marking.tlp"), None);
    }

    /// A null element returns out of the script, so the members gathered
    /// before it are discarded too.
    #[test]
    fn a_null_record_abandons_the_whole_walk() {
        let mut event = Event::new(json!({ "threatq": { "sources": [
            { "name": "TAXII Feed" },
            null,
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            THREATQ_SOURCES
        ));
        assert_eq!(event.get("threat.indicator.provider"), None);
    }

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/cybereason_poll_malop/default.rs`.
    ///
    /// The same walk again, reading its member through `.get("<member>")`,
    /// folding the value on the way in, and writing its allow-list INLINE at
    /// the test rather than binding it to a local first.
    const CYBEREASON_OS_TYPES: &str = r#"def os_types = new ArrayList(); for (def obj : ctx.json.machines) {\n  if (obj.containsKey(\"os_type\") && [\"linux\", \"macos\", \"unix\", \"windows\", \"ios\", \"android\"].contains((obj.get(\"os_type\")).toLowerCase())) {\n    os_types.add(obj.get(\"os_type\").toLowerCase());\n  }\n} if (os_types.size() > 0){\n  if (ctx.host.os == null) {\n    ctx.host.os = new HashMap();\n  }\n  ctx.host.os.type= os_types;\n}"#;

    /// The fold and the allow-list are what this script gathers BY, so a reader
    /// dropping either writes the vendor's own casing into a closed ECS
    /// vocabulary.
    #[test]
    fn a_folded_member_is_tested_and_gathered_in_the_folded_spelling() {
        assert_eq!(
            parse_gather_members(&crate::common::normalise(CYBEREASON_OS_TYPES)),
            Some(GatherMembers::new(
                "json.machines",
                false,
                vec![
                    GatheredColumn::new("os_type", "host.os.type")
                        .allowing(&["linux", "macos", "unix", "windows", "ios", "android"])
                        .folded(),
                ],
            ))
        );

        let mut event = Event::new(json!({ "json": { "machines": [
            { "os_type": "Windows" },
            { "os_type": "LINUX" },
            { "os_type": "Solaris" },
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            CYBEREASON_OS_TYPES
        ));
        assert_eq!(
            event.get("host.os.type"),
            Some(&json!(["windows", "linux"]))
        );
    }

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/jamf_protect_alerts/default.rs`.
    ///
    /// The SAME walk as threatq's, spelled `for (def v : ctx.x)` rather than
    /// `for (v in ctx.x)` and declaring its accumulator `ArrayList names =`
    /// rather than `def names =`.
    const JAMF_RELATED_USERS: &str = r#"if (ctx.jamf_protect?.alerts?.input?.related?.users != null && ctx.jamf_protect.alerts.input.related.users.size() > 0) {\n    ArrayList userNames = new ArrayList();\n\n    for (def user : ctx.jamf_protect.alerts.input.related.users) {\n        if (user.containsKey('name') && user['name'] != null) {\n            userNames.add(user['name']);\n        }\n    }\n    if (userNames.size() > 0) {\n        ctx.related = ctx.related ?: new HashMap();\n        ctx.related.user = userNames;\n    }\n}  \n"#;

    /// Neither spelling says anything about what the loop DOES, so both read
    /// here. `CollectFromList` claimed this one and named the member off the
    /// guard -- `related.user` came back as an empty list, which the module's
    /// own closing `drop_empty` then removed, so four events lost the field
    /// with nothing extra and no error to show for it.
    #[test]
    fn the_colon_spelling_and_a_typed_accumulator_read_the_same_walk() {
        assert_eq!(
            parse_gather_members(&crate::common::normalise(JAMF_RELATED_USERS)),
            Some(GatherMembers::new(
                "jamf_protect.alerts.input.related.users",
                false,
                vec![GatheredColumn::new("name", "related.user")],
            ))
        );

        // The WRITTEN value: every user's name, in the list's own order.
        let mut event = Event::new(
            json!({ "jamf_protect": { "alerts": { "input": { "related": {
                "users": [
                    { "uid": 0, "name": "root" },
                    { "uid": 501, "name": "local-admin" },
                ]
            }}}}}),
        );
        assert!(crate::common::try_known_painless(
            &mut event,
            JAMF_RELATED_USERS
        ));
        assert_eq!(
            event.get("related.user"),
            Some(&json!(["root", "local-admin"]))
        );
    }

    /// The script's own `size() > 0` guard: a walk that gathers nothing leaves
    /// no empty list behind for a later prune to have to clean up.
    #[test]
    fn a_users_list_with_no_names_writes_no_field_at_all() {
        let mut event = Event::new(
            json!({ "jamf_protect": { "alerts": { "input": { "related": {
                "users": [{ "uid": 0 }]
            }}}}}),
        );
        assert!(crate::common::try_known_painless(
            &mut event,
            JAMF_RELATED_USERS
        ));
        assert_eq!(event.get("related.user"), None);
    }

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/ti_threatconnect_indicator/default.rs`.
    ///
    /// The member is read through `.get('<member>')` and the guard tests a
    /// DIFFERENT member for presence, which is what sent it to
    /// `CollectFromList` -- where the member came back named `get` and the
    /// technique names were written as an empty list.
    const THREATCONNECT_TECHNIQUES: &str = r#"def t_names = new ArrayList();\nfor (def obj : ctx.json.tags.data) {\n  if (obj.get('techniqueId') != null) {\n    t_names.add(obj.get('name'));\n  }\n}\nif (t_names.size() > 0){\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  ctx.threat.technique.name = t_names;\n}"#;

    /// The `add` says what is gathered, whichever way the record is read.
    #[test]
    fn a_member_read_through_get_is_gathered_by_its_own_name() {
        assert_eq!(
            parse_gather_members(&crate::common::normalise(THREATCONNECT_TECHNIQUES)),
            Some(GatherMembers::new(
                "json.tags.data",
                false,
                vec![GatheredColumn::new("name", "threat.technique.name")],
            ))
        );

        let mut event = Event::new(json!({ "json": { "tags": { "data": [
            { "techniqueId": "T1055.005", "name": "userexecution:maliciouslink" },
            { "name": "no-technique-id" },
        ]}}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            THREATCONNECT_TECHNIQUES
        ));
        assert_eq!(
            event.get("threat.technique.name"),
            Some(&json!(["userexecution:maliciouslink", "no-technique-id"]))
        );
    }

    #[test]
    fn the_neighbouring_walks_over_the_same_grammar_are_declined() {
        // gdacs reads dotted members and writes unguarded; threatconnect adds
        // a local it cut out of the label. Claiming either would write the
        // wrong list, and gdacs would lose the empty one it leaves behind.
        for (name, script) in [
            ("gdacs", GDACS_COUNTRIES),
            ("ti_threatconnect", THREATCONNECT_LABELS),
        ] {
            assert!(
                parse_gather_members(&crate::common::normalise(script)).is_none(),
                "{name} was claimed"
            );
        }
    }

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/ti_mandiant_advantage_threat_intelligence/default.rs`.
    ///
    /// One walk that dedupes one member, FLATTENS a second that is itself a
    /// list, and folds a boolean deciding the indicator's TLP.
    const MANDIANT_SOURCES: &str = r#"def providers = new ArrayList();\ndef categories = new ArrayList();\ndef is_osint = true;\nfor (source in ctx.json.sources) {\n  if (source == null) {\n    return;\n  }\n  if (source.containsKey(\"source_name\") && source[\"source_name\"] != null && !providers.contains(source[\"source_name\"])) {\n    providers.add(source[\"source_name\"]);\n  }\n  if (source.containsKey(\"category\") && source[\"category\"] != null) {\n    categories.addAll(source[\"category\"]);\n  }\n  if (source.containsKey(\"osint\") && !source[\"osint\"]) {\n    is_osint = false;\n  }\n}\nif (providers.size() > 0) {\n  if (ctx.threat.indicator.provider == null) {\n    ctx.threat.indicator.provider = new HashMap();\n  }\n  ctx.threat.indicator.provider = providers;\n}\nif (categories.size() > 0) {\n  if (ctx.indicator_categories == null) {\n    ctx.indicator_categories = new HashMap();\n  }\n  ctx.indicator_categories = categories;\n}\nif (is_osint) {\n  ctx.tlp_color = \"GREEN\";\n}\nelse {\n  ctx.tlp_color = \"RED\";\n}"#;

    /// Verbatim from the same module. One list of associations sorted into TWO
    /// columns by a `type` member neither column gathers, one accumulator
    /// written to two targets, and a literal stamped beside the second.
    const MANDIANT_ASSOCIATIONS: &str = r#"def groups = new ArrayList();\ndef software = new ArrayList();\nfor (association in ctx.json.attributed_associations) {\n  if (association == null) {\n    return;\n  }\n  if (association.containsKey(\"type\") && association[\"type\"] == \"threat-actor\" && !groups.contains(association[\"name\"])) {\n    groups.add(association[\"name\"]);\n  }\n  else if (association.containsKey(\"type\") && association[\"type\"] == \"malware\" && !software.contains(association[\"name\"])) {\n    software.add(association[\"name\"]);\n  }\n}\nif (groups.size() > 0) {\n  if (ctx.threat.group == null) {\n    ctx.threat.group = new HashMap();\n  }\n  if (ctx.threat.group.name == null) {\n    ctx.threat.group.name = new HashMap();\n  }\n  if (ctx.threat.group.id == null) {\n    ctx.threat.group.id = new HashMap();\n  }\n  ctx.threat.group.id = groups;\n  ctx.threat.group.name = groups;\n}\nif (software.size() > 0) {\n  if (ctx.threat.software == null) {\n    ctx.threat.software = new HashMap();\n  }\n  if (ctx.threat.software.type == null) {\n    ctx.threat.software.type = \"Malware\";\n  }\n  if (ctx.threat.software.name == null) {\n    ctx.threat.software.name = new HashMap();\n  }\n  ctx.threat.software.name = software;\n}"#;

    #[test]
    fn one_walk_dedupes_a_member_flattens_a_list_and_folds_a_flag() {
        assert_eq!(
            parse_gather_members(&crate::common::normalise(MANDIANT_SOURCES)),
            Some(
                GatherMembers::new(
                    "json.sources",
                    true,
                    vec![
                        GatheredColumn::new("source_name", "threat.indicator.provider").deduped(),
                        GatheredColumn::new("category", "indicator_categories").flattened(),
                    ],
                )
                .with_flag("osint", "tlp_color", "GREEN", "RED")
            )
        );
    }

    /// The capture's own ipv4 event: five sources, three category lists between
    /// them, and every source OSINT.
    #[test]
    fn the_categories_of_every_source_are_flattened_into_one_list() {
        let mut event = Event::new(json!({ "json": { "sources": [
            { "source_name": "voipbl", "osint": true, "category": [] },
            { "source_name": "greensnow", "osint": true, "category": ["exploit/vuln-scanning", "exploit"] },
            { "source_name": "sblam_blacklist", "osint": true, "category": ["spam/sender", "spam"] },
            { "source_name": "blocklist_net_ua", "osint": true, "category": [] },
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            MANDIANT_SOURCES
        ));
        assert_eq!(
            event.get("indicator_categories"),
            Some(&json!([
                "exploit/vuln-scanning",
                "exploit",
                "spam/sender",
                "spam"
            ]))
        );
        assert_eq!(event.get("tlp_color"), Some(&json!("GREEN")));
    }

    /// The capture's url event: the same source listed twice, and not OSINT.
    /// Without the dedupe the provider came back `["Mandiant","Mandiant"]`, and
    /// without the flag the TLP was never written at all.
    #[test]
    fn a_source_listed_twice_is_one_provider_and_a_private_source_is_red() {
        let mut event = Event::new(json!({ "json": { "sources": [
            { "source_name": "Mandiant", "osint": false, "category": [] },
            { "source_name": "Mandiant", "osint": false, "category": [] },
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            MANDIANT_SOURCES
        ));
        assert_eq!(
            event.get("threat.indicator.provider"),
            Some(&json!(["Mandiant"]))
        );
        assert_eq!(event.get("tlp_color"), Some(&json!("RED")));
        // Every source's category list was empty, so the guarded write never
        // ran and no empty list is left behind.
        assert_eq!(event.get("indicator_categories"), None);
    }

    /// One record that is not OSINT clears the flag for the whole walk, however
    /// many OSINT ones surround it.
    #[test]
    fn one_private_source_among_public_ones_clears_the_flag() {
        let mut event = Event::new(json!({ "json": { "sources": [
            { "source_name": "blocklist_de", "osint": true },
            { "source_name": "Mandiant", "osint": false },
            { "source_name": "phishtank", "osint": true },
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            MANDIANT_SOURCES
        ));
        assert_eq!(event.get("tlp_color"), Some(&json!("RED")));
    }

    #[test]
    fn one_list_is_sorted_into_two_columns_by_a_member_neither_gathers() {
        assert_eq!(
            parse_gather_members(&crate::common::normalise(MANDIANT_ASSOCIATIONS)),
            Some(GatherMembers::new(
                "json.attributed_associations",
                true,
                vec![
                    GatheredColumn::new("name", "threat.group.id")
                        .also_into("threat.group.name")
                        .deduped()
                        .selected_on("type", "threat-actor"),
                    GatheredColumn::new("name", "threat.software.name")
                        .deduped()
                        .selected_on("type", "malware")
                        .with_literal("threat.software.type", "Malware"),
                ],
            ))
        );

        // The capture's md5 event. Ignoring the selector wrote both names into
        // both columns; reading only the last assignment dropped the group id;
        // and the literal beside the list is the software's own type.
        let mut event = Event::new(json!({ "json": { "attributed_associations": [
            { "id": "threat-actor--09b0", "name": "UNC3313", "type": "threat-actor" },
            { "id": "malware--999a", "name": "SMUGPIGEON", "type": "malware" },
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            MANDIANT_ASSOCIATIONS
        ));
        assert_eq!(event.get("threat.group.id"), Some(&json!(["UNC3313"])));
        assert_eq!(event.get("threat.group.name"), Some(&json!(["UNC3313"])));
        assert_eq!(
            event.get("threat.software.name"),
            Some(&json!(["SMUGPIGEON"]))
        );
        assert_eq!(event.get("threat.software.type"), Some(&json!("Malware")));
    }

    /// The literal is stamped only where the column gathered something, because
    /// the script writes it inside the same `size() > 0` guard.
    #[test]
    fn a_column_that_gathers_nothing_stamps_no_literal_either() {
        let mut event = Event::new(json!({ "json": { "attributed_associations": [
            { "name": "UNC961", "type": "threat-actor" },
        ]}}));
        assert!(crate::common::try_known_painless(
            &mut event,
            MANDIANT_ASSOCIATIONS
        ));
        assert_eq!(event.get("threat.group.name"), Some(&json!(["UNC961"])));
        assert_eq!(event.get("threat.software.name"), None);
        assert_eq!(event.get("threat.software.type"), None);
    }
}
