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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = { event.has_value("json.modifiedTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.modifiedTime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.modifiedTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.baseScore") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.cpe") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.cvssV3Vector") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.cvssVector") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.dependencies") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.description") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.family") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.modifiedTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.name") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.patchPubDate") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.pluginModDate") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.riskFactor") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.sourceFile") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.temporalScore") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.type") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.version") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.vprContext") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.vprScore") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.vulnPubDate") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("event"))?;

            if event.has_value("json.id") {
                event.rename("json.id", "tenable_sc.plugin.id")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "tenable_sc.plugin.name")?;
            }

            if event.has_value("json.description") {
                event.rename("json.description", "tenable_sc.plugin.description")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "tenable_sc.plugin.type")?;
            }

            if event.has_value("json.copyright") {
                event.rename("json.copyright", "tenable_sc.plugin.copyright")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.version") {
                    if let Some(val) = event.get("json.version") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.version".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.plugin.version", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if event.has_value("json.sourceFile") {
                event.rename("json.sourceFile", "tenable_sc.plugin.source_file")?;
            }

            if event.has_value("json.dependencies") {
                if let Some(s) = event.get_string("json.dependencies") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("tenable_sc.plugin.dependencies", Value::Array(parts))?;
                }
            }

            if event.has_value("json.requiredPorts") {
                event.rename("json.requiredPorts", "tenable_sc.plugin.required_ports")?;
            }

            if event.has_value("json.requiredUDPPorts") {
                event.rename(
                    "json.requiredUDPPorts",
                    "tenable_sc.plugin.required_udp_ports",
                )?;
            }

            if event.has_value("json.cpe") {
                if let Some(s) = event.get_string("json.cpe") {
                    let mut parts: Vec<Value> = cached_regex!("\\n")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("tenable_sc.plugin.cpe", Value::Array(parts))?;
                }
            }

            if event.has_value("json.srcPort") {
                event.rename("json.srcPort", "tenable_sc.plugin.src_port")?;
            }

            if event.has_value("json.dstPort") {
                event.rename("json.dstPort", "tenable_sc.plugin.dst_port")?;
            }

            if event.has_value("json.protocol") {
                event.rename("json.protocol", "tenable_sc.plugin.protocol")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("tenable_sc.plugin.protocol").cloned() {
                    event.set("network.transport", v)?;
                }
                Ok(())
            })();

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.riskFactor") {
                event.rename("json.riskFactor", "tenable_sc.plugin.risk_factor")?;
            }

            if event.has_value("json.solution") {
                event.rename("json.solution", "tenable_sc.plugin.solution")?;
            }

            if event.has_value("json.seeAlso") {
                if let Some(s) = event.get_string("json.seeAlso") {
                    let mut parts: Vec<Value> = cached_regex!("\\n")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("tenable_sc.plugin.see_also", Value::Array(parts))?;
                }
            }

            if event.has_value("json.synopsis") {
                event.rename("json.synopsis", "tenable_sc.plugin.synopsis")?;
            }

            let _cond = { event.get_str("json.checkType") != Some("") };
            if _cond {
                if event.has_value("json.checkType") {
                    event.rename("json.checkType", "tenable_sc.plugin.check_type")?;
                }
            }

            if event.has_value("json.exploitEase") {
                event.rename("json.exploitEase", "tenable_sc.plugin.exploit.ease")?;
            }

            if event.has_value("json.exploitAvailable") {
                event.rename(
                    "json.exploitAvailable",
                    "tenable_sc.plugin.exploit.is_available",
                )?;
            }

            if event.has_value("json.exploitFrameworks") {
                event.rename(
                    "json.exploitFrameworks",
                    "tenable_sc.plugin.exploit.frameworks",
                )?;
            }

            let _cond = { event.get_str("json.cvssVector") != Some("") };
            if _cond {
                if event.has_value("json.cvssVector") {
                    event.rename("json.cvssVector", "tenable_sc.plugin.cvss_vector")?;
                }
            }

            let _cond = { event.get_str("json.cvssVectorBF") != Some("0") };
            if _cond {
                if event.has_value("json.cvssVectorBF") {
                    event.rename("json.cvssVectorBF", "tenable_sc.plugin.cvss_vector_bf")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.baseScore") {
                    if let Some(val) = event.get("json.baseScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.baseScore".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.plugin.base_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.temporalScore") {
                    if let Some(val) = event.get("json.temporalScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.temporalScore".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.plugin.temporal_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            let _cond = { event.get_str("json.cvssV3Vector") != Some("") };
            if _cond {
                if event.has_value("json.cvssV3Vector") {
                    event.rename("json.cvssV3Vector", "tenable_sc.plugin.cvssv3_vector")?;
                }
            }

            let _cond = { event.get_str("json.cvssV3VectorBF") != Some("0") };
            if _cond {
                if event.has_value("json.cvssV3VectorBF") {
                    event.rename("json.cvssV3VectorBF", "tenable_sc.plugin.cvssv3_vector_bf")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.cvssV3BaseScore") {
                    if let Some(val) = event.get("json.cvssV3BaseScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cvssV3BaseScore".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.plugin.cvssv3_base_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.cvssV3TemporalScore") {
                    if let Some(val) = event.get("json.cvssV3TemporalScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cvssV3TemporalScore".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.plugin.cvssv3_temporal_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.vprScore") {
                    if let Some(val) = event.get("json.vprScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vprScore".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.plugin.vpr.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "json.vprContext", "json.vprContext")?;
                Ok(())
            })();

            let _cond = { event.has_value("json.vprContext") };
            if _cond {
                // Painless script
                // Source: def parts = ctx.json.vprContext; if (parts != null && parts.length > 0) {\n  Map map = new HashMap();\n  for (int i = 0; i < parts.length; i++) {\n    map.put(parts[i]['id'], parts[i]['value'])\n  }\n  ctx.tenable_sc.plugin.vpr.context = map;\n  ctx.tenable_sc.plugin.vpr.context._original = parts;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def parts = ctx.json.vprContext; if (parts != null && parts.length > 0) {\n  Map map = new HashMap();\n  for (int i = 0; i < parts.length; i++) {\n    map.put(parts[i]['id'], parts[i]['value'])\n  }\n  ctx.tenable_sc.plugin.vpr.context = map;\n  ctx.tenable_sc.plugin.vpr.context._original = parts;\n}"#
                    ),
                )?;
            }

            if event.has_value("json.stigSeverity") {
                event.rename("json.stigSeverity", "tenable_sc.plugin.stig_severity")?;
            }

            let _cond = { event.has_value("json.pluginPubDate") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_plugin_published", json!(false))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.pluginPubDate") != Some("-1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_plugin_published", json!(true))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.pluginPubDate")
                    && event.get_str("json.pluginPubDate") != Some("-1")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.pluginPubDate") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("tenable_sc.plugin.plugin_pub_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.pluginPubDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = { event.has_value("json.pluginModDate") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_plugin_modified", json!(false))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.pluginModDate") != Some("-1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_plugin_modified", json!(true))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.pluginModDate")
                    && event.get_str("json.pluginModDate") != Some("-1")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.pluginModDate") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("tenable_sc.plugin.plugin_mod_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.pluginModDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = { event.has_value("json.patchPubDate") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_patch_published", json!(false))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.patchPubDate") != Some("-1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_patch_published", json!(true))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.patchPubDate")
                    && event.get_str("json.patchPubDate") != Some("-1")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.patchPubDate") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("tenable_sc.plugin.patch_pub_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.patchPubDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = { event.has_value("json.patchModDate") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_patch_modified", json!(false))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.patchModDate") != Some("-1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_patch_modified", json!(true))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.patchModDate")
                    && event.get_str("json.patchModDate") != Some("-1")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.patchModDate") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("tenable_sc.plugin.patch_mod_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.patchModDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = { event.has_value("json.vulnPubDate") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_vulnerability_published", json!(false))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.vulnPubDate") != Some("-1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("tenable_sc.plugin.is_vulnerability_published", json!(true))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.vulnPubDate")
                    && event.get_str("json.vulnPubDate") != Some("-1")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.vulnPubDate") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("tenable_sc.plugin.vuln_pub_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.vulnPubDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = { event.has_value("json.modifiedTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.modifiedTime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("tenable_sc.plugin.modified_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.modifiedTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            if event.has_value("json.md5") {
                event.rename("json.md5", "tenable_sc.plugin.md5")?;
            }

            if event.has_value("json.xrefs") {
                if let Some(s) = event.get_string("json.xrefs") {
                    let mut parts: Vec<Value> = s.split(", ").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("tenable_sc.plugin.xrefs", Value::Array(parts))?;
                }
            }

            if event.has_value("json.source") {
                event.rename("json.source", "tenable_sc.plugin.source")?;
            }

            if event.has_value("json.family.id") {
                event.rename("json.family.id", "tenable_sc.plugin.family.id")?;
            }

            if event.has_value("json.family.name") {
                event.rename("json.family.name", "tenable_sc.plugin.family.name")?;
            }

            if event.has_value("json.family.type") {
                event.rename("json.family.type", "tenable_sc.plugin.family.type")?;
            }

            let _cond = { event.has_value("tenable_sc.plugin.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("tenable_sc.plugin.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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

            event.remove("json");

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
