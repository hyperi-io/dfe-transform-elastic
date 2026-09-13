// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Members renamed inside every item of a list.
//!
//! Atlassian's cloud streams arrive with the vendor's own key names and are
//! reshaped to match the self-hosted ones before anything else reads them:
//! `typeName` becomes `type`, `fieldName` becomes BOTH `i18nKey` and `key`,
//! `changedTo` and `changedFrom` become `to` and `from`. Every processor after
//! it names the result.
//!
//! Unmatched it costs `atlassian_jira` `jira.audit.affected_objects` on 82
//! events and `jira.audit.changed_values` on 50, which between them unlock 70
//! of its 95 unmatched events, and `atlassian_confluence` the same pair on 47.
//!
//! **The key ORDER is part of the answer, not a detail.** Painless `put`
//! appends and `remove` deletes, so a renamed member lands at the END of its
//! item: the captured `changed_values` reads `i18nKey`, `key`, `to` whatever
//! order the input had. Inserting then removing reproduces that; building a
//! fresh map would not.

use serde_json::{Map, Value};

use dfe_core::Event;

/// One member renamed inside every item of a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemRename {
    /// The member the vendor wrote.
    from: String,
    /// Where it goes. More than one because jira writes `fieldName` to both
    /// `i18nKey` and `key`.
    to: Vec<String>,
}

impl ItemRename {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(from: impl Into<String>, to: Vec<String>) -> Self {
        Self {
            from: from.into(),
            to,
        }
    }
}

/// One list, and the renames applied to every item in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListWalk {
    /// The list's dotted path.
    list: String,
    /// Applied in the order the script applies them.
    renames: Vec<ItemRename>,
}

impl ListWalk {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(list: impl Into<String>, renames: Vec<ItemRename>) -> Self {
        Self {
            list: list.into(),
            renames,
        }
    }
}

/// A list guaranteed to exist, and the one item appended into it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnsureItem {
    /// The list, created empty when absent.
    list: String,
    /// The item appended, when it is present and not already there.
    item: String,
}

impl EnsureItem {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(list: impl Into<String>, item: impl Into<String>) -> Self {
        Self {
            list: list.into(),
            item: item.into(),
        }
    }
}

/// The whole script: an optional preamble, then the walks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItemRenames {
    /// Atlassian's cloud streams carry ONE object where the self-hosted ones
    /// carry a list, so the list is created and the object put into it.
    ensure: Option<EnsureItem>,
    /// One per list walked, in script order.
    lists: Vec<ListWalk>,
}

impl ListItemRenames {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(ensure: Option<EnsureItem>, lists: Vec<ListWalk>) -> Self {
        Self { ensure, lists }
    }

    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    #[must_use]
    pub fn direct_call(&self) -> String {
        let ensure = self.ensure.as_ref().map_or_else(
            || "None".to_owned(),
            |ensure| {
                format!(
                    "Some(EnsureItem::new({:?}, {:?}))",
                    ensure.list, ensure.item
                )
            },
        );
        let lists: Vec<String> = self
            .lists
            .iter()
            .map(|walk| {
                let renames: Vec<String> = walk
                    .renames
                    .iter()
                    .map(|rename| {
                        let to: Vec<String> = rename
                            .to
                            .iter()
                            .map(|to| format!("{to:?}.into()"))
                            .collect();
                        format!(
                            "ItemRename::new({:?}, vec![{}])",
                            rename.from,
                            to.join(", ")
                        )
                    })
                    .collect();
                format!(
                    "ListWalk::new({:?}, vec![{}])",
                    walk.list,
                    renames.join(", ")
                )
            })
            .collect();
        format!(
            "list_item_renames(event, &ListItemRenames::new({ensure}, vec![{}]));",
            lists.join(", "),
        )
    }
}

/// Apply the preamble and every walk.
pub fn list_item_renames(event: &mut Event, pattern: &ListItemRenames) -> bool {
    if let Some(ensure) = &pattern.ensure {
        ensure_item(event, ensure);
    }
    for walk in &pattern.lists {
        let Some(items) = event.get(&walk.list).and_then(Value::as_array).cloned() else {
            continue;
        };
        let rebuilt: Vec<Value> = items
            .into_iter()
            .map(|item| match item {
                Value::Object(members) => Value::Object(rename_members(members, &walk.renames)),
                other => other,
            })
            .collect();
        let _ = event.set(&walk.list, Value::Array(rebuilt));
    }
    true
}

