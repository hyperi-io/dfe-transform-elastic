// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_alert` pipeline.
pub struct PipelineAlert;

impl Transform for PipelineAlert {
    fn name(&self) -> &str {
        "pipeline_alert"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("behavior")) };
            if _cond {
            event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.has_value("json.properties.EntityType") && event.get_str("json.properties.EntityType").is_some_and(|s| s.to_lowercase() == "file") };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = { event.has_value("json.properties.EntityType") && event.get_str("json.properties.EntityType").is_some_and(|s| s.to_lowercase() == "process") };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

            let _cond = { event.has_value("json.properties.EntityType") && event.get_str("json.properties.EntityType").is_some_and(|s| s.to_lowercase() == "device") };
            if _cond {
                event.append("event.category", json!("host"))?;
            }

            let _cond = { event.has_value("json.properties.EntityType") && event.get_str("json.properties.EntityType").is_some_and(|s| s.to_lowercase() == "user") };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = { event.has_value("json.properties.Category") && event.get_str("json.properties.Category").is_some_and(|s| ["malware", "ransomware"].contains(&s.to_lowercase().as_str())) };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category") != Some("AdvancedHunting-AlertInfo") && event.has_value("json.properties.Category") && event.get_str("json.properties.Category").is_some_and(|s| ["persistence", "privilegeescalation", "suspiciousactivity", "threatmanagement"].contains(&s.to_lowercase().as_str())) };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
                event.append("event.type", json!("indicator"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get("json.properties.Categories").is_some_and(|v| v.is_string()) && event.get_str("json.properties.Categories") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.Categories", "json.properties.Categories")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_properties_Categories")?;
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

            let _cond = { event.get("json.properties.AttackTechniques").is_some_and(|v| v.is_string()) && event.get_str("json.properties.AttackTechniques") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.properties.AttackTechniques", "json.properties.AttackTechniques")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_properties_AttackTechniques")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.RemoteIP") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.RemoteIP") {
                if let Some(val) = event.get("json.properties.RemoteIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.RemoteIP".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.remote.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_RemoteIP")?;
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
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_FileSize")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.LocalIP") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.LocalIP") {
                if let Some(val) = event.get("json.properties.LocalIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.LocalIP".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.local.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_LocalIP")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.properties.StartTime") && event.get_str("json.properties.StartTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.StartTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.start_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_StartTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.properties.EndTime") && event.get_str("json.properties.EndTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.EndTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.end_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_EndTime")?;
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

                if event.has("json.properties.AlertId") {
                    event.rename("json.properties.AlertId", "m365_defender.event.alert.id")?;
                }

                if event.has("json.properties.ServiceSource") {
                    event.rename("json.properties.ServiceSource", "m365_defender.event.service_source")?;
                }

                if event.has("json.properties.DeviceName") {
                    event.rename("json.properties.DeviceName", "m365_defender.event.device.name")?;
                }

                if event.has("json.properties.NetworkMessageId") {
                    event.rename("json.properties.NetworkMessageId", "m365_defender.event.network.message_id")?;
                }

                if event.has("json.properties.OAuthApplicationId") {
                    event.rename("json.properties.OAuthApplicationId", "m365_defender.event.oauth_application_id")?;
                }

                if event.has("json.properties.RemoteUrl") {
                    event.rename("json.properties.RemoteUrl", "m365_defender.event.remote.url")?;
                }

                if event.has("json.properties.AttackTechniques") {
                    event.rename("json.properties.AttackTechniques", "m365_defender.event.attack_techniques")?;
                }

                if event.has("json.properties.AccountObjectId") {
                    event.rename("json.properties.AccountObjectId", "m365_defender.event.account.object_id")?;
                }

                if event.has("json.properties.Category") {
                    event.rename("json.properties.Category", "m365_defender.event.alert.category")?;
                }

                if event.has("json.properties.Categories") {
                    event.rename("json.properties.Categories", "m365_defender.event.alert.categories")?;
                }

                if event.has("json.properties.DetectionSource") {
                    event.rename("json.properties.DetectionSource", "m365_defender.event.detection.source")?;
                }

                if event.has("json.properties.MachineGroup") {
                    event.rename("json.properties.MachineGroup", "m365_defender.event.machine_group")?;
                }

                if event.has("json.properties.DeviceId") {
                    event.rename("json.properties.DeviceId", "m365_defender.event.device.id")?;
                }

                if event.has("json.properties.EvidenceDirection") {
                    event.rename("json.properties.EvidenceDirection", "m365_defender.event.evidence.direction")?;
                }

                if event.has("json.properties.ProcessCommandLine") {
                    event.rename("json.properties.ProcessCommandLine", "m365_defender.event.process.command_line")?;
                }

                if event.has("json.properties.RegistryKey") {
                    event.rename("json.properties.RegistryKey", "m365_defender.event.registry.key")?;
                }

                if event.has("json.properties.RegistryValueName") {
                    event.rename("json.properties.RegistryValueName", "m365_defender.event.registry.value_name")?;
                }

                if event.has("json.properties.RegistryValueData") {
                    event.rename("json.properties.RegistryValueData", "m365_defender.event.registry.value_data")?;
                }

                if event.has("json.properties.SHA1") {
                    event.rename("json.properties.SHA1", "m365_defender.event.sha1")?;
                }

                if event.has("json.properties.FolderPath") {
                    event.rename("json.properties.FolderPath", "m365_defender.event.folder_path")?;
                }

                if event.has("json.properties.SHA256") {
                    event.rename("json.properties.SHA256", "m365_defender.event.sha256")?;
                }

                if event.has("json.properties.FileName") {
                    event.rename("json.properties.FileName", "m365_defender.event.file.name")?;
                }

                if event.has("json.properties.ThreatFamily") {
                    event.rename("json.properties.ThreatFamily", "m365_defender.event.threat.family")?;
                }

                if event.has("json.properties.AccountSid") {
                    event.rename("json.properties.AccountSid", "m365_defender.event.account.sid")?;
                }

                if event.has("json.properties.AccountName") {
                    event.rename("json.properties.AccountName", "m365_defender.event.account.name")?;
                }

                if event.has("json.properties.Title") {
                    event.rename("json.properties.Title", "m365_defender.event.title")?;
                }

                if event.has("json.properties.AccountDomain") {
                    event.rename("json.properties.AccountDomain", "m365_defender.event.account.domain")?;
                }

                if event.has("json.properties.AccountUpn") {
                    event.rename("json.properties.AccountUpn", "m365_defender.event.account.upn")?;
                }

                if event.has("json.properties.AdditionalFields") {
                    event.rename("json.properties.AdditionalFields", "m365_defender.event.additional_fields")?;
                }

                if event.has("json.properties.Application") {
                    event.rename("json.properties.Application", "m365_defender.event.application")?;
                }

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

                if event.has("json.properties.ActionType") {
                    event.rename("json.properties.ActionType", "m365_defender.event.action.type")?;
                }

                if event.has("json.properties.BehaviorId") {
                    event.rename("json.properties.BehaviorId", "m365_defender.event.behavior_id")?;
                }

                if event.has("json.properties.CloudResourceType") {
                    event.rename("json.properties.CloudResourceType", "m365_defender.event.cloud_resource_type")?;
                }

                if event.has("json.properties.CloudResourceId") {
                    event.rename("json.properties.CloudResourceId", "m365_defender.event.cloud_resource_id")?;
                }

                if event.has("json.properties.CloudSubscriptionId") {
                    event.rename("json.properties.CloudSubscriptionId", "m365_defender.event.cloud_subscription_id")?;
                }

                if event.has("json.properties.CloudPlatform") {
                    event.rename("json.properties.CloudPlatform", "m365_defender.event.cloud_platform")?;
                }

                if event.has("json.properties.DataSources") {
                    event.rename("json.properties.DataSources", "m365_defender.event.data_sources")?;
                }

                if event.has("json.properties.Description") {
                    event.rename("json.properties.Description", "m365_defender.event.description")?;
                }

                if event.has("json.properties.DetailedEntityRole") {
                    event.rename("json.properties.DetailedEntityRole", "m365_defender.event.detailed_entity_role")?;
                }

                if event.has("json.properties.EmailSubject") {
                    event.rename("json.properties.EmailSubject", "m365_defender.event.email.subject")?;
                }

                if event.has("json.properties.EntityType") {
                    event.rename("json.properties.EntityType", "m365_defender.event.entity_type")?;
                }

                if event.has("json.properties.EvidenceRole") {
                    event.rename("json.properties.EvidenceRole", "m365_defender.event.evidence.role")?;
                }

                if event.has("json.properties.EntityRole") {
                    event.rename("json.properties.EntityRole", "m365_defender.event.entity_role")?;
                }

            if let Some(v) = event.get("m365_defender.event.alert.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.service_source").cloned() {
                event.set("event.provider", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.behavior_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.remote.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reference", v)?;
            }

            if event.has_value("json.properties.Severity") {
                map_strings(event, "json.properties.Severity", "m365_defender.event.severity", str::to_lowercase)?;
            }

            let _cond = { event.get("m365_defender.event.severity").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.m365_defender.event.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.m365_defender.event.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("m365_defender.event.device.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }

            if event.has_value("host.name") {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
            }

            if let Some(v) = event.get("m365_defender.event.device.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.evidence.direction").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.direction", v)?;
            }

            if event.has_value("network.direction") {
                map_strings(event, "network.direction", "network.direction", str::to_lowercase)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.command_line").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.command_line", v)?;
            }

            let _cond = { event.has_value("process.command_line") && event.get_str("process.command_line") != Some("") };
            if _cond {
                // Painless script
                // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n    ctx.process.executable = ctx.process.args[0];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n    ctx.process.executable = ctx.process.args[0];\n}"#))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.registry.key").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.registry.key", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) && event.has_value("m365_defender.event.registry.value_data") };
            if _cond {
                event.append_unique("threat.indicator.registry.data.strings", json!(event.get("m365_defender.event.registry.value_data").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.registry.value_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.registry.value", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.directory", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.size", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.threat.family").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.group.name", v)?;
            }
            }

            let _cond = { event.get("m365_defender.event.attack_techniques").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def subtechnique_name = new ArrayList();\ndef subtechnique_id = new ArrayList();\nif (!(ctx.threat instanceof HashMap)) {\n  ctx.threat = new HashMap();\n}\nif (!(ctx.threat.technique instanceof HashMap)) {\n  ctx.threat.technique = new HashMap();\n}\nif (!(ctx.threat.technique.subtechnique instanceof HashMap)) {\n  ctx.threat.technique.subtechnique = new HashMap();\n}\nfor (item in ctx.m365_defender.event.attack_techniques) {\n  subtechnique_name.add(item.substring(0,item.lastIndexOf(' ')));\n  subtechnique_id.add(item.substring(item.indexOf('(')+1,item.indexOf(')')));\n}\nctx.threat.technique.subtechnique.id = subtechnique_id;\nctx.threat.technique.subtechnique.name = subtechnique_name;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def subtechnique_name = new ArrayList();\ndef subtechnique_id = new ArrayList();\nif (!(ctx.threat instanceof HashMap)) {\n  ctx.threat = new HashMap();\n}\nif (!(ctx.threat.technique instanceof HashMap)) {\n  ctx.threat.technique = new HashMap();\n}\nif (!(ctx.threat.technique.subtechnique instanceof HashMap)) {\n  ctx.threat.technique.subtechnique = new HashMap();\n}\nfor (item in ctx.m365_defender.event.attack_techniques) {\n  subtechnique_name.add(item.substring(0,item.lastIndexOf(' ')));\n  subtechnique_id.add(item.substring(item.indexOf('(')+1,item.indexOf(')')));\n}\nctx.threat.technique.subtechnique.id = subtechnique_id;\nctx.threat.technique.subtechnique.name = subtechnique_name;\n"#))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("m365_defender.event.attack_techniques") && event.get_str("m365_defender.event.attack_techniques") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                let sorted = event.get("m365_defender.event.attack_techniques").and_then(|v| sort_values(v, false));
                if event.has("m365_defender.event.attack_techniques") {
                    match sorted {
                        Some(sorted) => event.set("m365_defender.event.attack_techniques", Value::Array(sorted))?,
                        None => return Err(TransformError::ParseError {
                            path: "m365_defender.event.attack_techniques".into(),
                            message: "cannot sort: not an array of one comparable kind".into(),
                        }),
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "sort")?;
                event.set("_ingest.on_failure_processor_tag", "sort_m365_defender_event_attack_techniques")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("threat.technique.subtechnique.id") && event.get_str("threat.technique.subtechnique.id") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                let sorted = event.get("threat.technique.subtechnique.id").and_then(|v| sort_values(v, false));
                if event.has("threat.technique.subtechnique.id") {
                    match sorted {
                        Some(sorted) => event.set("threat.technique.subtechnique.id", Value::Array(sorted))?,
                        None => return Err(TransformError::ParseError {
                            path: "threat.technique.subtechnique.id".into(),
                            message: "cannot sort: not an array of one comparable kind".into(),
                        }),
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "sort")?;
                event.set("_ingest.on_failure_processor_tag", "sort_threat_technique_subtechnique_id")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("threat.technique.subtechnique.name") && event.get_str("threat.technique.subtechnique.name") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                let sorted = event.get("threat.technique.subtechnique.name").and_then(|v| sort_values(v, false));
                if event.has("threat.technique.subtechnique.name") {
                    match sorted {
                        Some(sorted) => event.set("threat.technique.subtechnique.name", Value::Array(sorted))?,
                        None => return Err(TransformError::ParseError {
                            path: "threat.technique.subtechnique.name".into(),
                            message: "cannot sort: not an array of one comparable kind".into(),
                        }),
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "sort")?;
                event.set("_ingest.on_failure_processor_tag", "sort_threat_technique_subtechnique_name")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.directory", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.registry.key").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("registry.key", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("threat")), serde_json::Value::String(s) => s.contains("threat"), _ => false })) && event.has_value("m365_defender.event.registry.value_data") };
            if _cond {
                event.append_unique("registry.data.strings", json!(event.get("m365_defender.event.registry.value_data").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("m365_defender.event.remote.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.local.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.account.sid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.account.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.account.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.account.object_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.email.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.network.message_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.message_id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.cloud_platform").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("cloud.provider", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.title").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("user.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.account.object_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.account.object_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("m365_defender.event.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("m365_defender.event.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("m365_defender.event.timestamp");
                event.remove("m365_defender.event.remote.url");
                event.remove("m365_defender.event.alert.id");
                event.remove("m365_defender.event.cloud_platform");
                event.remove("m365_defender.event.service_source");
                event.remove("m365_defender.event.device.name");
                event.remove("m365_defender.event.device.id");
                event.remove("m365_defender.event.evidence.direction");
                event.remove("m365_defender.event.process.command_line");
                event.remove("m365_defender.event.registry.key");
                event.remove("m365_defender.event.registry.value_name");
                event.remove("m365_defender.event.registry.value_data");
                event.remove("m365_defender.event.remote.ip");
                event.remove("m365_defender.event.folder_path");
                event.remove("m365_defender.event.sha1");
                event.remove("m365_defender.event.sha256");
                event.remove("m365_defender.event.file.name");
                event.remove("m365_defender.event.file.size");
                event.remove("m365_defender.event.threat.family");
                event.remove("m365_defender.event.account.name");
                event.remove("m365_defender.event.account.domain");
                event.remove("m365_defender.event.account.sid");
                event.remove("m365_defender.event.account.object_id");
                event.remove("m365_defender.event.network.message_id");
                event.remove("m365_defender.event.email.subject");
                event.remove("m365_defender.event.title");
                event.remove("m365_defender.event.behavior_id");
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
