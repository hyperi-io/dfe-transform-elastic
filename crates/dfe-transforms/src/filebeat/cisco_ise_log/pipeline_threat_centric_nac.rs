// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_threat_centric_nac` pipeline.
pub struct PipelineThreatCentricNac;

impl Transform for PipelineThreatCentricNac {
    fn name(&self) -> &str {
        "pipeline_threat_centric_nac"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                event.append("event.type", json!("info"))?;

            let _cond = { event.get_i64("cisco_ise.log.segment.number") == Some(0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    let _ = cached_grok!("^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)?;
                }
            }

            let _cond = { event.has_value("cisco_ise.log.segment.number") && event.get_i64("cisco_ise.log.segment.number").is_some_and(|n| n > 0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:cisco_ise.log.log_details_raw},
                    let _ = cached_grok!("^%{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)?;
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
                    let _ = cached_grok!("^%{DATA:event.action}:").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("91110") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["91004", "91018"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("configuration"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") };
            if _cond {
                // Painless script
                // Source: def eventCategory = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"91110\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"91004\",\"91018\"], \"name\": \"configuration\"]\n];\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def eventCategory = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"91110\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"91004\",\"91018\"], \"name\": \"configuration\"]\n];\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\n"#))?;
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
                if event.has_value("cisco_ise.log.log_details.Details") {
                    event.rename("cisco_ise.log.log_details.Details", "cisco_ise.log.details")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AdapterInstanceName") {
                    event.rename("cisco_ise.log.log_details.AdapterInstanceName", "cisco_ise.log.adapter_instance.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AdapterInstanceUuid") {
                    event.rename("cisco_ise.log.log_details.AdapterInstanceUuid", "cisco_ise.log.adapter_instance.uuid")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Status") {
                    event.rename("cisco_ise.log.log_details.Status", "cisco_ise.log.status")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Connectivity") {
                    event.rename("cisco_ise.log.log_details.Connectivity", "cisco_ise.log.connectivity")?;
                }
                Ok(())
            })();

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