/// Create the list when it is absent, then append the item once.
fn ensure_item(event: &mut Event, ensure: &EnsureItem) {
    if event.get(&ensure.list).and_then(Value::as_array).is_none() {
        let _ = event.set(&ensure.list, Value::Array(Vec::new()));
    }
    let Some(item) = event.get(&ensure.item).cloned() else {
        return;
    };
    let Some(held) = event.get(&ensure.list).and_then(Value::as_array) else {
        return;
    };
    // `contains` on the whole item, which is what the script guards on -- the
    // same object arriving twice must not be appended twice.
    if held.iter().any(|already| already == &item) {
        return;
    }
    let mut appended = held.clone();
    appended.push(item);
    let _ = event.set(&ensure.list, Value::Array(appended));
}

/// Insert each destination, then take the source away.
///
/// In that order, and one rename at a time, because it is what decides the key
/// ORDER: Painless appends on `put` and the captured output carries the
/// renamed members at the end.
fn rename_members(mut members: Map<String, Value>, renames: &[ItemRename]) -> Map<String, Value> {
    for rename in renames {
        let Some(value) = members.get(&rename.from).cloned() else {
            continue;
        };
        for to in &rename.to {
            members.insert(to.clone(), value.clone());
        }
        // `shift_remove`, never `remove`: under `preserve_order` the latter is
        // `swap_remove` and would drop the LAST key into the freed slot.
        members.shift_remove(&rename.from);
    }
    members
}

/// Read the preamble and the walks, or decline.
///
/// Declines on any statement it does not recognise. A walk that renames some
/// members and silently drops another leaves the item half reshaped, and every
/// processor downstream names the result.
#[must_use]
pub fn parse_list_item_renames(script: &str) -> Option<ListItemRenames> {
    let ensure = parse_ensure(script);
    let mut lists = Vec::new();
    let mut from = 0usize;
    while let Some(offset) = script[from..].find("for (def ") {
        let at = from + offset;
        let (walk, end) = parse_walk(script, at)?;
        lists.push(walk);
        from = end;
    }
    if lists.is_empty() {
        return None;
    }
    Some(ListItemRenames::new(ensure, lists))
}

/// `if(ctx.<list> == null) { ArrayList .. } if(ctx.<item> != null && !ctx.<list>.contains(..))`
fn parse_ensure(script: &str) -> Option<EnsureItem> {
    let (head, tail) = script.split_once(".contains(ctx.")?;
    let list = path_before(head, ".contains")?;
    let item = clean(tail.split(')').next()?);
    // The append must be of the same item the guard tested. Compared against
    // the script with its null-safe navigation flattened, because the paths
    // above have been flattened and `ctx.json?.objectItem` would not match.
    if !script
        .replace("?.", ".")
        .contains(&format!(".add(ctx.{item})"))
    {
        return None;
    }
    (!list.is_empty() && !item.is_empty()).then(|| EnsureItem::new(list, item))
}

/// One `for (def j = 0; j < ctx.<list>.length; j++) { .. }` and its renames.
fn parse_walk(script: &str, at: usize) -> Option<(ListWalk, usize)> {
    let header = &script[at..script[at..]
        .find(")\u{20}{")
        .or_else(|| script[at..].find("){"))?
        + at];
    let list = clean(header.split(" < ctx.").nth(1)?.split(".length").next()?);
    if list.is_empty() {
        return None;
    }

    // The body, to the next loop or the end -- the renames all sit inside it.
    let after = at + header.len();
    let end = script[after..]
        .find("for (def ")
        .map_or(script.len(), |next| after + next);
    let body = &script[after..end];

    let mut renames = Vec::new();
    let mut from = 0usize;
    while let Some(offset) = body[from..].find(".put(") {
        let put = from + offset;
        let arguments = body[put + ".put(".len()..].split(')').next()?;
        let (target, source) = arguments.split_once(',')?;
        let target = target.trim().trim_matches(['\'', '"']).to_owned();
        let member = source.trim().rsplit_once('.')?.1.trim().to_owned();
        if target.is_empty() || member.is_empty() {
            return None;
        }
        match renames.last_mut() {
            // jira writes one member to two destinations with two `put` calls.
            Some(ItemRename { from: held, to }) if held == &member => to.push(target),
            _ => renames.push(ItemRename::new(member.clone(), vec![target])),
        }
        // Every rename must take its source away, or the item keeps both.
        if !body.contains(&format!(".remove('{member}')"))
            && !body.contains(&format!(".remove(\"{member}\")"))
        {
            return None;
        }
        from = put + ".put(".len();
    }
    (!renames.is_empty()).then(|| (ListWalk::new(list, renames), end))
}

