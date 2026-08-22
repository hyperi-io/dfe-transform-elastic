// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `use_flattened_debug` pipeline.
pub struct UseFlattenedDebug;

impl Transform for UseFlattenedDebug {
    fn name(&self) -> &str {
        "use_flattened_debug"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.debugContext.debugData").cloned() {
                    event.set("okta.debug_context.debug_data.flattened", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) =
                    event.get_string("okta.debug_context.debug_data.flattened.logOnlySecurityData")
                {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "okta.debug_context.debug_data.flattened.logOnlySecurityData"
                                .into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData",
                        parsed,
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("okta.debug_context.debug_data.flattened.behaviors") {
                    if let Some(input) =
                        event.get_string("okta.debug_context.debug_data.flattened.behaviors")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("{") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("}") else {
                                break 'dissect false;
                            };
                            captured.push((
                                "okta.debug_context.debug_data.flattened.behaviors",
                                &remaining[..pos],
                            ));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("}") else {
                                break 'dissect false;
                            };
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

            let _cond = { event.has_value("okta.debug_context.debug_data.flattened.behaviors") };
            if _cond {
                if let Some(kv_str) =
                    event.get_string("okta.debug_context.debug_data.flattened.behaviors")
                {
                    for pair in kv_str.split(", ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "okta.debug_context.debug_data.flattened.behaviors".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                event.set(&format!("_behaviors_object.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("_behaviors_object") };
            if _cond {
                if event
                    .remove("okta.debug_context.debug_data.flattened.behaviors")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.debug_context.debug_data.flattened.behaviors".into(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("_behaviors_object") {
                    event.rename(
                        "_behaviors_object",
                        "okta.debug_context.debug_data.flattened.behaviors",
                    )?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("okta.debug_context.debug_data.flattened.risk") };
            if _cond {
                if let Some(v) = event
                    .get("okta.debug_context.debug_data.flattened.risk")
                    .cloned()
                {
                    event.set("okta.debug_context.debug_data.flattened.risk_object", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("okta.debug_context.debug_data.flattened.risk") {
                    if let Some(input) =
                        event.get_string("okta.debug_context.debug_data.flattened.risk")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("{") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("}") else {
                                break 'dissect false;
                            };
                            captured.push((
                                "okta.debug_context.debug_data.flattened.risk",
                                &remaining[..pos],
                            ));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("}") else {
                                break 'dissect false;
                            };
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

            let _cond = { event.has_value("okta.debug_context.debug_data.flattened.risk") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(kv_str) =
                        event.get_string("okta.debug_context.debug_data.flattened.risk")
                    {
                        for pair in kv_str.split(", ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "okta.debug_context.debug_data.flattened.risk".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    event.set(&format!("_risk_object.{}", key), value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "kv")?;
                    event.set("_ingest.on_failure_processor_tag", "kv_risk")?;
                    if event.remove("_risk_object").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_risk_object".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_risk_object") };
            if _cond {
                if event
                    .remove("okta.debug_context.debug_data.flattened.risk_object")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.debug_context.debug_data.flattened.risk_object".into(),
                    });
                }
            }

            let _cond = {
                event.has_value("okta.debug_context.debug_data.flattened.risk_object")
                    && event.has_value("okta.debug_context.debug_data.flattened.risk")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) =
                        event.get_string("okta.debug_context.debug_data.flattened.risk")
                    {
                        // Grok pattern: level=%{NOTSPACE:_risk_object.level}
                        if !cached_grok!("level=%{NOTSPACE:_risk_object.level}")
                            .extract_into(&input, event)?
                        {}
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("okta.debug_context.debug_data.flattened.risk_object")
                    && event.has_value("okta.debug_context.debug_data.flattened.risk")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) =
                        event.get_string("okta.debug_context.debug_data.flattened.risk")
                    {
                        // Grok pattern: reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)
                        if !cached_grok!("reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)")
                            .extract_into(&input, event)?
                        {
                            // Grok pattern: reasons=%{DATA:_risk_object.reasons}$
                            if !cached_grok!("reasons=%{DATA:_risk_object.reasons}$")
                                .extract_into(&input, event)?
                            {}
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_risk_object") };
            if _cond {
                if event
                    .remove("okta.debug_context.debug_data.flattened.risk")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.debug_context.debug_data.flattened.risk".into(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("_risk_object") {
                    event.rename(
                        "_risk_object",
                        "okta.debug_context.debug_data.flattened.risk",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.deviceFingerprint") {
                    event.rename(
                        "json.debugContext.debugData.deviceFingerprint",
                        "okta.debug_context.debug_data.device_fingerprint",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.requestId") {
                    event.rename(
                        "json.debugContext.debugData.requestId",
                        "okta.debug_context.debug_data.request_id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.requestUri") {
                    event.rename(
                        "json.debugContext.debugData.requestUri",
                        "okta.debug_context.debug_data.request_uri",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.threatSuspected") {
                    event.rename(
                        "json.debugContext.debugData.threatSuspected",
                        "okta.debug_context.debug_data.threat_suspected",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.url") {
                    event.rename(
                        "json.debugContext.debugData.url",
                        "okta.debug_context.debug_data.url",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.dtHash") {
                    event.rename(
                        "json.debugContext.debugData.dtHash",
                        "okta.debug_context.debug_data.dt_hash",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.clientSecret") {
                    event.rename(
                        "json.debugContext.debugData.clientSecret",
                        "okta.debug_context.debug_data.client_secret",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.requestedScopes") {
                    event.rename(
                        "json.debugContext.debugData.requestedScopes",
                        "okta.debug_context.debug_data.requested_scopes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.grantedScopes") {
                    event.rename(
                        "json.debugContext.debugData.grantedScopes",
                        "okta.debug_context.debug_data.granted_scopes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.debugContext.debugData.grantType") {
                    event.rename(
                        "json.debugContext.debugData.grantType",
                        "okta.debug_context.debug_data.grant_type",
                    )?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value(
                    "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level",
                ) && event.get_str(
                    "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level",
                ) != Some("")
            };
            if _cond {
                event.set("okta.debug_context.debug_data.risk_level", json!(event.get("okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value(
                    "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
                ) && event.get_str(
                    "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
                ) != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string(
                    "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
                ) {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set(
                        "okta.debug_context.debug_data.risk_reasons",
                        Value::Array(parts),
                    )?;
                }
            }

            let _cond = {
                !event.has_value("okta.debug_context.debug_data.risk_level")
                    && event.has_value("okta.debug_context.debug_data.flattened.risk.level")
                    && event.get_str("okta.debug_context.debug_data.flattened.risk.level")
                        != Some("")
            };
            if _cond {
                event.set(
                    "okta.debug_context.debug_data.risk_level",
                    json!(
                        event
                            .get("okta.debug_context.debug_data.flattened.risk.level")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("okta.debug_context.debug_data.factor")
                    && event.has_value("okta.debug_context.debug_data.flattened.factor")
                    && event.get_str("okta.debug_context.debug_data.flattened.factor") != Some("")
            };
            if _cond {
                event.set(
                    "okta.debug_context.debug_data.factor",
                    json!(
                        event
                            .get("okta.debug_context.debug_data.flattened.factor")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("okta.debug_context.debug_data.risk_reasons")
                    && event.has_value("okta.debug_context.debug_data.flattened.risk.reasons")
                    && event.get_str("okta.debug_context.debug_data.flattened.risk.reasons")
                        != Some("")
            };
            if _cond {
                if let Some(s) =
                    event.get_string("okta.debug_context.debug_data.flattened.risk.reasons")
                {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set(
                        "okta.debug_context.debug_data.risk_reasons",
                        Value::Array(parts),
                    )?;
                }
            }

            let _cond = { event.has_value("okta.debug_context.debug_data.flattened.tunnels") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) =
                        event.get_string("okta.debug_context.debug_data.flattened.tunnels")
                    {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "okta.debug_context.debug_data.flattened.tunnels".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("okta.debug_context.debug_data.flattened.tunnels", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_okta_debug_context_debug_data_flattened_tunnels",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            // Painless script
            // Source: def src = ctx.okta?.debug_context?.debug_data?.flattened?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"def src = ctx.okta?.debug_context?.debug_data?.flattened?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
