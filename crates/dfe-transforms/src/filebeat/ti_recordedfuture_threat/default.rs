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

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.dataset", json!("ti_recordedfuture.threat"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            event.set("threat.feed.name", json!("Recorded Future"))?;

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

            let _cond = {
                event.has_value("event.original")
                    && event
                        .get_str("event.original")
                        .is_some_and(|s| s.starts_with("{"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "json")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: (format!(
                            "Failed decoding message field as JSON: {}",
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ))
                        .to_string(),
                    });
                }
            }

            let _cond = { !event.has_value("json") };
            if _cond {
                // on_failure: 1 handler(s)
                let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                    // Begin nested pipeline: "decode_csv"
                    if let Some(csv_str) = event.get_string("event.original") {
                        let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                        let mut rdr = csv::ReaderBuilder::new()
                            .delimiter(b',')
                            .quote(b'\"')
                            .has_headers(false)
                            .from_reader(csv_str.as_bytes());
                        if let Some(Ok(record)) = rdr.records().next() {
                            if let Some(val) = record.get(0) {
                                if !val.is_empty() {
                                    event.set("_tmp_.col0", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                if !val.is_empty() {
                                    event.set("_tmp_.col1", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("_tmp_.col2", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("_tmp_.col3", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("_tmp_.col4", val)?;
                                }
                            }
                        }
                    }
                    let _cond = { event.get_str("_tmp_.col0") == Some("Name") };
                    if _cond {
                        return Ok(TransformResult::Drop);
                    }
                    // Painless script
                    // Source: def cols = params[ ctx._tmp_.col4 == null? \"default\" : \"hash\" ]; def src = ctx._tmp_; def dst = new HashMap(); for (entry in cols.entrySet()) {\n  dst[entry.getValue()] = src[entry.getKey()];\n} ctx['json'] = dst;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def cols = params[ ctx._tmp_.col4 == null? \"default\" : \"hash\" ]; def src = ctx._tmp_; def dst = new HashMap(); for (entry in cols.entrySet()) {\n  dst[entry.getValue()] = src[entry.getKey()];\n} ctx['json'] = dst;\n"#
                        ),
                        cached_params!(
                            "{\"default\":{\"col0\":\"Name\",\"col1\":\"Risk\",\"col2\":\"RiskString\",\"col3\":\"EvidenceDetails\"},\"hash\":{\"col0\":\"Name\",\"col1\":\"Algorithm\",\"col2\":\"Risk\",\"col3\":\"RiskString\",\"col4\":\"EvidenceDetails\"}}"
                        ),
                    )?;
                    if event.remove("_tmp_").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_tmp_".into(),
                        });
                    }
                    // End nested pipeline: "decode_csv"
                    Ok(TransformResult::Continue)
                })(event);
                if let Ok(TransformResult::Drop) = outcome {
                    return Ok(TransformResult::Drop);
                }
                if let Err(err) = outcome {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: (format!(
                            "Failed decoding message field as CSV: {}",
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ))
                        .to_string(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "json.EvidenceDetails", "_temp_.EvidenceDetails")?;
                Ok(())
            })();

            if event.has_value("_temp_.EvidenceDetails.EvidenceDetails") {
                event.rename(
                    "_temp_.EvidenceDetails.EvidenceDetails",
                    "json.evidence_details",
                )?;
            }

            if event.has_value("json.evidence_details") {
                foreach_array(event, "json.evidence_details", |event| {
                    event.append(
                        "_temp_.providers",
                        json!(
                            event
                                .get("_ingest._value.EvidenceString")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("_temp_.providers") {
                foreach_array(event, "_temp_.providers", |event| {
                    gsub_field(
                        event,
                        "_ingest._value",
                        "_ingest._value",
                        cached_regex!("^(?:.+sources?(?: including)?: (.*?)(?:\\. |$))|(.*)$"),
                        "$1",
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("_temp_.providers") {
                foreach_array(event, "_temp_.providers", |event| {
                    if let Some(s) = event.get_string("_ingest._value") {
                        let mut parts: Vec<Value> = cached_regex!(", *")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("_ingest._value", Value::Array(parts))?;
                    }
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx._temp_ != null && ctx._temp_.providers != null) {\n  ctx._temp_.providers = ctx._temp_.providers.stream()\n    .filter(p -> p != null && p.size() > 0 && !p.get(0).isEmpty())\n    .collect(Collectors.toList());\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx._temp_ != null && ctx._temp_.providers != null) {\n  ctx._temp_.providers = ctx._temp_.providers.stream()\n    .filter(p -> p != null && p.size() > 0 && !p.get(0).isEmpty())\n    .collect(Collectors.toList());\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("_temp_.providers") {
                foreach_array(event, "_temp_.providers", |event| {
                    if event.has_value("_ingest._value") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("_ingest._value").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    event.append_unique(
                                        "threat.indicator.provider",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
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
                                    "_ingest._value",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event.get("json.evidence_details").is_some_and(|v| v.is_array()) && event.get("json.evidence_details").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def out = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n\n  return out;\n}\nList evidence_details = new ArrayList();\nfor (evidence in ctx.json.evidence_details){\n  evidence_details.add(keysToSnakeCase(evidence));\n}\nctx.json.evidence_details = evidence_details;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def out = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n\n  return out;\n}\nList evidence_details = new ArrayList();\nfor (evidence in ctx.json.evidence_details){\n  evidence_details.add(keysToSnakeCase(evidence));\n}\nctx.json.evidence_details = evidence_details;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script-evidence_details_snakecase",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.evidence_details") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.evidence_details").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.timestamp")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.timestamp", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.timestamp".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_time")?;
                                event.remove("_ingest._value.timestamp");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
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
                            "json.evidence_details",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.has_value("json.Algorithm") };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.has_value("json.Algorithm") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def key = params[ctx.json.Algorithm]; if (key == null) {\n  throw new Exception(\"Unsupported hash algorithm '\" + ctx.json.Algorithm + \"'\");\n} def hashes = [key:ctx.json.Name]; ctx[\"_hashes\"] = hashes;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def key = params[ctx.json.Algorithm]; if (key == null) {\n  throw new Exception(\"Unsupported hash algorithm '\" + ctx.json.Algorithm + \"'\");\n} def hashes = [key:ctx.json.Name]; ctx[\"_hashes\"] = hashes;"#
                        ),
                        cached_params!(
                            "{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\",\"SHA-384\":\"sha384\",\"SHA-512\":\"sha512\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to map fileHashes field: {}",
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("_hashes") {
                event.rename("_hashes", "threat.indicator.file.hash")?;
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.Name") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Name".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.ip", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("threat.indicator.ip")
                    && !(event.get("threat.indicator.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = {
                event.has_value("threat.indicator.ip")
                    && event.get("threat.indicator.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                !event.has_value("threat.indicator.type")
                    && event.get("json.Name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/")),
                        serde_json::Value::String(s) => s.contains("/"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(event, "json.Name", "threat.indicator.url", true, false)?
                        && event
                            .get_str("json.Name")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "json.Name".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                    if let Some(v) = event.get("json.Name").cloned() {
                        event.set("threat.indicator.url.original", v)?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                if let Some(v) = event.get("json.Name").cloned() {
                    event.set("threat.indicator.url.full", v)?;
                }
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("domain-name")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                let v = json!(
                    event
                        .get("json.Name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.domain", v)?;
                }
            }

            let _cond = {
                event.get("json.evidence_details").is_some_and(|v| v.is_array()) && event.get("json.evidence_details").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def sum_sources_count = 0;\nfor (evidence in ctx.json.evidence_details){\n  if (evidence['sources_count'] != null){\n    sum_sources_count += evidence['sources_count'];\n  }\n}\nctx.threat.indicator.scanner_stats = sum_sources_count;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def sum_sources_count = 0;\nfor (evidence in ctx.json.evidence_details){\n  if (evidence['sources_count'] != null){\n    sum_sources_count += evidence['sources_count'];\n  }\n}\nctx.threat.indicator.scanner_stats = sum_sources_count;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script-scanner_stats")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.get("json.evidence_details").is_some_and(|v| v.is_array()) && event.get("json.evidence_details").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def sum_sightings_count = 0;\nfor (evidence in ctx.json.evidence_details){\n  if (evidence['sightings_count'] != null){\n    sum_sightings_count += evidence['sightings_count'];\n  }\n}\nctx.threat.indicator.sightings = sum_sightings_count;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def sum_sightings_count = 0;\nfor (evidence in ctx.json.evidence_details){\n  if (evidence['sightings_count'] != null){\n    sum_sightings_count += evidence['sightings_count'];\n  }\n}\nctx.threat.indicator.sightings = sum_sightings_count;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script-sightings")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.Risk") {
                    if let Some(val) = event.get("json.Risk") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Risk".into(),
                                message,
                            }
                        })?;
                        event.set("event.risk_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Risk score `{}` cannot be converted to float: {}",
                        event
                            .get("json.Risk")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.RiskString") {
                event.rename("json.RiskString", "json.risk_string")?;
            }

            event.rename("json.Name", "json.name")?;

            event.rename("json", "recordedfuture")?;

            let _cond = { event.has_value("_conf.list") };
            if _cond {
                event.rename("_conf.list", "recordedfuture.list")?;
            }

            event.remove("recordedfuture.Algorithm");
            event.remove("recordedfuture.EvidenceDetails");
            event.remove("recordedfuture.Name");
            event.remove("recordedfuture.Risk");
            event.remove("_temp_");
            event.remove("_conf");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
