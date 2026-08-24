// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_email` pipeline.
pub struct PipelineEmail;

impl Transform for PipelineEmail {
    fn name(&self) -> &str {
        "pipeline_email"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { !event.has_value("m365_defender.event.category") || event.get_str("m365_defender.event.category") == Some("") };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("Event does not contain a valid category.").to_string(),
                });
            }

            event.set("event.kind", json!("event"))?;

            let _cond = { !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("urlclickevents"))) };
            if _cond {
                event.append("event.category", json!("email"))?;
            }

            let _cond = { event.has_value("json.properties.FileType") };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = { !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("urlclickevents"))) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("json.properties.DetectionMethods") && event.get("json.properties.DetectionMethods").is_some_and(|v| v.is_string()) && event.get_str("json.properties.DetectionMethods") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.DetectionMethods", "json.properties.DetectionMethods")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_detection_methods")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.properties.ConfidenceLevel") && event.get("json.properties.ConfidenceLevel").is_some_and(|v| v.is_string()) && event.get_str("json.properties.ConfidenceLevel") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.ConfidenceLevel", "json.properties.ConfidenceLevel")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_confidence_level")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.properties.AdditionalFields") && event.get("json.properties.AdditionalFields").is_some_and(|v| v.is_string()) && event.get_str("json.properties.AdditionalFields") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.AdditionalFields", "json.properties.AdditionalFields")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_additional_fields")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                // Painless script
                // Source: def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_clicked_through = isTruthy(ctx.json?.properties?.IsClickedThrough);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_clicked_through = isTruthy(ctx.json?.properties?.IsClickedThrough);\n"#))?;

            let _cond = { event.get_str("json.properties.EmailClusterId") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.EmailClusterId") {
                if let Some(val) = event.get("json.properties.EmailClusterId") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.EmailClusterId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.email.cluster_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_email_cluster_id")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.IPAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.IPAddress") {
                if let Some(val) = event.get("json.properties.IPAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IPAddress".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ip_address")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.FileSize") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.FileSize") {
                if let Some(val) = event.get("json.properties.FileSize") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.FileSize".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_size")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.SenderIPv4") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.SenderIPv4") {
                if let Some(val) = event.get("json.properties.SenderIPv4") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.SenderIPv4".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.sender.ipv4", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sender_ipv4")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.SenderIPv6") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.SenderIPv6") {
                if let Some(val) = event.get("json.properties.SenderIPv6") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.SenderIPv6".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.sender.ipv6", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sender_ipv6")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.AttachmentCount") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.AttachmentCount") {
                if let Some(val) = event.get("json.properties.AttachmentCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.AttachmentCount".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.attachment_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attachment_count")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.BulkComplaintLevel") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.BulkComplaintLevel") {
                if let Some(val) = event.get("json.properties.BulkComplaintLevel") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.BulkComplaintLevel".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.bulk_complaint_level", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bulk_complaint_level")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.ReportId") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.ReportId") {
                if let Some(val) = event.get("json.properties.ReportId") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.ReportId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.report_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_ReportId")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.UrlCount") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.UrlCount") {
                if let Some(val) = event.get("json.properties.UrlCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.UrlCount".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.url_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_url_count")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.IsExternalThread") {
                if let Some(val) = event.get("json.properties.IsExternalThread") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IsExternalThread".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.is_external_thread", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_IsExternalThread_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.IsOwnedThread") {
                if let Some(val) = event.get("json.properties.IsOwnedThread") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IsOwnedThread".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.is_owned_thread", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_IsOwnedThread_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.SenderEmailAddress") {
                if let Some(input) = event.get_string("json.properties.SenderEmailAddress") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else { break 'dissect false };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("user.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "json.properties.SenderEmailAddress".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_json_properties_SenderEmailAddress_70aca97e")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has("json.properties.SenderFromAddress") {
                    event.rename("json.properties.SenderFromAddress", "m365_defender.event.sender.from_address")?;
                }

                if event.has("json.properties.NetworkMessageId") {
                    event.rename("json.properties.NetworkMessageId", "m365_defender.event.network.message_id")?;
                }

                if event.has("json.properties.FileType") {
                    event.rename("json.properties.FileType", "m365_defender.event.file.type")?;
                }

                if event.has("json.properties.RecipientEmailAddress") {
                    event.rename("json.properties.RecipientEmailAddress", "m365_defender.event.recipient.email_address")?;
                }

                if event.has("json.properties.ActionType") {
                    event.rename("json.properties.ActionType", "m365_defender.event.action.type")?;
                }

                if event.has("json.properties.SHA256") {
                    event.rename("json.properties.SHA256", "m365_defender.event.sha256")?;
                }

                if event.has("json.properties.FileName") {
                    event.rename("json.properties.FileName", "m365_defender.event.file.name")?;
                }

                if event.has("json.properties.EmailDirection") {
                    event.rename("json.properties.EmailDirection", "m365_defender.event.email.direction")?;
                }

                if event.has("json.properties.Subject") {
                    event.rename("json.properties.Subject", "m365_defender.event.subject")?;
                }

                if event.has("json.properties.DetectionMethods") {
                    event.rename("json.properties.DetectionMethods", "m365_defender.event.detection.methods")?;
                }

                if event.has("json.properties.RecipientObjectId") {
                    event.rename("json.properties.RecipientObjectId", "m365_defender.event.recipient.object_id")?;
                }

                if event.has("json.properties.SenderDisplayName") {
                    event.rename("json.properties.SenderDisplayName", "m365_defender.event.sender.display_name")?;
                }

                if event.has("json.properties.SenderObjectId") {
                    event.rename("json.properties.SenderObjectId", "m365_defender.event.sender.object_id")?;
                }

                if event.has("json.properties.ThreatNames") {
                    event.rename("json.properties.ThreatNames", "m365_defender.event.threat.names")?;
                }

                if event.has("json.properties.ThreatTypes") {
                    event.rename("json.properties.ThreatTypes", "m365_defender.event.threat.types")?;
                }

                if event.has("json.properties.ConfidenceLevel") {
                    event.rename("json.properties.ConfidenceLevel", "m365_defender.event.confidence_level")?;
                }

                if event.has("json.properties.AuthenticationDetails") {
                    event.rename("json.properties.AuthenticationDetails", "m365_defender.event.authentication_details")?;
                }

                if event.has("json.properties.AdditionalFields") {
                    event.rename("json.properties.AdditionalFields", "m365_defender.event.additional_fields")?;
                }

                if event.has("json.properties.Connectors") {
                    event.rename("json.properties.Connectors", "m365_defender.event.connectors")?;
                }

                if event.has("json.properties.DeliveryAction") {
                    event.rename("json.properties.DeliveryAction", "m365_defender.event.delivery.action")?;
                }

                if event.has("json.properties.DeliveryLocation") {
                    event.rename("json.properties.DeliveryLocation", "m365_defender.event.delivery.location")?;
                }

                if event.has("json.properties.EmailAction") {
                    event.rename("json.properties.EmailAction", "m365_defender.event.email.action")?;
                }

                if event.has("json.properties.EmailActionPolicy") {
                    event.rename("json.properties.EmailActionPolicy", "m365_defender.event.email.action_policy")?;
                }

                if event.has("json.properties.EmailActionPolicyGuid") {
                    event.rename("json.properties.EmailActionPolicyGuid", "m365_defender.event.email.action_policy_guid")?;
                }

                if event.has("json.properties.EmailLanguage") {
                    event.rename("json.properties.EmailLanguage", "m365_defender.event.email.language")?;
                }

                if event.has("json.properties.InternetMessageId") {
                    event.rename("json.properties.InternetMessageId", "m365_defender.event.internet_message_id")?;
                }

                if event.has("json.properties.OrgLevelAction") {
                    event.rename("json.properties.OrgLevelAction", "m365_defender.event.org_level.action")?;
                }

                if event.has("json.properties.OrgLevelPolicy") {
                    event.rename("json.properties.OrgLevelPolicy", "m365_defender.event.org_level.policy")?;
                }

                if event.has("json.properties.SenderFromDomain") {
                    event.rename("json.properties.SenderFromDomain", "m365_defender.event.sender.from_domain")?;
                }

                if event.has("json.properties.SenderMailFromAddress") {
                    event.rename("json.properties.SenderMailFromAddress", "m365_defender.event.sender.mail_from_address")?;
                }

                if event.has("json.properties.SenderMailFromDomain") {
                    event.rename("json.properties.SenderMailFromDomain", "m365_defender.event.sender.mail_from_domain")?;
                }

                if event.has("json.properties.UserLevelAction") {
                    event.rename("json.properties.UserLevelAction", "m365_defender.event.user_level_action")?;
                }

                if event.has("json.properties.UserLevelPolicy") {
                    event.rename("json.properties.UserLevelPolicy", "m365_defender.event.user_level_policy")?;
                }

                if event.has("json.properties.ActionResult") {
                    event.rename("json.properties.ActionResult", "m365_defender.event.action.result")?;
                }

                if event.has("json.properties.ActionTrigger") {
                    event.rename("json.properties.ActionTrigger", "m365_defender.event.action.trigger")?;
                }

                if event.has("json.properties.Action") {
                    event.rename("json.properties.Action", "m365_defender.event.action.value")?;
                }

                if event.has("json.properties.Url") {
                    event.rename("json.properties.Url", "m365_defender.event.url")?;
                }

                if event.has("json.properties.UrlDomain") {
                    event.rename("json.properties.UrlDomain", "m365_defender.event.url_domain")?;
                }

                if event.has("json.properties.UrlLocation") {
                    event.rename("json.properties.UrlLocation", "m365_defender.event.url_location")?;
                }

                if event.has("json.properties.AccountUpn") {
                    event.rename("json.properties.AccountUpn", "m365_defender.event.account.upn")?;
                }

                if event.has("json.properties.UrlChain") {
                    event.rename("json.properties.UrlChain", "m365_defender.event.url_chain")?;
                }

                if event.has("json.properties.Workload") {
                    event.rename("json.properties.Workload", "m365_defender.event.workload")?;
                }

                if event.has("json.properties.GroupId") {
                    event.rename("json.properties.GroupId", "m365_defender.event.group_id")?;
                }

                if event.has("json.properties.GroupName") {
                    event.rename("json.properties.GroupName", "m365_defender.event.group_name")?;
                }

                if event.has("json.properties.LastEditedTime") {
                    event.rename("json.properties.LastEditedTime", "m365_defender.event.last_edited_time")?;
                }

                if event.has("json.properties.MessageFormatSubtype") {
                    event.rename("json.properties.MessageFormatSubtype", "m365_defender.event.message_format_subtype")?;
                }

                if event.has("json.properties.MessageFormatType") {
                    event.rename("json.properties.MessageFormatType", "m365_defender.event.message_format_type")?;
                }

                if event.has("json.properties.MessageId") {
                    event.rename("json.properties.MessageId", "m365_defender.event.message_id")?;
                }

                if event.has("json.properties.MessageSubject") {
                    event.rename("json.properties.MessageSubject", "m365_defender.event.message_subject")?;
                }

                if event.has("json.properties.MessageVersion") {
                    event.rename("json.properties.MessageVersion", "m365_defender.event.message_version")?;
                }

                if event.has("json.properties.ParentMessageId") {
                    event.rename("json.properties.ParentMessageId", "m365_defender.event.parent_message_id")?;
                }

                if event.has("json.properties.SenderType") {
                    event.rename("json.properties.SenderType", "m365_defender.event.sender_type")?;
                }

                if event.has("json.properties.TeamsMessageId") {
                    event.rename("json.properties.TeamsMessageId", "m365_defender.event.teams_message_id")?;
                }

                if event.has("json.properties.ThreadId") {
                    event.rename("json.properties.ThreadId", "m365_defender.event.thread_id")?;
                }

                if event.has("json.properties.ThreadSubtype") {
                    event.rename("json.properties.ThreadSubtype", "m365_defender.event.thread_subtype")?;
                }

                if event.has("json.properties.SenderEmailAddress") {
                    event.rename("json.properties.SenderEmailAddress", "m365_defender.event.sender_email_address")?;
                }

                if event.has("json.properties.LatestDeliveryLocation") {
                    event.rename("json.properties.LatestDeliveryLocation", "m365_defender.event.latest_delivery_location")?;
                }

                if event.has("json.properties.SafetyTip") {
                    event.rename("json.properties.SafetyTip", "m365_defender.event.safety_tip")?;
                }

            if let Some(v) = event.get("m365_defender.event.file.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.extension", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha256", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.action.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if event.has_value("event.action") {
                gsub_field(event, "event.action", "event.action", cached_regex!(" "), "-")?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.from_address") };
            if _cond {
                event.append_unique("email.from.address", json!(event.get("m365_defender.event.sender.from_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.recipient.email_address") };
            if _cond {
                event.append_unique("email.to.address", json!(event.get("m365_defender.event.recipient.email_address").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("m365_defender.event.network.message_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.local_id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.email.direction").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.direction", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.internet_message_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.message_id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.message_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.message_id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.message_subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.subject", v)?;
            }

            let _cond = { event.has_value("m365_defender.event.sender_email_address") };
            if _cond {
                event.append_unique("email.from.address", json!(event.get("m365_defender.event.sender_email_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.ipv4") };
            if _cond {
                event.append_unique("source.ip", json!(event.get("m365_defender.event.sender.ipv4").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.ipv6") };
            if _cond {
                event.append_unique("source.ip", json!(event.get("m365_defender.event.sender.ipv6").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("m365_defender.event.ip_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.ip", v)?;
            }

            let _cond = { event.has_value("m365_defender.event.url") };
            if _cond {
                uri_parts(event, "m365_defender.event.url", "url", true, false)?;
            }

            if let Some(v) = event.get("m365_defender.event.group_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.group.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.group_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.group.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.sender_email_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.from_address") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.sender.from_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.recipient.email_address") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.recipient.email_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.ipv4") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("m365_defender.event.sender.ipv4").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.ipv6") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("m365_defender.event.sender.ipv6").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("m365_defender.event.ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.from_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("m365_defender.event.sender.from_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sender.mail_from_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("m365_defender.event.sender.mail_from_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.url_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("m365_defender.event.url_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sender_email_address") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.sender_email_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("m365_defender.event.sender.from_address");
                event.remove("m365_defender.event.network.message_id");
                event.remove("m365_defender.event.internet_message_id");
                event.remove("m365_defender.event.recipient.email_address");
                event.remove("m365_defender.event.file.type");
                event.remove("m365_defender.event.sha256");
                event.remove("m365_defender.event.file.name");
                event.remove("m365_defender.event.file.size");
                event.remove("m365_defender.event.email.direction");
                event.remove("m365_defender.event.subject");
                event.remove("m365_defender.event.sender.ipv4");
                event.remove("m365_defender.event.sender.ipv6");
                event.remove("m365_defender.event.ip_address");
                event.remove("m365_defender.event.action.type");
                event.remove("m365_defender.event.group_id");
                event.remove("m365_defender.event.group_name");
                event.remove("m365_defender.event.message_id");
                event.remove("m365_defender.event.message_subject");
                event.remove("m365_defender.event.sender_email_address");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