/// A dotted path, with the null-safe navigation and the `ctx.` prefix gone.
fn clean(term: &str) -> String {
    term.trim()
        .replace("?.", ".")
        .trim_start_matches("ctx.")
        .trim()
        .to_owned()
}

/// The `ctx` path immediately before `marker`.
fn path_before(text: &str, marker: &str) -> Option<String> {
    let head = &text[..text.rfind(marker).unwrap_or(text.len())];
    let raw = &head[head.rfind("ctx.")?..];
    Some(clean(raw))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Atlassian's cloud stream, verbatim from
    /// `pipelines/atlassian_jira/audit/cloud.yml`.
    ///
    /// The captured `changed_values` reads `i18nKey`, `key`, `to` -- the
    /// renamed members at the END, whatever order the input had. That is what
    /// insert-then-remove gives and a rebuilt map would not.
    #[test]
    fn every_item_takes_its_renames_in_key_order() {
        let script = "if(ctx.jira?.audit?.affected_objects == null) {\n    \
            ArrayList items = new ArrayList();\n    \
            ctx.jira?.audit.put(\"affected_objects\", items);\n}\n\
            if(ctx.json?.objectItem != null && \
            !ctx.jira?.audit?.affected_objects.contains(ctx.json?.objectItem)) {\n    \
            ctx.jira?.audit?.affected_objects.add(ctx.json?.objectItem);\n}\n\
            if(ctx.jira?.audit?.affected_objects != null) {\n    \
            for (def j = 0; j < ctx.jira?.audit?.affected_objects.length; j++) {\n        \
            if(ctx.jira.audit.affected_objects[j]?.typeName != null) {\n            \
            ctx.jira.audit.affected_objects[j].put('type', \
            ctx.jira.audit.affected_objects[j].typeName);\n            \
            ctx.jira.audit.affected_objects[j].remove('typeName');\n        }\n    }\n}\n\
            if(ctx.jira?.audit?.changed_values != null) {\n    \
            for (def j = 0; j < ctx.jira?.audit?.changed_values.length; j++) {\n        \
            if(ctx.jira.audit.changed_values[j]?.fieldName != null) {\n            \
            ctx.jira.audit.changed_values[j].put('i18nKey', \
            ctx.jira.audit.changed_values[j].fieldName);\n            \
            ctx.jira.audit.changed_values[j].put('key', \
            ctx.jira.audit.changed_values[j].fieldName);\n            \
            ctx.jira.audit.changed_values[j].remove('fieldName');\n        }\n        \
            if(ctx.jira.audit.changed_values[j]?.changedTo != null) {\n            \
            ctx.jira.audit.changed_values[j].put('to', \
            ctx.jira.audit.changed_values[j].changedTo);\n            \
            ctx.jira.audit.changed_values[j].remove('changedTo');\n        }\n    }\n}";
        let pattern = parse_list_item_renames(script).expect("the reshape is recognised");

        let mut event = Event::new(json!({
            "json": { "objectItem": { "typeName": "User", "name": "test.user" } },
            "jira": { "audit": { "changed_values": [
                { "fieldName": "Name", "changedTo": "Version 3.0" }
            ] } }
        }));
        assert!(list_item_renames(&mut event, &pattern));

        // The single object became the list the self-hosted stream carries.
        let objects = event.get("jira.audit.affected_objects").unwrap();
        assert_eq!(objects.as_array().unwrap().len(), 1);
        assert_eq!(objects[0]["type"], json!("User"));
        assert!(objects[0].get("typeName").is_none());

        // One member to TWO destinations, and the renamed keys land last.
        let changed = event.get("jira.audit.changed_values").unwrap();
        let keys: Vec<&str> = changed[0]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, ["i18nKey", "key", "to"]);
        assert_eq!(changed[0]["i18nKey"], json!("Name"));
        assert_eq!(changed[0]["key"], json!("Name"));
        assert_eq!(changed[0]["to"], json!("Version 3.0"));
    }

    /// Confluence's twin, verbatim from
    /// `pipelines/atlassian_confluence/audit/cloud.yml`. Same job, different
    /// member names throughout and no null-safe navigation on the append --
    /// which is what the reader has to be indifferent to.
    #[test]
    fn the_confluence_twin_reads_the_same_way() {
        let script = "if(ctx.confluence.audit.affected_objects == null) {\n    \
            ArrayList items = new ArrayList();\n    \
            ctx.confluence.audit.put(\"affected_objects\", items);\n}\n\
            if(ctx.json?.affectedObject != null && \
            !ctx.confluence?.audit?.affected_objects.contains(ctx.json?.affectedObject)) {\n    \
            ctx.confluence.audit.affected_objects.add(ctx.json?.affectedObject);\n}\n\
            if(ctx.confluence.audit.affected_objects != null) {\n    \
            for (def j = 0; j < ctx.confluence?.audit?.affected_objects.length; j++) {\n        \
            if(ctx.confluence.audit.affected_objects[j]?.objectType != null) {\n            \
            ctx.confluence.audit.affected_objects[j].put('type', \
            ctx.confluence.audit.affected_objects[j].objectType);\n            \
            ctx.confluence.audit.affected_objects[j].remove('objectType');\n        }\n    }\n}\n\
            if(ctx.confluence.audit.changed_values != null) {\n    \
            for (def j = 0; j < ctx.confluence.audit.changed_values.length; j++) {\n        \
            if(ctx.confluence.audit.changed_values[j]?.name != null) {\n            \
            ctx.confluence.audit.changed_values[j].put('i18nKey', \
            ctx.confluence.audit.changed_values[j].name);\n            \
            ctx.confluence.audit.changed_values[j].put('key', \
            ctx.confluence.audit.changed_values[j].name);\n            \
            ctx.confluence.audit.changed_values[j].remove('name');\n        }\n        \
            if(ctx.confluence.audit.changed_values[j]?.newValue != null) {\n            \
            ctx.confluence.audit.changed_values[j].put('to', \
            ctx.confluence.audit.changed_values[j].newValue);\n            \
            ctx.confluence.audit.changed_values[j].remove('newValue');\n        }\n        \
            if(ctx.confluence.audit.changed_values[j]?.oldValue != null) {\n            \
            ctx.confluence.audit.changed_values[j].put('from', \
            ctx.confluence.audit.changed_values[j].oldValue);\n            \
            ctx.confluence.audit.changed_values[j].remove('oldValue');\n        }\n    }\n}";
        let pattern = parse_list_item_renames(script).expect("the twin is recognised");

        let mut event = Event::new(json!({
            "json": { "affectedObject": { "objectType": "Page", "name": "Home" } },
            "confluence": { "audit": { "changed_values": [
                { "name": "Title", "newValue": "After", "oldValue": "Before" }
            ] } }
        }));
        assert!(list_item_renames(&mut event, &pattern));

        let objects = event.get("confluence.audit.affected_objects").unwrap();
        assert_eq!(objects[0]["type"], json!("Page"));
        assert!(objects[0].get("objectType").is_none());

        let changed = event.get("confluence.audit.changed_values").unwrap();
        let keys: Vec<&str> = changed[0]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, ["i18nKey", "key", "to", "from"]);
        assert_eq!(changed[0]["from"], json!("Before"));
    }

    /// The item is appended once, however often the script runs.
    #[test]
    fn an_item_already_in_the_list_is_not_appended_again() {
        let ensure = EnsureItem::new("a.list", "src.item");
        let pattern = ListItemRenames::new(Some(ensure), vec![ListWalk::new("a.list", vec![])]);
        let mut event = Event::new(json!({
            "a": { "list": [{ "id": 1 }] },
            "src": { "item": { "id": 1 } }
        }));
        assert!(list_item_renames(&mut event, &pattern));
        assert_eq!(event.get("a.list").unwrap().as_array().unwrap().len(), 1);
    }

    /// A rename with no removal beside it is declined: the item would keep both
    /// spellings and ship a field Elasticsearch does not.
    #[test]
    fn a_rename_that_leaves_its_source_behind_is_declined() {
        let script = "if(ctx.a?.b != null) {\n  for (def j = 0; j < ctx.a.b.length; j++) {\n    \
            ctx.a.b[j].put('type', ctx.a.b[j].typeName);\n  }\n}";
        assert!(parse_list_item_renames(script).is_none());
    }
}
