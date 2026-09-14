// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `salesforce`'s apex scripts, transcribed.
//!
//! An Apex event log line arrives as one flat object of upper-case column
//! names, and the whole data stream hangs off lifting those columns onto
//! `salesforce.apex` with the right JSON type. Everything after it -- the
//! action, the duration, the outcome, the URL -- reads what the lift wrote.

use serde_json::{Map, Value};

use super::Entry;
use crate::helpers::painless_to_string;
use dfe_core::event::Event;

/// Where the lifted columns live.
const APEX: &str = "salesforce.apex";

/// The columns stored as floating point, with the name each takes.
const FLOAT_COLUMNS: &[(&str, &str)] = &[
    ("CALLOUT_TIME", "callout_time"),
    ("CPU_TIME", "cpu_time"),
    ("DB_CPU_TIME", "db_cpu_time"),
    ("DB_TOTAL_TIME", "db_total_time"),
    ("EXECUTE_MS", "execute_ms"),
    ("FETCH_MS", "fetch_ms"),
    ("LIMIT_USAGE_PERCENT", "limit_usage_pct"),
    ("THROUGHPUT", "throughput"),
    ("RUN_TIME", "run_time"),
];

/// The columns stored as whole numbers, with the name each takes.
const LONG_COLUMNS: &[(&str, &str)] = &[
    ("DB_BLOCKS", "db_blocks"),
    ("LIMIT", "limit"),
    ("NUMBER_FIELDS", "fields_count"),
    ("NUMBER_SOQL_QUERIES", "soql_queries_count"),
    ("OFFSET", "offset"),
    ("ROWS", "rows_total"),
    ("ROWS_FETCHED", "rows_fetched"),
    ("ROWS_PROCESSED", "rows_processed"),
];

/// The columns carried across as they arrived, with the name each takes.
const RENAMED_COLUMNS: &[(&str, &str)] = &[
    ("ACTION", "action"),
    ("CLASS_NAME", "class_name"),
    ("CLIENT_NAME", "client_name"),
    ("ENTITY", "entity"),
    ("ENTITY_NAME", "entity_name"),
    ("ENTRY_POINT", "entry_point"),
    ("EVENT_TYPE", "event_type"),
    ("FILTER", "filter"),
    ("IS_LONG_RUNNING_REQUEST", "is_long_running_request"),
    ("LOGIN_KEY", "login_key"),
    ("MEDIA_TYPE", "media_type"),
    ("MESSAGE", "message"),
    ("METHOD_NAME", "method_name"),
    ("ORDERBY", "orderby"),
    ("ORGANIZATION_ID", "organization_id"),
    ("QUERY", "query"),
    ("QUIDDITY", "quiddity"),
    ("REQUEST_ID", "request_id"),
    ("REQUEST_STATUS", "request_status"),
    ("SELECT", "select"),
    ("SUBQUERIES", "subqueries"),
    ("TRIGGER_ID", "trigger_id"),
    ("TRIGGER_NAME", "trigger_name"),
    ("TRIGGER_TYPE", "trigger_type"),
    ("TYPE", "type"),
    ("URI", "uri"),
    ("URI_ID_DERIVED", "uri_derived_id"),
    ("USER_AGENT", "user_agent"),
    ("USER_ID_DERIVED", "user_id_derived"),
];

/// `salesforce/apex`, the untagged lifting script: every Apex event-log column
/// the package declares, under `salesforce.apex`.
///
/// The three tables differ only in the type the value lands as -- a float, a
/// whole number, or whatever the column already held. A numeric column is only
/// converted from a non-empty string, so a column the vendor sent as a number
/// already is left out.
///
/// `Float.parseFloat` is 32-bit, and the rounding shows: `DB_TOTAL_TIME`
/// arrives as `81604961` and Elasticsearch stores `81604960.0`, because the
/// nearest float at that magnitude is eight away. [`java_float`] reproduces it.
fn lift_apex_columns(event: &mut Event, _params: &Value) {
    let columns = match event.get("json") {
        Some(Value::Object(json)) => json.clone(),
        _ => Map::new(),
    };
    let mut apex = match event.get(APEX) {
        Some(Value::Object(existing)) => existing.clone(),
        _ => Map::new(),
    };

    for (column, name) in FLOAT_COLUMNS {
        if let Some(text) = non_empty_string(&columns, column)
            && let Some(number) = java_float(text)
        {
            apex.insert((*name).to_owned(), Value::from(number));
        }
    }

    for (column, name) in LONG_COLUMNS {
        if let Some(text) = non_empty_string(&columns, column)
            && let Ok(number) = text.parse::<i64>()
        {
            apex.insert((*name).to_owned(), Value::from(number));
        }
    }

    for (column, name) in RENAMED_COLUMNS {
        if let Some(value) = columns.get(*column).filter(|value| !value.is_null()) {
            apex.insert((*name).to_owned(), value.clone());
        }
    }

    let _ = event.set(APEX, Value::Object(apex));
}

/// `Float.parseFloat` followed by the JSON number Elasticsearch writes for it.
///
/// The parse itself is 32-bit, and a document carries what `Float.toString`
/// then prints -- the shortest decimal that round-trips as a float, which is
/// what Rust's own `f32` rendering gives. Reading that back as an `f64` is the
/// number the capture holds, and a straight 64-bit parse is NOT: it keeps
/// digits the float never had.
fn java_float(text: &str) -> Option<f64> {
    let narrow = text.trim().parse::<f32>().ok()?;
    if !narrow.is_finite() {
        return None;
    }
    format!("{narrow}").parse::<f64>().ok()
}

