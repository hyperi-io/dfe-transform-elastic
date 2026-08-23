// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `no_use_flattened_debug` pipeline.
pub struct NoUseFlattenedDebug;

impl Transform for NoUseFlattenedDebug {
    fn name(&self) -> &str {
        "no_use_flattened_debug"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.debugContext.debugData",
                    "okta.debug_context.debug_data",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(
                    event,
                    "okta.debug_context.debug_data.logOnlySecurityData",
                    "okta.debug_context.debug_data.logOnlySecurityData",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("okta.debug_context.debug_data.behaviors") {
                    if let Some(input) = event.get_string("okta.debug_context.debug_data.behaviors")
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
                                "okta.debug_context.debug_data.behaviors",
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

            let _cond = { event.has_value("okta.debug_context.debug_data.behaviors") };
            if _cond {
                if let Some(kv_str) = event.get_string("okta.debug_context.debug_data.behaviors") {
                    for pair in kv_str.split(", ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "okta.debug_context.debug_data.behaviors".into(),
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
                    .remove("okta.debug_context.debug_data.behaviors")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.debug_context.debug_data.behaviors".into(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("_behaviors_object") {
                    event.rename(
                        "_behaviors_object",
                        "okta.debug_context.debug_data.behaviors",
                    )?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("okta.debug_context.debug_data.risk") };
            if _cond {
                if let Some(v) = event.get("okta.debug_context.debug_data.risk").cloned() {
                    event.set("okta.debug_context.debug_data.risk_object", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("okta.debug_context.debug_data.risk") {
                    if let Some(input) = event.get_string("okta.debug_context.debug_data.risk") {
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
                            captured
                                .push(("okta.debug_context.debug_data.risk", &remaining[..pos]));
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

            let _cond = { event.has_value("okta.debug_context.debug_data.risk") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("okta.debug_context.debug_data.risk") {
                        for pair in kv_str.split(", ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "okta.debug_context.debug_data.risk".into(),
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
                    .remove("okta.debug_context.debug_data.risk_object")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.debug_context.debug_data.risk_object".into(),
                    });
                }
            }

            let _cond = {
                event.has_value("okta.debug_context.debug_data.risk_object")
                    && event.has_value("okta.debug_context.debug_data.risk")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("okta.debug_context.debug_data.risk") {
                        // Grok pattern: level=%{NOTSPACE:_risk_object.level}
                        let _ = cached_grok!("level=%{NOTSPACE:_risk_object.level}")
                            .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("okta.debug_context.debug_data.risk_object")
                    && event.has_value("okta.debug_context.debug_data.risk")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("okta.debug_context.debug_data.risk") {
                        // Grok pattern: reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)
                        // Grok pattern: reasons=%{DATA:_risk_object.reasons}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "reasons=%{DATA:_risk_object.reasons}, (?:%{NOTSPACE}=)"
                                ),
                                cached_grok!("reasons=%{DATA:_risk_object.reasons}$"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_risk_object") };
            if _cond {
                if event.remove("okta.debug_context.debug_data.risk").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "okta.debug_context.debug_data.risk".into(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("_risk_object") {
                    event.rename("_risk_object", "okta.debug_context.debug_data.risk")?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("okta.debug_context.debug_data.logOnlySecurityData.risk.level")
                    && event.get_str("okta.debug_context.debug_data.logOnlySecurityData.risk.level")
                        != Some("")
            };
            if _cond {
                event.set(
                    "okta.debug_context.debug_data.risk_level",
                    json!(
                        event
                            .get("okta.debug_context.debug_data.logOnlySecurityData.risk.level")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("okta.debug_context.debug_data.logOnlySecurityData.risk.reasons")
                    && event
                        .get_str("okta.debug_context.debug_data.logOnlySecurityData.risk.reasons")
                        != Some("")
            };
            if _cond {
                if let Some(s) = event
                    .get_string("okta.debug_context.debug_data.logOnlySecurityData.risk.reasons")
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

            let _cond = {
                !event.has_value("okta.debug_context.debug_data.risk_level")
                    && event.has_value("okta.debug_context.debug_data.risk.level")
                    && event.get_str("okta.debug_context.debug_data.risk.level") != Some("")
            };
            if _cond {
                event.set(
                    "okta.debug_context.debug_data.risk_level",
                    json!(
                        event
                            .get("okta.debug_context.debug_data.risk.level")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("okta.debug_context.debug_data.factor")
                    && event.has_value("okta.debug_context.debug_data.factor")
                    && event.get_str("okta.debug_context.debug_data.factor") != Some("")
            };
            if _cond {
                event.set(
                    "okta.debug_context.debug_data.factor",
                    json!(
                        event
                            .get("okta.debug_context.debug_data.factor")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("okta.debug_context.debug_data.risk_reasons")
                    && event.has_value("okta.debug_context.debug_data.risk.reasons")
                    && event.get_str("okta.debug_context.debug_data.risk.reasons") != Some("")
            };
            if _cond {
                if let Some(s) = event.get_string("okta.debug_context.debug_data.risk.reasons") {
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

            let _cond = { event.has_value("okta.debug_context.debug_data.tunnels") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "okta.debug_context.debug_data.tunnels",
                        "okta.debug_context.debug_data.tunnels",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_okta_debug_context_debug_data_tunnels",
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
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event
                            .remove("okta.debug_context.debug_data.tunnels")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "okta.debug_context.debug_data.tunnels".into(),
                            });
                        }
                        Ok(())
                    })();
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // Painless script
            // Source: def src = ctx.okta?.debug_context?.debug_data?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def src = ctx.okta?.debug_context?.debug_data?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("okta.debug_context.debug_data.deviceFingerprint") {
                    event.rename(
                        "okta.debug_context.debug_data.deviceFingerprint",
                        "okta.debug_context.debug_data.device_fingerprint",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("okta.debug_context.debug_data.dtHash") {
                    event.rename(
                        "okta.debug_context.debug_data.dtHash",
                        "okta.debug_context.debug_data.dt_hash",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("okta.debug_context.debug_data.requestId") {
                    event.rename(
                        "okta.debug_context.debug_data.requestId",
                        "okta.debug_context.debug_data.request_id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("okta.debug_context.debug_data.requestUri") {
                    event.rename(
                        "okta.debug_context.debug_data.requestUri",
                        "okta.debug_context.debug_data.request_uri",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("okta.debug_context.debug_data.threatSuspected") {
                    event.rename(
                        "okta.debug_context.debug_data.threatSuspected",
                        "okta.debug_context.debug_data.threat_suspected",
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

            let _cond = { event.has_value("okta.debug_context.debug_data") };
            if _cond {
                // Painless script
                // Source: String underscore(String s) {\n  return /[ -]/.matcher(s).replaceAll('_');\n}\ndef renameKeys(Map src) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      dst[underscore(key)] = renameKeys(value);\n    } else if (value instanceof List) {\n      for (int i = 0; i < value.length; i++) {\n        if (value[i] instanceof Map) {\n          value[i] = renameKeys(value[i]);\n        }\n      }\n      dst[underscore(key)] = value;\n    } else {\n      dst[underscore(key)] = value;\n    }\n  }\n  return dst;\n}\nctx.okta.debug_context.debug_data = renameKeys(ctx.okta.debug_context.debug_data)\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String underscore(String s) {\n  return /[ -]/.matcher(s).replaceAll('_');\n}\ndef renameKeys(Map src) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      dst[underscore(key)] = renameKeys(value);\n    } else if (value instanceof List) {\n      for (int i = 0; i < value.length; i++) {\n        if (value[i] instanceof Map) {\n          value[i] = renameKeys(value[i]);\n        }\n      }\n      dst[underscore(key)] = value;\n    } else {\n      dst[underscore(key)] = value;\n    }\n  }\n  return dst;\n}\nctx.okta.debug_context.debug_data = renameKeys(ctx.okta.debug_context.debug_data)\n"#
                    ),
                )?;
            }

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
