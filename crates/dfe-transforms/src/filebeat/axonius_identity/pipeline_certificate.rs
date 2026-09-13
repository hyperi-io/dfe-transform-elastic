// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_certificate` pipeline.
pub struct PipelineCertificate;

impl Transform for PipelineCertificate {
    fn name(&self) -> &str {
        "pipeline_certificate"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("axonius.identity.begins_on") && event.get_str("axonius.identity.begins_on") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("axonius.identity.begins_on") {
                    match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss 'GMT'", "yyyy-MM-dd", "EEE,dd MMM yyyy HH:mm:ss 'GMT'"], None, None) {
                        Some(parsed) => event.set("axonius.identity.begins_on", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "axonius.identity.begins_on".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_begins_on")?;
                        event.remove("axonius.identity.begins_on");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("axonius.identity.begins_on").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.start", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("axonius.identity.bit_size") {
                if let Some(val) = event.get("axonius.identity.bit_size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "axonius.identity.bit_size".into(),
                            message,
                        })?;
                    event.set("axonius.identity.bit_size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bit_size_to_long")?;
                        event.remove("axonius.identity.bit_size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("axonius.identity.expires_on") && event.get_str("axonius.identity.expires_on") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("axonius.identity.expires_on") {
                    match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss 'GMT'", "yyyy-MM-dd", "EEE,dd MMM yyyy HH:mm:ss 'GMT'"], None, None) {
                        Some(parsed) => event.set("axonius.identity.expires_on", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "axonius.identity.expires_on".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_expires_on")?;
                        event.remove("axonius.identity.expires_on");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("axonius.identity.expires_on").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.end", v)?;
            }

            let _cond = { event.has_value("axonius.identity.issuer.common_name") };
            if _cond {
                event.append_unique("file.x509.issuer.common_name", json!(event.get("axonius.identity.issuer.common_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.identity.issuer.country_name") };
            if _cond {
                event.append_unique("file.x509.issuer.country", json!(event.get("axonius.identity.issuer.country_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.identity.issuer.organization") };
            if _cond {
                event.append_unique("file.x509.issuer.organization", json!(event.get("axonius.identity.issuer.organization").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("axonius.identity.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.serial_number", v)?;
            }

            let _cond = { event.has_value("axonius.identity.subject.common_name") };
            if _cond {
                event.append_unique("file.x509.subject.common_name", json!(event.get("axonius.identity.subject.common_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.identity.subject.country_name") };
            if _cond {
                event.append_unique("file.x509.subject.country", json!(event.get("axonius.identity.subject.country_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.identity.subject.locality") };
            if _cond {
                event.append_unique("file.x509.subject.locality", json!(event.get("axonius.identity.subject.locality").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.identity.subject.organization") };
            if _cond {
                event.append_unique("file.x509.subject.organization", json!(event.get("axonius.identity.subject.organization").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("axonius.identity.subject.state") };
            if _cond {
                event.append_unique("file.x509.subject.state_or_province", json!(event.get("axonius.identity.subject.state").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}'\n{}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}'\n", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
