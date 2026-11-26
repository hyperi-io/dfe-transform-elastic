// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Named members gathered out of every record of a list, each into a list of
//! its own, written only where the walk came to something.
//!
//! `ti_threatq` walks its sources once and takes two members out of them, the
//! second filtered against a literal allow-list.
//!
//! `CollectFromList` claims the same loop and reads the member name off the
//! GUARD, so both columns came back named `containsKey`, the allow-list was
//! dropped and each target was written as an empty list. The member is read
//! here off the `add` instead, which is the statement that says what is
//! gathered.
//!
//! The member is read off a SUBSCRIPT or a `.get()`, never a dotted field.
//! gdacs walks its countries the same way over `c.countryname` and writes its
//! three lists unguarded, and that script is `CollectFromList`'s -- taking it
//! here would drop the empty list it is meant to leave behind.
//!
//! An add of a LOCAL is read where the local's declaration cuts the walked
//! record's own member: `ti_threatconnect` labels its security labels
//! `TLP:AMBER` and gathers the half after the colon, testing that piece against
//! the same allow-list `ti_threatq` spells. One transform between the read and
//! the test is a property of the script, not a second pattern.
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Selector {
    key: String,
    value: String,
}

/// A boolean folded across the walk and written as one of two literals.
///
/// mandiant's `is_osint` opens true, goes false at the first source whose
/// `osint` member is present and false, and decides the indicator's TLP.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GatheredFlag {
    /// The member whose presence-and-falsehood clears the flag.
    member: String,
    target: String,
    /// Written when no record cleared the flag, and when one did.
    when_set: String,
    when_clear: String,
}

/// The cut a script takes out of a member before it gathers the piece.
///
/// `ti_threatconnect` labels its security labels `TLP:AMBER` and gathers the
/// half after the colon, which is the spelling its allow-list holds.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Split {
    separator: String,
    index: usize,
}

/// One member gathered, and where it lands.
///
/// The three booleans are independent modifiers of the same gather rather than
/// correlated states -- how the value enters the list, whether it is folded on
/// the way in, and whether a repeat is kept -- so they stay separate flags.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    /// The cut taken out of the member before it is tested and gathered.
    split: Option<Split>,
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
            split: None,
        }
    }

    /// Admit only these literal values.
    #[must_use]
    pub fn allowing(mut self, allowed: &[&str]) -> Self {
        self.allowed = allowed.iter().map(|held| (*held).to_owned()).collect();
        self
    }

    /// Gather the piece at `index` after cutting the member on `separator`.
    #[must_use]
    pub fn cut_on(mut self, separator: impl Into<String>, index: usize) -> Self {
        self.split = Some(Split {
            separator: separator.into(),
            index,
        });
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

/// One append found in the loop body: what it gathers, and how.
struct Append<'a> {
    /// Everything ahead of the append, which carries its guard.
    guard: &'a str,
    /// The record key the gathered value is read from.
    member: String,
    /// `addAll` -- the member is a list flattened in, not a value appended.
    flatten: bool,
    /// `.toLowerCase()` on the way in, which is also the spelling the
    /// allow-list is tested in.
    lowercase: bool,
    /// The cut taken out of the member before it is tested and gathered.
    split: Option<Split>,
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
///
/// The append of a LOCAL is read after these, because a local resolves only
/// where its declaration cuts the walked record's own member -- see
/// [`appended_local`].
fn appended<'a>(body: &'a str, var: &str, accumulator: &str) -> Option<Append<'a>> {
    direct_needles(var, accumulator)
        .iter()
        .find_map(|(needle, closing, flatten)| {
            let (guard, rest) = body.split_once(needle.as_str())?;
            let (member, after) = rest.split_once(*closing)?;
            let member = member.trim().trim_matches(['\'', '"']).to_owned();
            if member.is_empty() || !rest.starts_with(['"', '\'']) {
                return None;
            }
            Some(Append {
                guard,
                member,
                flatten: *flatten,
                // `.toLowerCase()` on the way into the list, which cybereason
                // applies to the value AND to the allow-list test that admits it.
                lowercase: after.trim_start().starts_with(".toLowerCase()"),
                split: None,
            })
        })
        .or_else(|| appended_local(body, var, accumulator))
}

/// The four spellings of an append taken straight off the walked record.
fn direct_needles(var: &str, accumulator: &str) -> [(String, char, bool); 4] {
    [
        (format!("{accumulator}.add({var}["), ']', false),
        (format!("{accumulator}.add({var}.get("), ')', false),
        (format!("{accumulator}.addAll({var}["), ']', true),
        (format!("{accumulator}.addAll({var}.get("), ')', true),
    ]
}

/// Whether the walk appends off the record at all, however the member reads.
///
/// A needle that matches and a member that will not parse is an append this
/// reader cannot resolve, which declines the WHOLE script -- writing some of a
/// walk and not the rest is the thing [`Column::Unreadable`] exists to stop.
fn appends_off_record(body: &str, var: &str, accumulator: &str) -> bool {
    direct_needles(var, accumulator)
        .iter()
        .any(|(needle, _, _)| body.contains(needle.as_str()))
}

