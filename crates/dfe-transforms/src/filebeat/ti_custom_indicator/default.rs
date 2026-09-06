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
            let _cond = { event.has_value("error.message") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            let _cond = { event.has_value("_conf.feed_name") };
            if _cond {
                if let Some(v) = event.get("_conf.feed_name").cloned() {
                    event.set("threat.feed.name", v)?;
                }
            }

            let _cond = { event.has_value("_conf.feed_reference") };
            if _cond {
                if let Some(v) = event.get("_conf.feed_reference").cloned() {
                    event.set("threat.feed.reference", v)?;
                }
            }

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

            parse_json_field(event, "event.original", "stix")?;

            let _cond = {
                event.get_str("stix.type") != Some("indicator")
                    && event.get_bool("_conf.restrict_stix") == Some(true)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.get_str("stix.spec_version") != Some("2.1")
                    && event.get_bool("_conf.restrict_stix") == Some(true)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("stix.created") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("stix.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("stix.pattern") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("stix.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("stix.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "stix.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("stix.modified") };
            if _cond {
                if let Some(date_str) = event.get_as_string("stix.modified") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "stix.modified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.last_seen") };
            if _cond {
                if let Some(v) = event.get("threat.indicator.last_seen").cloned() {
                    event.set("threat.indicator.modified_at", v)?;
                }
            }

            let _cond = { event.has_value("stix.valid_from") };
            if _cond {
                if let Some(date_str) = event.get_as_string("stix.valid_from") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "stix.valid_from".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("stix.name") {
                event.rename("stix.name", "threat.indicator.name")?;
            }

            if event.has_value("stix.description") {
                event.rename("stix.description", "threat.indicator.description")?;
            }

            let _cond = { event.has_value("stix.confidence") };
            if _cond {
                // Painless script
                // Source: def value = ctx.stix.confidence; if (value == 0) {\n  ctx.threat.indicator.confidence = \"None\";\n} else if (value >= 1 && value <= 29) {\n  ctx.threat.indicator.confidence = \"Low\";\n} else if (value >= 30 && value <= 69) {\n  ctx.threat.indicator.confidence = \"Medium\";\n} else if (value >= 70 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n} else {\n  ctx.threat.indicator.confidence = \"Not Specified\";\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def value = ctx.stix.confidence; if (value == 0) {\n  ctx.threat.indicator.confidence = \"None\";\n} else if (value >= 1 && value <= 29) {\n  ctx.threat.indicator.confidence = \"Low\";\n} else if (value >= 30 && value <= 69) {\n  ctx.threat.indicator.confidence = \"Medium\";\n} else if (value >= 70 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n} else {\n  ctx.threat.indicator.confidence = \"Not Specified\";\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("stix.labels") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("stix.labels") {
                        foreach_array(event, "stix.labels", |event| {
                            event.append(
                                "tags",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            event.remove("stix.labels");

            if event.has_value("tags") {
                gsub_field(event, "tags", "tags", cached_regex!("\\\\\\\""), "\\\"")?;
            }

            let _cond = { event.has_value("stix.valid_until") };
            if _cond {
                if let Some(date_str) = event.get_as_string("stix.valid_until") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("stix.ioc_expiration_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "stix.valid_until".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                !event.has_value("stix.ioc_expiration_date")
                    && event.has_value("stix.revoked")
                    && event.get_bool("stix.revoked") == Some(true)
            };
            if _cond {
                if let Some(v) = event.get("threat.indicator.modified_at").cloned() {
                    event.set("stix.ioc_expiration_date", v)?;
                }
            }

            let _cond = {
                !event.has_value("stix.ioc_expiration_date")
                    && event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ZonedDateTime ioc_expiration_date; ZonedDateTime updated_at; def dur = ctx._conf.ioc_expiration_duration;\nif (ctx.threat.indicator.last_seen != null) {\n  updated_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n  updated_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n}\nif (dur instanceof String){\n  String time_unit = dur.substring(dur.length() -  1, dur.length());\n  String time_value = dur.substring(0, dur.length() - 1);\n  if (time_unit == 'd') {\n    ioc_expiration_date = updated_at.plusDays(Long.parseLong(time_value));\n  } else if (time_unit == 'h') {\n    ioc_expiration_date = updated_at.plusHours(Long.parseLong(time_value));\n  } else if (time_unit == 'm') {\n    ioc_expiration_date = updated_at.plusMinutes(Long.parseLong(time_value));\n  } else {\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n    ioc_expiration_date = updated_at.plusDays(90L);\n  }\n} else {\n    ioc_expiration_date = updated_at.plusDays(90L);\n}\nctx.stix.ioc_expiration_date = ioc_expiration_date;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime ioc_expiration_date; ZonedDateTime updated_at; def dur = ctx._conf.ioc_expiration_duration;\nif (ctx.threat.indicator.last_seen != null) {\n  updated_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n  updated_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n}\nif (dur instanceof String){\n  String time_unit = dur.substring(dur.length() -  1, dur.length());\n  String time_value = dur.substring(0, dur.length() - 1);\n  if (time_unit == 'd') {\n    ioc_expiration_date = updated_at.plusDays(Long.parseLong(time_value));\n  } else if (time_unit == 'h') {\n    ioc_expiration_date = updated_at.plusHours(Long.parseLong(time_value));\n  } else if (time_unit == 'm') {\n    ioc_expiration_date = updated_at.plusMinutes(Long.parseLong(time_value));\n  } else {\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n    ioc_expiration_date = updated_at.plusDays(90L);\n  }\n} else {\n    ioc_expiration_date = updated_at.plusDays(90L);\n}\nctx.stix.ioc_expiration_date = ioc_expiration_date;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_conf_ioc_expiration",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("stix.ioc_expiration_date") };
            if _cond {
                // Painless script
                // Source: if (ctx.stix.valid_until != null) {\n  ctx.stix.ioc_expiration_reason = params.valid_until;\n} else if (ctx.stix.revoked != null && ctx.stix.revoked == true) {\n  ctx.stix.ioc_expiration_reason = params.revoked;\n} else {\n  ctx.stix.ioc_expiration_reason = params.default;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.stix.valid_until != null) {\n  ctx.stix.ioc_expiration_reason = params.valid_until;\n} else if (ctx.stix.revoked != null && ctx.stix.revoked == true) {\n  ctx.stix.ioc_expiration_reason = params.revoked;\n} else {\n  ctx.stix.ioc_expiration_reason = params.default;\n}\n"#
                    ),
                    cached_params!(
                        "{\"valid_until\":\"Expiration set from valid_until field\",\"revoked\":\"Expiration set from revoked field\",\"default\":\"Expiration set by Elastic from the integration's parameter `IOC Expiration Duration`\"}"
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("stix.ioc_expiration_date") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("stix.ioc_expiration_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "stix.ioc_expiration_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "date_ioc_expiration_date",
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
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "Processor {} with tag {} in pipeline {} failed",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            if event.has_value("_conf.ioc_expiration_duration") {
                event.rename(
                    "_conf.ioc_expiration_duration",
                    "stix.ioc_expiration_duration",
                )?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[autonomous-system"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("autonomous-system"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[domain-name"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[email"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[file"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[ipv4-addr"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[ipv6-addr"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[url"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[windows-registry-key"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("windows-registry-key"))?;
            }

            let _cond = {
                event
                    .get_str("stix.pattern")
                    .is_some_and(|s| s.starts_with("[x509"))
            };
            if _cond {
                event.set("threat.indicator.type", json!("x509-certificate"))?;
            }

            let _cond =
                { event.has_value("threat.indicator.type") && event.has_value("stix.pattern") };
            if _cond {
                if let Some(s) = event.get_string("stix.pattern") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+AND\\s+|\\s+OR\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("stix._patterns", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("autonomous-system") };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-asn"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: ^\\[?autonomous-system:number%{SPACE}=%{SPACE}%{INT:_tmp.as_number}\\]?
                                        // Grok pattern: ^\\[?autonomous-system:number%{SPACE}=%{SPACE}'%{INT:_tmp.as_number}'\\]?
                                        if !extract_first_match(
                                            &[
                                                cached_grok!(
                                                    "^\\[?autonomous-system:number%{SPACE}=%{SPACE}%{INT:_tmp.as_number}\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?autonomous-system:number%{SPACE}=%{SPACE}'%{INT:_tmp.as_number}'\\]?"
                                                ),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.as_number") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.as.number",
                                        json!(
                                            event
                                                .get("_tmp.as_number")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-asn"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            if event.has_value("threat.indicator.as.number") {
                foreach_array(event, "threat.indicator.as.number", |event| {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "integer").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("domain-name") };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-domain-name"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: ^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?
                                        // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                                        // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?
                                        // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                                        // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?
                                        // Grok pattern: ^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                                        if !extract_first_match(
                                            &[
                                                cached_grok!(
                                                    "^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"
                                                ),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.url") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.url.original",
                                        json!(
                                            event
                                                .get("_tmp.url")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.ip") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.ip",
                                        json!(
                                            event
                                                .get("_tmp.ip")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.ip") };
                                if _cond {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_tmp.ip")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-domain-name"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-email"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: ^\\[?email-addr:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                                        // Grok pattern: ^\\[?email-message:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                                        // Grok pattern: ^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                                        // Grok pattern: ^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?
                                        if !extract_first_match(
                                            &[
                                                cached_grok!(
                                                    "^\\[?email-addr:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?email-message:value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?email-message:from_ref.value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?email-message:to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.email_addr}'\\]?"
                                                ),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.email_addr") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.email.address",
                                        json!(
                                            event
                                                .get("_tmp.email_addr")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-email"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("file") };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-file"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: (?i:^\\[?file:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?)
                                        // Grok pattern: (?i:^\\[?file:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?)
                                        // Grok pattern: (?i:^\\[?file:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?)
                                        // Grok pattern: ^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:_tmp.filename}'\\]?
                                        if !extract_first_match(
                                            &[
                                                cached_grok!(
                                                    "(?i:^\\[?file:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?)"
                                                ),
                                                cached_grok!(
                                                    "(?i:^\\[?file:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?)"
                                                ),
                                                cached_grok!(
                                                    "(?i:^\\[?file:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?)"
                                                ),
                                                cached_grok!(
                                                    "^\\[?file:name%{SPACE}=%{SPACE}'%{DATA:_tmp.filename}'\\]?"
                                                ),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.md5") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.file.hash.md5",
                                        json!(
                                            event
                                                .get("_tmp.md5")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha1") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.file.hash.sha1",
                                        json!(
                                            event
                                                .get("_tmp.sha1")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha256") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.file.hash.sha256",
                                        json!(
                                            event
                                                .get("_tmp.sha256")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.filename") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.file.name",
                                        json!(
                                            event
                                                .get("_tmp.filename")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.md5") };
                                if _cond {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_tmp.md5")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha1") };
                                if _cond {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_tmp.sha1")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha256") };
                                if _cond {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_tmp.sha256")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-file"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get_str("threat.indicator.type")
                    .is_some_and(|s| s.starts_with("ip"))
            };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-ip"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                                        // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?
                                        // Grok pattern: ^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?
                                        if !extract_first_match(
                                            &[
                                                cached_grok!(
                                                    "^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?"
                                                ),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.ip") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.ip",
                                        json!(
                                            event
                                                .get("_tmp.ip")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.ip") };
                                if _cond {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_tmp.ip")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-ip"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-url"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: ^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?
                                        if !cached_grok!(
                                            "^\\[?url:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?"
                                        )
                                        .extract_into(&input, event)?
                                        {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.url") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.url.original",
                                        json!(
                                            event
                                                .get("_tmp.url")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-url"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("windows-registry-key") };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-windows-registry"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: ^\\[?windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_path}'\\]?
                                        // Grok pattern: ^\\[?windows-registry-key:key%{SPACE}LIKE%{SPACE}'%{DATA:_tmp.reg_path}'\\]?
                                        // Grok pattern: ^\\[?windows-registry-value-type:name%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_key}'\\]?
                                        // Grok pattern: ^\\[?windows-registry-value-type:data%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_value}'\\]?
                                        if !extract_first_match(
                                            &[
                                                cached_grok!(
                                                    "^\\[?windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_path}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?windows-registry-key:key%{SPACE}LIKE%{SPACE}'%{DATA:_tmp.reg_path}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?windows-registry-value-type:name%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_key}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?windows-registry-value-type:data%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_value}'\\]?"
                                                ),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.reg_path") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.registry.path",
                                        json!(
                                            event
                                                .get("_tmp.reg_path")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.reg_key") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.registry.key",
                                        json!(
                                            event
                                                .get("_tmp.reg_key")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.reg_value") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.registry.value",
                                        json!(
                                            event
                                                .get("_tmp.reg_value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-windows-registry"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("x509-certificate") };
            if _cond {
                if event.has_value("stix._patterns") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("stix._patterns").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // Begin nested pipeline: "indicator-x509"
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?)
                                        // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?)
                                        // Grok pattern: (?i:^\\[?x509-certificate:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?)
                                        // Grok pattern: ^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:_tmp.serial_number}'\\]?
                                        // Grok pattern: ^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:_tmp.signature_algorithm}'\\]?
                                        // Grok pattern: ^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:_tmp.version_number}'\\]?
                                        // Grok pattern: ^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_after}'\\]?
                                        // Grok pattern: ^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_before}'\\]?
                                        // Grok pattern: ^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:_tmp.issuer}'\\]?
                                        // Grok pattern: ^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:_tmp.subject}'\\]?
                                        if !extract_first_match(
                                            &[
                                                cached_grok!(
                                                    "(?i:^\\[?x509-certificate:hashes\\.'?MD5'?%{SPACE}=%{SPACE}'%{DATA:_tmp.md5}'\\]?)"
                                                ),
                                                cached_grok!(
                                                    "(?i:^\\[?x509-certificate:hashes\\.'?SHA-?1'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha1}'\\]?)"
                                                ),
                                                cached_grok!(
                                                    "(?i:^\\[?x509-certificate:hashes\\.'?SHA-?256'?%{SPACE}=%{SPACE}'%{DATA:_tmp.sha256}'\\]?)"
                                                ),
                                                cached_grok!(
                                                    "^\\[?x509-certificate:serial_number%{SPACE}=%{SPACE}'%{DATA:_tmp.serial_number}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?x509-certificate:signature_algorithm%{SPACE}=%{SPACE}'%{DATA:_tmp.signature_algorithm}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?x509-certificate:version%{SPACE}=%{SPACE}'%{DATA:_tmp.version_number}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?x509-certificate:validity_not_after%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_after}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?x509-certificate:validity_not_before%{SPACE}=%{SPACE}'%{TIMESTAMP_ISO8601:_tmp.not_before}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?x509-certificate:issuer%{SPACE}=%{SPACE}'%{DATA:_tmp.issuer}'\\]?"
                                                ),
                                                cached_grok!(
                                                    "^\\[?x509-certificate:subject%{SPACE}=%{SPACE}'%{DATA:_tmp.subject}'\\]?"
                                                ),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
                                        }
                                    }
                                    Ok(())
                                })();
                                let _cond = { event.has_value("_tmp.md5") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.file.hash.md5",
                                        json!(
                                            event
                                                .get("_tmp.md5")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha1") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.file.hash.sha1",
                                        json!(
                                            event
                                                .get("_tmp.sha1")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha256") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.file.hash.sha256",
                                        json!(
                                            event
                                                .get("_tmp.sha256")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.serial_number") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.serial_number",
                                        json!(
                                            event
                                                .get("_tmp.serial_number")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.signature_algorithm") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.signature_algorithm",
                                        json!(
                                            event
                                                .get("_tmp.signature_algorithm")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.version_number") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.version_number",
                                        json!(
                                            event
                                                .get("_tmp.version_number")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.distinguished_name",
                                        json!(
                                            event
                                                .get("_tmp.issuer")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.distinguished_name",
                                        json!(
                                            event
                                                .get("_tmp.subject")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer") };
                                if _cond {
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if let Some(kv_str) = event.get_string("_tmp.issuer") {
                                            for pair in cached_regex!("(?<!\\\\),")
                                                .split(&kv_str)
                                                .into_iter()
                                            {
                                                if pair.trim().is_empty() {
                                                    continue;
                                                }
                                                let Some((key, value)) = pair.split_once("=")
                                                else {
                                                    return Err(TransformError::ParseError {
                                                        path: "_tmp.issuer".into(),
                                                        message: format!(
                                                            "does not contain value_split: {pair}"
                                                        ),
                                                    });
                                                };
                                                {
                                                    let key = key.trim_matches(|c| " ".contains(c));
                                                    if !key.is_empty() {
                                                        kv_put(
                                                            event,
                                                            &format!("_tmp.issuer_fields.{}", key),
                                                            value,
                                                        )?;
                                                    }
                                                }
                                            }
                                        }
                                        Ok(())
                                    })();
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.CN") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.common_name",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.CN")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.C") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.country",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.C")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.L") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.locality",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.L")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.O") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.organization",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.O")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.OU") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.organizational_unit",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.OU")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.S") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.state_or_province",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.S")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.ST") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.state_or_province",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.ST")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.issuer_fields.P") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.issuer.state_or_province",
                                        json!(
                                            event
                                                .get("_tmp.issuer_fields.P")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject") };
                                if _cond {
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if let Some(kv_str) = event.get_string("_tmp.subject") {
                                            for pair in cached_regex!("(?<!\\\\),")
                                                .split(&kv_str)
                                                .into_iter()
                                            {
                                                if pair.trim().is_empty() {
                                                    continue;
                                                }
                                                let Some((key, value)) = pair.split_once("=")
                                                else {
                                                    return Err(TransformError::ParseError {
                                                        path: "_tmp.subject".into(),
                                                        message: format!(
                                                            "does not contain value_split: {pair}"
                                                        ),
                                                    });
                                                };
                                                {
                                                    let key = key.trim_matches(|c| " ".contains(c));
                                                    if !key.is_empty() {
                                                        kv_put(
                                                            event,
                                                            &format!("_tmp.subject_fields.{}", key),
                                                            value,
                                                        )?;
                                                    }
                                                }
                                            }
                                        }
                                        Ok(())
                                    })();
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.CN") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.common_name",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.CN")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.C") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.country",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.C")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.L") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.locality",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.L")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.O") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.organization",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.O")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.OU") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.organizational_unit",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.OU")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.S") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.state_or_province",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.S")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.ST") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.state_or_province",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.ST")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.subject_fields.P") };
                                if _cond {
                                    event.append(
                                        "threat.indicator.x509.subject.state_or_province",
                                        json!(
                                            event
                                                .get("_tmp.subject_fields.P")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.md5") };
                                if _cond {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_tmp.md5")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha1") };
                                if _cond {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_tmp.sha1")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.sha256") };
                                if _cond {
                                    event.append_unique(
                                        "related.hash",
                                        json!(
                                            event
                                                .get("_tmp.sha256")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
                                let _cond = { event.has_value("_tmp.not_after") };
                                if _cond {
                                    if let Some(date_str) = event.get_as_string("_tmp.not_after") {
                                        match parse_date_out(
                                            &date_str,
                                            &["ISO8601", "yyyy-MM-dd HH:mm:ssz"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => event
                                                .set("threat.indicator.x509.not_after", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_tmp.not_after".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                }
                                let _cond = { event.has_value("_tmp.not_before") };
                                if _cond {
                                    if let Some(date_str) = event.get_as_string("_tmp.not_before") {
                                        match parse_date_out(
                                            &date_str,
                                            &["ISO8601", "yyyy-MM-dd HH:mm:ssz"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => event
                                                .set("threat.indicator.x509.not_before", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_tmp.not_before".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                }
                                event.remove("_tmp");
                                // End nested pipeline: "indicator-x509"
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
                                "stix._patterns",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            event.remove("_conf");
            event.remove("stix._patterns");

            // Painless script
            // Source: void unescape(Object obj) {\n  if (obj instanceof Map) {\n    for (entry in ((Map)obj).entrySet()) {\n      Object value = entry.getValue();\n      unescape(value);\n      if (value instanceof String) {\n        entry.setValue(((String)value).replace('\\\\\\\\', '\\\\'));\n      }\n    }\n  } else if (obj instanceof List) {\n    for (int i = 0; i < ((List)obj).size(); i++) {\n      Object value = ((List)obj).get(i);\n      unescape(value);\n      if (value instanceof String) {\n        ((List)obj).set(i, ((String)value).replace('\\\\\\\\', '\\\\'));\n      }\n    }\n  }\n} unescape(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void unescape(Object obj) {\n  if (obj instanceof Map) {\n    for (entry in ((Map)obj).entrySet()) {\n      Object value = entry.getValue();\n      unescape(value);\n      if (value instanceof String) {\n        entry.setValue(((String)value).replace('\\\\\\\\', '\\\\'));\n      }\n    }\n  } else if (obj instanceof List) {\n    for (int i = 0; i < ((List)obj).size(); i++) {\n      Object value = ((List)obj).get(i);\n      unescape(value);\n      if (value instanceof String) {\n        ((List)obj).set(i, ((String)value).replace('\\\\\\\\', '\\\\'));\n      }\n    }\n  }\n} unescape(ctx);\n"#
                ),
            )?;

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n} drop(ctx);\n
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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
