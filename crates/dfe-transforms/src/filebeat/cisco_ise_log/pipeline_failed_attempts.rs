// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_failed_attempts` pipeline.
pub struct PipelineFailedAttempts;

impl Transform for PipelineFailedAttempts {
    fn name(&self) -> &str {
        "pipeline_failed_attempts"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_i64("cisco_ise.log.segment.number") == Some(0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("cisco_ise.log.segment.number") && event.get_i64("cisco_ise.log.segment.number").is_some_and(|n| n > 0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSSSSS", "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_9ef85c6a")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("event.timezone") && event.get_str("event.timezone") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSSSSS", "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_1d2a12b9")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("cisco_ise.log.message.description") && event.get_str("cisco_ise.log.message.description") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                    // Grok pattern: ^%{DATA:event.action}:
                    if !cached_grok!("^%{DATA:event.action}:").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("failure"))?;
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5400", "5401", "5405", "5411", "5412", "5418", "5423", "5435", "5440", "5448"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5440") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("session"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5400", "5401", "5405", "5411", "5412", "5418", "5423", "5435", "5440", "5448"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5405", "5411", "5418", "5435"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("end"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5440") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("start"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5402", "5403", "5407"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5402", "5403", "5407"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5402", "5403", "5407"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("denied"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5408", "5409", "5410"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5408", "5409", "5410"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5408", "5409", "5410"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5449"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5449"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("end"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5406", "5441", "5442", "5443"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5406", "5441", "5442", "5443"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("denied"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5419", "5422"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5419", "5422"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("denied"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5413", "5414", "5417", "5420", "5421", "5434", "5436", "5437", "5438", "5439"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5413", "5414", "5417", "5420", "5421", "5434", "5436", "5437", "5438", "5439"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5416") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5416") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("end"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5450") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5450") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("connection"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5415") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("iam"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5415") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("user"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5415") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("change"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5451") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5451") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("end"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5452") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5452") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                    for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "cisco_ise.log.log_details_raw".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.log_details.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Response") {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.Response") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("{") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("}") else { break 'dissect false };
                        captured.push(("_tmp.response", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("}") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
            }
                Ok(())
            })();

                event.remove("cisco_ise.log.log_details.Response");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("_tmp.response") {
                    for pair in kv_str.split("; ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.response".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.response.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.log_details.acme-av-pair") && event.get("cisco_ise.log.log_details.acme-av-pair").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("cisco_ise.log.log_details.acme-av-pair") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cisco_ise.log.log_details.acme-av-pair").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(kv_str) = event.get_string("_ingest._value") {
                            for pair in kv_str.split(", ") {
                            if pair.trim().is_empty() {
                            continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value".into(),
                            message: format!("does not contain value_split: {pair}"),
                            });
                            };
                            {
                            if !key.is_empty() {
                            kv_put(event, &format!("cisco_ise.log.acme-av-pair.{}", key), value)?;
                            }
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "kv")?;
                            event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("cisco_ise.log.log_details.acme-av-pair", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.acme-av-pair") && !(event.get("cisco_ise.log.log_details.acme-av-pair").is_some_and(|v| v.is_array())) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("cisco_ise.log.log_details.acme-av-pair") {
                    for pair in kv_str.split(", ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "cisco_ise.log.log_details.acme-av-pair".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.acme-av-pair.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set("_ingest.on_failure_processor_tag", "kv_cisco_ise_log_log_details_acme-av-pair_to_cisco_ise_log_acme-av-pair_6cc5cb52")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("cisco_ise.log.log_details.acme-av-pair");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AcsSessionID") {
                    event.rename("cisco_ise.log.log_details.AcsSessionID", "cisco_ise.log.acs.session.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.ADDomain") {
                    event.rename("cisco_ise.log.log_details.ADDomain", "cisco_ise.log.ad.domain.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AllowedProtocolMatchedRule") {
                    event.rename("cisco_ise.log.log_details.AllowedProtocolMatchedRule", "cisco_ise.log.allowed_protocol.matched.rule")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                    event.rename("cisco_ise.log.log_details.AuthenticationIdentityStore", "cisco_ise.log.authentication.identity_store")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AuthenticationMethod") {
                    event.rename("cisco_ise.log.log_details.AuthenticationMethod", "cisco_ise.log.authentication.method")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Called-Station-ID") {
                    event.rename("cisco_ise.log.log_details.Called-Station-ID", "cisco_ise.log.called_station.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Calling-Station-ID") {
                    event.rename("cisco_ise.log.log_details.Calling-Station-ID", "cisco_ise.log.calling_station.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.CPMSessionID") {
                    event.rename("cisco_ise.log.log_details.CPMSessionID", "cisco_ise.log.cpm.session.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.DetailedInfo") {
                    event.rename("cisco_ise.log.log_details.DetailedInfo", "cisco_ise.log.detailed_info")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Device Type") {
                    event.rename("cisco_ise.log.log_details.Device Type", "cisco_ise.log.device.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.EAP-Key-Name") {
                    event.rename("cisco_ise.log.log_details.EAP-Key-Name", "cisco_ise.log.eap_key.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.EapAuthentication") {
                    event.rename("cisco_ise.log.log_details.EapAuthentication", "cisco_ise.log.eap.authentication")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.EapChainingResult") {
                    event.rename("cisco_ise.log.log_details.EapChainingResult", "cisco_ise.log.eap.chaining_result")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.EapTunnel") {
                    event.rename("cisco_ise.log.log_details.EapTunnel", "cisco_ise.log.eap.tunnel")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.EndPointMACAddress") {
                    event.rename("cisco_ise.log.log_details.EndPointMACAddress", "cisco_ise.log.endpoint.mac.address")?;
                }
                Ok(())
            })();

            if event.has_value("cisco_ise.log.endpoint.mac.address") {
                gsub_field(event, "cisco_ise.log.endpoint.mac.address", "cisco_ise.log.endpoint.mac.address", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("cisco_ise.log.endpoint.mac.address") {
                map_strings(event, "cisco_ise.log.endpoint.mac.address", "cisco_ise.log.endpoint.mac.address", str::to_uppercase)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("cisco_ise.log.endpoint.mac.address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.mac", v)?;
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.FailureReason") {
                    event.rename("cisco_ise.log.log_details.FailureReason", "cisco_ise.log.failure.reason")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Framed-IP-Address") {
                if let Some(val) = event.get("cisco_ise.log.log_details.Framed-IP-Address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.Framed-IP-Address".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.framed.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Framed-IP-Address_to_cisco_ise_log_framed_ip_a7ec88de")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.Framed-IP-Address");

            let _cond = { event.has_value("cisco_ise.log.framed.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_ise.log.framed.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Framed-MTU") {
                if let Some(val) = event.get("cisco_ise.log.log_details.Framed-MTU") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.Framed-MTU".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.framed.mtu", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Framed-MTU_to_cisco_ise_log_framed_mtu_58cef504")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.Framed-MTU");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.GroupsOrAttributesProcessFailure") {
                if let Some(val) = event.get("cisco_ise.log.log_details.GroupsOrAttributesProcessFailure") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.GroupsOrAttributesProcessFailure".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.groups.process_failure", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_GroupsOrAttributesProcessFailure_to_cisco_ise_log_groups_process_failure_91f199c0")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.GroupsOrAttributesProcessFailure");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IdentitySelectionMatchedRule") {
                    event.rename("cisco_ise.log.log_details.IdentitySelectionMatchedRule", "cisco_ise.log.identity.selection.matched.rule")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.ISEPolicySetName") {
                    event.rename("cisco_ise.log.log_details.ISEPolicySetName", "cisco_ise.log.ise.policy.set_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Location") {
                    event.rename("cisco_ise.log.log_details.Location", "cisco_ise.log.location")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.NAS-IP-Address") {
                if let Some(val) = event.get("cisco_ise.log.log_details.NAS-IP-Address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.NAS-IP-Address".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.nas.ip", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.remove("cisco_ise.log.log_details.NAS-IP-Address");
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "remove")?;
                        if event.remove("NAS-IP-Address").is_none() {
                            return Err(TransformError::FieldNotFound { path: "NAS-IP-Address".into() });
                        }
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("cisco_ise.log.nas.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_ise.log.nas.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.NAS-Port") {
                if let Some(val) = event.get("cisco_ise.log.log_details.NAS-Port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.NAS-Port".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.nas.port.number", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-Port_to_cisco_ise_log_nas_port_number_8385ddaf")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.NAS-Port");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NAS-Port-Id") {
                    event.rename("cisco_ise.log.log_details.NAS-Port-Id", "cisco_ise.log.nas.port.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NAS-Port-Type") {
                    event.rename("cisco_ise.log.log_details.NAS-Port-Type", "cisco_ise.log.nas.port.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceGroups") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceGroups", "cisco_ise.log.network.device.groups")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceName") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceName", "cisco_ise.log.network.device.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.RadiusPacketType") {
                    event.rename("cisco_ise.log.log_details.RadiusPacketType", "cisco_ise.log.radius_packet.type")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.RequestLatency") {
                if let Some(val) = event.get("cisco_ise.log.log_details.RequestLatency") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.RequestLatency".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.request.latency", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_RequestLatency_to_cisco_ise_log_request_latency_bafbebd7")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.RequestLatency");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.SelectedAccessService") {
                    event.rename("cisco_ise.log.log_details.SelectedAccessService", "cisco_ise.log.selected.access.service")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.SelectedAuthenticationIdentityStores") {
                    event.rename("cisco_ise.log.log_details.SelectedAuthenticationIdentityStores", "cisco_ise.log.selected.authentication.identity_stores")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Service-Type") {
                    event.rename("cisco_ise.log.log_details.Service-Type", "cisco_ise.log.service.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.State") {
                    event.rename("cisco_ise.log.log_details.State", "cisco_ise.log.state")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.UseCase") {
                    event.rename("cisco_ise.log.log_details.UseCase", "cisco_ise.log.usecase")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.DestinationIPAddress") {
                if let Some(val) = event.get("cisco_ise.log.log_details.DestinationIPAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.DestinationIPAddress".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationIPAddress_to_destination_ip_a431dedf")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.DestinationIPAddress");

            let _cond = { event.has_value("destination.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.DestinationPort") {
                if let Some(val) = event.get("cisco_ise.log.log_details.DestinationPort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.DestinationPort".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_ad144c54")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.DestinationPort");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.Device IP Address".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Device_IP_Address_to_client_ip_b34586ce")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.Device IP Address");

            let _cond = { event.has_value("client.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Device Port") {
                if let Some(val) = event.get("cisco_ise.log.log_details.Device Port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.Device Port".into(),
                            message,
                        })?;
                    event.set("client.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Device_Port_to_client_port_cf795c9b")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.Device Port");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Acct-Session-Id") {
                    event.rename("cisco_ise.log.log_details.Acct-Session-Id", "cisco_ise.log.acct.session.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Acct-Status-Type") {
                    event.rename("cisco_ise.log.log_details.Acct-Status-Type", "cisco_ise.log.acct.status.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.DTLSSupport") {
                    event.rename("cisco_ise.log.log_details.DTLSSupport", "cisco_ise.log.dtls_support")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.IpAddress") {
                if let Some(val) = event.get("cisco_ise.log.log_details.IpAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.IpAddress".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_IpAddress_to_source_ip_e43c3b18")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.IpAddress");

            let _cond = { event.has_value("source.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IPSEC") {
                    event.rename("cisco_ise.log.log_details.IPSEC", "cisco_ise.log.ipsec")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Model Name") {
                    event.rename("cisco_ise.log.log_details.Model Name", "cisco_ise.log.model.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Network Device Profile") {
                    event.rename("cisco_ise.log.log_details.Network Device Profile", "cisco_ise.log.network.device.profile")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileId") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceProfileId", "cisco_ise.log.network.device.profile_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileName") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceProfileName", "cisco_ise.log.network.device.profile_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.OpenSSLErrorMessage") {
                    event.rename("cisco_ise.log.log_details.OpenSSLErrorMessage", "cisco_ise.log.openssl.error.message")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.OpenSSLErrorStack") {
                    event.rename("cisco_ise.log.log_details.OpenSSLErrorStack", "cisco_ise.log.openssl.error.stack")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.PortalName") {
                    event.rename("cisco_ise.log.log_details.PortalName", "cisco_ise.log.portal.name")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.ResponseTime") {
                if let Some(val) = event.get("cisco_ise.log.log_details.ResponseTime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.ResponseTime".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.response.time", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ResponseTime_to_cisco_ise_log_response_time_1c336c93")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.ResponseTime");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Session-Timeout") {
                if let Some(val) = event.get("cisco_ise.log.log_details.Session-Timeout") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.Session-Timeout".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.session.timeout", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Session-Timeout_to_cisco_ise_log_session_timeout_9f86aa72")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.Session-Timeout");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Step") {
                    event.rename("cisco_ise.log.log_details.Step", "cisco_ise.log.step")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.StepLatency") {
                    event.rename("cisco_ise.log.log_details.StepLatency", "cisco_ise.log.step_latency")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.TLSCipher") {
                    event.rename("cisco_ise.log.log_details.TLSCipher", "cisco_ise.log.tls.cipher")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.TLSVersion") {
                    event.rename("cisco_ise.log.log_details.TLSVersion", "cisco_ise.log.tls.version")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.TotalFailedAttempts") {
                if let Some(val) = event.get("cisco_ise.log.log_details.TotalFailedAttempts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.TotalFailedAttempts".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.total.failed_attempts", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_TotalFailedAttempts_to_cisco_ise_log_total_failed_attempts_0829b850")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.TotalFailedAttempts");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.TotalFailedTime") {
                if let Some(val) = event.get("cisco_ise.log.log_details.TotalFailedTime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.TotalFailedTime".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.total.failed_time", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_TotalFailedTime_to_cisco_ise_log_total_failed_time_daedbbd4")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.TotalFailedTime");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.UserType") {
                    event.rename("cisco_ise.log.log_details.UserType", "cisco_ise.log.user.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Protocol") {
                    event.rename("cisco_ise.log.log_details.Protocol", "network.protocol")?;
                }
                Ok(())
            })();

            if event.has_value("network.protocol") {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("cisco_ise.log.log_details.UserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "append")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("cisco_ise.log.log_details.UserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "append")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("cisco_ise.log.log_details.UserName");

            let _cond = { event.has_value("cisco_ise.log.log_details.User-Name") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("cisco_ise.log.log_details.User-Name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "append")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.User-Name") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("cisco_ise.log.log_details.User-Name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "append")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("cisco_ise.log.log_details.User-Name");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
