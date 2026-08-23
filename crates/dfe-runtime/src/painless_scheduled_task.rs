// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Windows Security's decoded scheduled-task XML, normalised.
//!
//! The agent decodes the task XML before it arrives, so this reads a MAP
//! rather than parsing anything: `decode_xml` lower-cases element names, groups
//! same-name children into lists, and renders an empty element as an empty
//! string. Which trigger and action names exist, and what each one is called in
//! ECS, comes from the processor's params rather than from here.

use serde_json::{Map, Value, json};

use crate::event::Event;

/// The event codes whose `_tmp.scheduled_task` this normalises.
const CODES: [&str; 5] = ["4698", "4699", "4700", "4701", "4702"];

/// Normalise `winlog._tmp.scheduled_task` into `winlog.scheduled_task`.
///
/// Publishes only once every section has normalised, which is the script's own
/// rule: an empty result leaves the field absent.
pub(crate) fn run(event: &mut Event, params: &Map<String, Value>) -> bool {
    let Some(code) = event
        .get("event.code")
        .map(crate::painless_helpers::painless_to_string)
    else {
        return true;
    };
    if !CODES.contains(&code.as_str()) {
        return true;
    }

    // 4702 carries the NEW content; every other code the current one.
    let member = if code == "4702" {
        "task_content_new"
    } else {
        "task_content"
    };
    let Some(decoded) = event
        .get(&format!("winlog._tmp.scheduled_task.{member}"))
        .and_then(Value::as_object)
        .cloned()
    else {
        return true;
    };
    // decode_xml keeps the document element, and a task that lost it is still
    // the task.
    let task = decoded
        .get("task")
        .and_then(Value::as_object)
        .unwrap_or(&decoded);

    let mut normalized = Map::new();
    if let Some(uri) = task
        .get("registrationinfo")
        .and_then(Value::as_object)
        .and_then(|block| text(block.get("uri")))
    {
        normalized.insert("uri".into(), json!(uri));
    }
    // Each section is size-gated: a block that yielded nothing is left out
    // rather than written empty.
    if let Some(principals) = normalise_principals(task).filter(|found| !found.is_empty()) {
        normalized.insert("principals".into(), Value::Array(principals));
    }
    if let Some(settings) = normalise_settings(task).filter(|found| !found.is_empty()) {
        normalized.insert("settings".into(), Value::Object(settings));
    }
    if let Some(triggers) = normalise_triggers(task, params).filter(|found| !found.is_empty()) {
        normalized.insert("triggers".into(), Value::Array(triggers));
    }
    if let Some(actions) = normalise_actions(task, params).filter(|found| !found.is_empty()) {
        normalized.insert("actions".into(), Value::Array(actions));
    }

    if !normalized.is_empty() {
        let _ = event.set("winlog.scheduled_task", Value::Object(normalized));
    }
    true
}

/// The text of a decoded element: a map carries it under `#text`, a scalar IS
/// it, and an empty string is no value at all.
fn text(value: Option<&Value>) -> Option<String> {
    let raw = match value? {
        Value::Object(map) => map.get("#text")?,
        other => other,
    };
    let text = crate::painless_helpers::painless_to_string(raw);
    (!text.is_empty()).then_some(text)
}

/// `true`/`1` and `false`/`0`, case-insensitively; anything else is no value.
fn bool_value(value: Option<&Value>) -> Option<bool> {
    match text(value)?.to_lowercase().as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

/// One decoded child as a list: same-name elements are already one, a single
/// element is a list of one, and an absent one is empty.
fn as_list(value: Option<&Value>) -> Vec<Value> {
    match value {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Array(items)) => items.clone(),
        Some(other) => vec![other.clone()],
    }
}

fn put_text(target: &mut Map<String, Value>, field: &str, source: &Value, key: &str) {
    if let Some(source) = source.as_object()
        && let Some(value) = text(source.get(key))
    {
        target.insert(field.into(), json!(value));
    }
}

/// A `%XX` escape for the characters the canonical definition uses as its own
/// syntax -- `%`, `|`, `=` -- plus the C0 controls and DEL.
fn escape_definition_value(value: &str) -> String {
    use std::fmt::Write as _;

    let mut escaped = String::with_capacity(value.len());
    for unit in value.encode_utf16() {
        if unit == 37 || unit == 124 || unit == 61 || unit < 32 || unit == 127 {
            // Cannot fail: writing to a String.
            let _ = write!(escaped, "%{unit:02X}");
        } else if let Some(c) = char::from_u32(u32::from(unit)) {
            escaped.push(c);
        }
    }
    escaped
}

