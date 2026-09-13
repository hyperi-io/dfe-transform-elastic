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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { event.get_str("winlog.level") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.level")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("log.level", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("winlog.time_created") };
            if _cond {
                // on_failure: 3 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("winlog.time_created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "winlog.time_created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "time_created_date")?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.remove("winlog.time_created").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "winlog.time_created".into(),
                            });
                        }
                        Ok(())
                    })();
                    event.append(
                        "error.message",
                        json!(format!(
                            "fail-{}",
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: (format!(
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
                        ))
                        .to_string(),
                    });
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set(
                "event.code",
                json!(
                    event
                        .get("winlog.event_id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("event.category", Value::Array(vec![json!("process")]))?;

            event.set("event.type", Value::Array(vec![json!("start")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.record_id") {
                    if let Some(val) = event.get("winlog.record_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.record_id".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.record_id", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("winlog.user.identifier")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("winlog.event_data.User") };
            if _cond {
                if let Some(s) = event.get_string("winlog.event_data.User") {
                    let mut parts: Vec<Value> = cached_regex!("\\\\")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_temp.user_parts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!(
                        event
                            .get("_temp.user_parts.0")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.domain", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!(
                        event
                            .get("_temp.user_parts.1")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.user.name") {
                        event.rename("winlog.user.name", "user.name")?;
                    }
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.FileHashLength") {
                    if let Some(val) = event.get("winlog.user_data.FileHashLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.FileHashLength".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.FileHashLength", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("winlog.user_data.FileHashLength").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.FileHashLength".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.FilePathLength") {
                    if let Some(val) = event.get("winlog.user_data.FilePathLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.FilePathLength".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.FilePathLength", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("winlog.user_data.FilePathLength").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.FilePathLength".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.FqbnLength") {
                    if let Some(val) = event.get("winlog.user_data.FqbnLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.FqbnLength".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.FqbnLength", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("winlog.user_data.FqbnLength").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.FqbnLength".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.FullFilePathLength") {
                    if let Some(val) = event.get("winlog.user_data.FullFilePathLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.FullFilePathLength".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.FullFilePathLength", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event
                        .remove("winlog.user_data.FullFilePathLength")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.FullFilePathLength".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.PolicyNameLength") {
                    if let Some(val) = event.get("winlog.user_data.PolicyNameLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.PolicyNameLength".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.PolicyNameLength", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("winlog.user_data.PolicyNameLength").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.PolicyNameLength".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.RuleNameLength") {
                    if let Some(val) = event.get("winlog.user_data.RuleNameLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.RuleNameLength".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.RuleNameLength", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("winlog.user_data.RuleNameLength").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.RuleNameLength".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.RuleSddlLength") {
                    if let Some(val) = event.get("winlog.user_data.RuleSddlLength") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.RuleSddlLength".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.RuleSddlLength", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("winlog.user_data.RuleSddlLength").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.RuleSddlLength".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("winlog.user_data.TargetProcessId") {
                    if let Some(val) = event.get("winlog.user_data.TargetProcessId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.user_data.TargetProcessId".into(),
                                message,
                            }
                        })?;
                        event.set("winlog.user_data.TargetProcessId", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("winlog.user_data.TargetProcessId").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "winlog.user_data.TargetProcessId".into(),
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

            let _cond = { event.get_str("winlog.user_data.Fqbn") != Some("-") };
            if _cond {
                if event.has_value("winlog.user_data.Fqbn") {
                    if let Some(input) = event.get_string("winlog.user_data.Fqbn") {
                        // Grok pattern: ^CN=(?P<tmp_file_x509_subject_common_name>.*),%{SPACE}O=(?P<tmp_file_x509_subject_organization>.*),%{SPACE}L=(?P<tmp_file_x509_subject_locality>.*),%{SPACE}S=(?P<tmp_file_x509_subject_state_or_province>.*),%{SPACE}C=(?P<tmp_file_x509_subject_country>[^\\\\]*)\\\\(?P<file_pe_product>[^\\\\]*)\\\\(?P<file_pe_original_file_name>[^\\\\]*)\\\\(?P<file_pe_file_version>.*)$
                        // Grok pattern: ^CN=(?P<tmp_file_x509_subject_common_name>.*),%{SPACE}O=(?P<tmp_file_x509_subject_organization>.*),%{SPACE}L=(?P<tmp_file_x509_subject_locality>.*),%{SPACE}C=(?P<tmp_file_x509_subject_country>[^\\\\]*)\\\\(?P<file_pe_product>[^\\\\]*)\\\\(?P<file_pe_original_file_name>[^\\\\]*)\\\\(?P<file_pe_file_version>.*)$
                        // Grok pattern: ^CN=(?P<tmp_file_x509_subject_common_name>.*),%{SPACE}O=(?P<tmp_file_x509_subject_organization>.*),%{SPACE}S=(?P<tmp_file_x509_subject_state_or_province>.*),%{SPACE}C=(?P<tmp_file_x509_subject_country>[^\\\\]*)\\\\(?P<file_pe_product>[^\\\\]*)\\\\(?P<file_pe_original_file_name>[^\\\\]*)\\\\(?P<file_pe_file_version>.*)$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^CN=(?P<tmp_file_x509_subject_common_name>.*),%{SPACE}O=(?P<tmp_file_x509_subject_organization>.*),%{SPACE}L=(?P<tmp_file_x509_subject_locality>.*),%{SPACE}S=(?P<tmp_file_x509_subject_state_or_province>.*),%{SPACE}C=(?P<tmp_file_x509_subject_country>[^\\\\]*)\\\\(?P<file_pe_product>[^\\\\]*)\\\\(?P<file_pe_original_file_name>[^\\\\]*)\\\\(?P<file_pe_file_version>.*)$",
                                    [
                                        (
                                            "tmp_file_x509_subject_common_name",
                                            "tmp.file.x509.subject.common_name"
                                        ),
                                        (
                                            "tmp_file_x509_subject_organization",
                                            "tmp.file.x509.subject.organization"
                                        ),
                                        (
                                            "tmp_file_x509_subject_locality",
                                            "tmp.file.x509.subject.locality"
                                        ),
                                        (
                                            "tmp_file_x509_subject_state_or_province",
                                            "tmp.file.x509.subject.state_or_province"
                                        ),
                                        (
                                            "tmp_file_x509_subject_country",
                                            "tmp.file.x509.subject.country"
                                        ),
                                        ("file_pe_product", "file.pe.product"),
                                        (
                                            "file_pe_original_file_name",
                                            "file.pe.original_file_name"
                                        ),
                                        ("file_pe_file_version", "file.pe.file_version")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^CN=(?P<tmp_file_x509_subject_common_name>.*),%{SPACE}O=(?P<tmp_file_x509_subject_organization>.*),%{SPACE}L=(?P<tmp_file_x509_subject_locality>.*),%{SPACE}C=(?P<tmp_file_x509_subject_country>[^\\\\]*)\\\\(?P<file_pe_product>[^\\\\]*)\\\\(?P<file_pe_original_file_name>[^\\\\]*)\\\\(?P<file_pe_file_version>.*)$",
                                    [
                                        (
                                            "tmp_file_x509_subject_common_name",
                                            "tmp.file.x509.subject.common_name"
                                        ),
                                        (
                                            "tmp_file_x509_subject_organization",
                                            "tmp.file.x509.subject.organization"
                                        ),
                                        (
                                            "tmp_file_x509_subject_locality",
                                            "tmp.file.x509.subject.locality"
                                        ),
                                        (
                                            "tmp_file_x509_subject_country",
                                            "tmp.file.x509.subject.country"
                                        ),
                                        ("file_pe_product", "file.pe.product"),
                                        (
                                            "file_pe_original_file_name",
                                            "file.pe.original_file_name"
                                        ),
                                        ("file_pe_file_version", "file.pe.file_version")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^CN=(?P<tmp_file_x509_subject_common_name>.*),%{SPACE}O=(?P<tmp_file_x509_subject_organization>.*),%{SPACE}S=(?P<tmp_file_x509_subject_state_or_province>.*),%{SPACE}C=(?P<tmp_file_x509_subject_country>[^\\\\]*)\\\\(?P<file_pe_product>[^\\\\]*)\\\\(?P<file_pe_original_file_name>[^\\\\]*)\\\\(?P<file_pe_file_version>.*)$",
                                    [
                                        (
                                            "tmp_file_x509_subject_common_name",
                                            "tmp.file.x509.subject.common_name"
                                        ),
                                        (
                                            "tmp_file_x509_subject_organization",
                                            "tmp.file.x509.subject.organization"
                                        ),
                                        (
                                            "tmp_file_x509_subject_state_or_province",
                                            "tmp.file.x509.subject.state_or_province"
                                        ),
                                        (
                                            "tmp_file_x509_subject_country",
                                            "tmp.file.x509.subject.country"
                                        ),
                                        ("file_pe_product", "file.pe.product"),
                                        (
                                            "file_pe_original_file_name",
                                            "file.pe.original_file_name"
                                        ),
                                        ("file_pe_file_version", "file.pe.file_version")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            event.append(
                "file.x509.subject.locality",
                json!(
                    event
                        .get("tmp.file.x509.subject.locality")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append(
                "file.x509.subject.common_name",
                json!(
                    event
                        .get("tmp.file.x509.subject.common_name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append(
                "file.x509.subject.organization",
                json!(
                    event
                        .get("tmp.file.x509.subject.organization")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append(
                "file.x509.subject.country",
                json!(
                    event
                        .get("tmp.file.x509.subject.country")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append(
                "file.x509.subject.state_or_province",
                json!(
                    event
                        .get("tmp.file.x509.subject.state_or_province")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.remove("tmp");

            let _cond = { event.get_str("winlog.user_data.FullFilePath") != Some("-") };
            if _cond {
                if event.has_value("winlog.user_data.FullFilePath") {
                    if let Some(input) = event.get_string("winlog.user_data.FullFilePath") {
                        // Grok pattern: (?P<file_name>([^\\\\]*$))
                        if !cached_grok_mapped!(
                            "(?P<file_name>([^\\\\]*$))",
                            [("file_name", "file.name")]
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = { event.get_str("winlog.user_data.FileHash") != Some("-") };
            if _cond {
                if let Some(v) = event
                    .get("winlog.user_data.FileHash")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha256", v)?;
                }
            }

            if event.has_value("error.code") {
                if let Some(val) = event.get("error.code") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "error.code".into(),
                            message,
                        }
                    })?;
                    event.set("error.code", converted)?;
                }
            }

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
