// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_administrative_and_operational_audit` pipeline.
pub struct PipelineAdministrativeAndOperationalAudit;

impl Transform for PipelineAdministrativeAndOperationalAudit {
    fn name(&self) -> &str {
        "pipeline_administrative_and_operational_audit"
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

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("60067") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details_raw") {
                    // Grok pattern: ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, OperationMessageText={%{DATA:cisco_ise.log.log_details.OperationMessageText}}
                    if !cached_grok!("ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, OperationMessageText={%{DATA:cisco_ise.log.log_details.OperationMessageText}}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { ["61025", "61026"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details_raw") {
                    // Grok pattern: ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, , OperationMessageText=%{DATA:cisco_ise.log.log_details.OperationMessageText}, AcsInstance=%{GREEDYDATA:cisco_ise.log.log_details.AcsInstance}
                    if !cached_grok!("ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, , OperationMessageText=%{DATA:cisco_ise.log.log_details.OperationMessageText}, AcsInstance=%{GREEDYDATA:cisco_ise.log.log_details.AcsInstance}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_cisco_ise_log_log_details_raw_7eca9d29")?;
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
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
            if _cond {
                if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                    for pair in cached_regex!("(?<!\\\\), ").split(&kv_str).into_iter() {
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
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.log_detail") {
                    // Grok pattern: ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}
                    // Grok pattern: ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, OperationMessageText=%{GREEDYDATA:cisco_ise.log.log_details.OperationMessageText}
                    // Grok pattern: ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}
                    // Grok pattern: ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{GREEDYDATA:cisco_ise.log.log_details.ObjectName}
                    if !extract_first_match(
                        &[
                            cached_grok!("ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}"),
                            cached_grok!("ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, OperationMessageText=%{GREEDYDATA:cisco_ise.log.log_details.OperationMessageText}"),
                            cached_grok!("ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}"),
                            cached_grok!("ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{GREEDYDATA:cisco_ise.log.log_details.ObjectName}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
            if _cond {
                event.remove("cisco_ise.log.log_details.log_detail");
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.ConfigChangeData") {
                    // Grok pattern: ^%{DATA:_tmp.temp}, Log Severity Level = %{DATA:cisco_ise.log.log_details.LogSeverityLevel}\\\\,Local Logging = %{DATA:cisco_ise.log.log_details.LocalLogging}\\\\,Assigned Targets = {%{DATA:cisco_ise.log.log_details.AssignedTargets}}
                    if !cached_grok!("^%{DATA:_tmp.temp}, Log Severity Level = %{DATA:cisco_ise.log.log_details.LogSeverityLevel}\\\\,Local Logging = %{DATA:cisco_ise.log.log_details.LocalLogging}\\\\,Assigned Targets = {%{DATA:cisco_ise.log.log_details.AssignedTargets}}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details_raw") {
                    // Grok pattern: ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, %{GREEDYDATA:cisco_ise.log.log_details.log_detail}
                    if !cached_grok!("ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, %{GREEDYDATA:cisco_ise.log.log_details.log_detail}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.log_detail") {
                    // Grok pattern: AdminSession=%{DATA:cisco_ise.log.log_details.AdminSession}, AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}
                    // Grok pattern: AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}
                    // Grok pattern: AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, %{GREEDYDATA:cisco_ise.log.log_details.log_description}
                    if !extract_first_match(
                        &[
                            cached_grok!("AdminSession=%{DATA:cisco_ise.log.log_details.AdminSession}, AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}"),
                            cached_grok!("AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}"),
                            cached_grok!("AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, %{GREEDYDATA:cisco_ise.log.log_details.log_description}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
            if _cond {
                event.remove("cisco_ise.log.log_details.log_detail");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.log_description") {
                if let Some(kv_str) = event.get_string("cisco_ise.log.log_details.log_description") {
                    for pair in kv_str.split(", ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "cisco_ise.log.log_details.log_description".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.log_details.{}", key), value)?;
                            }
                        }
                    }
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set("_ingest.on_failure_processor_tag", "kv_cisco_ise_log_log_details_log_description_to_cisco_ise_log_log_details_d92c0c09")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.log_description");

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.ConfigChangeData") {
                    // Grok pattern: ^%{DATA:_tmp.temp}, %{GREEDYDATA:_tmp.ConfigChangeData}$
                    if !cached_grok!("^%{DATA:_tmp.temp}, %{GREEDYDATA:_tmp.ConfigChangeData}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("_tmp.ConfigChangeData") {
                    for pair in kv_str.split(", ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.ConfigChangeData".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.log_details.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { !(["60067", "61025", "61026", "52001", "52002"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))) };
            if _cond {
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
                            let key = key.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.log_details.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("60067") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("cisco_ise.log.log_details.OperationMessageText") {
                    for pair in kv_str.split(", ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "cisco_ise.log.log_details.OperationMessageText".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.log_details.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("cisco_ise.log.log_details.AssignedTargets") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("cisco_ise.log.assigned_targets", Value::Array(parts))?;
                }
                Ok(())
            })();
            }

                event.remove("cisco_ise.log.log_details.AssignedTargets");

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

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["51001", "51002", "51020", "51021", "52000", "52001", "52002", "60077", "60078", "60461", "61077", "58005", "60094", "60093", "60134", "60188", "60116", "60080", "60115", "60081", "60084"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("iam"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["51001", "51002", "51020", "51021", "60077", "60078", "61077", "60188", "60116", "60080", "60115", "60081"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["61025", "61026", "60134"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["60067", "60070", "60456", "58005"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("process"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["52000", "52001", "52002", "60084"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("configuration"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["51001", "51002", "51020", "51021"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("admin"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["52001", "60084"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("change"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["61025", "61026"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("connection"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("52000") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("creation"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("52002") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("deletion"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("61026") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("end"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["60116", "60080", "60115", "60081"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("user"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["51001", "51002", "51020", "51021", "52000", "52001", "52002", "60067", "60070", "60077", "60078", "60456", "60461", "61025", "61026", "61077", "58005", "60094", "60093", "60134", "60188", "60116", "60080", "60115", "60081", "60084"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["60067", "60456", "61025"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("start"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["51001", "51002", "60078", "60080", "60115", "60116", "61077"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["51000", "51020", "51021", "60077", "60081", "60188"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("failure"))?;
                Ok(())
            })();
            }

            // SKIPPED: condition not transpiled: ctx.cisco_ise?.log?.message?.code == '60084' && (ctx.cisco_ise?.log?.log_details?.OperationMessageText ?: '') =~ /successfully/i
            #[allow(unreachable_code, unused_variables)]
            if false {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            // SKIPPED: condition not transpiled: ctx.cisco_ise?.log?.message?.code == '60084' && (ctx.cisco_ise?.log?.log_details?.OperationMessageText ?: '') =~ /(?:failed|failure|unsuccessful|error)/i
            #[allow(unreachable_code, unused_variables)]
            if false {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("failure"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_ise.log.message.code") == Some("60084") && !event.has_value("event.outcome") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("unknown"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def eventCategory = new ArrayList();\ndef eventType = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60077\",\"60078\",\"60461\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"iam\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"60077\",\"60078\",\"61077\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"61025\",\"61026\",\"60134\"], \"name\": \"network\"],\n  [\"messageCodeArray\": [\"60067\",\"60070\",\"60456\",\"58005\"], \"name\": \"process\"],\n  [\"messageCodeArray\": [\"52000\",\"52001\",\"52002\",\"60084\"], \"name\": \"configuration\"]\n];\ndef typeReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\"], \"name\": \"admin\"],\n  [\"messageCodeArray\": [\"52001\",\"60084\"], \"name\": \"change\"],\n  [\"messageCodeArray\": [\"61025\", \"61026\"], \"name\": \"connection\"],\n  [\"messageCodeArray\": [\"52000\"], \"name\": \"creation\"],\n  [\"messageCodeArray\": [\"52002\"], \"name\": \"deletion\"],\n  [\"messageCodeArray\": [\"61026\"], \"name\": \"end\"],\n  [\"messageCodeArray\": [\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"user\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60067\",\"60070\",\"60077\",\"60078\",\"60456\",\"60461\",\"61025\",\"61026\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"info\"],\n  [\"messageCodeArray\": [\"60067\",\"60456\",\"61025\"], \"name\": \"start\"]\n];\n\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nfor (entry in typeReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventType.add(entry.name);\n  }\n}\n\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\nctx.event.type = eventType;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def eventCategory = new ArrayList();\ndef eventType = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60077\",\"60078\",\"60461\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"iam\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"60077\",\"60078\",\"61077\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"61025\",\"61026\",\"60134\"], \"name\": \"network\"],\n  [\"messageCodeArray\": [\"60067\",\"60070\",\"60456\",\"58005\"], \"name\": \"process\"],\n  [\"messageCodeArray\": [\"52000\",\"52001\",\"52002\",\"60084\"], \"name\": \"configuration\"]\n];\ndef typeReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\"], \"name\": \"admin\"],\n  [\"messageCodeArray\": [\"52001\",\"60084\"], \"name\": \"change\"],\n  [\"messageCodeArray\": [\"61025\", \"61026\"], \"name\": \"connection\"],\n  [\"messageCodeArray\": [\"52000\"], \"name\": \"creation\"],\n  [\"messageCodeArray\": [\"52002\"], \"name\": \"deletion\"],\n  [\"messageCodeArray\": [\"61026\"], \"name\": \"end\"],\n  [\"messageCodeArray\": [\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"user\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60067\",\"60070\",\"60077\",\"60078\",\"60456\",\"60461\",\"61025\",\"61026\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"info\"],\n  [\"messageCodeArray\": [\"60067\",\"60456\",\"61025\"], \"name\": \"start\"]\n];\n\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nfor (entry in typeReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventType.add(entry.name);\n  }\n}\n\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\nctx.event.type = eventType;\n"#))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AcsInstance") {
                    event.rename("cisco_ise.log.log_details.AcsInstance", "cisco_ise.log.acs.instance")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AdminInterface") {
                    event.rename("cisco_ise.log.log_details.AdminInterface", "cisco_ise.log.admin.interface")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.AdminIPAddress") {
                if let Some(val) = event.get("cisco_ise.log.log_details.AdminIPAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.AdminIPAddress".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_AdminIPAddress_to_client_ip_732626ad")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.AdminIPAddress");

            let _cond = { event.has_value("client.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AdminName") {
                    event.rename("cisco_ise.log.log_details.AdminName", "client.user.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AdminSession") {
                    event.rename("cisco_ise.log.log_details.AdminSession", "cisco_ise.log.admin.session")?;
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

                if event.has_value("cisco_ise.log.log_details.ConfigChangeData") {
                    event.rename("cisco_ise.log.log_details.ConfigChangeData", "cisco_ise.log.config_change.data")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Component") {
                    event.rename("cisco_ise.log.log_details.Component", "cisco_ise.log.component")?;
                }
                Ok(())
            })();

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
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_baa6773e")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.DestinationPort");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.FailureReason") {
                    event.rename("cisco_ise.log.log_details.FailureReason", "cisco_ise.log.failure.reason")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.FailureFlag") {
                if let Some(val) = event.get("cisco_ise.log.log_details.FailureFlag") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.FailureFlag".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.failure.flag", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_FailureFlag_to_cisco_ise_log_failure_flag_3adca0f9")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.FailureFlag");

                if event.has_value("cisco_ise.log.log_details.LocalLogging") {
                    event.rename("cisco_ise.log.log_details.LocalLogging", "cisco_ise.log.local_logging")?;
                }

            let _cond = { !event.has_value("log.syslog.severity.name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.LogSeverityLevel") {
                    event.rename("cisco_ise.log.log_details.LogSeverityLevel", "log.syslog.severity.name")?;
                }
                Ok(())
            })();
            }

                event.remove("cisco_ise.log.log_details.LogSeverityLevel");

                if event.has_value("cisco_ise.log.log_details.LogErrorMessage") {
                    event.rename("cisco_ise.log.log_details.LogErrorMessage", "cisco_ise.log.log_error.message")?;
                }

                if event.has_value("cisco_ise.log.log_details.LoggerName") {
                    event.rename("cisco_ise.log.log_details.LoggerName", "log.logger")?;
                }

                if event.has_value("cisco_ise.log.log_details.MessageCode") {
                    event.rename("cisco_ise.log.log_details.MessageCode", "cisco_ise.log.message.code")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.FeedServiceFeed") {
                    event.rename("cisco_ise.log.log_details.FeedServiceFeed", "cisco_ise.log.feed_service.feed.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.FeedServiceFeedVersion") {
                    event.rename("cisco_ise.log.log_details.FeedServiceFeedVersion", "cisco_ise.log.feed_service.feed.version")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.FeedServiceHost") {
                    event.rename("cisco_ise.log.log_details.FeedServiceHost", "cisco_ise.log.feed_service.host")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.FeedServicePort") {
                    event.rename("cisco_ise.log.log_details.FeedServicePort", "cisco_ise.log.feed_service.port")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.log_details.FeedServiceQueryToTime") && event.get_str("cisco_ise.log.log_details.FeedServiceQueryToTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cisco_ise.log.log_details.FeedServiceQueryToTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("cisco_ise.log.feed_service.query.to_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cisco_ise.log.log_details.FeedServiceQueryToTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_log_details_FeedServiceQueryToTime_to_cisco_ise_log_feed_service_query_to_time_6ec220c3")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("cisco_ise.log.log_details.FeedServiceQueryToTime");

            let _cond = { event.has_value("cisco_ise.log.log_details.FeedServiceQueryFromTime") && event.get_str("cisco_ise.log.log_details.FeedServiceQueryFromTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cisco_ise.log.log_details.FeedServiceQueryFromTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("cisco_ise.log.feed_service.query.from_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cisco_ise.log.log_details.FeedServiceQueryFromTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_log_details_FeedServiceQueryFromTime_to_cisco_ise_log_feed_service_query_from_time_5f7ec641")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("cisco_ise.log.log_details.FeedServiceQueryFromTime");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IdentityGroup") {
                    event.rename("cisco_ise.log.log_details.IdentityGroup", "cisco_ise.log.identity.group")?;
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
                    event.set("host.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_IpAddress_to_host_ip_0f848bff")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.IpAddress");

            let _cond = { event.has_value("host.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("host.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.ObjectName") {
                    event.rename("cisco_ise.log.log_details.ObjectName", "cisco_ise.log.object.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.ObjectInternalID") {
                    event.rename("cisco_ise.log.log_details.ObjectInternalID", "cisco_ise.log.object.internal.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.ObjectType") {
                    event.rename("cisco_ise.log.log_details.ObjectType", "cisco_ise.log.object.type")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["60080", "60081"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.OperationMessageText") {
                    // Grok pattern: (?:Accepted|Failed) password for (?:invalid user )?%{DATA:user.name} from %{IP:source.ip}
                    // Grok pattern: Invalid user %{DATA:user.name} from %{IP:source.ip}
                    if !extract_first_match(
                        &[
                            cached_grok!("(?:Accepted|Failed) password for (?:invalid user )?%{DATA:user.name} from %{IP:source.ip}"),
                            cached_grok!("Invalid user %{DATA:user.name} from %{IP:source.ip}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

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
                if event.has_value("cisco_ise.log.log_details.OperationMessageText") {
                    event.rename("cisco_ise.log.log_details.OperationMessageText", "cisco_ise.log.operation_message.text")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.PsnHostName") {
                    event.rename("cisco_ise.log.log_details.PsnHostName", "cisco_ise.log.psn.hostname")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.psn.hostname") && event.get_str("cisco_ise.log.psn.hostname") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hosts", json!(event.get("cisco_ise.log.psn.hostname").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.RequestResponseType") {
                    event.rename("cisco_ise.log.log_details.RequestResponseType", "cisco_ise.log.request_response.type")?;
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
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ResponseTime_to_cisco_ise_log_response_time_bd1b08c1")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.ResponseTime");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.UserName") {
                    event.rename("cisco_ise.log.log_details.UserName", "user.name")?;
                }
                Ok(())
            })();

            // SKIPPED: condition not transpiled: ctx.user?.name == null && ctx.client?.user?.name != null && ctx.client.user.name.indexOf('@') <= 0
            #[allow(unreachable_code, unused_variables)]
            if false {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("client.user.name").cloned() {
                event.set("user.name", v)?;
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

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
