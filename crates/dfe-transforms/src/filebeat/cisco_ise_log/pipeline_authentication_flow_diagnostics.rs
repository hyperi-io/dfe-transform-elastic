// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_authentication_flow_diagnostics` pipeline.
pub struct PipelineAuthenticationFlowDiagnostics;

impl Transform for PipelineAuthenticationFlowDiagnostics {
    fn name(&self) -> &str {
        "pipeline_authentication_flow_diagnostics"
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
                    // Grok pattern: %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    let _ = cached_grok!("%{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("iam"))?;
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["22040", "22057", "22061", "22060", "22037"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AcsSessionID") {
                    event.rename("cisco_ise.log.log_details.AcsSessionID", "cisco_ise.log.acs.session.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                    event.rename("cisco_ise.log.log_details.AuthenticationIdentityStore", "cisco_ise.log.selected.authentication.identity_stores")?;
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
                if event.has_value("cisco_ise.log.log_details.CurrentIDStoreName") {
                    event.rename("cisco_ise.log.log_details.CurrentIDStoreName", "cisco_ise.log.currentid.store_name")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.log_details.DestinationIPAddress") };
            if _cond {
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
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationIPAddress_to_destination_ip_bf35ca8e")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
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

            // SKIPPED: condition not transpiled: ctx.cisco_ise?.log?.log_details['NAS-IP-Address'] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
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
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-IP-Address_to_cisco_ise_log_nas_ip_04e2547a")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("cisco_ise.log.log_details.NAS-IP-Address");

            let _cond = { event.has_value("cisco_ise.log.nas.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_ise.log.nas.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("cisco_ise.log.log_details.OriginalUserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("cisco_ise.log.log_details.OriginalUserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                event.remove("cisco_ise.log.log_details.OriginalUserName");

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("cisco_ise.log.log_details.UserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("cisco_ise.log.log_details.UserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                event.remove("cisco_ise.log.log_details.UserName");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.SelectedAccessService") {
                    event.rename("cisco_ise.log.log_details.SelectedAccessService", "cisco_ise.log.selected.access.service")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex") {
                if let Some(val) = event.get("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.workflow.current_id.store_index", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_WorkflowCurrentIDStoreIndex_to_cisco_ise_log_workflow_current_id_store_index_8a3cdeab")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.WorkflowIfAuthenticationFailed") {
                    event.rename("cisco_ise.log.log_details.WorkflowIfAuthenticationFailed", "cisco_ise.log.workflow.if.authentication_failed")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.WorkflowIfProcessError") {
                    event.rename("cisco_ise.log.log_details.WorkflowIfProcessError", "cisco_ise.log.workflow.if.process_error")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.WorkflowIfUserNotFound") {
                    event.rename("cisco_ise.log.log_details.WorkflowIfUserNotFound", "cisco_ise.log.workflow.if.user_not_found")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.WorkflowSequenceType") {
                    event.rename("cisco_ise.log.log_details.WorkflowSequenceType", "cisco_ise.log.workflow.sequence.type")?;
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
