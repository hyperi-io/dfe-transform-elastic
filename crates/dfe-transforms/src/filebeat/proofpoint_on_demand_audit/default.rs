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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.guid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.ts") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.audit?.action == null || params.get(ctx.json.audit.action.toLowerCase()) == null) {\n  return;\n}\nparams.get(ctx.json.audit.action.toLowerCase()).forEach((k, v) -> {\n  if (v instanceof List) {\n    ctx.event[k] = new ArrayList(v);\n  } else {\n    ctx.event[k] = v;\n  }\n});
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"if (ctx.json?.audit?.action == null || params.get(ctx.json.audit.action.toLowerCase()) == null) {\n  return;\n}\nparams.get(ctx.json.audit.action.toLowerCase()).forEach((k, v) -> {\n  if (v instanceof List) {\n    ctx.event[k] = new ArrayList(v);\n  } else {\n    ctx.event[k] = v;\n  }\n});"#
                    ),
                    cached_params!(
                        "{\"create\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"delete\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"download\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"edit\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"execute\":{\"category\":[\"configuration\"],\"type\":[\"access\",\"info\"]},\"login\":{\"category\":[\"authentication\"],\"type\":[\"start\"]},\"logout\":{\"category\":[\"authentication\"],\"type\":[\"end\"]},\"read\":{\"category\":[\"configuration\"],\"type\":[\"access\",\"info\"]}}"
                    ),
                )?;
                Ok(())
            })();

            event.set("observer.vendor", json!("Proofpoint"))?;

            event.set("observer.product", json!("Proofpoint On Demand"))?;

            if event.has("json.audit.action") {
                event.rename("json.audit.action", "proofpoint_on_demand.audit.action")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let lowered = s.to_lowercase();
                    event.set("event.action", lowered)?;
                }
            }

            if event.has("json.guid") {
                event.rename("json.guid", "proofpoint_on_demand.audit.guid")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.guid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has("json.audit.level") {
                event.rename("json.audit.level", "proofpoint_on_demand.audit.level")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.level")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.level", v)?;
            }

            if event.has("json.metadata.customerId") {
                event.rename(
                    "json.metadata.customerId",
                    "proofpoint_on_demand.audit.metadata.customer_id",
                )?;
            }

            let _cond = { event.get_str("json.metadata.origin.data.agent") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.metadata.origin.data.agent") {
                        if let Some(val) = event.get("json.metadata.origin.data.agent") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.metadata.origin.data.agent".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "proofpoint_on_demand.audit.metadata.origin.data.agent_ip",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_metadata_origin_data_agent_to_ip",
                    )?;
                    if event.has("json.metadata.origin.data.agent") {
                        event.rename(
                            "json.metadata.origin.data.agent",
                            "proofpoint_on_demand.audit.metadata.origin.data.agent",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.metadata.origin.data.agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond =
                { event.has_value("proofpoint_on_demand.audit.metadata.origin.data.agent") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.metadata.origin.data.agent")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("proofpoint_on_demand.audit.metadata.origin.data.agent_ip") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.metadata.origin.data.agent_ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("proofpoint_on_demand.audit.metadata.origin.data.agent_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.metadata.origin.data.agent_ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.metadata.origin.data.cid") {
                event.rename(
                    "json.metadata.origin.data.cid",
                    "proofpoint_on_demand.audit.metadata.origin.data.cid",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.metadata.origin.data.cid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if event.has_value("json.metadata.origin.data.version") {
                if let Some(val) = event.get("json.metadata.origin.data.version") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.metadata.origin.data.version".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "proofpoint_on_demand.audit.metadata.origin.data.version",
                        converted,
                    )?;
                }
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.metadata.origin.data.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            if event.has_value("json.metadata.origin.schemaVersion") {
                if let Some(val) = event.get("json.metadata.origin.schemaVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.metadata.origin.schemaVersion".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "proofpoint_on_demand.audit.metadata.origin.schema_version",
                        converted,
                    )?;
                }
            }

            if event.has("json.metadata.origin.type") {
                event.rename(
                    "json.metadata.origin.type",
                    "proofpoint_on_demand.audit.metadata.origin.type",
                )?;
            }

            let _cond = {
                event
                    .get("json.metadata.trace")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.metadata.trace").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.ts") {
                                if let Some(parsed) =
                                    parse_date_out(&date_str, &["ISO8601"], None, None)
                                {
                                    event.set("_ingest._value.ts", parsed)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_metadata_trace_ts",
                            )?;
                            event.remove("_ingest._value.ts");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.metadata.trace", Value::Array(out))?;
                }
            }

            if event.has("json.metadata.trace") {
                event.rename(
                    "json.metadata.trace",
                    "proofpoint_on_demand.audit.metadata.trace",
                )?;
            }

            if event.has("json.audit.resourceName") {
                event.rename(
                    "json.audit.resourceName",
                    "proofpoint_on_demand.audit.resource_name",
                )?;
            }

            if event.has("json.audit.resourceType") {
                event.rename(
                    "json.audit.resourceType",
                    "proofpoint_on_demand.audit.resource_type",
                )?;
            }

            if event.has("json.audit.service.cid") {
                event.rename(
                    "json.audit.service.cid",
                    "proofpoint_on_demand.audit.service.cid",
                )?;
            }

            if event.has("json.audit.service.customerId") {
                event.rename(
                    "json.audit.service.customerId",
                    "proofpoint_on_demand.audit.service.customer_id",
                )?;
            }

            if event.has("json.audit.service.id") {
                event.rename(
                    "json.audit.service.id",
                    "proofpoint_on_demand.audit.service.id",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.service.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.id", v)?;
            }

            let _cond = { event.get_str("json.audit.service.ipAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.audit.service.ipAddress") {
                        if let Some(val) = event.get("json.audit.service.ipAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.audit.service.ipAddress".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("proofpoint_on_demand.audit.service.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_audit_service_ipAddress_to_ip",
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

            let _cond = { event.has_value("proofpoint_on_demand.audit.service.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.service.ip_address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("json.audit.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.audit.tags").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.value") {
                            if let Some(val) = event.get("_ingest._value.value") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.value".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.value", converted)?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.audit.tags", Value::Array(out))?;
                }
            }

            if event.has("json.audit.tags") {
                event.rename("json.audit.tags", "proofpoint_on_demand.audit.tags")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.tags")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("labels", v)?;
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.audit.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.put(\"labels\", new HashMap()); for (tag in ctx.proofpoint_on_demand.audit.tags) {\n  ctx.labels.put(tag.name, tag.value);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"ctx.put(\"labels\", new HashMap()); for (tag in ctx.proofpoint_on_demand.audit.tags) {\n  ctx.labels.put(tag.name, tag.value);\n}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.audit.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.proofpoint_on_demand.audit.action == null) {\n  return;\n} String action = ctx.proofpoint_on_demand.audit.action + '.'; String outcome; for (tag in ctx.proofpoint_on_demand.audit.tags) {\n  if (tag.name.startsWith(action)) {\n    if (tag.value.toLowerCase() == \"true\") {\n      outcome = \"success\";\n    } else if (tag.value.toLowerCase() == \"false\") {\n      outcome = \"failure\";\n    }\n  }\n} ctx.action.put(\"outcome\", outcome);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"if (ctx.proofpoint_on_demand.audit.action == null) {\n  return;\n} String action = ctx.proofpoint_on_demand.audit.action + '.'; String outcome; for (tag in ctx.proofpoint_on_demand.audit.tags) {\n  if (tag.name.startsWith(action)) {\n    if (tag.value.toLowerCase() == \"true\") {\n      outcome = \"success\";\n    } else if (tag.value.toLowerCase() == \"false\") {\n      outcome = \"failure\";\n    }\n  }\n} ctx.action.put(\"outcome\", outcome);"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.ts") && event.get_str("json.ts") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ts") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("proofpoint_on_demand.audit.ts", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ts")?;
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

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has("json.audit.user.email") {
                event.rename(
                    "json.audit.user.email",
                    "proofpoint_on_demand.audit.user.email",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.audit.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.user.email")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.audit.user.id") {
                event.rename("json.audit.user.id", "proofpoint_on_demand.audit.user.id")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.audit.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.user.id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.audit.user.ipAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.audit.user.ipAddress") {
                        if let Some(val) = event.get("json.audit.user.ipAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.audit.user.ipAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("proofpoint_on_demand.audit.user.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_audit_user_ipAddress_to_ip",
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

            if let Some(v) = event
                .get("proofpoint_on_demand.audit.user.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.audit.user.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.user.ip_address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has("audit.user.roleAssigned") {
                event.rename(
                    "audit.user.roleAssigned",
                    "proofpoint_on_demand.audit.user.roles_assigned",
                )?;
            }

            if event.has("json.audit.user.rolesAssigned") {
                event.rename(
                    "json.audit.user.rolesAssigned",
                    "proofpoint_on_demand.audit.user.roles_assigned",
                )?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.audit.user.roles_assigned") };
            if _cond {
                event.append_unique(
                    "source.user.roles",
                    json!(
                        event
                            .get("proofpoint_on_demand.audit.user.roles_assigned")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("proofpoint_on_demand.audit.action");
                event.remove("proofpoint_on_demand.audit.guid");
                event.remove("proofpoint_on_demand.audit.metadata.origin.data.agent");
                event.remove("proofpoint_on_demand.audit.metadata.origin.data.agent_ip");
                event.remove("proofpoint_on_demand.audit.metadata.origin.data.cid");
                event.remove("proofpoint_on_demand.audit.metadata.origin.data.version");
                event.remove("proofpoint_on_demand.audit.service.id");
                event.remove("proofpoint_on_demand.audit.ts");
                event.remove("proofpoint_on_demand.audit.user.email");
                event.remove("proofpoint_on_demand.audit.user.id");
                event.remove("proofpoint_on_demand.audit.user.ip_address");
                event.remove("proofpoint_on_demand.audit.user.roles_assigned");
                event.remove("proofpoint_on_demand.audit.level");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
