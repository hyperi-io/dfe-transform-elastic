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

use serde_json::Value;

use crate::params::{balanced, clean_path};
use dfe_core::Event;

/// One member gathered, and where it lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatheredColumn {
    /// The record key gathered.
    member: String,
    target: String,
    /// The literal values the script admits, empty where it filters nothing.
    allowed: Vec<String>,
}

/// Members gathered out of a list's records into parallel lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatherMembers {
    list: String,
    /// Whether a null element abandons the whole script.
    stop_on_null_element: bool,
    columns: Vec<GatheredColumn>,
}

impl GatherMembers {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        list: impl Into<String>,
        stop_on_null_element: bool,
        columns: Vec<(String, String, Vec<String>)>,
    ) -> Self {
        Self {
            list: list.into(),
            stop_on_null_element,
            columns: columns
                .into_iter()
                .map(|(member, target, allowed)| GatheredColumn {
                    member,
                    target,
                    allowed,
                })
                .collect(),
        }
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

/// Read the list, the members gathered and where each is written, or decline.
#[must_use]
pub fn parse_gather_members(script: &str) -> Option<GatherMembers> {
    let (var, list, after) = walked_list(script)?;
    let (body, tail) = balanced(after.trim_start(), '{', '}')?;
    let lists = literal_lists(script);

    let mut columns = Vec::new();
    for accumulator in accumulators(script) {
        // `<acc>.add(<var>["<member>"])` -- the add says what is gathered, and
        // an add of anything but the walked record's own member is a value
        // this reader cannot resolve.
        let appended = format!("{accumulator}.add({var}[");
        let Some((guard, rest)) = body.split_once(&appended) else {
            continue;
        };
        let (member, _) = rest.split_once(']')?;
        let member = member.trim().trim_matches(['\'', '"']).to_owned();
        if member.is_empty() || !rest.starts_with(['"', '\'']) {
            return None;
        }

        // `ctx.<target> = <acc>;`, guarded on the walk having come to
        // something. Without the guard the script writes an empty list, which
        // is a different answer and a different matcher.
        if !script.contains(&format!("{accumulator}.size() > 0")) {
            return None;
        }
        let assignment = format!("= {accumulator};");
        let (written, _) = script.rsplit_once(&assignment)?;
        let target = clean_path(
            written
                .trim()
                .rsplit(['\n', ' ', '{', '}', ';'])
                .next()?
                .trim()
                .strip_prefix("ctx")?
                .trim_start_matches(['?', '.']),
        );
        if target.is_empty() || target.contains(['(', '[']) {
            return None;
        }

        // The allow-list is whichever literal list the arm's own guard tests.
        let arm = guard.rsplit_once("if (").map_or(guard, |(_, arm)| arm);
        let allowed = lists
            .iter()
            .find(|(name, _)| arm.contains(&format!("{name}.contains(")))
            .map(|(_, members)| members.clone())
            .unwrap_or_default();

        columns.push((
            script.rfind(&assignment).unwrap_or(usize::MAX),
            GatheredColumn {
                member,
                target,
                allowed,
            },
        ));
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
        let mut gathered = Vec::new();
        for record in &records {
            let Some(value) = record.get(&column.member).filter(|v| !v.is_null()) else {
                continue;
            };
            if !column.allowed.is_empty()
                && !value
                    .as_str()
                    .is_some_and(|text| column.allowed.iter().any(|held| held == text))
            {
                continue;
            }
            gathered.push(value.clone());
        }
        if !gathered.is_empty() {
            let _ = event.set(&column.target, Value::Array(gathered));
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
                    (
                        "tlp_name".to_owned(),
                        "threat.indicator.marking.tlp".to_owned(),
                        ["WHITE", "GREEN", "AMBER", "RED", "CLEAR", "AMBER+STRICT"]
                            .map(str::to_owned)
                            .to_vec(),
                    ),
                    (
                        "name".to_owned(),
                        "threat.indicator.provider".to_owned(),
                        Vec::new(),
                    ),
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
                vec![("name".to_owned(), "related.user".to_owned(), Vec::new())],
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
}
