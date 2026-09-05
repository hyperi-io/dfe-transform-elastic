// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_policy_diagnostics` pipeline.
pub struct PipelinePolicyDiagnostics;

impl Transform for PipelinePolicyDiagnostics {
    fn name(&self) -> &str {
        "pipeline_policy_diagnostics"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("configuration"))?;

            event.append("event.type", json!("info"))?;

            let _cond = { event.get_i64("cisco_ise.log.segment.number") == Some(0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.has_value("cisco_ise.log.segment.number")
                    && event
                        .get_i64("cisco_ise.log.segment.number")
                        .is_some_and(|n| n > 0)
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{GREEDYDATA:cisco_ise.log.log_details_raw},")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSS",
                            "yyyy-MM-dd HH:mm:ss.SSSSSS",
                            "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]",
                        ],
                        None,
                        None,
                    ) {
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
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "date__tmp_timestamp_9ef85c6a",
                )?;
                event.remove("_tmp.timestamp");
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("event.timezone") && event.get_str("event.timezone") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSS",
                                "yyyy-MM-dd HH:mm:ss.SSSSSS",
                                "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__tmp_timestamp_1d2a12b9",
                    )?;
                    event.remove("_tmp.timestamp");
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                                kv_put(
                                    event,
                                    &format!("cisco_ise.log.log_details.{}", key),
                                    value,
                                )?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("cisco_ise.log.message.description")
                    && event.get_str("cisco_ise.log.message.description") != Some("")
            };
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                    if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.log_details.Device IP Address".into(),
                                message,
                            }
                        })?;
                        event.set("client.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cisco_ise_log_log_details_Device_IP_Address_to_client_ip_b34586ce",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("cisco_ise.log.log_details.Protocol") {
                event.rename("cisco_ise.log.log_details.Protocol", "network.protocol")?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.RequestReceivedTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cisco_ise.log.log_details.RequestReceivedTime")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("cisco_ise.log.request.received_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.RequestReceivedTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_log_details_RequestReceivedTime_to_cisco_ise_log_request_received_time_c877ce0d")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.remove("cisco_ise.log.log_details.RequestReceivedTime");

            if event.has_value("cisco_ise.log.log_details.PolicyType") {
                event.rename(
                    "cisco_ise.log.log_details.PolicyType",
                    "cisco_ise.log.policy.type",
                )?;
            }

            if event.has_value("cisco_ise.log.log_details.AcsSessionID") {
                event.rename(
                    "cisco_ise.log.log_details.AcsSessionID",
                    "cisco_ise.log.acs.session.id",
                )?;
            }

            if event.has_value("cisco_ise.log.log_details.AuthorizationPolicyMatchedRule") {
                event.rename(
                    "cisco_ise.log.log_details.AuthorizationPolicyMatchedRule",
                    "cisco_ise.log.auth.policy.matched.rule",
                )?;
            }

            if event.has_value("cisco_ise.log.log_details.CurrentIDStoreName") {
                event.rename(
                    "cisco_ise.log.log_details.CurrentIDStoreName",
                    "cisco_ise.log.currentid.store_name",
                )?;
            }

            if event.has_value("cisco_ise.log.log_details.ISEPolicySetName") {
                event.rename(
                    "cisco_ise.log.log_details.ISEPolicySetName",
                    "cisco_ise.log.ise.policy.set_name",
                )?;
            }

            if event.has_value("cisco_ise.log.log_details.IdentityPolicyMatchedRule") {
                event.rename(
                    "cisco_ise.log.log_details.IdentityPolicyMatchedRule",
                    "cisco_ise.log.identity.policy.matched.rule",
                )?;
            }

            if event.has_value("cisco_ise.log.log_details.IdentitySelectionMatchedRule") {
                event.rename(
                    "cisco_ise.log.log_details.IdentitySelectionMatchedRule",
                    "cisco_ise.log.identity.selection.matched.rule",
                )?;
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "user.name",
                        json!(
                            event
                                .get("cisco_ise.log.log_details.OriginalUserName")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("cisco_ise.log.log_details.OriginalUserName")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.remove("cisco_ise.log.log_details.OriginalUserName");

            if event.has_value("cisco_ise.log.log_details.SelectedAccessService") {
                event.rename(
                    "cisco_ise.log.log_details.SelectedAccessService",
                    "cisco_ise.log.selected.access.service",
                )?;
            }

            if event.has_value("cisco_ise.log.log_details.SelectedAuthorizationProfiles") {
                event.rename(
                    "cisco_ise.log.log_details.SelectedAuthorizationProfiles",
                    "cisco_ise.log.selected.authorization.profiles",
                )?;
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "user.name",
                        json!(
                            event
                                .get("cisco_ise.log.log_details.UserName")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("cisco_ise.log.log_details.UserName")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.remove("cisco_ise.log.log_details.UserName");

            if event.has_value("cisco_ise.log.log_details.CPMSessionID") {
                event.rename(
                    "cisco_ise.log.log_details.CPMSessionID",
                    "cisco_ise.log.cpm.session.id",
                )?;
            }

            event.remove("_tmp");
            event.remove("cisco_ise.log.log_details_raw");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
