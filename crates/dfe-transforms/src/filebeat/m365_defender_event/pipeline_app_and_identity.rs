// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_app_and_identity` pipeline.
pub struct PipelineAppAndIdentity;

impl Transform for PipelineAppAndIdentity {
    fn name(&self) -> &str {
        "pipeline_app_and_identity"
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

            let _cond = { !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo")) };
            if _cond {
            event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identityinfo")) };
            if _cond {
            event.set("event.kind", json!("asset"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identitylogonevents")) };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identityinfo")) };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("cloudauditevents")) };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

                event.append("event.type", json!("info"))?;

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identityinfo")) };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = { event.get("json.properties.ActivityObjects").is_some_and(|v| v.is_string()) && event.get_str("json.properties.ActivityObjects") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.ActivityObjects", "json.properties.ActivityObjects")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_properties_ActivityObjects")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.RawEventData").is_some_and(|v| v.is_string()) && event.get_str("json.properties.RawEventData") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.RawEventData", "json.properties.RawEventData")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_properties_RawEventData")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.AdditionalFields").is_some_and(|v| v.is_string()) && event.get_str("json.properties.AdditionalFields") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.AdditionalFields", "json.properties.AdditionalFields")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_properties_AdditionalFields")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.DestinationIPAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.DestinationIPAddress") {
                if let Some(val) = event.get("json.properties.DestinationIPAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.DestinationIPAddress".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.destination.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_DestinationIPAddress")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.DestinationPort") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.DestinationPort") {
                if let Some(val) = event.get("json.properties.DestinationPort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.DestinationPort".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_DestinationPort")?;
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
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_IPAddress")?;
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

            let _cond = { event.get_str("json.properties.AppInstanceId") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.AppInstanceId") {
                if let Some(val) = event.get("json.properties.AppInstanceId") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.AppInstanceId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.app_instance_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_AppInstanceId")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.Port") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.Port") {
                if let Some(val) = event.get("json.properties.Port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.Port".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_Port")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.ApplicationId") != Some("") };
            if _cond {
            if event.has_value("json.properties.ApplicationId") {
                if let Some(val) = event.get("json.properties.ApplicationId") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.ApplicationId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.application_id", converted)?;
                }
            }
            }

            let _cond = { event.has_value("json.properties.DataAggregationStartTime") && event.get_str("json.properties.DataAggregationStartTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.DataAggregationStartTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.data_aggregation_start_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_DataAggregationStartTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.properties.DataAggregationEndTime") && event.get_str("json.properties.DataAggregationEndTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.DataAggregationEndTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.data_aggregation_end_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_DataAggregationEndTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.IpAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.IpAddress") {
                if let Some(val) = event.get("json.properties.IpAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IpAddress".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.storage_ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_IpAddress_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            if event.has_value("json.properties.OperationsCount") {
                if let Some(val) = event.get("json.properties.OperationsCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.OperationsCount".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.operations_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_OperationsCount_to_long")?;
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
            if event.has_value("json.properties.SuccessfulOperationsCount") {
                if let Some(val) = event.get("json.properties.SuccessfulOperationsCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.SuccessfulOperationsCount".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.successful_operations_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_SuccessfulOperationsCount_to_long")?;
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
            if event.has_value("json.properties.IsKnownSuspiciousIp") {
                if let Some(val) = event.get("json.properties.IsKnownSuspiciousIp") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IsKnownSuspiciousIp".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.is_known_suspicious_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_IsKnownSuspiciousIp_to_boolean")?;
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
            if event.has_value("json.properties.IsPrivateIp") {
                if let Some(val) = event.get("json.properties.IsPrivateIp") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IsPrivateIp".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.is_private_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_IsPrivateIp_to_boolean")?;
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
            if event.has_value("json.properties.FailedOperationsCount") {
                if let Some(val) = event.get("json.properties.FailedOperationsCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.FailedOperationsCount".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.failed_operations_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_FailedOperationsCount_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.properties.FirstEventTimestamp") && event.get_str("json.properties.FirstEventTimestamp") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.FirstEventTimestamp") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.first_event_timestamp", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_FirstEventTimestamp")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.properties.LastEventTimestamp") && event.get_str("json.properties.LastEventTimestamp") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.LastEventTimestamp") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.last_event_timestamp", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_LastEventTimestamp")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            if event.has_value("json.properties.TotalResponseLength") {
                if let Some(val) = event.get("json.properties.TotalResponseLength") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.TotalResponseLength".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.total_response_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_TotalResponseLength_to_long")?;
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
            if event.has_value("json.properties.SuccessfulReadOperations") {
                if let Some(val) = event.get("json.properties.SuccessfulReadOperations") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.SuccessfulReadOperations".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.successful_read_operations", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_SuccessfulReadOperations_to_long")?;
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
            if event.has_value("json.properties.DistinctGetOperations") {
                if let Some(val) = event.get("json.properties.DistinctGetOperations") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.DistinctGetOperations".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.distinct_get_operations", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_DistinctGetOperations_to_long")?;
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
            if event.has_value("json.properties.AnonymousSuccessfulOperations") {
                if let Some(val) = event.get("json.properties.AnonymousSuccessfulOperations") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.AnonymousSuccessfulOperations".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.anonymous_successful_operations", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_AnonymousSuccessfulOperations_to_long")?;
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
            if event.has_value("json.properties.HasAnonymousResourceNotFoundFailures") {
                if let Some(val) = event.get("json.properties.HasAnonymousResourceNotFoundFailures") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.HasAnonymousResourceNotFoundFailures".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.has_anonymous_resource_not_found_failures", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_HasAnonymousResourceNotFoundFailures_to_boolean")?;
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
            if event.has_value("json.properties.IsTorExitNode") {
                if let Some(val) = event.get("json.properties.IsTorExitNode") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IsTorExitNode".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.is_tor_exit_node", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_IsTorExitNode_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                // Painless script
                // Source: def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_admin_operation = isTruthy(ctx.json?.properties?.IsAdminOperation);\n ctx.m365_defender.event.is_anonymous_proxy = isTruthy(ctx.json?.properties?.IsAnonymousProxy);\n ctx.m365_defender.event.is_external_user = isTruthy(ctx.json?.properties?.IsExternalUser);\n ctx.m365_defender.event.is_impersonated = isTruthy(ctx.json?.properties?.IsImpersonated);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_admin_operation = isTruthy(ctx.json?.properties?.IsAdminOperation);\n ctx.m365_defender.event.is_anonymous_proxy = isTruthy(ctx.json?.properties?.IsAnonymousProxy);\n ctx.m365_defender.event.is_external_user = isTruthy(ctx.json?.properties?.IsExternalUser);\n ctx.m365_defender.event.is_impersonated = isTruthy(ctx.json?.properties?.IsImpersonated);\n"#))?;

            let _cond = { !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo")) };
            if _cond {
                if event.has("json.properties.City") {
                    event.rename("json.properties.City", "m365_defender.event.city")?;
                }
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identityinfo")) };
            if _cond {
                if event.has("json.properties.City") {
                    event.rename("json.properties.City", "m365_defender.event.account.city")?;
                }
            }

                if event.has("json.properties.AccountName") {
                    event.rename("json.properties.AccountName", "m365_defender.event.account.name")?;
                }

                if event.has("json.properties.IsAccountEnabled") {
                    event.rename("json.properties.IsAccountEnabled", "m365_defender.event.account.is_enabled")?;
                }

                if event.has("json.properties.AccountSid") {
                    event.rename("json.properties.AccountSid", "m365_defender.event.account.sid")?;
                }

                if event.has("json.properties.AccountDomain") {
                    event.rename("json.properties.AccountDomain", "m365_defender.event.account.domain")?;
                }

                if event.has("json.properties.DeviceType") {
                    event.rename("json.properties.DeviceType", "m365_defender.event.device.type")?;
                }

                if event.has("json.properties.ActionType") {
                    event.rename("json.properties.ActionType", "m365_defender.event.action.type")?;
                }

                if event.has("json.properties.OSPlatform") {
                    event.rename("json.properties.OSPlatform", "m365_defender.event.os.platform")?;
                }

                if event.has("json.properties.DeviceName") {
                    event.rename("json.properties.DeviceName", "m365_defender.event.device.name")?;
                }

                if event.has("json.properties.TargetDeviceName") {
                    event.rename("json.properties.TargetDeviceName", "m365_defender.event.target.device_name")?;
                }

                if event.has("json.properties.Isp") {
                    event.rename("json.properties.Isp", "m365_defender.event.isp")?;
                }

                if event.has("json.properties.ISP") {
                    event.rename("json.properties.ISP", "m365_defender.event.isp")?;
                }

                if event.has("json.properties.AdditionalFields") {
                    event.rename("json.properties.AdditionalFields", "m365_defender.event.additional_fields")?;
                }

                if event.has("json.properties.AccountObjectId") {
                    event.rename("json.properties.AccountObjectId", "m365_defender.event.account.object_id")?;
                }

                if event.has("json.properties.AccountDisplayName") {
                    event.rename("json.properties.AccountDisplayName", "m365_defender.event.account.display_name")?;
                }

                if event.has("json.properties.UserAgent") {
                    event.rename("json.properties.UserAgent", "m365_defender.event.user_agent")?;
                }

            let _cond = { !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo")) };
            if _cond {
                if event.has("json.properties.Country") {
                    event.rename("json.properties.Country", "m365_defender.event.country")?;
                }
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo") };
            if _cond {
                if event.has("json.properties.Country") {
                    event.rename("json.properties.Country", "m365_defender.event.account.country")?;
                }
            }

                if event.has("json.properties.CountryCode") {
                    event.rename("json.properties.CountryCode", "m365_defender.event.country_code")?;
                }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo") };
            if _cond {
                if event.has("json.properties.State") {
                    event.rename("json.properties.State", "m365_defender.event.account.state")?;
                }
            }

                if event.has("json.properties.Protocol") {
                    event.rename("json.properties.Protocol", "m365_defender.event.protocol")?;
                }

                if event.has("json.properties.AccountUpn") {
                    event.rename("json.properties.AccountUpn", "m365_defender.event.account.upn")?;
                }

                if event.has("json.properties.Application") {
                    event.rename("json.properties.Application", "m365_defender.event.application")?;
                }

                if event.has("json.properties.DestinationDeviceName") {
                    event.rename("json.properties.DestinationDeviceName", "m365_defender.event.destination.device_name")?;
                }

                if event.has("json.properties.FailureReason") {
                    event.rename("json.properties.FailureReason", "m365_defender.event.failure_reason")?;
                }

                if event.has("json.properties.Location") {
                    event.rename("json.properties.Location", "m365_defender.event.location")?;
                }

                if event.has("json.properties.AccountType") {
                    event.rename("json.properties.AccountType", "m365_defender.event.account.type")?;
                }

                if event.has("json.properties.LogonType") {
                    event.rename("json.properties.LogonType", "m365_defender.event.logon.type")?;
                }

                if event.has("json.properties.AccountId") {
                    event.rename("json.properties.AccountId", "m365_defender.event.account.id")?;
                }

                if event.has("json.properties.TargetAccountDisplayName") {
                    event.rename("json.properties.TargetAccountDisplayName", "m365_defender.event.target.account_display_name")?;
                }

                if event.has("json.properties.Query") {
                    event.rename("json.properties.Query", "m365_defender.event.query.value")?;
                }

                if event.has("json.properties.QueryTarget") {
                    event.rename("json.properties.QueryTarget", "m365_defender.event.query.target")?;
                }

                if event.has("json.properties.QueryType") {
                    event.rename("json.properties.QueryType", "m365_defender.event.query.type")?;
                }

                if event.has("json.properties.TargetAccountUpn") {
                    event.rename("json.properties.TargetAccountUpn", "m365_defender.event.target.account_upn")?;
                }

                if event.has("json.properties.ActivityObjects") {
                    event.rename("json.properties.ActivityObjects", "m365_defender.event.activity.objects")?;
                }

                if event.has("json.properties.ActivityType") {
                    event.rename("json.properties.ActivityType", "m365_defender.event.activity.type")?;
                }

                if event.has("json.properties.IPCategory") {
                    event.rename("json.properties.IPCategory", "m365_defender.event.ip_category")?;
                }

                if event.has("json.properties.IPTags") {
                    event.rename("json.properties.IPTags", "m365_defender.event.ip_tags")?;
                }

                if event.has("json.properties.ObjectId") {
                    event.rename("json.properties.ObjectId", "m365_defender.event.object.id")?;
                }

                if event.has("json.properties.ObjectName") {
                    event.rename("json.properties.ObjectName", "m365_defender.event.object.name")?;
                }

                if event.has("json.properties.ObjectType") {
                    event.rename("json.properties.ObjectType", "m365_defender.event.object.type")?;
                }

                if event.has("json.properties.RawEventData") {
                    event.rename("json.properties.RawEventData", "m365_defender.event.raw_event_data")?;
                }

                if event.has("json.properties.UserAgentTags") {
                    event.rename("json.properties.UserAgentTags", "m365_defender.event.user_agent_tags")?;
                }

                if event.has("json.properties.OnPremSid") {
                    event.rename("json.properties.OnPremSid", "m365_defender.event.account.on_prem_sid")?;
                }

                if event.has("json.properties.SourceProvider") {
                    event.rename("json.properties.SourceProvider", "m365_defender.event.source_provider")?;
                }

                if event.has("json.properties.SourceSystem") {
                    event.rename("json.properties.SourceSystem", "m365_defender.event.source_system")?;
                }

                if event.has("json.properties.AssignedRoles") {
                    event.rename("json.properties.AssignedRoles", "m365_defender.event.account.assigned_roles")?;
                }

                if event.has("json.properties.ChangeSource") {
                    event.rename("json.properties.ChangeSource", "m365_defender.event.change_source")?;
                }

                if event.has("json.properties.EmailAddress") {
                    event.rename("json.properties.EmailAddress", "m365_defender.event.account.email_address")?;
                }

                if event.has("json.properties.Address") {
                    event.rename("json.properties.Address", "m365_defender.event.account.address")?;
                }

                if event.has("json.properties.Phone") {
                    event.rename("json.properties.Phone", "m365_defender.event.account.phone")?;
                }

                if event.has("json.properties.Manager") {
                    event.rename("json.properties.Manager", "m365_defender.event.account.manager")?;
                }

                if event.has("json.properties.SipProxyAddress") {
                    event.rename("json.properties.SipProxyAddress", "m365_defender.event.account.sip_proxy_address")?;
                }

                if event.has("json.properties.CreatedDateTime") {
                    event.rename("json.properties.CreatedDateTime", "m365_defender.event.account.created")?;
                }

                if event.has("json.properties.JobTitle") {
                    event.rename("json.properties.JobTitle", "m365_defender.event.account.job_title")?;
                }

                if event.has("json.properties.Department") {
                    event.rename("json.properties.Department", "m365_defender.event.account.department")?;
                }

                if event.has("json.properties.EmployeeId") {
                    event.rename("json.properties.EmployeeId", "m365_defender.event.account.employee_id")?;
                }

                if event.has("json.properties.Surname") {
                    event.rename("json.properties.Surname", "m365_defender.event.account.surname")?;
                }

                if event.has("json.properties.GivenName") {
                    event.rename("json.properties.GivenName", "m365_defender.event.account.given_name")?;
                }

                if event.has("json.properties.Type") {
                    event.rename("json.properties.Type", "m365_defender.event.type")?;
                }

                if event.has("json.properties.DistinguishedName") {
                    event.rename("json.properties.DistinguishedName", "m365_defender.event.account.distinguished_name")?;
                }

                if event.has("json.properties.CloudSid") {
                    event.rename("json.properties.CloudSid", "m365_defender.event.account.cloud_sid")?;
                }

                if event.has("json.properties.Tags") {
                    event.rename("json.properties.Tags", "m365_defender.event.account.tags")?;
                }

                if event.has("json.properties.BlastRadius") {
                    event.rename("json.properties.BlastRadius", "m365_defender.event.account.blast_radius")?;
                }

                if event.has("json.properties.OtherMailAddresses") {
                    event.rename("json.properties.OtherMailAddresses", "m365_defender.event.account.other_mail_addresses")?;
                }

                if event.has("json.properties.CompanyName") {
                    event.rename("json.properties.CompanyName", "m365_defender.event.account.company_name")?;
                }

                if event.has("json.properties.DeletedDateTime") {
                    event.rename("json.properties.DeletedDateTime", "m365_defender.event.account.deleted_date_time")?;
                }

                if event.has("json.properties.CriticalityLevel") {
                    event.rename("json.properties.CriticalityLevel", "m365_defender.event.account.criticality_level")?;
                }

                if event.has("json.properties.RiskLevel") {
                    event.rename("json.properties.RiskLevel", "m365_defender.event.account.risk_level")?;
                }

                if event.has("json.properties.RiskLevelDetails") {
                    event.rename("json.properties.RiskLevelDetails", "m365_defender.event.account.risk_level_details")?;
                }

                if event.has("json.properties.DataSource") {
                    event.rename("json.properties.DataSource", "m365_defender.event.data_source")?;
                }

                if event.has("json.properties.OperationName") {
                    event.rename("json.properties.OperationName", "m365_defender.event.properties_operation_name")?;
                }

                if event.has("json.properties.ResourceId") {
                    event.rename("json.properties.ResourceId", "m365_defender.event.resource_id")?;
                }

                if event.has("json.properties.ContainerId") {
                    event.rename("json.properties.ContainerId", "m365_defender.event.container_id")?;
                }

                if event.has("json.properties.ContainerImageName") {
                    event.rename("json.properties.ContainerImageName", "m365_defender.event.container_image_name")?;
                }

                if event.has("json.properties.ContainerName") {
                    event.rename("json.properties.ContainerName", "m365_defender.event.container_name")?;
                }

                if event.has("json.properties.KubernetesNamespace") {
                    event.rename("json.properties.KubernetesNamespace", "m365_defender.event.kubernetes_namespace")?;
                }

                if event.has("json.properties.KubernetesPodName") {
                    event.rename("json.properties.KubernetesPodName", "m365_defender.event.kubernetes_pod_name")?;
                }

                if event.has("json.properties.KubernetesResource") {
                    event.rename("json.properties.KubernetesResource", "m365_defender.event.kubernetes_resource")?;
                }

                if event.has("json.properties.ParentProcessId") {
                    event.rename("json.properties.ParentProcessId", "m365_defender.event.parent_process_id")?;
                }

                if event.has("json.properties.ParentProcessName") {
                    event.rename("json.properties.ParentProcessName", "m365_defender.event.parent_process_name")?;
                }

                if event.has("json.properties.ProcessCurrentWorkingDirectory") {
                    event.rename("json.properties.ProcessCurrentWorkingDirectory", "m365_defender.event.process_current_working_directory")?;
                }

                if event.has("json.properties.ProcessName") {
                    event.rename("json.properties.ProcessName", "m365_defender.event.process_name")?;
                }

                if event.has("json.properties.DataSources") {
                    event.rename("json.properties.DataSources", "m365_defender.event.data_sources")?;
                }

                if event.has("json.properties.ResourceGroup") {
                    event.rename("json.properties.ResourceGroup", "m365_defender.event.resource_group")?;
                }

                if event.has("json.properties.StorageAccount") {
                    event.rename("json.properties.StorageAccount", "m365_defender.event.storage_account")?;
                }

                if event.has("json.properties.StorageContainer") {
                    event.rename("json.properties.StorageContainer", "m365_defender.event.storage_container")?;
                }

                if event.has("json.properties.StorageFileShare") {
                    event.rename("json.properties.StorageFileShare", "m365_defender.event.storage_file_share")?;
                }

                if event.has("json.properties.ServiceType") {
                    event.rename("json.properties.ServiceType", "m365_defender.event.service_type")?;
                }

                if event.has("json.properties.UserAgentHeader") {
                    event.rename("json.properties.UserAgentHeader", "m365_defender.event.user_agent_header")?;
                }

                if event.has("json.properties.OperationNamesList") {
                    event.rename("json.properties.OperationNamesList", "m365_defender.event.operation_names_list")?;
                }

                if event.has("json.properties.AuthenticationType") {
                    event.rename("json.properties.AuthenticationType", "m365_defender.event.authentication_type")?;
                }

            if event.has_value("json.properties.AccountTenantId") {
                if let Some(val) = event.get("json.properties.AccountTenantId") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.AccountTenantId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.account_tenant_id", converted)?;
                }
            }

                if event.has("json.properties.AccountApplicationId") {
                    event.rename("json.properties.AccountApplicationId", "m365_defender.event.account_application_id")?;
                }

                if event.has("json.properties.SuspiciousUserAgentName") {
                    event.rename("json.properties.SuspiciousUserAgentName", "m365_defender.event.suspicious_user_agent_name")?;
                }

                if event.has("json.properties.HashReputationMd5List") {
                    event.rename("json.properties.HashReputationMd5List", "m365_defender.event.hash_reputation_md5_list")?;
                }

                if event.has("json.properties.SubscriptionId") {
                    event.rename("json.properties.SubscriptionId", "m365_defender.event.subscription_id")?;
                }

                if event.has("json.properties.CountryName") {
                    event.rename("json.properties.CountryName", "m365_defender.event.country_name")?;
                }

                if event.has("json.properties.CityName") {
                    event.rename("json.properties.CityName", "m365_defender.event.city_name")?;
                }

                if event.has("json.properties.ProvinceName") {
                    event.rename("json.properties.ProvinceName", "m365_defender.event.province_name")?;
                }

                if event.has("json.properties.ClientSystemServiceName") {
                    event.rename("json.properties.ClientSystemServiceName", "m365_defender.event.client_system_service_name")?;
                }

                if event.has("json.properties.ClientCloudPlatformName") {
                    event.rename("json.properties.ClientCloudPlatformName", "m365_defender.event.client_cloud_platform_name")?;
                }

            if let Some(v) = event.get("m365_defender.event.kubernetes_namespace").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.namespace", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.kubernetes_resource").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.resource.name", v)?;
            }

            let _cond = { event.has_value("m365_defender.event.kubernetes_namespace") || event.has_value("m365_defender.event.kubernetes_pod_name") || event.has_value("m365_defender.event.kubernetes_resource") };
            if _cond {
            event.set("orchestrator.type", json!("kubernetes"))?;
            }

            if let Some(v) = event.get("m365_defender.event.target.device_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.domain", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.destination.ip_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.device.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
            }

            let _cond = { event.has_value("m365_defender.event.ip_address") && event.get_str("m365_defender.event.ip_address") != Some("") };
            if _cond {
                event.append("host.ip", json!(event.get("m365_defender.event.ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.os.platform") && event.get_str("m365_defender.event.os.platform").is_some_and(|s| s.to_lowercase().contains("windows")) };
            if _cond {
            event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { event.has_value("m365_defender.event.os.platform") && event.get_str("m365_defender.event.os.platform").is_some_and(|s| s.to_lowercase().contains("linux")) };
            if _cond {
            event.set("host.os.type", json!("linux"))?;
            }

            let _cond = { event.has_value("m365_defender.event.os.platform") && event.get_str("m365_defender.event.os.platform").is_some_and(|s| s.to_lowercase().contains("macos")) };
            if _cond {
            event.set("host.os.type", json!("macos"))?;
            }

            let _cond = { event.has_value("m365_defender.event.additional_fields.SourceComputerOperatingSystemType") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.additional_fields.SourceComputerOperatingSystemType").cloned() {
                event.set("host.os.type", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.os.platform").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.device.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.type", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.container_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.container_image_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.image.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.container_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.parent_process_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pid", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.parent_process_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process_current_working_directory").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.working_directory", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.name", v)?;
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

            let _cond = { (!event.has_value("m365_defender.event.failure_reason") || event.get_str("m365_defender.event.failure_reason") == Some("")) && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase() == "advancedhunting-identitylogonevents") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { (event.has_value("m365_defender.event.failure_reason") && event.get_str("m365_defender.event.failure_reason") != Some("")) && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase() == "advancedhunting-identitylogonevents") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            if let Some(v) = event.get("m365_defender.event.report_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
            }

            if let Some(v) = event.get("m365_defender.event.account.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identitylogonevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.account.sid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("cloudappevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.account.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identityinfo")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.account.object_id").cloned() {
                event.set("user.id", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.account.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.raw_event_data.UserId").filter(|v| !painless_is_empty_value(v)).cloned() {
                if !event.has("user.name") {
                    event.set("user.name", v)?;
                }
            }

            if let Some(v) = event.get("m365_defender.event.account.display_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                if !event.has("user.name") {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("m365_defender.event.account.display_name") && !event.has_value("user.full_name") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("identityinfo")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.account.display_name").cloned() {
                event.set("user.full_name", v)?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.account.email_address") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.account.email_address").cloned() {
                event.set("user.email", v)?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.user_agent") && event.get_str("m365_defender.event.user_agent") != Some("") };
            if _cond {
                if let Some(ua_str) = event.get_string("m365_defender.event.user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }

            let _cond = { event.has_value("m365_defender.event.user_agent_header") && event.get_str("m365_defender.event.user_agent_header") != Some("") };
            if _cond {
                if let Some(ua_str) = event.get_string("m365_defender.event.user_agent_header") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }

            let _cond = { event.has_value("m365_defender.event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("cloudauditevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.ip_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.ip", v)?;
            }
            }

            let _cond = { event.get_bool("m365_defender.event.is_private_ip") == Some(false) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.storage_ip_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.ip", v)?;
            }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.account.display_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.account.display_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.target.account_display_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.target.account_display_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.account.cloud_sid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.account.cloud_sid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.account.on_prem_sid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.account.on_prem_sid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.account.upn") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.account.upn").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.account.email_address") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.account.email_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("destination.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "host.ip", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("user.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.account.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.account.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("m365_defender.event.target.device_name");
                event.remove("m365_defender.event.destination.ip_address");
                event.remove("m365_defender.event.destination.port");
                event.remove("m365_defender.event.device.name");
                event.remove("m365_defender.event.ip_address");
                event.remove("m365_defender.event.os.platform");
                event.remove("m365_defender.event.device.type");
                event.remove("m365_defender.event.protocol");
                event.remove("m365_defender.event.account.domain");
                event.remove("m365_defender.event.account.email_address");
                event.remove("m365_defender.event.account.sid");
                event.remove("m365_defender.event.account.id");
                event.remove("m365_defender.event.account.display_name");
                event.remove("m365_defender.event.account.name");
                event.remove("m365_defender.event.action.type");
                event.remove("m365_defender.event.report_id");
                event.remove("m365_defender.event.container_id");
                event.remove("m365_defender.event.container_image_name");
                event.remove("m365_defender.event.container_name");
                event.remove("m365_defender.event.kubernetes_namespace");
                event.remove("m365_defender.event.kubernetes_resource");
                event.remove("m365_defender.event.parent_process_id");
                event.remove("m365_defender.event.parent_process_name");
                event.remove("m365_defender.event.process_current_working_directory");
                event.remove("m365_defender.event.process_name");
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
