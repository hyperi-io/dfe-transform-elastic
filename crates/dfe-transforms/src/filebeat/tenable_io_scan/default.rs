// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.category", Value::Array(vec![json!("configuration")]))?;

            let _cond = { event.get_str("event.reason") == Some("want_more") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "json")?;

            event.set("event.kind", json!("state"))?;

            let _cond = {
                event.has_value("json.creation_date")
                    && event.get_str("json.creation_date") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.creation_date") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("json.creation_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.creation_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.last_modification_date")
                    && event.get_str("json.last_modification_date") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.last_modification_date") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("json.last_modification_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_modification_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.starttime") && event.get_str("json.starttime") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.starttime") {
                    match parse_date_out(&date_str, &["yyyyMMdd'T'HHmmss"], None, None) {
                        Some(parsed) => event.set("json.starttime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.starttime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.scan_details.info.scan_start")
                    && event.get_str("json.scan_details.info.scan_start") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.scan_details.info.scan_start") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("json.scan_details.info.scan_start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.scan_details.info.scan_start".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.scan_details.info.scan_end")
                    && event.get_str("json.scan_details.info.scan_end") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.scan_details.info.scan_end") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("json.scan_details.info.scan_end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.scan_details.info.scan_end".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.scan_details.info.timestamp")
                    && event.get_str("json.scan_details.info.timestamp") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.scan_details.info.timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("json.scan_details.info.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.scan_details.info.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.scan_details.history") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.scan_details.history").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creation_date")
                                {
                                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.creation_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creation_date".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "json.scan_details.history",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.scan_details.history") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.scan_details.history").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.last_modification_date")
                                {
                                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                                        Some(parsed) => event
                                            .set("_ingest._value.last_modification_date", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_modification_date"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "json.scan_details.history",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event.has_value("json.scan_details.info.scanner_start")
                    && event.get_str("json.scan_details.info.scanner_start") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.scan_details.info.scanner_start")
                    {
                        match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("json.scan_details.info.scanner_start", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.scan_details.info.scanner_start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.scan_details.info.scanner_end")
                    && event.get_str("json.scan_details.info.scanner_end") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.scan_details.info.scanner_end")
                    {
                        match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("json.scan_details.info.scanner_end", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.scan_details.info.scanner_end".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            event.remove("json.scan_details.filters");

            if event.has_value("json") {
                event.rename("json", "tenable_io.scan")?;
            }

            let _cond = { event.has_value("tenable_io.scan.scan_details") };
            if _cond {
                // Painless script
                // Source: def details = ctx.tenable_io.scan.scan_details;\ndef ipv4Pat = /^(25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]?\\d)(\\.(25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]?\\d)){3}$/;\ndef ipv6Pat = /^[0-9A-Fa-f:]*:[0-9A-Fa-f:.]*$/;\nif (ctx.related == null) { ctx.related = new HashMap(); }\nif (details.info?.targets instanceof String) {\n  for (def t : details.info.targets.splitOnToken(',')) {\n    def v = t.trim();\n    if (v.length() == 0) { continue; }\n    def base = v;\n    int slash = base.indexOf('/');\n    if (slash >= 0) { base = base.substring(0, slash); }\n    int dash = base.indexOf('-');\n    if (dash > 0 && base.indexOf(':') < 0) { base = base.substring(0, dash); }\n    base = base.trim();\n    if (ipv4Pat.matcher(base).matches() || ipv6Pat.matcher(base).matches()) {\n      if (ctx.related.ip == null) { ctx.related.ip = new ArrayList(); }\n      if (!ctx.related.ip.contains(base)) { ctx.related.ip.add(base); }\n    } else {\n      if (ctx.related.hosts == null) { ctx.related.hosts = new ArrayList(); }\n      if (!ctx.related.hosts.contains(v)) { ctx.related.hosts.add(v); }\n    }\n  }\n}\nif (details.hosts instanceof List) {\n  for (def host : (List) details.hosts) {\n    if (host?.hostname instanceof String && host.hostname.length() > 0) {\n      def v = host.hostname;\n      def base = v;\n      int slash = base.indexOf('/');\n      if (slash >= 0) { base = base.substring(0, slash); }\n      int dash = base.indexOf('-');\n      if (dash > 0 && base.indexOf(':') < 0) { base = base.substring(0, dash); }\n      base = base.trim();\n      if (ipv4Pat.matcher(base).matches() || ipv6Pat.matcher(base).matches()) {\n        if (ctx.related.ip == null) { ctx.related.ip = new ArrayList(); }\n        if (!ctx.related.ip.contains(base)) { ctx.related.ip.add(base); }\n      } else {\n        if (ctx.related.hosts == null) { ctx.related.hosts = new ArrayList(); }\n        if (!ctx.related.hosts.contains(v)) { ctx.related.hosts.add(v); }\n      }\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def details = ctx.tenable_io.scan.scan_details;\ndef ipv4Pat = /^(25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]?\\d)(\\.(25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]?\\d)){3}$/;\ndef ipv6Pat = /^[0-9A-Fa-f:]*:[0-9A-Fa-f:.]*$/;\nif (ctx.related == null) { ctx.related = new HashMap(); }\nif (details.info?.targets instanceof String) {\n  for (def t : details.info.targets.splitOnToken(',')) {\n    def v = t.trim();\n    if (v.length() == 0) { continue; }\n    def base = v;\n    int slash = base.indexOf('/');\n    if (slash >= 0) { base = base.substring(0, slash); }\n    int dash = base.indexOf('-');\n    if (dash > 0 && base.indexOf(':') < 0) { base = base.substring(0, dash); }\n    base = base.trim();\n    if (ipv4Pat.matcher(base).matches() || ipv6Pat.matcher(base).matches()) {\n      if (ctx.related.ip == null) { ctx.related.ip = new ArrayList(); }\n      if (!ctx.related.ip.contains(base)) { ctx.related.ip.add(base); }\n    } else {\n      if (ctx.related.hosts == null) { ctx.related.hosts = new ArrayList(); }\n      if (!ctx.related.hosts.contains(v)) { ctx.related.hosts.add(v); }\n    }\n  }\n}\nif (details.hosts instanceof List) {\n  for (def host : (List) details.hosts) {\n    if (host?.hostname instanceof String && host.hostname.length() > 0) {\n      def v = host.hostname;\n      def base = v;\n      int slash = base.indexOf('/');\n      if (slash >= 0) { base = base.substring(0, slash); }\n      int dash = base.indexOf('-');\n      if (dash > 0 && base.indexOf(':') < 0) { base = base.substring(0, dash); }\n      base = base.trim();\n      if (ipv4Pat.matcher(base).matches() || ipv6Pat.matcher(base).matches()) {\n        if (ctx.related.ip == null) { ctx.related.ip = new ArrayList(); }\n        if (!ctx.related.ip.contains(base)) { ctx.related.ip.add(base); }\n      } else {\n        if (ctx.related.hosts == null) { ctx.related.hosts = new ArrayList(); }\n        if (!ctx.related.hosts.contains(v)) { ctx.related.hosts.add(v); }\n      }\n    }\n  }\n}\n"#
                    ),
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
