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

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("state"))?;

            event.set("vulnerability.scanner.vendor", json!("Tenable"))?;

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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                event.has_value("json.data.plugin_details")
                    && event
                        .get("json.data.plugin_details")
                        .is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.attributes.plugin_modification_date") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.get_str("json.id") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.id") {
                        if let Some(val) = event.get("json.id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.id".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.id", converted)?;
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
            }

            if event.has_value("json.name") {
                event.rename("json.name", "tenable_io.plugin.name")?;
            }

            let _cond = {
                event.has_value("json.attributes.plugin_modification_date")
                    && event.get_str("json.attributes.plugin_modification_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.plugin_modification_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "tenable_io.plugin.attributes.plugin.modification_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.plugin_modification_date".into(),
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

            if let Some(v) = event
                .get("tenable_io.plugin.attributes.plugin.modification_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.attributes.plugin_version") {
                event.rename(
                    "json.attributes.plugin_version",
                    "tenable_io.plugin.attributes.plugin.version",
                )?;
            }

            let _cond = { event.get_str("json.attributes.exploited_by_malware") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploited_by_malware") {
                        if let Some(val) = event.get("json.attributes.exploited_by_malware") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploited_by_malware".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.exploited_by.malware",
                                converted,
                            )?;
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
            }

            if event.has_value("json.attributes.description") {
                event.rename(
                    "json.attributes.description",
                    "tenable_io.plugin.attributes.description",
                )?;
            }

            let _cond = { event.get_str("json.id") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.unsupported_by_vendor") {
                        if let Some(val) = event.get("json.attributes.unsupported_by_vendor") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.unsupported_by_vendor".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.unsupported_by_vendor",
                                converted,
                            )?;
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
            }

            let _cond = { event.get_str("json.attributes.cvss_temporal_score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.cvss_temporal_score") {
                        if let Some(val) = event.get("json.attributes.cvss_temporal_score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.cvss_temporal_score".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.cvss.temporal.score",
                                converted,
                            )?;
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
            }

            let _cond = {
                event.has_value("json.attributes.patch_publication_date")
                    && event.get_str("json.attributes.patch_publication_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.patch_publication_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "tenable_io.plugin.attributes.patch_publication_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.patch_publication_date".into(),
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

            if event.has_value("json.attributes.see_also") {
                event.rename(
                    "json.attributes.see_also",
                    "tenable_io.plugin.attributes.see_also",
                )?;
            }

            if let Some(v) = event
                .get("tenable_io.plugin.attributes.see_also")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.reference", v)?;
            }

            let _cond = { event.get_str("json.attributes.default_account") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.default_account") {
                        if let Some(val) = event.get("json.attributes.default_account") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.default_account".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.default_account", converted)?;
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
            }

            let _cond = { event.get_str("json.attributes.exploit_available") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploit_available") {
                        if let Some(val) = event.get("json.attributes.exploit_available") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploit_available".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("tenable_io.plugin.attributes.exploit_available", converted)?;
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
            }

            if event.has_value("json.attributes.cve") {
                event.rename("json.attributes.cve", "tenable_io.plugin.attributes.cve")?;
            }

            if let Some(v) = event
                .get("tenable_io.plugin.attributes.cve")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            let _cond = { event.get_str("json.attributes.cvss_base_score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.cvss_base_score") {
                        if let Some(val) = event.get("json.attributes.cvss_base_score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.cvss_base_score".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.cvss.base_score", converted)?;
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
            }

            let _cond = { event.get_str("json.attributes.exploit_framework_canvas") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploit_framework_canvas") {
                        if let Some(val) = event.get("json.attributes.exploit_framework_canvas") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploit_framework_canvas".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.exploit_framework.canvas",
                                converted,
                            )?;
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
            }

            if event.has_value("json.attributes.solution") {
                event.rename(
                    "json.attributes.solution",
                    "tenable_io.plugin.attributes.solution",
                )?;
            }

            if event.has_value("json.attributes.cvss_vector.raw") {
                event.rename(
                    "json.attributes.cvss_vector.raw",
                    "tenable_io.plugin.attributes.cvss.vector.raw",
                )?;
            }

            if event.has_value("json.attributes.cvss_vector.AccessVector") {
                event.rename(
                    "json.attributes.cvss_vector.AccessVector",
                    "tenable_io.plugin.attributes.cvss.vector.access.vector",
                )?;
            }

            if event.has_value("json.attributes.cvss_vector.Availability-Impact") {
                event.rename(
                    "json.attributes.cvss_vector.Availability-Impact",
                    "tenable_io.plugin.attributes.cvss.vector.availability_impact",
                )?;
            }

            if event.has_value("json.attributes.cvss_vector.Authentication") {
                event.rename(
                    "json.attributes.cvss_vector.Authentication",
                    "tenable_io.plugin.attributes.cvss.vector.authentication",
                )?;
            }

            if event.has_value("json.attributes.cvss_vector.Integrity-Impact") {
                event.rename(
                    "json.attributes.cvss_vector.Integrity-Impact",
                    "tenable_io.plugin.attributes.cvss.vector.integrity_impact",
                )?;
            }

            if event.has_value("json.attributes.cvss_vector.AccessComplexity") {
                event.rename(
                    "json.attributes.cvss_vector.AccessComplexity",
                    "tenable_io.plugin.attributes.cvss.vector.access.complexity",
                )?;
            }

            if event.has_value("json.attributes.cvss_vector.Confidentiality-Impact") {
                event.rename(
                    "json.attributes.cvss_vector.Confidentiality-Impact",
                    "tenable_io.plugin.attributes.cvss.vector.confidentiality_impact",
                )?;
            }

            let _cond =
                { event.get_str("json.attributes.exploit_framework_exploithub") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploit_framework_exploithub") {
                        if let Some(val) = event.get("json.attributes.exploit_framework_exploithub")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploit_framework_exploithub".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.exploit_framework.hub",
                                converted,
                            )?;
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
            }

            if event.has_value("json.attributes.cpe") {
                event.rename("json.attributes.cpe", "tenable_io.plugin.attributes.cpe")?;
            }

            let _cond = {
                event.has_value("json.attributes.plugin_publication_date")
                    && event.get_str("json.attributes.plugin_publication_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.plugin_publication_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "tenable_io.plugin.attributes.plugin.publication_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.plugin_publication_date".into(),
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

            let _cond = { event.get_str("json.attributes.exploit_framework_core") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploit_framework_core") {
                        if let Some(val) = event.get("json.attributes.exploit_framework_core") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploit_framework_core".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.exploit_framework.core",
                                converted,
                            )?;
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
            }

            let _cond = { event.get_str("json.attributes.in_the_news") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.in_the_news") {
                        if let Some(val) = event.get("json.attributes.in_the_news") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.in_the_news".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.in_the_news", converted)?;
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
            }

            let _cond = { event.get_str("json.attributes.has_patch") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.has_patch") {
                        if let Some(val) = event.get("json.attributes.has_patch") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.has_patch".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.has_patch", converted)?;
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
            }

            if event.has_value("json.attributes.xref") {
                event.rename("json.attributes.xref", "tenable_io.plugin.attributes.xref")?;
            }

            let _cond = { event.get_str("json.attributes.malware") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.malware") {
                        if let Some(val) = event.get("json.attributes.malware") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.malware".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.malware", converted)?;
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
            }

            let _cond =
                { event.get_str("json.attributes.exploit_framework_d2_elliot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploit_framework_d2_elliot") {
                        if let Some(val) = event.get("json.attributes.exploit_framework_d2_elliot")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploit_framework_d2_elliot".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.exploit_framework.d2_elliot",
                                converted,
                            )?;
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
            }

            if event.has_value("json.attributes.xrefs") {
                event.rename(
                    "json.attributes.xrefs",
                    "tenable_io.plugin.attributes.xrefs",
                )?;
            }

            if event.has_value("json.attributes.risk_factor") {
                event.rename(
                    "json.attributes.risk_factor",
                    "tenable_io.plugin.attributes.risk_factor",
                )?;
            }

            if event.has_value("json.attributes.synopsis") {
                event.rename(
                    "json.attributes.synopsis",
                    "tenable_io.plugin.attributes.synopsis",
                )?;
            }

            let _cond = { event.get_str("json.attributes.cvss3_temporal_score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.cvss3_temporal_score") {
                        if let Some(val) = event.get("json.attributes.cvss3_temporal_score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.cvss3_temporal_score".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.cvss3.temporal.score",
                                converted,
                            )?;
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
            }

            if let Some(v) = event
                .get("tenable_io.plugin.attributes.cvss3.temporal.score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.temporal", v)?;
            }

            let _cond = { event.get_str("json.attributes.exploited_by_nessus") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploited_by_nessus") {
                        if let Some(val) = event.get("json.attributes.exploited_by_nessus") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploited_by_nessus".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.exploited_by.nessus",
                                converted,
                            )?;
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
            }

            let _cond = { event.get_str("json.attributes.cvss3_base_score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.cvss3_base_score") {
                        if let Some(val) = event.get("json.attributes.cvss3_base_score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.cvss3_base_score".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("tenable_io.plugin.attributes.cvss3.base_score", converted)?;
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
            }

            if let Some(v) = event
                .get("tenable_io.plugin.attributes.cvss3.base_score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.base", v)?;
            }

            let _cond =
                { event.get_str("json.attributes.exploit_framework_metasploit") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.exploit_framework_metasploit") {
                        if let Some(val) = event.get("json.attributes.exploit_framework_metasploit")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.exploit_framework_metasploit".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.exploit_framework.metasploit",
                                converted,
                            )?;
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
            }

            if event.has_value("json.attributes.plugin_type") {
                event.rename(
                    "json.attributes.plugin_type",
                    "tenable_io.plugin.attributes.plugin.type",
                )?;
            }

            let _cond = { event.get_str("json.attributes.vpr.score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.vpr.score") {
                        if let Some(val) = event.get("json.attributes.vpr.score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.vpr.score".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.vpr.score", converted)?;
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
            }

            let _cond = {
                event.has_value("json.attributes.vpr.updated")
                    && event.get_str("json.attributes.vpr.updated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.vpr.updated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("tenable_io.plugin.attributes.vpr.updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.vpr.updated".into(),
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

            let _cond = {
                event.get_str("json.attributes.vpr.drivers.age_of_vuln.lower_bound") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.vpr.drivers.age_of_vuln.lower_bound") {
                        if let Some(val) =
                            event.get("json.attributes.vpr.drivers.age_of_vuln.lower_bound")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.vpr.drivers.age_of_vuln.lower_bound"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.vpr.drivers.age_of_vuln.lower_bound",
                                converted,
                            )?;
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
            }

            let _cond = {
                event.get_str("json.attributes.vpr.drivers.age_of_vuln.upper_bound") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.vpr.drivers.age_of_vuln.upper_bound") {
                        if let Some(val) =
                            event.get("json.attributes.vpr.drivers.age_of_vuln.upper_bound")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.vpr.drivers.age_of_vuln.upper_bound"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.vpr.drivers.age_of_vuln.upper_bound",
                                converted,
                            )?;
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
            }

            if event.has_value("json.attributes.vpr.drivers.exploit_code_maturity") {
                event.rename(
                    "json.attributes.vpr.drivers.exploit_code_maturity",
                    "tenable_io.plugin.attributes.vpr.drivers.exploit_code_maturity",
                )?;
            }

            let _cond =
                { event.get_str("json.attributes.vpr.drivers.cvss3_impact_score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.vpr.drivers.cvss3_impact_score") {
                        if let Some(val) =
                            event.get("json.attributes.vpr.drivers.cvss3_impact_score")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.vpr.drivers.cvss3_impact_score".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "tenable_io.plugin.attributes.vpr.drivers.cvss3_impact_score",
                                converted,
                            )?;
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
            }

            let _cond = {
                event.get_str("json.attributes.vpr.drivers.cvss_impact_score_predicted") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.vpr.drivers.cvss_impact_score_predicted") {
                        if let Some(val) =
                            event.get("json.attributes.vpr.drivers.cvss_impact_score_predicted")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.vpr.drivers.cvss_impact_score_predicted"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.vpr.drivers.cvss_impact_score_predicted", converted)?;
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
            }

            if event.has_value("json.attributes.vpr.drivers.threat_intensity_last28") {
                event.rename(
                    "json.attributes.vpr.drivers.threat_intensity_last28",
                    "tenable_io.plugin.attributes.vpr.drivers.threat_intensity_last28",
                )?;
            }

            let _cond = {
                event.get_str("json.attributes.vpr.drivers.threat_recency.lower_bound") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.vpr.drivers.threat_recency.lower_bound") {
                        if let Some(val) =
                            event.get("json.attributes.vpr.drivers.threat_recency.lower_bound")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.vpr.drivers.threat_recency.lower_bound"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.vpr.drivers.threat_recency.lower_bound", converted)?;
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
            }

            let _cond = {
                event.get_str("json.attributes.vpr.drivers.threat_recency.upper_bound") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.vpr.drivers.threat_recency.upper_bound") {
                        if let Some(val) =
                            event.get("json.attributes.vpr.drivers.threat_recency.upper_bound")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.vpr.drivers.threat_recency.upper_bound"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.vpr.drivers.threat_recency.upper_bound", converted)?;
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
            }

            if event.has_value("json.attributes.vpr.drivers.threat_sources_last28") {
                event.rename(
                    "json.attributes.vpr.drivers.threat_sources_last28",
                    "tenable_io.plugin.attributes.vpr.drivers.threat_sources_last28",
                )?;
            }

            if event.has_value("json.attributes.vpr.drivers.product_coverage") {
                event.rename(
                    "json.attributes.vpr.drivers.product_coverage",
                    "tenable_io.plugin.attributes.vpr.drivers.product_coverage",
                )?;
            }

            if event.has_value("json.attributes.intel_type") {
                event.rename(
                    "json.attributes.intel_type",
                    "tenable_io.plugin.attributes.intel_type",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.vuln_publication_date")
                    && event.get_str("json.attributes.vuln_publication_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.vuln_publication_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "tenable_io.plugin.attributes.vuln_publication_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.vuln_publication_date".into(),
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

            let _cond = { event.get_str("json.attributes.always_run") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.always_run") {
                        if let Some(val) = event.get("json.attributes.always_run") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.always_run".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.always_run", converted)?;
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
            }

            let _cond = { event.get_str("json.attributes.compliance") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.compliance") {
                        if let Some(val) = event.get("json.attributes.compliance") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.compliance".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.compliance", converted)?;
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
            }

            if event.has_value("json.attributes.cvss_temporal_vector.raw") {
                event.rename(
                    "json.attributes.cvss_temporal_vector.raw",
                    "tenable_io.plugin.attributes.cvss.temporal.vector.raw",
                )?;
            }

            if event.has_value("json.attributes.cvss_temporal_vector.Exploitability") {
                event.rename(
                    "json.attributes.cvss_temporal_vector.Exploitability",
                    "tenable_io.plugin.attributes.cvss.temporal.vector.exploitability",
                )?;
            }

            if event.has_value("json.attributes.cvss_temporal_vector.RemediationLevel") {
                event.rename(
                    "json.attributes.cvss_temporal_vector.RemediationLevel",
                    "tenable_io.plugin.attributes.cvss.temporal.vector.remediation_level",
                )?;
            }

            if event.has_value("json.attributes.cvss_temporal_vector.ReportConfidence") {
                event.rename(
                    "json.attributes.cvss_temporal_vector.ReportConfidence",
                    "tenable_io.plugin.attributes.cvss.temporal.vector.report_confidence",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.raw") {
                event.rename(
                    "json.attributes.cvss3_vector.raw",
                    "tenable_io.plugin.attributes.cvss3.vector.raw",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.AttackVector") {
                event.rename(
                    "json.attributes.cvss3_vector.AttackVector",
                    "tenable_io.plugin.attributes.cvss3.vector.attack.vector",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.AttackComplexity") {
                event.rename(
                    "json.attributes.cvss3_vector.AttackComplexity",
                    "tenable_io.plugin.attributes.cvss3.vector.attack.complexity",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.PrivilegesRequired") {
                event.rename(
                    "json.attributes.cvss3_vector.PrivilegesRequired",
                    "tenable_io.plugin.attributes.cvss3.vector.privileges_required",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.UserInteraction") {
                event.rename(
                    "json.attributes.cvss3_vector.UserInteraction",
                    "tenable_io.plugin.attributes.cvss3.vector.user_interaction",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.Scope") {
                event.rename(
                    "json.attributes.cvss3_vector.Scope",
                    "tenable_io.plugin.attributes.cvss3.vector.scope",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.Confidentiality-Impact") {
                event.rename(
                    "json.attributes.cvss3_vector.Confidentiality-Impact",
                    "tenable_io.plugin.attributes.cvss3.vector.confidentiality_impact",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.Integrity-Impact") {
                event.rename(
                    "json.attributes.cvss3_vector.Integrity-Impact",
                    "tenable_io.plugin.attributes.cvss3.vector.integrity_impact",
                )?;
            }

            if event.has_value("json.attributes.cvss3_vector.Availability-Impact") {
                event.rename(
                    "json.attributes.cvss3_vector.Availability-Impact",
                    "tenable_io.plugin.attributes.cvss3.vector.availability_impact",
                )?;
            }

            if event.has_value("json.attributes.cvss3_temporal_vector.raw") {
                event.rename(
                    "json.attributes.cvss3_temporal_vector.raw",
                    "tenable_io.plugin.attributes.cvss3.temporal.vector.raw",
                )?;
            }

            if event.has_value("json.attributes.cvss3_temporal_vector.ExploitCodeMaturity") {
                event.rename(
                    "json.attributes.cvss3_temporal_vector.ExploitCodeMaturity",
                    "tenable_io.plugin.attributes.cvss3.temporal.vector.exploit_code_maturity",
                )?;
            }

            if event.has_value("json.attributes.cvss3_temporal_vector.RemediationLevel") {
                event.rename(
                    "json.attributes.cvss3_temporal_vector.RemediationLevel",
                    "tenable_io.plugin.attributes.cvss3.temporal.vector.remediation_level",
                )?;
            }

            if event.has_value("json.attributes.cvss3_temporal_vector.ReportConfidence") {
                event.rename(
                    "json.attributes.cvss3_temporal_vector.ReportConfidence",
                    "tenable_io.plugin.attributes.cvss3.temporal.vector.report_confidence",
                )?;
            }

            let _cond = { event.get_str("json.attributes.bid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.attributes.bid") {
                        if let Some(val) = event.get("json.attributes.bid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.attributes.bid".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.plugin.attributes.bid", converted)?;
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
            }

            event.remove("json");

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
                event.remove("tenable_io.plugin.attributes.plugin.modification_date");
                event.remove("tenable_io.plugin.attributes.cve");
                event.remove("tenable_io.plugin.attributes.see_also");
                event.remove("tenable_io.plugin.attributes.cvss3.base_score");
                event.remove("tenable_io.plugin.attributes.cvss3.temporal.score");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
