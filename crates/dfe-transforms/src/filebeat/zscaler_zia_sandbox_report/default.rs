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
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
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
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);"#
                ),
            )?;

            let _cond = {
                event.has_value("json.Classification.Type")
                    && event.get_str("json.Classification.Type") == Some("MALICIOUS")
            };
            if _cond {
                event.append_unique("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("json.Classification.Type")
                    && event.get_str("json.Classification.Type") == Some("BENIGN")
            };
            if _cond {
                event.append_unique("event.category", json!("file"))?;
            }

            let _cond = {
                event.has_value("json.Classification.Type")
                    && event.get_str("json.Classification.Type") == Some("SUSPICIOUS")
            };
            if _cond {
                event.append_unique("event.category", json!("intrusion_detection"))?;
            }

            event.append_unique("event.type", json!("info"))?;

            let _cond = {
                event.has_value("json.Classification.Type")
                    && event.get_str("json.Classification.Type") == Some("BENIGN")
            };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                event.has_value("json.Classification.Type")
                    && (event.get_str("json.Classification.Type") == Some("MALICIOUS")
                        || event.get_str("json.Classification.Type") == Some("SUSPICIOUS"))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("json.Classification.Category") {
                event.rename(
                    "json.Classification.Category",
                    "zscaler_zia.sandbox_report.classification.category",
                )?;
            }

            if event.has("json.Classification.DetectedMalware") {
                event.rename(
                    "json.Classification.DetectedMalware",
                    "zscaler_zia.sandbox_report.classification.detected_malware",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.Classification.Max Score") {
                    if let Some(val) = event.get("json.Classification.Max Score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Classification.Max Score".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "zscaler_zia.sandbox_report.classification.max_score",
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
                    "convert_Classification_Max_Score_to_double",
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

            let _cond = { event.get_str("json.Classification.Score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Classification.Score") {
                        if let Some(val) = event.get("json.Classification.Score") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Classification.Score".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "zscaler_zia.sandbox_report.classification.score",
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
                        "convert_Classification_Score_to_double",
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
                .get("zscaler_zia.sandbox_report.classification.score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.risk_score", v)?;
            }

            event.set("threat.indicator.type", json!("file"))?;

            if event.has("json.Classification.Type") {
                event.rename(
                    "json.Classification.Type",
                    "zscaler_zia.sandbox_report.classification.type",
                )?;
            }

            if event.has("json.FileProperties.RootCA") {
                event.rename(
                    "json.FileProperties.RootCA",
                    "zscaler_zia.sandbox_report.file_properties.root_ca",
                )?;
            }

            if event.has("json.FileProperties.DigitalCerificate") {
                event.rename(
                    "json.FileProperties.DigitalCerificate",
                    "zscaler_zia.sandbox_report.file_properties.digital_cerificate",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.FileProperties.FileSize") {
                    if let Some(val) = event.get("json.FileProperties.FileSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.FileProperties.FileSize".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "zscaler_zia.sandbox_report.file_properties.file_size",
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
                    "convert_File_Properties_File_Size_to_long",
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

            if let Some(v) = event
                .get("zscaler_zia.sandbox_report.file_properties.file_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if event.has("json.FileProperties.FileType") {
                event.rename(
                    "json.FileProperties.FileType",
                    "zscaler_zia.sandbox_report.file_properties.file_type",
                )?;
            }

            if event.has("json.FileProperties.Issuer") {
                event.rename(
                    "json.FileProperties.Issuer",
                    "zscaler_zia.sandbox_report.file_properties.issuer",
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.sandbox_report.file_properties.issuer") };
            if _cond {
                event.append_unique(
                    "file.x509.issuer.common_name",
                    json!(
                        event
                            .get("zscaler_zia.sandbox_report.file_properties.issuer")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.FileProperties.MD5") {
                event.rename(
                    "json.FileProperties.MD5",
                    "zscaler_zia.sandbox_report.file_properties.md5",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_report.file_properties.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.FileProperties.SHA1") {
                event.rename(
                    "json.FileProperties.SHA1",
                    "zscaler_zia.sandbox_report.file_properties.sha1",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_report.file_properties.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.FileProperties.Sha256") {
                event.rename(
                    "json.FileProperties.Sha256",
                    "zscaler_zia.sandbox_report.file_properties.sha256",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_report.file_properties.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.FileProperties.SSDeep") {
                event.rename(
                    "json.FileProperties.SSDeep",
                    "zscaler_zia.sandbox_report.file_properties.ssdeep",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_report.file_properties.ssdeep")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.ssdeep", v)?;
            }

            let _cond = { event.has_value("file.hash.ssdeep") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.ssdeep")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.Networking.Risk") {
                event.rename(
                    "json.Networking.Risk",
                    "zscaler_zia.sandbox_report.networking.risk",
                )?;
            }

            if event.has("json.Networking.Signature") {
                event.rename(
                    "json.Networking.Signature",
                    "zscaler_zia.sandbox_report.networking.signature",
                )?;
            }

            if event.has("json.Networking.SignatureSources") {
                event.rename(
                    "json.Networking.SignatureSources",
                    "zscaler_zia.sandbox_report.networking.signature_sources",
                )?;
            }

            if event.has("json.Persistence.Risk") {
                event.rename(
                    "json.Persistence.Risk",
                    "zscaler_zia.sandbox_report.persistence.risk",
                )?;
            }

            if event.has("json.Persistence.Signature") {
                event.rename(
                    "json.Persistence.Signature",
                    "zscaler_zia.sandbox_report.persistence.signature",
                )?;
            }

            if event.has("json.Persistence.SignatureSources") {
                event.rename(
                    "json.Persistence.SignatureSources",
                    "zscaler_zia.sandbox_report.persistence.signature_sources",
                )?;
            }

            if event.has("json.SecurityBypass.Risk") {
                event.rename(
                    "json.SecurityBypass.Risk",
                    "zscaler_zia.sandbox_report.security_bypass.risk",
                )?;
            }

            if event.has("json.SecurityBypass.Signature") {
                event.rename(
                    "json.SecurityBypass.Signature",
                    "zscaler_zia.sandbox_report.security_bypass.signature",
                )?;
            }

            if event.has("json.SecurityBypass.SignatureSources") {
                event.rename(
                    "json.SecurityBypass.SignatureSources",
                    "zscaler_zia.sandbox_report.security_bypass.signature_sources",
                )?;
            }

            if event.has("json.Stealth.Risk") {
                event.rename(
                    "json.Stealth.Risk",
                    "zscaler_zia.sandbox_report.stealth.risk",
                )?;
            }

            if event.has("json.Stealth.Signature") {
                event.rename(
                    "json.Stealth.Signature",
                    "zscaler_zia.sandbox_report.stealth.signature",
                )?;
            }

            if event.has("json.Stealth.SignatureSources") {
                event.rename(
                    "json.Stealth.SignatureSources",
                    "zscaler_zia.sandbox_report.stealth.signature_sources",
                )?;
            }

            if event.has("json.Origin.Risk") {
                event.rename("json.Origin.Risk", "zscaler_zia.sandbox_report.origin.risk")?;
            }

            if event.has("json.Origin.Language") {
                event.rename(
                    "json.Origin.Language",
                    "zscaler_zia.sandbox_report.origin.language",
                )?;
            }

            if event.has("json.Origin.Country") {
                event.rename(
                    "json.Origin.Country",
                    "zscaler_zia.sandbox_report.origin.country",
                )?;
            }

            if event.has("json.Exploit.Risk") {
                event.rename(
                    "json.Exploit.Risk",
                    "zscaler_zia.sandbox_report.exploit.risk",
                )?;
            }

            if event.has("json.Exploit.Signature") {
                event.rename(
                    "json.Exploit.Signature",
                    "zscaler_zia.sandbox_report.exploit.signature",
                )?;
            }

            if event.has("json.Exploit.SignatureSources") {
                event.rename(
                    "json.Exploit.SignatureSources",
                    "zscaler_zia.sandbox_report.exploit.signature_sources",
                )?;
            }

            if event.has_value("json.Summary.Analysis") {
                if let Some(val) = event.get("json.Summary.Analysis") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.Summary.Analysis".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.sandbox_report.summary.analysis", converted)?;
                }
            }

            if event.has("json.Summary.Category") {
                event.rename(
                    "json.Summary.Category",
                    "zscaler_zia.sandbox_report.summary.category",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.Summary.Duration") {
                    if let Some(val) = event.get("json.Summary.Duration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Summary.Duration".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.sandbox_report.summary.duration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_Duration_to_long",
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

            let _cond = { event.has_value("zscaler_zia.sandbox_report.summary.duration") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event.duration = ctx.zscaler_zia.sandbox_report.summary.duration * 1000000;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"ctx.event.duration = ctx.zscaler_zia.sandbox_report.summary.duration * 1000000;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_duration_ms_to_ns",
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

            if event.has("json.Summary.FileType") {
                event.rename(
                    "json.Summary.FileType",
                    "zscaler_zia.sandbox_report.summary.file.type",
                )?;
            }

            let _cond = {
                event.has_value("json.Summary.StartTime")
                    && event.get_str("json.Summary.StartTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Summary.StartTime") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["MMM dd, yyyy hh:mm:ss a", "MMM d, yyyy h:m:s a", "UNIX"],
                            None,
                            None,
                        ) {
                            event.set("zscaler_zia.sandbox_report.summary.start_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_StartTime")?;
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
                .get("zscaler_zia.sandbox_report.summary.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if event.has("json.Summary.Status") {
                event.rename(
                    "json.Summary.Status",
                    "zscaler_zia.sandbox_report.summary.status",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_report.summary.status")
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

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let re = cached_regex!(" ");
                            let replaced = re.replace_all(&s, "-").into_owned();
                            event.set("event.action", replaced)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
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

            if event.has("json.Summary.TimeUnit") {
                event.rename(
                    "json.Summary.TimeUnit",
                    "zscaler_zia.sandbox_report.summary.time_unit",
                )?;
            }

            if event.has("json.Summary.Url") {
                event.rename("json.Summary.Url", "zscaler_zia.sandbox_report.summary.url")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_report.summary.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.original", v)?;
            }

            if event.has_value("url.original") {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            if event.has("json.SystemSummary.Risk") {
                event.rename(
                    "json.SystemSummary.Risk",
                    "zscaler_zia.sandbox_report.system_summary.risk",
                )?;
            }

            if event.has("json.SystemSummary.Signature") {
                event.rename(
                    "json.SystemSummary.Signature",
                    "zscaler_zia.sandbox_report.system_summary.signature",
                )?;
            }

            if event.has("json.SystemSummary.SignatureSources") {
                event.rename(
                    "json.SystemSummary.SignatureSources",
                    "zscaler_zia.sandbox_report.system_summary.signature_sources",
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
                event.remove("zscaler_zia.sandbox_report.classification.score");
                event.remove("zscaler_zia.sandbox_report.file_properties.file_size");
                event.remove("zscaler_zia.sandbox_report.file_properties.issuer");
                event.remove("zscaler_zia.sandbox_report.file_properties.md5");
                event.remove("zscaler_zia.sandbox_report.file_properties.sha1");
                event.remove("zscaler_zia.sandbox_report.file_properties.sha256");
                event.remove("zscaler_zia.sandbox_report.summary.duration");
                event.remove("zscaler_zia.sandbox_report.summary.start_time");
                event.remove("zscaler_zia.sandbox_report.summary.url");
                event.remove("zscaler_zia.sandbox_report.file_properties.ssdeep");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