/// A column's text where it holds a non-empty string, absent otherwise.
fn non_empty_string<'a>(columns: &'a Map<String, Value>, column: &str) -> Option<&'a str> {
    match columns.get(column) {
        Some(Value::String(text)) if !text.is_empty() => Some(text),
        _ => None,
    }
}

/// `salesforce/apex`, the untagged action script: `event.action`, from the
/// pipeline's own table of Apex event types.
fn event_action_from_event_type(event: &mut Event, params: &Value) {
    let Some(event_type) = event.get_str(&path(APEX, "event_type")) else {
        return;
    };
    let event_type = event_type.to_lowercase();
    let Some(action) = params
        .get("event_action_map")
        .and_then(|table| table.get(&event_type))
        .cloned()
    else {
        return;
    };
    let _ = event.set("event.action", action);
}

/// `salesforce/apex`, the untagged ECS script: `event.duration`,
/// `event.outcome` and `event.url`.
///
/// Each of the three reads a different column depending on which Apex event
/// type produced the line, and the duration column is named differently by
/// every one of them.
fn event_fields_from_apex(event: &mut Event, _params: &Value) {
    let columns = match event.get("json") {
        Some(Value::Object(json)) => json.clone(),
        _ => Map::new(),
    };
    let event_type = event.get_string(&path(APEX, "event_type"));
    let event_type = event_type.as_deref();
    let run_time = event.get(&path(APEX, "run_time")).cloned();
    let uri = event.get(&path(APEX, "uri")).cloned();

    if event_type == Some("ApexCallout")
        && let Some(time) = parse_column(&columns, "TIME")
    {
        let _ = event.set("event.duration", time);
    } else if let Some(time) = parse_column(&columns, "EXEC_TIME")
        .filter(|_| event_type == Some("ApexTrigger") || event_type == Some("ApexExecution"))
    {
        let _ = event.set("event.duration", time);
    } else if let Some(time) =
        run_time.filter(|_| event_type == Some("ApexRestApi") || event_type == Some("ApexSoap"))
    {
        let _ = event.set("event.duration", time);
    } else if let Some(time) = parse_column(&columns, "TOTAL_MS")
        .filter(|_| event_type == Some("ExternalCustomApexCallout"))
    {
        let _ = event.set("event.duration", time);
    }

    let success = flag(&columns, "SUCCESS");
    let status = flag(&columns, "STATUS");
    if success == Some(true) || status == Some(true) {
        let _ = event.set("event.outcome", "success");
    } else if success == Some(false) || status == Some(false) {
        let _ = event.set("event.outcome", "failure");
    }

    if let Some(url) = columns
        .get("URL")
        .filter(|value| !value.is_null() && event_type == Some("ApexCallout"))
    {
        let _ = event.set("event.url", url.clone());
    } else if let Some(url) = uri.filter(|value| {
        !value.is_null()
            && event_type != Some("ApexCallout")
            && event_type != Some("ExternalCustomApexCallout")
    }) {
        let _ = event.set("event.url", url);
    }
}

/// A duration column parsed the way `Float.parseFloat` reads it, absent where
/// the column is.
fn parse_column(columns: &Map<String, Value>, column: &str) -> Option<Value> {
    let text = columns.get(column).filter(|value| !value.is_null())?;
    java_float(&painless_to_string(text)).map(Value::from)
}

/// Whether an outcome column reads `1`, absent where the column is.
fn flag(columns: &Map<String, Value>, column: &str) -> Option<bool> {
    let value = columns.get(column).filter(|value| !value.is_null())?;
    Some(value.as_str() == Some("1"))
}

/// `salesforce/apex`, the untagged user-agent script: the browser name behind
/// the three-digit code Salesforce reports.
fn user_agent_from_code(event: &mut Event, params: &Value) {
    let Some(reported) = event.get(&path(APEX, "user_agent")) else {
        return;
    };
    let reported = painless_to_string(reported);
    let code: String = reported.chars().take(3).collect();
    if code.chars().count() < 3 {
        return;
    }
    let Some(name) = params
        .get("user_agent_map")
        .and_then(|table| table.get(&code))
        .cloned()
    else {
        return;
    };
    event.update(&path(APEX, "user_agent"), name);
}

/// One field under `salesforce.apex`.
fn path(prefix: &str, field: &str) -> String {
    format!("{prefix}.{field}")
}

/// Every `salesforce` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "8100a79a487f11754491a15b0585a9f2730f72567e03d9c44467412121247832",
        source: "salesforce",
        name: "lift_apex_columns",
        run: lift_apex_columns,
    },
    Entry {
        hash: "4533268d51d91ac557a9d8ab79ebf8fb934964f183fd095d031017d4e24987ae",
        source: "salesforce",
        name: "event_action_from_event_type",
        run: event_action_from_event_type,
    },
    Entry {
        hash: "91eab36f223e73257eee1b99917b7f503960808d110ba3389cdea86af702cd1d",
        source: "salesforce",
        name: "event_fields_from_apex",
        run: event_fields_from_apex,
    },
    Entry {
        hash: "0ee51f4a072cb6812c2e9c16adffb86373395381249cc60b675f82c07a273ea0",
        source: "salesforce",
        name: "user_agent_from_code",
        run: user_agent_from_code,
    },
];