/// `v=1|key=value|key=value`, one field per key in the params order, with an
/// absent key contributing its empty `key=`.
fn canonical_definition(values: &Map<String, Value>, keys: &[Value]) -> String {
    let mut definition = String::from("v=1");
    for key in keys {
        let Some(key) = key.as_str() else { continue };
        definition.push('|');
        definition.push_str(key);
        definition.push('=');
        match values.get(key) {
            Some(value) if !value.is_null() => definition.push_str(&escape_definition_value(
                &crate::painless_helpers::painless_to_string(value),
            )),
            _ => {}
        }
    }
    definition
}

fn normalise_principals(task: &Map<String, Value>) -> Option<Vec<Value>> {
    let block = task.get("principals")?.as_object()?;

    let mut principals = Vec::new();
    for value in as_list(block.get("principal")) {
        if !value.is_object() {
            continue;
        }
        let mut principal = Map::new();
        put_text(&mut principal, "id", &value, "id");
        for (key, field) in [
            ("userid", "user"),
            ("groupid", "group"),
            ("logontype", "logon"),
        ] {
            if let Some(found) = text(value.get(key)) {
                // The identifier keys differ from the element names, and the
                // logon one is `type` rather than an identifier at all.
                let member = if field == "logon" {
                    "type"
                } else {
                    "identifier"
                };
                principal.insert(field.into(), json!({ member: found }));
            }
        }
        put_text(&mut principal, "run_level", &value, "runlevel");
        if !principal.is_empty() {
            principals.push(Value::Object(principal));
        }
    }
    Some(principals)
}

fn normalise_settings(task: &Map<String, Value>) -> Option<Map<String, Value>> {
    let block = task.get("settings")?.as_object()?;

    let mut settings = Map::new();
    for key in ["enabled", "hidden"] {
        if let Some(value) = bool_value(block.get(key)) {
            settings.insert(key.into(), json!(value));
        }
    }
    Some(settings)
}

fn normalise_triggers(
    task: &Map<String, Value>,
    params: &Map<String, Value>,
) -> Option<Vec<Value>> {
    let block = task.get("triggers")?.as_object()?;
    let types = params.get("trigger_types")?.as_object()?;
    let keys = params.get("trigger_keys")?.as_array()?;
    let definition_keys = params.get("trigger_definition_keys")?.as_array()?;

    let mut triggers = Vec::new();
    // decode_xml groups children by NAME, so cross-type document order is gone
    // and the params order is what stands in for it.
    for key in keys {
        let Some(key) = key.as_str() else { continue };
        let Some(kind) = types.get(key).and_then(Value::as_str) else {
            continue;
        };
        for value in as_list(block.get(key)) {
            let element = value.as_object();
            // An empty element decodes as an empty scalar and still defines a
            // trigger; a non-empty scalar cannot be one.
            if element.is_none() && text(Some(&value)).is_some() {
                continue;
            }

            let mut trigger = Map::new();
            trigger.insert("type".into(), json!(kind));
            // Task Scheduler defaults an omitted Enabled to true.
            let enabled = element
                .filter(|block| block.contains_key("enabled"))
                .map_or(Some(true), |block| bool_value(block.get("enabled")));
            if let Some(enabled) = enabled {
                trigger.insert("enabled".into(), json!(enabled));
            }

            if let Some(repetition_block) = element
                .and_then(|block| block.get("repetition"))
                .and_then(Value::as_object)
            {
                let mut repetition = Map::new();
                for key in ["interval", "duration"] {
                    if let Some(found) = text(repetition_block.get(key)) {
                        repetition.insert(key.into(), json!(found));
                    }
                }
                if let Some(stop) = bool_value(repetition_block.get("stopatdurationend")) {
                    repetition.insert("stop_at_duration_end".into(), json!(stop));
                }
                if !repetition.is_empty() {
                    trigger.insert("repetition".into(), Value::Object(repetition));
                }
            }

            let mut definition_values = Map::new();
            for key in ["type", "enabled"] {
                if let Some(value) = trigger.get(key) {
                    definition_values.insert(key.into(), value.clone());
                }
            }
            if let Some(repetition) = trigger.get("repetition").and_then(Value::as_object) {
                for key in ["interval", "duration", "stop_at_duration_end"] {
                    if let Some(value) = repetition.get(key) {
                        definition_values.insert(key.into(), value.clone());
                    }
                }
            }
            trigger.insert(
                "definition".into(),
                json!(canonical_definition(&definition_values, definition_keys)),
            );
            triggers.push(Value::Object(trigger));
        }
    }
    Some(triggers)
}

