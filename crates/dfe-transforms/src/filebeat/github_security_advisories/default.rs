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
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("vulnerability")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
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
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.rename("json.cve_id", "github.security_advisory.cve_id")?;

            if let Some(v) = event
                .get("github.security_advisory.cve_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            event.rename("json.credits", "github.security_advisory.credits")?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.cvss.score") {
                    if let Some(val) = event.get("json.cvss.score") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cvss.score".into(),
                                message,
                            }
                        })?;
                        event.set("github.security_advisory.cvss.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cvss_score_to_float",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("github.security_advisory.cvss.score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.base", v)?;
            }

            event.rename(
                "json.cvss.vector_string",
                "github.security_advisory.cvss.vector_string",
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.cvss_severities.cvss_v3.score") {
                    if let Some(val) = event.get("json.cvss_severities.cvss_v3.score") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cvss_severities.cvss_v3.score".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "github.security_advisory.cvss_severities.cvss_v3.score",
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
                    "convert_cvss_severities_v3_score_to_float",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.rename(
                "json.cvss_severities.cvss_v3.vector_string",
                "github.security_advisory.cvss_severities.cvss_v3.vector_string",
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.cvss_severities.cvss_v4.score") {
                    if let Some(val) = event.get("json.cvss_severities.cvss_v4.score") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cvss_severities.cvss_v4.score".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "github.security_advisory.cvss_severities.cvss_v4.score",
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
                    "convert_cvss_severities_v4_score_to_float",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.rename(
                "json.cvss_severities.cvss_v4.vector_string",
                "github.security_advisory.cvss_severities.cvss_v4.vector_string",
            )?;

            event.rename("json.cwes", "github.security_advisory.cwes")?;

            event.rename("json.description", "github.security_advisory.description")?;

            if let Some(v) = event
                .get("github.security_advisory.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.description", v)?;
            }

            let _cond = { event.has_value("json.epss.percentage") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("json.epss.percentage") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.epss.percentage".into(),
                                message,
                            }
                        })?;
                        event.set("github.security_advisory.epss.percentage", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_epss_percentage_to_float",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond = { event.has_value("json.epss.percentile") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("json.epss.percentile") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.epss.percentile".into(),
                                message,
                            }
                        })?;
                        event.set("github.security_advisory.epss.percentile", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_epss_percentile_to_float",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            event.rename("json.ghsa_id", "github.security_advisory.ghsa_id")?;

            let _cond = { event.has_value("json.github_reviewed_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.github_reviewed_at") {
                    match parse_date_out(
                        &date_str,
                        &["strict_date_optional_time_nanos"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("github.security_advisory.github_reviewed_at", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.github_reviewed_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("github.security_advisory.json.html_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.reference", v)?;
            }

            let _cond =
                { event.has_value("json.html_url") && event.get_str("json.html_url") != Some("") };
            if _cond {
                uri_parts(event, "json.html_url", "url", true, false)?;
            }

            let _cond =
                { event.has_value("url.original") && event.get_str("url.original") != Some("") };
            if _cond {
                event.set(
                    "url.full",
                    json!(
                        event
                            .get("url.original")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.rename("json.html_url", "github.security_advisory.html_url")?;

            event.rename("json.identifiers", "github.security_advisory.identifiers")?;

            let _cond = { event.has_value("json.nvd_published_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.nvd_published_at") {
                    match parse_date_out(
                        &date_str,
                        &["strict_date_optional_time_nanos"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("github.security_advisory.nvd_published_at", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.nvd_published_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.published_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.published_at") {
                    match parse_date_out(
                        &date_str,
                        &["strict_date_optional_time_nanos"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("github.security_advisory.published_at", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.published_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.rename("json.references", "github.security_advisory.references")?;

            event.rename(
                "json.repository_advisory_url",
                "github.security_advisory.repository_advisory_url",
            )?;

            event.rename("json.severity", "github.security_advisory.severity")?;

            if let Some(v) = event
                .get("github.security_advisory.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.severity", v)?;
            }

            event.rename(
                "json.source_code_location",
                "github.security_advisory.source_code_location",
            )?;

            event.rename("json.summary", "github.security_advisory.summary")?;

            event.rename("json.type", "github.security_advisory.type")?;

            let _cond = { event.has_value("json.updated_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.updated_at") {
                    match parse_date_out(
                        &date_str,
                        &["strict_date_optional_time_nanos"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("github.security_advisory.updated_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.updated_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.rename("json.url", "github.security_advisory.url")?;

            event.rename(
                "json.vulnerabilities",
                "github.security_advisory.vulnerabilities",
            )?;

            let _cond = { event.has_value("json.withdrawn_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.withdrawn_at") {
                    match parse_date_out(
                        &date_str,
                        &["strict_date_optional_time_nanos"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("github.security_advisory.withdrawn_at", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.withdrawn_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.set("vulnerability.classification", json!("CVSS"))?;

            event.set("vulnerability.enumeration", json!("CVE"))?;

            if event.remove("json").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json".into(),
                });
            }

            // Painless script
            // Source: boolean drop(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndrop(ctx);"#
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
                        "Processor '{}' {}failed with message '{}'",
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
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