/// `<acc>.add(<local>)`, where the local holds a CUT of the record's member.
///
/// `ti_threatconnect` names its security labels `TLP:AMBER` and gathers the
/// half after the colon.
///
/// Same walk, same accumulator, same literal allow-list as `ti_threatq` --
/// one transform between the read and the test. So it is read here rather than
/// given a pattern of its own, which also keeps the call site `direct`.
///
/// The local must be declared from the WALKED record. A local built from
/// anything else is a value this reader cannot resolve, and declining is what
/// stops it gathering the wrong thing.
fn appended_local<'a>(body: &'a str, var: &str, accumulator: &str) -> Option<Append<'a>> {
    for (needle, flatten) in [
        (format!("{accumulator}.add("), false),
        (format!("{accumulator}.addAll("), true),
    ] {
        let Some((guard, rest)) = body.split_once(needle.as_str()) else {
            continue;
        };
        let Some((local, _)) = rest.split_once(')') else {
            continue;
        };
        let local = local.trim();
        if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let Some((member, split, lowercase)) = declared_cut(guard, var, local) else {
            continue;
        };
        return Some(Append {
            guard,
            member,
            flatten,
            lowercase,
            split: Some(split),
        });
    }
    None
}

/// The quoted literal a call opens with, and the text after the closing `)`.
fn quoted_argument(text: &str) -> Option<(String, &str)> {
    let quote @ ('"' | '\'') = text.chars().next()? else {
        return None;
    };
    let (value, after) = text[quote.len_utf8()..].split_once(quote)?;
    Some((value.to_owned(), after.strip_prefix(')')?))
}

/// The record member a local holds, and the cut taken out of it.
///
/// Held to a declaration off the WALKED record --
/// `<local> = <var>.get("<member>").splitOnToken("<sep>")[<index>]`, and the
/// subscript spelling of the same read. The nearest preceding declaration is
/// the one that binds, which is why it is read back from the append.
fn declared_cut(guard: &str, var: &str, local: &str) -> Option<(String, Split, bool)> {
    let (_, rest) = guard.rsplit_once(&format!(" {local} = "))?;
    let rest = rest.trim_start();
    let (member, rest) = [(format!("{var}.get("), ')'), (format!("{var}["), ']')]
        .iter()
        .find_map(|(opening, closing)| {
            let tail = rest.strip_prefix(opening.as_str())?;
            if !tail.starts_with(['"', '\'']) {
                return None;
            }
            let (member, after) = tail.split_once(*closing)?;
            let member = member.trim().trim_matches(['"', '\'']).to_owned();
            (!member.is_empty()).then_some((member, after))
        })?;

    let (separator, rest) = quoted_argument(rest.strip_prefix(".splitOnToken(")?)?;
    if separator.is_empty() {
        return None;
    }
    let (index, rest) = rest.strip_prefix('[')?.split_once(']')?;
    let index: usize = index.trim().parse().ok()?;

    Some((
        member,
        Split { separator, index },
        rest.trim_start().starts_with(".toLowerCase()"),
    ))
}

/// Read one accumulator's column.
fn column(
    script: &str,
    body: &str,
    var: &str,
    accumulator: &str,
    lists: &[(String, Vec<String>)],
) -> Column {
    let Some(append) = appended(body, var, accumulator) else {
        return if appends_off_record(body, var, accumulator) {
            Column::Unreadable
        } else {
            Column::NotGathered
        };
    };
    match read_column(script, &append, var, accumulator, lists) {
        Some((at, read)) => Column::Read(at, Box::new(read)),
        None => Column::Unreadable,
    }
}

/// The gather itself, once the append has been found.
fn read_column(
    script: &str,
    append: &Append<'_>,
    var: &str,
    accumulator: &str,
    lists: &[(String, Vec<String>)],
) -> Option<(usize, GatheredColumn)> {
    let guard = append.guard;

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
        .map_or_default(|(terminal, _)| literal_assignments(terminal));

    Some((
        at,
        GatheredColumn {
            member: append.member.clone(),
            targets: targets.into_iter().map(|(_, path)| path).collect(),
            allowed,
            lowercase: append.lowercase,
            flatten: append.flatten,
            dedupe: arm.contains(&format!("!{accumulator}.contains(")),
            selector: selector(arm, var),
            literals,
            split: append.split.clone(),
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
                // The cut happens BEFORE the test, because the script tests the
                // piece against its allow-list and gathers that piece.
                // A value the separator is absent from reaches no index, which
                // is the record the vendor's own guard skips.
                let value = match (&column.split, value.as_str()) {
                    (Some(split), Some(text)) => {
                        let Some(piece) = text.split(split.separator.as_str()).nth(split.index)
                        else {
                            continue;
                        };
                        Value::String(piece.to_owned())
                    }
                    (Some(_), None) => continue,
                    (None, _) => value.clone(),
                };
                // The fold happens BEFORE the test, because the script tests the
                // folded spelling and gathers the folded value.
                let value = match (column.lowercase, value.as_str()) {
                    (true, Some(text)) => Value::String(text.to_lowercase()),
                    _ => value,
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

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(clippy::needless_raw_string_hashes)]
#[path = "gather_members_tests.rs"]
mod tests;