fn normalise_actions(task: &Map<String, Value>, params: &Map<String, Value>) -> Option<Vec<Value>> {
    let block = task.get("actions")?.as_object()?;
    let types = params.get("action_types")?.as_object()?;
    let keys = params.get("action_keys")?.as_array()?;
    let definition_keys = params.get("action_definition_keys")?.as_array()?;

    let context = text(block.get("context"));
    let mut actions = Vec::new();
    for key in keys {
        let Some(key) = key.as_str() else { continue };
        let Some(kind) = types.get(key).and_then(Value::as_str) else {
            continue;
        };
        for value in as_list(block.get(key)) {
            if !value.is_object() {
                continue;
            }
            let mut action = Map::new();
            action.insert("type".into(), json!(kind));
            if let Some(context) = &context {
                action.insert("context".into(), json!(context));
            }
            match kind {
                "exec" => {
                    for (key, field) in [
                        ("command", "command"),
                        ("arguments", "arguments"),
                        ("workingdirectory", "working_directory"),
                    ] {
                        put_text(&mut action, field, &value, key);
                    }
                }
                "com_handler" => put_text(&mut action, "class_id", &value, "classid"),
                _ => {}
            }
            // The action's OWN map is the definition source, so a field added
            // above is in it.
            let definition = canonical_definition(&action, definition_keys);
            action.insert("definition".into(), json!(definition));
            actions.push(Value::Object(action));
        }
    }
    Some(actions)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn params() -> Map<String, Value> {
        json!({
            "action_types": { "exec": "exec", "comhandler": "com_handler" },
            "action_keys": ["exec", "comhandler"],
            "trigger_types": { "boottrigger": "boot", "calendartrigger": "calendar" },
            "trigger_keys": ["boottrigger", "calendartrigger"],
            "action_definition_keys": [
                "type", "context", "command", "arguments", "working_directory", "class_id"
            ],
            "trigger_definition_keys": [
                "type", "enabled", "interval", "duration", "stop_at_duration_end"
            ],
        })
        .as_object()
        .unwrap()
        .clone()
    }

    /// An empty trigger element decodes as an empty STRING and still defines a
    /// trigger, with Task Scheduler's default `enabled`.
    #[test]
    fn an_empty_trigger_element_is_still_a_trigger() {
        let mut event = Event::new(json!({
            "event": { "code": "4698" },
            "winlog": { "_tmp": { "scheduled_task": { "task_content": { "task": {
                "registrationinfo": { "uri": "\\Updater" },
                "triggers": { "boottrigger": "" },
            }}}}},
        }));

        assert!(run(&mut event, &params()));
        assert_eq!(
            event.get_str("winlog.scheduled_task.uri"),
            Some("\\Updater")
        );
        assert_eq!(
            event.get("winlog.scheduled_task.triggers"),
            Some(&json!([{
                "type": "boot",
                "enabled": true,
                "definition": "v=1|type=boot|enabled=true|interval=|duration=|stop_at_duration_end=",
            }]))
        );
    }

    /// The canonical definition escapes its own syntax characters, and an
    /// action reads its definition off the fields it just wrote.
    #[test]
    fn an_action_definition_escapes_the_separator_characters() {
        let mut event = Event::new(json!({
            "event": { "code": "4702" },
            "winlog": { "_tmp": { "scheduled_task": { "task_content_new": {
                "actions": {
                    "context": "Author",
                    "exec": { "command": "cmd.exe", "arguments": "/c a|b=c%d" },
                },
            }}}},
        }));

        assert!(run(&mut event, &params()));
        assert_eq!(
            event.get("winlog.scheduled_task.actions"),
            Some(&json!([{
                "type": "exec",
                "context": "Author",
                "command": "cmd.exe",
                "arguments": "/c a|b=c%d",
                "definition": "v=1|type=exec|context=Author|command=cmd.exe|arguments=/c a%7Cb%3Dc%25d|working_directory=|class_id=",
            }]))
        );
    }

    /// An event code the script does not name leaves the document alone.
    #[test]
    fn another_event_code_writes_nothing() {
        let mut event = Event::new(json!({
            "event": { "code": "4624" },
            "winlog": { "_tmp": { "scheduled_task": { "task_content": { "task": {
                "registrationinfo": { "uri": "\\Updater" },
            }}}}},
        }));

        assert!(run(&mut event, &params()));
        assert!(event.get("winlog.scheduled_task").is_none());
    }
}
