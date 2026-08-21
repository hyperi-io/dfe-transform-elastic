// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_device` pipeline.
pub struct PipelineDevice;

impl Transform for PipelineDevice {
    fn name(&self) -> &str {
        "pipeline_device"
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

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("devicelogonevents")) };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceinfo")) || (event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")) && event.has_value("json.properties.ActionType") && !(event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().ends_with("apicall"))) && !(event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("driverload")))) };
            if _cond {
                event.append("event.category", json!("host"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")) && event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().ends_with("apicall")) };
            if _cond {
                event.append("event.category", json!("api"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("driverload")) };
            if _cond {
                event.append("event.category", json!("driver"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("devicefileevents")) || event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("devicefilecertificateinfo")) };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceprocessevents")) || event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("cloudprocessevents")) };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")) };
            if _cond {
                event.append("event.category", json!("library"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("devicenetworkevents")) || event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("devicenetworkinfo")) };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceregistryevents")) };
            if _cond {
                event.append("event.category", json!("registry"))?;
            }

            let _cond = { (event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("host")), serde_json::Value::String(s) => s.contains("host"), _ => false }))) || (event.has_value("json.properties.ActionType") && (event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("connectionfound")) || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("networksignatureinspected")) || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("devicenetworkinfo")))) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) && event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase() == "filedeleted") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) && event.has_value("json.properties.ActionType") && (event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase() == "filemodified") || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase() == "filerenamed")) };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) && event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase() == "filecreated") };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.has_value("event.category") && !event.has_value("event.type") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && (event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("connectionsuccess")) || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("driverload"))) };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("connectionfailed")) };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && (event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("connectionrequest")) || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("listeningconnectioncreated")) || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("processcreated")) || event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))) };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("inboundconnectionaccepted")) };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("registrykeycreated")) };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && (event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("registrykeydeleted")) || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("registryvaluedeleted"))) };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && (event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("registrykeyrenamed")) || event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("registryvalueset"))) };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("imageloaded")) };
            if _cond {
            event.set("json.properties.ActionType", json!("load"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                event.append("event.type", json!("protocol"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) && !event.has_value("event.type") && event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().starts_with("open")) };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) && !event.has_value("event.type") && event.has_value("json.properties.ActionType") && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().starts_with("write")) };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.has_value("event.category") && !event.has_value("event.type") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get("json.properties.AdditionalFields").is_some_and(|v| v.is_string()) && event.get_str("json.properties.AdditionalFields") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.properties.AdditionalFields") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "json.properties.AdditionalFields".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json.properties.AdditionalFields", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_json_properties_AdditionalFields")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has("json.properties.AdditionalFields") {
                    event.rename("json.properties.AdditionalFields", "m365_defender.event.additional_fields")?;
                }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.direction") {
                    event.rename("m365_defender.event.additional_fields.direction", "m365_defender.event.network_direction")?;
                }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.qclass_name") {
                    event.rename("m365_defender.event.additional_fields.qclass_name", "m365_defender.event.dns.qclass_name")?;
                }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.query") {
                    event.rename("m365_defender.event.additional_fields.query", "m365_defender.event.dns.query")?;
                }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.qtype_name") {
                    event.rename("m365_defender.event.additional_fields.qtype_name", "m365_defender.event.dns.qtype_name")?;
                }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.rcode_name") {
                    event.rename("m365_defender.event.additional_fields.rcode_name", "m365_defender.event.dns.rcode_name")?;
                }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("m365_defender.event.additional_fields.answers") && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("m365_defender.event.additional_fields.answers") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "m365_defender.event.additional_fields.answers".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("m365_defender.event.additional_fields.answers", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_m365_defender_event_additional_fields_answers")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.answers") {
                    event.rename("m365_defender.event.additional_fields.answers", "m365_defender.event.dns.answers")?;
                }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("m365_defender.event.additional_fields.TTLs") && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("m365_defender.event.additional_fields.TTLs") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "m365_defender.event.additional_fields.TTLs".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("m365_defender.event.additional_fields.TTLs", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_m365_defender_event_additional_fields_ttls")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.TTLs") {
                    event.rename("m365_defender.event.additional_fields.TTLs", "m365_defender.event.dns.ttls")?;
                }
            }

            let _cond = { event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_object()) && event.has_value("json.properties.ActionType") && event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("json.properties.ActionType").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
                // Painless script
                // Source: def af = ctx.m365_defender.event.additional_fields;\nList ecs_flags = [\"AA\", \"TC\", \"RD\", \"RA\", \"AD\", \"CD\", \"DO\"];\nList flags = [];\nif (af instanceof Map) {\n    for (def flag: ecs_flags) {\n        if (af[flag] != null && af[flag] == \"true\") {\n            flags.add(flag);\n        }\n    }\n}\nif (!ctx.m365_defender.event.containsKey('dns')) {\n    ctx.m365_defender.event.dns = new HashMap();\n}\nctx.m365_defender.event.dns.header_flags = flags;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(event, cached_script!(r#"def af = ctx.m365_defender.event.additional_fields;\nList ecs_flags = [\"AA\", \"TC\", \"RD\", \"RA\", \"AD\", \"CD\", \"DO\"];\nList flags = [];\nif (af instanceof Map) {\n    for (def flag: ecs_flags) {\n        if (af[flag] != null && af[flag] == \"true\") {\n            flags.add(flag);\n        }\n    }\n}\nif (!ctx.m365_defender.event.containsKey('dns')) {\n    ctx.m365_defender.event.dns = new HashMap();\n}\nctx.m365_defender.event.dns.header_flags = flags;\n"#))?;
            }

            let _cond = { event.get("json.properties.CrlDistributionPointUrls").is_some_and(|v| v.is_string()) && event.get_str("json.properties.CrlDistributionPointUrls") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.properties.CrlDistributionPointUrls") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "json.properties.CrlDistributionPointUrls".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json.properties.CrlDistributionPointUrls", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_json_properties_CrlDistributionPointUrls")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.LoggedOnUsers").is_some_and(|v| v.is_string()) && event.get_str("json.properties.LoggedOnUsers") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.properties.LoggedOnUsers") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "json.properties.LoggedOnUsers".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json.properties.LoggedOnUsers", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_json_properties_LoggedOnUsers")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.ConnectedNetworks").is_some_and(|v| v.is_string()) && event.get_str("json.properties.ConnectedNetworks") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.properties.ConnectedNetworks") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "json.properties.ConnectedNetworks".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json.properties.ConnectedNetworks", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_json_properties_ConnectedNetworks")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.DefaultGateways").is_some_and(|v| v.is_string()) && event.get_str("json.properties.DefaultGateways") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.properties.DefaultGateways") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "json.properties.DefaultGateways".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json.properties.DefaultGateways", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_json_properties_DefaultGateways")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.DnsAddresses").is_some_and(|v| v.is_string()) && event.get_str("json.properties.DnsAddresses") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.properties.DnsAddresses") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "json.properties.DnsAddresses".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json.properties.DnsAddresses", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_json_properties_DnsAddresses")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.IPAddresses").is_some_and(|v| v.is_string()) && event.get_str("json.properties.IPAddresses") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.properties.IPAddresses") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "json.properties.IPAddresses".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json.properties.IPAddresses", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_json_properties_IPAddresses")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.CertificateExpirationTime").is_some_and(|v| v.is_string()) && event.get_str("json.properties.CertificateExpirationTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.CertificateExpirationTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.certificate.expiration_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_properties_CertificateExpirationTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.InitiatingProcessCreationTime").is_some_and(|v| v.is_string()) && event.get_str("json.properties.InitiatingProcessCreationTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.InitiatingProcessCreationTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.initiating_process.creation_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_properties_InitiatingProcessCreationTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.InitiatingProcessParentCreationTime").is_some_and(|v| v.is_string()) && event.get_str("json.properties.InitiatingProcessParentCreationTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.InitiatingProcessParentCreationTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.initiating_process.parent_creation_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_properties_InitiatingProcessParentCreationTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.ProcessCreationTime").is_some_and(|v| v.is_string()) && event.get_str("json.properties.ProcessCreationTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.ProcessCreationTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.process.creation_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_properties_ProcessCreationTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.CertificateCountersignatureTime").is_some_and(|v| v.is_string()) && event.get_str("json.properties.CertificateCountersignatureTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.CertificateCountersignatureTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.certificate.countersignature_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_properties_CertificateCountersignatureTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.properties.CertificateCreationTime").is_some_and(|v| v.is_string()) && event.get_str("json.properties.CertificateCreationTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.properties.CertificateCreationTime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("m365_defender.event.certificate.creation_time", parsed)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_properties_CertificateCreationTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.InitiatingProcessFileSize") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.InitiatingProcessFileSize") {
                if let Some(val) = event.get("json.properties.InitiatingProcessFileSize") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.InitiatingProcessFileSize".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.initiating_process.file_size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_InitiatingProcessFileSize")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.InitiatingProcessLogonId") != Some("") };
            if _cond {
            if event.has_value("json.properties.InitiatingProcessLogonId") {
                if let Some(val) = event.get("json.properties.InitiatingProcessLogonId") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.InitiatingProcessLogonId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.initiating_process.logon_id", converted)?;
                }
            }
            }

            let _cond = { event.get_str("json.properties.LogonId") != Some("") };
            if _cond {
            if event.has_value("json.properties.LogonId") {
                if let Some(val) = event.get("json.properties.LogonId") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.LogonId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.logon.id", converted)?;
                }
            }
            }

            let _cond = { event.get_str("json.properties.ProcessId") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.ProcessId") {
                if let Some(val) = event.get("json.properties.ProcessId") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.ProcessId".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.process.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_ProcessId")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
            }

            let _cond = { event.get_str("json.properties.IPv4Dhcp") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.IPv4Dhcp") {
                if let Some(val) = event.get("json.properties.IPv4Dhcp") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IPv4Dhcp".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.ipv4_dhcp", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_IPv4Dhcp")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.properties.IPv6Dhcp") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.properties.IPv6Dhcp") {
                if let Some(val) = event.get("json.properties.IPv6Dhcp") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.IPv6Dhcp".into(),
                            message,
                        })?;
                    event.set("m365_defender.event.ipv6_dhcp", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_properties_IPv6Dhcp")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                // Painless script
                // Source: def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_root_signer_microsoft = isTruthy(ctx.json?.properties?.IsRootSignerMicrosoft);\n ctx.m365_defender.event.is_signed = isTruthy(ctx.json?.properties?.IsSigned);\n ctx.m365_defender.event.is_trusted = isTruthy(ctx.json?.properties?.IsTrusted);\n ctx.m365_defender.event.is_azure_info_protection_applied = isTruthy(ctx.json?.properties?.IsAzureInfoProtectionApplied);\n ctx.m365_defender.event.is_azure_ad_joined = isTruthy(ctx.json?.properties?.IsAzureADJoined);\n ctx.m365_defender.event.is_local_admin = isTruthy(ctx.json?.properties?.IsLocalAdmin);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(event, cached_script!(r#"def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_root_signer_microsoft = isTruthy(ctx.json?.properties?.IsRootSignerMicrosoft);\n ctx.m365_defender.event.is_signed = isTruthy(ctx.json?.properties?.IsSigned);\n ctx.m365_defender.event.is_trusted = isTruthy(ctx.json?.properties?.IsTrusted);\n ctx.m365_defender.event.is_azure_info_protection_applied = isTruthy(ctx.json?.properties?.IsAzureInfoProtectionApplied);\n ctx.m365_defender.event.is_azure_ad_joined = isTruthy(ctx.json?.properties?.IsAzureADJoined);\n ctx.m365_defender.event.is_local_admin = isTruthy(ctx.json?.properties?.IsLocalAdmin);\n"#))?;

                if event.has("json.properties.FolderPath") {
                    event.rename("json.properties.FolderPath", "m365_defender.event.folder_path")?;
                }

                if event.has("json.properties.MD5") {
                    event.rename("json.properties.MD5", "m365_defender.event.md5")?;
                }

                if event.has("json.properties.SHA1") {
                    event.rename("json.properties.SHA1", "m365_defender.event.sha1")?;
                }

                if event.has("json.properties.SHA256") {
                    event.rename("json.properties.SHA256", "m365_defender.event.sha256")?;
                }

                if event.has("json.properties.FileName") {
                    event.rename("json.properties.FileName", "m365_defender.event.file.name")?;
                }

                if event.has("json.properties.FileSize") {
                    event.rename("json.properties.FileSize", "m365_defender.event.file.size")?;
                }

                if event.has("json.properties.DeviceName") {
                    event.rename("json.properties.DeviceName", "m365_defender.event.device.name")?;
                }

                if event.has("json.properties.DeviceId") {
                    event.rename("json.properties.DeviceId", "m365_defender.event.device.id")?;
                }

                if event.has("json.properties.InitiatingProcessCommandLine") {
                    event.rename("json.properties.InitiatingProcessCommandLine", "m365_defender.event.initiating_process.command_line")?;
                }

                if event.has("json.properties.AzureResourceId") {
                    event.rename("json.properties.AzureResourceId", "m365_defender.event.azure_resource_id")?;
                }

                if event.has("json.properties.AwsResourceName") {
                    event.rename("json.properties.AwsResourceName", "m365_defender.event.aws_resource_name")?;
                }

                if event.has("json.properties.GcpFullResourceName") {
                    event.rename("json.properties.GcpFullResourceName", "m365_defender.event.gcp_full_resource_name")?;
                }

                if event.has("json.properties.ContainerImageName") {
                    event.rename("json.properties.ContainerImageName", "m365_defender.event.container_image_name")?;
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

                if event.has("json.properties.ContainerName") {
                    event.rename("json.properties.ContainerName", "m365_defender.event.container_name")?;
                }

                if event.has("json.properties.ContainerId") {
                    event.rename("json.properties.ContainerId", "m365_defender.event.container_id")?;
                }

            if let Some(v) = event.get("m365_defender.event.container_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.container_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.id", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.container_image_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.image.name", v)?;
            }

                if event.has("json.properties.ProcessName") {
                    event.rename("json.properties.ProcessName", "m365_defender.event.process_name")?;
                }

                if event.has("json.properties.ParentProcessName") {
                    event.rename("json.properties.ParentProcessName", "m365_defender.event.parent_process_name")?;
                }

                if event.has("json.properties.ParentProcessId") {
                    event.rename("json.properties.ParentProcessId", "m365_defender.event.parent_process_id")?;
                }

                if event.has("json.properties.ProcessCurrentWorkingDirectory") {
                    event.rename("json.properties.ProcessCurrentWorkingDirectory", "m365_defender.event.process_current_working_directory")?;
                }

                if event.has("json.properties.InitiatingProcessMD5") {
                    event.rename("json.properties.InitiatingProcessMD5", "m365_defender.event.initiating_process.md5")?;
                }

                if event.has("json.properties.InitiatingProcessSHA1") {
                    event.rename("json.properties.InitiatingProcessSHA1", "m365_defender.event.initiating_process.sha1")?;
                }

                if event.has("json.properties.InitiatingProcessSHA256") {
                    event.rename("json.properties.InitiatingProcessSHA256", "m365_defender.event.initiating_process.sha256")?;
                }

                if event.has("json.properties.InitiatingProcessParentId") {
                    event.rename("json.properties.InitiatingProcessParentId", "m365_defender.event.initiating_process.parent_id")?;
                }

                if event.has("json.properties.InitiatingProcessId") {
                    event.rename("json.properties.InitiatingProcessId", "m365_defender.event.initiating_process.id")?;
                }

                if event.has("json.properties.RegistryKey") {
                    event.rename("json.properties.RegistryKey", "m365_defender.event.registry.key")?;
                }

                if event.has("json.properties.RegistryValueName") {
                    event.rename("json.properties.RegistryValueName", "m365_defender.event.registry.value_name")?;
                }

                if event.has("json.properties.CertificateSerialNumber") {
                    event.rename("json.properties.CertificateSerialNumber", "m365_defender.event.certificate.serial_number")?;
                }

                if event.has("json.properties.AccountName") {
                    event.rename("json.properties.AccountName", "m365_defender.event.account.name")?;
                }

                if event.has("json.properties.RequestProtocol") {
                    event.rename("json.properties.RequestProtocol", "m365_defender.event.request.protocol")?;
                }

                if event.has("json.properties.ActionType") {
                    event.rename("json.properties.ActionType", "m365_defender.event.action.type")?;
                }

                if event.has("json.properties.RequestAccountDomain") {
                    event.rename("json.properties.RequestAccountDomain", "m365_defender.event.request.account_domain")?;
                }

                if event.has("json.properties.RequestAccountName") {
                    event.rename("json.properties.RequestAccountName", "m365_defender.event.request.account_name")?;
                }

                if event.has("json.properties.OSArchitecture") {
                    event.rename("json.properties.OSArchitecture", "m365_defender.event.os.architecture")?;
                }

                if event.has("json.properties.OSPlatform") {
                    event.rename("json.properties.OSPlatform", "m365_defender.event.os.platform")?;
                }

                if event.has("json.properties.OSDistribution") {
                    event.rename("json.properties.OSDistribution", "m365_defender.event.os.distribution")?;
                }

                if event.has("json.properties.OSVersion") {
                    event.rename("json.properties.OSVersion", "m365_defender.event.os.version")?;
                }

                if event.has("json.properties.DeviceType") {
                    event.rename("json.properties.DeviceType", "m365_defender.event.device.type")?;
                }

                if event.has("json.properties.AccountDomain") {
                    event.rename("json.properties.AccountDomain", "m365_defender.event.account.domain")?;
                }

                if event.has("json.properties.ClientVersion") {
                    event.rename("json.properties.ClientVersion", "m365_defender.event.client_version")?;
                }

                if event.has("json.properties.DeviceCategory") {
                    event.rename("json.properties.DeviceCategory", "m365_defender.event.device.category")?;
                }

                if event.has("json.properties.MacAddress") {
                    event.rename("json.properties.MacAddress", "m365_defender.event.mac_address")?;
                }

                if event.has("json.properties.AccountSid") {
                    event.rename("json.properties.AccountSid", "m365_defender.event.account.sid")?;
                }

                if event.has("json.properties.RequestAccountSid") {
                    event.rename("json.properties.RequestAccountSid", "m365_defender.event.request.account_sid")?;
                }

                if event.has("json.properties.AppGuardContainerId") {
                    event.rename("json.properties.AppGuardContainerId", "m365_defender.event.app_guard_container_id")?;
                }

                if event.has("json.properties.FileOriginUrl") {
                    event.rename("json.properties.FileOriginUrl", "m365_defender.event.file.origin_url")?;
                }

                if event.has("json.properties.InitiatingProcessAccountDomain") {
                    event.rename("json.properties.InitiatingProcessAccountDomain", "m365_defender.event.initiating_process.account_domain")?;
                }

                if event.has("json.properties.InitiatingProcessAccountName") {
                    event.rename("json.properties.InitiatingProcessAccountName", "m365_defender.event.initiating_process.account_name")?;
                }

                if event.has("json.properties.AccountObjectId") {
                    event.rename("json.properties.AccountObjectId", "m365_defender.event.account.object_id")?;
                }

                if event.has("json.properties.InitiatingProcessAccountObjectId") {
                    event.rename("json.properties.InitiatingProcessAccountObjectId", "m365_defender.event.initiating_process.account_object_id")?;
                }

                if event.has("json.properties.InitiatingProcessAccountSid") {
                    event.rename("json.properties.InitiatingProcessAccountSid", "m365_defender.event.initiating_process.account_sid")?;
                }

                if event.has("json.properties.AccountUpn") {
                    event.rename("json.properties.AccountUpn", "m365_defender.event.account.upn")?;
                }

                if event.has("json.properties.InitiatingProcessAccountUpn") {
                    event.rename("json.properties.InitiatingProcessAccountUpn", "m365_defender.event.initiating_process.account_upn")?;
                }

                if event.has("json.properties.InitiatingProcessFileName") {
                    event.rename("json.properties.InitiatingProcessFileName", "m365_defender.event.initiating_process.file_name")?;
                }

                if event.has("json.properties.InitiatingProcessFolderPath") {
                    event.rename("json.properties.InitiatingProcessFolderPath", "m365_defender.event.initiating_process.folder_path")?;
                }

                if event.has("json.properties.InitiatingProcessParentFileName") {
                    event.rename("json.properties.InitiatingProcessParentFileName", "m365_defender.event.initiating_process.parent_file_name")?;
                }

                if event.has("json.properties.InitiatingProcessVersionInfoCompanyName") {
                    event.rename("json.properties.InitiatingProcessVersionInfoCompanyName", "m365_defender.event.initiating_process.version_info_company_name")?;
                }

                if event.has("json.properties.InitiatingProcessVersionInfoFileDescription") {
                    event.rename("json.properties.InitiatingProcessVersionInfoFileDescription", "m365_defender.event.initiating_process.version_info_file_description")?;
                }

                if event.has("json.properties.InitiatingProcessVersionInfoInternalFileName") {
                    event.rename("json.properties.InitiatingProcessVersionInfoInternalFileName", "m365_defender.event.initiating_process.version_info_internal_file_name")?;
                }

                if event.has("json.properties.InitiatingProcessVersionInfoOriginalFileName") {
                    event.rename("json.properties.InitiatingProcessVersionInfoOriginalFileName", "m365_defender.event.initiating_process.version_info_original_file_name")?;
                }

                if event.has("json.properties.InitiatingProcessVersionInfoProductName") {
                    event.rename("json.properties.InitiatingProcessVersionInfoProductName", "m365_defender.event.initiating_process.version_info_product_name")?;
                }

                if event.has("json.properties.InitiatingProcessVersionInfoProductVersion") {
                    event.rename("json.properties.InitiatingProcessVersionInfoProductVersion", "m365_defender.event.initiating_process.version_info_product_version")?;
                }

                if event.has("json.properties.ProcessCommandLine") {
                    event.rename("json.properties.ProcessCommandLine", "m365_defender.event.process.command_line")?;
                }

                if event.has("json.properties.ProcessTokenElevation") {
                    event.rename("json.properties.ProcessTokenElevation", "m365_defender.event.process.token_elevation")?;
                }

                if event.has("json.properties.ProcessVersionInfoCompanyName") {
                    event.rename("json.properties.ProcessVersionInfoCompanyName", "m365_defender.event.process.version_info_company_name")?;
                }

                if event.has("json.properties.ProcessVersionInfoFileDescription") {
                    event.rename("json.properties.ProcessVersionInfoFileDescription", "m365_defender.event.process.version_info_file_description")?;
                }

                if event.has("json.properties.ProcessVersionInfoInternalFileName") {
                    event.rename("json.properties.ProcessVersionInfoInternalFileName", "m365_defender.event.process.version_info_internal_file_name")?;
                }

                if event.has("json.properties.ProcessVersionInfoOriginalFileName") {
                    event.rename("json.properties.ProcessVersionInfoOriginalFileName", "m365_defender.event.process.version_info_original_file_name")?;
                }

                if event.has("json.properties.ProcessVersionInfoProductName") {
                    event.rename("json.properties.ProcessVersionInfoProductName", "m365_defender.event.process.version_info_product_name")?;
                }

                if event.has("json.properties.ProcessVersionInfoProductVersion") {
                    event.rename("json.properties.ProcessVersionInfoProductVersion", "m365_defender.event.process.version_info_product_version")?;
                }

                if event.has("json.properties.RegistryValueData") {
                    event.rename("json.properties.RegistryValueData", "m365_defender.event.registry.value_data")?;
                }

                if event.has("json.properties.RemoteDeviceName") {
                    event.rename("json.properties.RemoteDeviceName", "m365_defender.event.remote.device_name")?;
                }

                if event.has("json.properties.RemoteUrl") {
                    event.rename("json.properties.RemoteUrl", "m365_defender.event.remote.url")?;
                }

                if event.has("json.properties.CrlDistributionPointUrls") {
                    event.rename("json.properties.CrlDistributionPointUrls", "m365_defender.event.crl_distribution_point_urls")?;
                }

                if event.has("json.properties.Issuer") {
                    event.rename("json.properties.Issuer", "m365_defender.event.issuer")?;
                }

                if event.has("json.properties.IssuerHash") {
                    event.rename("json.properties.IssuerHash", "m365_defender.event.issuer_hash")?;
                }

                if event.has("json.properties.SignatureType") {
                    event.rename("json.properties.SignatureType", "m365_defender.event.signature_type")?;
                }

                if event.has("json.properties.Signer") {
                    event.rename("json.properties.Signer", "m365_defender.event.signer")?;
                }

                if event.has("json.properties.SignerHash") {
                    event.rename("json.properties.SignerHash", "m365_defender.event.signer_hash")?;
                }

                if event.has("json.properties.FileOriginReferrerUrl") {
                    event.rename("json.properties.FileOriginReferrerUrl", "m365_defender.event.file.origin_referrer_url")?;
                }

                if event.has("json.properties.InitiatingProcessIntegrityLevel") {
                    event.rename("json.properties.InitiatingProcessIntegrityLevel", "m365_defender.event.initiating_process.integrity_level")?;
                }

                if event.has("json.properties.InitiatingProcessTokenElevation") {
                    event.rename("json.properties.InitiatingProcessTokenElevation", "m365_defender.event.initiating_process.token_elevation")?;
                }

                if event.has("json.properties.PreviousFileName") {
                    event.rename("json.properties.PreviousFileName", "m365_defender.event.previous.file_name")?;
                }

                if event.has("json.properties.PreviousFolderPath") {
                    event.rename("json.properties.PreviousFolderPath", "m365_defender.event.previous.folder_path")?;
                }

                if event.has("json.properties.SensitivityLabel") {
                    event.rename("json.properties.SensitivityLabel", "m365_defender.event.sensitivity.label")?;
                }

                if event.has("json.properties.SensitivitySubLabel") {
                    event.rename("json.properties.SensitivitySubLabel", "m365_defender.event.sensitivity.sub_label")?;
                }

                if event.has("json.properties.ShareName") {
                    event.rename("json.properties.ShareName", "m365_defender.event.share_name")?;
                }

                if event.has("json.properties.FailureReason") {
                    event.rename("json.properties.FailureReason", "m365_defender.event.failure_reason")?;
                }

                if event.has("json.properties.AadDeviceId") {
                    event.rename("json.properties.AadDeviceId", "m365_defender.event.aad_device_id")?;
                }

                if event.has("json.properties.DeviceSubType") {
                    event.rename("json.properties.DeviceSubType", "m365_defender.event.device.sub_type")?;
                }

                if event.has("json.properties.JoinType") {
                    event.rename("json.properties.JoinType", "m365_defender.event.join_type")?;
                }

                if event.has("json.properties.MachineGroup") {
                    event.rename("json.properties.MachineGroup", "m365_defender.event.machine_group")?;
                }

                if event.has("json.properties.MergedDeviceIds") {
                    event.rename("json.properties.MergedDeviceIds", "m365_defender.event.merged_device_ids")?;
                }

                if event.has("json.properties.MergedToDeviceId") {
                    event.rename("json.properties.MergedToDeviceId", "m365_defender.event.merged_to_device_id")?;
                }

                if event.has("json.properties.SensorHealthState") {
                    event.rename("json.properties.SensorHealthState", "m365_defender.event.sensor_health_state")?;
                }

                if event.has("json.properties.IsExcluded") {
                    event.rename("json.properties.IsExcluded", "m365_defender.event.is_excluded")?;
                }

                if event.has("json.properties.ExclusionReason") {
                    event.rename("json.properties.ExclusionReason", "m365_defender.event.exclusion_reason")?;
                }

                if event.has("json.properties.AssetValue") {
                    event.rename("json.properties.AssetValue", "m365_defender.event.asset_value")?;
                }

                if event.has("json.properties.ExposureLevel") {
                    event.rename("json.properties.ExposureLevel", "m365_defender.event.exposure_level")?;
                }

                if event.has("json.properties.IsInternetFacing") {
                    event.rename("json.properties.IsInternetFacing", "m365_defender.event.is_internet_facing")?;
                }

                if event.has("json.properties.DeviceManualTags") {
                    event.rename("json.properties.DeviceManualTags", "m365_defender.event.device_manual_tags")?;
                }

                if event.has("json.properties.DeviceDynamicTags") {
                    event.rename("json.properties.DeviceDynamicTags", "m365_defender.event.device_dynamic_tags")?;
                }

                if event.has("json.properties.Model") {
                    event.rename("json.properties.Model", "m365_defender.event.model")?;
                }

                if event.has("json.properties.OnboardingStatus") {
                    event.rename("json.properties.OnboardingStatus", "m365_defender.event.onboarding_status")?;
                }

                if event.has("json.properties.OSBuild") {
                    event.rename("json.properties.OSBuild", "m365_defender.event.os.build")?;
                }

                if event.has("json.properties.OSVersionInfo") {
                    event.rename("json.properties.OSVersionInfo", "m365_defender.event.os.version_info")?;
                }

                if event.has("json.properties.RegistryDeviceTag") {
                    event.rename("json.properties.RegistryDeviceTag", "m365_defender.event.registry.device_tag")?;
                }

                if event.has("json.properties.Vendor") {
                    event.rename("json.properties.Vendor", "m365_defender.event.vendor")?;
                }

                if event.has("json.properties.LogonType") {
                    event.rename("json.properties.LogonType", "m365_defender.event.logon.type")?;
                }

                if event.has("json.properties.Protocol") {
                    event.rename("json.properties.Protocol", "m365_defender.event.protocol")?;
                }

                if event.has("json.properties.RemoteIPType") {
                    event.rename("json.properties.RemoteIPType", "m365_defender.event.remote.ip_type")?;
                }

                if event.has("json.properties.RegistryValueType") {
                    event.rename("json.properties.RegistryValueType", "m365_defender.event.registry.value_type")?;
                }

                if event.has("json.properties.LocalIPType") {
                    event.rename("json.properties.LocalIPType", "m365_defender.event.local.ip_type")?;
                }

                if event.has("json.properties.ConnectedNetworks") {
                    event.rename("json.properties.ConnectedNetworks", "m365_defender.event.connected_networks")?;
                }

                if event.has("json.properties.DefaultGateways") {
                    event.rename("json.properties.DefaultGateways", "m365_defender.event.default_gateways")?;
                }

                if event.has("json.properties.DnsAddresses") {
                    event.rename("json.properties.DnsAddresses", "m365_defender.event.dns_addresses")?;
                }

                if event.has("json.properties.IPAddresses") {
                    event.rename("json.properties.IPAddresses", "m365_defender.event.ip_addresses")?;
                }

                if event.has("json.properties.NetworkAdapterStatus") {
                    event.rename("json.properties.NetworkAdapterStatus", "m365_defender.event.network.adapter_status")?;
                }

                if event.has("json.properties.NetworkAdapterType") {
                    event.rename("json.properties.NetworkAdapterType", "m365_defender.event.network.adapter_type")?;
                }

                if event.has("json.properties.NetworkAdapterVendor") {
                    event.rename("json.properties.NetworkAdapterVendor", "m365_defender.event.network.adapter_vendor")?;
                }

                if event.has("json.properties.TunnelType") {
                    event.rename("json.properties.TunnelType", "m365_defender.event.tunnel_type")?;
                }

                if event.has("json.properties.InitiatingProcessSignatureStatus") {
                    event.rename("json.properties.InitiatingProcessSignatureStatus", "m365_defender.event.initiating_process.signature_status")?;
                }

                if event.has("json.properties.InitiatingProcessSignerType") {
                    event.rename("json.properties.InitiatingProcessSignerType", "m365_defender.event.initiating_process.signer_type")?;
                }

                if event.has("json.properties.ProcessIntegrityLevel") {
                    event.rename("json.properties.ProcessIntegrityLevel", "m365_defender.event.process.integrity_level")?;
                }

                if event.has("json.properties.PreviousRegistryKey") {
                    event.rename("json.properties.PreviousRegistryKey", "m365_defender.event.previous.registry_key")?;
                }

                if event.has("json.properties.PreviousRegistryValueData") {
                    event.rename("json.properties.PreviousRegistryValueData", "m365_defender.event.previous.registry_value_data")?;
                }

                if event.has("json.properties.PreviousRegistryValueName") {
                    event.rename("json.properties.PreviousRegistryValueName", "m365_defender.event.previous.registry_value_name")?;
                }

            let _cond = { event.has_value("json.properties.FileOriginIP") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("json.properties.FileOriginIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.FileOriginIP".into(),
                            message,
                        })?;
                    event.set("json.properties.FileOriginIP", converted)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_origin_ip")?;
                        if event.remove("json.properties.FileOriginIP").is_none() {
                            return Err(TransformError::FieldNotFound { path: "json.properties.FileOriginIP".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has("json.properties.FileOriginIP") {
                    event.rename("json.properties.FileOriginIP", "m365_defender.event.file.origin_ip")?;
                }

            let _cond = { event.has_value("json.properties.RemoteIP") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("json.properties.RemoteIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.RemoteIP".into(),
                            message,
                        })?;
                    event.set("json.properties.RemoteIP", converted)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_remote_ip")?;
                        if event.remove("json.properties.RemoteIP").is_none() {
                            return Err(TransformError::FieldNotFound { path: "json.properties.RemoteIP".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has("json.properties.RemoteIP") {
                    event.rename("json.properties.RemoteIP", "m365_defender.event.remote.ip")?;
                }

            let _cond = { event.has_value("json.properties.LocalIP") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("json.properties.LocalIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.LocalIP".into(),
                            message,
                        })?;
                    event.set("json.properties.LocalIP", converted)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_local_ip")?;
                        if event.remove("json.properties.LocalIP").is_none() {
                            return Err(TransformError::FieldNotFound { path: "json.properties.LocalIP".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has("json.properties.LocalIP") {
                    event.rename("json.properties.LocalIP", "m365_defender.event.local.ip")?;
                }

            let _cond = { event.has_value("json.properties.RequestSourceIP") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("json.properties.RequestSourceIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.RequestSourceIP".into(),
                            message,
                        })?;
                    event.set("json.properties.RequestSourceIP", converted)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_request_source_ip")?;
                        if event.remove("json.properties.RequestSourceIP").is_none() {
                            return Err(TransformError::FieldNotFound { path: "json.properties.RequestSourceIP".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has("json.properties.RequestSourceIP") {
                    event.rename("json.properties.RequestSourceIP", "m365_defender.event.request.source_ip")?;
                }

                if event.has("json.properties.RequestSourcePort") {
                    event.rename("json.properties.RequestSourcePort", "m365_defender.event.request.source_port")?;
                }

                if event.has("json.properties.RemotePort") {
                    event.rename("json.properties.RemotePort", "m365_defender.event.remote.port")?;
                }

                if event.has("json.properties.LocalPort") {
                    event.rename("json.properties.LocalPort", "m365_defender.event.local.port")?;
                }

            let _cond = { event.has_value("json.properties.PublicIP") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("json.properties.PublicIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.PublicIP".into(),
                            message,
                        })?;
                    event.set("json.properties.PublicIP", converted)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_public_ip")?;
                        if event.remove("json.properties.PublicIP").is_none() {
                            return Err(TransformError::FieldNotFound { path: "json.properties.PublicIP".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has("json.properties.PublicIP") {
                    event.rename("json.properties.PublicIP", "m365_defender.event.public_ip.value")?;
                }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents"))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }
            }

            let _cond = { event.has_value("file.path") && event.get_as_string("file.path").is_some_and(|s| s.len() > 1) };
            if _cond {
                // Painless script
                // Source: String path = ctx.file.path;\nString sep = \"/\";\nString windows_sep = \"\\\\\";\ndef idx = -1;\nif (path.contains(windows_sep)) {\n    idx = path.lastIndexOf(windows_sep);\n}\nelse {\n    idx = path.lastIndexOf(sep);\n} \nif (idx > -1) {\n    if (ctx.file.name == null) {\n        ctx.file.name = path.substring(idx+1);\n    }\n    ctx.file.directory = path.substring(0, idx);\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.extension == null) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(event, cached_script!(r#"String path = ctx.file.path;\nString sep = \"/\";\nString windows_sep = \"\\\\\";\ndef idx = -1;\nif (path.contains(windows_sep)) {\n    idx = path.lastIndexOf(windows_sep);\n}\nelse {\n    idx = path.lastIndexOf(sep);\n} \nif (idx > -1) {\n    if (ctx.file.name == null) {\n        ctx.file.name = path.substring(idx+1);\n    }\n    ctx.file.directory = path.substring(0, idx);\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.extension == null) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}"#))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents"))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.md5", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents"))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents"))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents"))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents"))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.certificate.expiration_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.certificate.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.serial_number", v)?;
            }

            let _cond = { event.has_value("m365_defender.event.issuer") };
            if _cond {
                event.append("file.x509.issuer.common_name", json!(event.get("m365_defender.event.issuer").map_or_else(String::new, painless_to_string)))?;
            }

            if let Some(v) = event.get("m365_defender.event.signer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.code_signature.subject_name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.is_signed").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.code_signature.exists", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.is_trusted").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.code_signature.trusted", v)?;
            }

            let _cond = { event.has_value("event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.path", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) };
            if _cond {
            let v = json!(format!("{}\\{}", event.get("m365_defender.event.folder_path").map_or_else(String::new, painless_to_string), event.get("m365_defender.event.file.name").map_or_else(String::new, painless_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("dll.path", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.hash.md5", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.hash.md5", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dll.Ext.size", v)?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| ["namedpipeevent", "dpapiaccessed", "ntallocatevirtualmemoryapicall", "getclipboarddata", "ntprotectvirtualmemoryapicall", "browserlaunchedtoopenurl", "processprimarytokenmodified", "powershellcommand", "clrunbackedmoduleloaded", "ldapsearch", "dnsqueryresponse", "ntallocatevirtualmemoryremoteapicall", "memoryremoteprotect", "screenshottaken", "antivirusscancompleted", "exploitguardwin32systemcallblocked", "getasynckeystateapicall", "appguardcreatecontainer", "exploitguardacgenforced", "writetolsassprocessmemory", "antivirusscancancelled", "controlflowguardviolation", "appcontrolpolicyapplied", "createremotethreadapicall", "auditpolicymodification", "ntmapviewofsectionremoteapicall", "appguardlaunchedwithurl", "appguardresumecontainer", "smartscreenurlwarning", "appguardbrowsetourl", "otheralertrelatedactivity", "antivirusscanfailed"].contains(&s.to_lowercase().as_str())) };
            if _cond {
            event.set("_temp_deviceevents_that_map_process", json!(true))?;
            }

            let _cond = { !event.has_value("_temp_deviceevents_that_map_process") && event.has_value("m365_defender.event.category") && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")) };
            if _cond {
            event.set("_temp_deviceevents_that_map_process", json!(false))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.executable", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.md5", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.command_line").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.command_line", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.hash.md5", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.group_leader.pid", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pid", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.creation_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.start", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.executable", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_creation_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.group_leader.start", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.group_leader.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_company_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pe.company", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_file_description").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pe.description", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_original_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pe.original_file_name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_product_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pe.product", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_product_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pe.file_version", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.signature_status").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.code_signature.status", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) && event.get_str("m365_defender.event.initiating_process.signature_status") == Some("Valid") };
            if _cond {
            event.set("process.parent.code_signature.exists", json!(true))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) && event.get_str("m365_defender.event.initiating_process.signature_status") == Some("Unsigned") };
            if _cond {
            event.set("process.parent.code_signature.exists", json!(false))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) && event.get_str("m365_defender.event.initiating_process.signature_status") == Some("Valid") };
            if _cond {
            event.set("process.parent.code_signature.status", json!("trusted"))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) && event.get_str("m365_defender.event.initiating_process.signature_status") == Some("Valid") };
            if _cond {
            event.set("process.parent.code_signature.trusted", json!(true))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(false))) && event.get_str("m365_defender.event.initiating_process.signature_status") == Some("Unsigned") };
            if _cond {
            event.set("process.parent.code_signature.trusted", json!(false))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) || event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.integrity_level").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.Ext.token.integrity_level_name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) };
            if _cond {
                // Painless script
                // Source: String actiontype = ctx.m365_defender.event.action.type;\ndef idx = actiontype.toLowerCase().lastIndexOf('apicall');\nctx._temp_process_Ext_api_name = actiontype.substring(0, idx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(event, cached_script!(r#"String actiontype = ctx.m365_defender.event.action.type;\ndef idx = actiontype.toLowerCase().lastIndexOf('apicall');\nctx._temp_process_Ext_api_name = actiontype.substring(0, idx);\n"#))?;
            }

                if event.has("_temp_process_Ext_api_name") {
                    event.rename("_temp_process_Ext_api_name", "process.Ext.api.name")?;
                }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.RegionSize") {
                    event.rename("m365_defender.event.additional_fields.RegionSize", "process.Ext.api.parameters.size")?;
                }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.ProtectionMask") {
                    event.rename("m365_defender.event.additional_fields.ProtectionMask", "process.Ext.api.parameters.protection")?;
                }
            }

            let _cond = { event.get_str("process.Ext.api.parameters.protection") != Some("") };
            if _cond {
            if event.has_value("process.Ext.api.parameters.protection") {
                if let Some(val) = event.get("process.Ext.api.parameters.protection") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "process.Ext.api.parameters.protection".into(),
                            message,
                        })?;
                    event.set("process.Ext.api.parameters.protection", converted)?;
                }
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.BaseAddress") {
                    event.rename("m365_defender.event.additional_fields.BaseAddress", "process.Ext.api.parameters.address")?;
                }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) };
            if _cond {
                if event.has("m365_defender.event.additional_fields.DesiredAccess") {
                    event.rename("m365_defender.event.additional_fields.DesiredAccess", "process.Ext.api.parameters.desired_access_numeric")?;
                }
            }

            if let Some(v) = event.get("m365_defender.event.process_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.parent_process_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.parent_process_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pid", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process_current_working_directory").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.working_directory", v)?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) && (event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| ["createremotethreadapicall", "readprocessmemoryapicall", "ntallocatevirtualmemoryremoteapicall", "openprocessapicall"].contains(&s.to_lowercase().as_str()))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("Target.process.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) && (event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| ["createremotethreadapicall", "readprocessmemoryapicall", "ntallocatevirtualmemoryremoteapicall", "openprocessapicall"].contains(&s.to_lowercase().as_str()))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.process.command_line").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("Target.process.command_line", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) && (event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| ["createremotethreadapicall", "readprocessmemoryapicall", "ntallocatevirtualmemoryremoteapicall", "openprocessapicall"].contains(&s.to_lowercase().as_str()))) && event.has_value("m365_defender.event.folder_path") && event.has_value("m365_defender.event.file.name") && event.get_str("m365_defender.event.file.name").is_some_and(|p| event.get_str("m365_defender.event.folder_path").is_some_and(|s| s.ends_with(p))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("Target.process.executable", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) && (event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| ["createremotethreadapicall", "readprocessmemoryapicall", "ntallocatevirtualmemoryremoteapicall", "openprocessapicall"].contains(&s.to_lowercase().as_str()))) && !event.has_value("Target.process.executable") };
            if _cond {
            let v = json!(format!("{}\\{}", event.get("m365_defender.event.folder_path").map_or_else(String::new, painless_to_string), event.get("m365_defender.event.file.name").map_or_else(String::new, painless_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("Target.process.executable", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.command_line").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.command_line", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.md5", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.sha1", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.sha256", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pid", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pid", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.creation_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.start", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents"))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.executable", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && event.has_value("m365_defender.event.initiating_process.folder_path") && event.has_value("m365_defender.event.initiating_process.file_name") && event.get_str("m365_defender.event.initiating_process.file_name").is_some_and(|p| event.get_str("m365_defender.event.initiating_process.folder_path").is_some_and(|s| s.ends_with(p))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.folder_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.executable", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) && !event.has_value("process.executable") };
            if _cond {
            let v = json!(format!("{}\\{}", event.get("m365_defender.event.initiating_process.folder_path").map_or_else(String::new, painless_to_string), event.get("m365_defender.event.initiating_process.file_name").map_or_else(String::new, painless_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("process.executable", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pid", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_creation_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.start", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.group_leader.pid", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.group_leader.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.parent_creation_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.group_leader.start", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_company_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.company", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_file_description").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.description", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_original_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.original_file_name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_product_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.product", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false })) && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_product_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.file_version", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || (!(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false })) && !(event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("deviceevents")))) || (event.has_value("_temp_deviceevents_that_map_process") && event.get_bool("_temp_deviceevents_that_map_process") == Some(true))) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.signature_status").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.code_signature.status", v)?;
            }
            }

                event.remove("_temp_deviceevents_that_map_process");

            if let Some(v) = event.get("m365_defender.event.process.command_line").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.creation_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.start", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pid", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.version_info_company_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.company", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.version_info_file_description").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.description", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.version_info_original_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.original_file_name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.version_info_product_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.product", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.process.version_info_product_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.file_version", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.device.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(s) = event.get_string("host.name") {
                    let lowered = s.to_lowercase();
                    event.set("host.name", lowered)?;
                }
            }

            let _cond = { event.has_value("m365_defender.event.device.name") };
            if _cond {
                if let Some(s) = event.get_string("m365_defender.event.device.name") {
                    let lowered = s.to_lowercase();
                    event.set("host.hostname", lowered)?;
                }
            }

            if let Some(v) = event.get("m365_defender.event.device.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("m365_defender.event.public_ip.value") && event.get_str("m365_defender.event.public_ip.value") != Some("") };
            if _cond {
                event.append("host.ip", json!(event.get("m365_defender.event.public_ip.value").map_or_else(String::new, painless_to_string)))?;
            }

            if let Some(v) = event.get("m365_defender.event.os.architecture").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.architecture", v)?;
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

            let _cond = { event.has_value("m365_defender.event.additional_fields") && !(event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_array())) && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixEffectiveGroup") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixEffectiveUser") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixFilePermissions") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixProcessGroupId") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixRealUser") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixSessionId") };
            if _cond {
            event.set("_tmp.posix", json!(true))?;
            }

            let _cond = { event.has_value("event.category") && event.get_bool("_tmp.posix") != Some(true) && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("registry")), serde_json::Value::String(s) => s.contains("registry"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false })) };
            if _cond {
            event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) && (event.has_value("m365_defender.event.initiating_process.account_sid") || event.has_value("m365_defender.event.request.account_sid")) };
            if _cond {
            event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { event.has_value("event.category") && event.get_bool("_tmp.posix") != Some(true) && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) && (event.has_value("m365_defender.event.initiating_process.account_sid")) };
            if _cond {
            event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { event.has_value("event.category") && event.get_bool("_tmp.posix") != Some(true) && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false }) && (event.has_value("m365_defender.event.initiating_process.account_sid") || event.has_value("m365_defender.event.account.sid")) };
            if _cond {
            event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { event.has_value("event.category") && event.get_bool("_tmp.posix") != Some(true) && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && (event.has_value("m365_defender.event.initiating_process.account_sid")) };
            if _cond {
            event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { !event.has_value("host.os.type") && event.get_bool("_tmp.posix") == Some(true) };
            if _cond {
            event.set("host.os.type", json!("unix"))?;
            }

            if let Some(v) = event.get("m365_defender.event.os.platform").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.os.distribution").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.platform", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.os.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.version", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.device.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.type", v)?;
            }

            if event.has_value("m365_defender.event.mac_address") {
                if let Some(s) = event.get_string("m365_defender.event.mac_address") {
                    let re = cached_regex!("[:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("m365_defender.event.mac_address", replaced)?;
                }
            }

            if event.has_value("m365_defender.event.mac_address") {
                if let Some(s) = event.get_string("m365_defender.event.mac_address") {
                    let uppered = s.to_uppercase();
                    event.set("m365_defender.event.mac_address", uppered)?;
                }
            }

            if let Some(v) = event.get("m365_defender.event.mac_address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("_tmp.mac", v)?;
            }

            let _cond = { !(event.get("_tmp.mac").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("-")), serde_json::Value::String(s) => s.contains("-"), _ => false })) };
            if _cond {
            if event.has_value("_tmp.mac") {
                if let Some(s) = event.get_string("_tmp.mac") {
                    let re = cached_regex!("(..)(?!$)");
                    let replaced = re.replace_all(&s, "$1-").into_owned();
                    event.set("_tmp.mac", replaced)?;
                }
            }
            }

            let _cond = { event.has_value("_tmp.mac") };
            if _cond {
                event.append_unique("host.mac", json!(event.get("_tmp.mac").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.registry.key") && event.get_str("m365_defender.event.registry.key") != Some("") };
            if _cond {
                if let Some(input) = event.get_string("m365_defender.event.registry.key") {
                    // Grok pattern: ^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$
                    if !cached_grok_mapped!("^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$", [("_tmp_registry_hive", "_tmp.registry.hive")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { !event.has_value("registry.key") && event.has_value("m365_defender.event.previous.registry_key") && event.get_str("m365_defender.event.previous.registry_key") != Some("") };
            if _cond {
                if let Some(input) = event.get_string("m365_defender.event.previous.registry_key") {
                    // Grok pattern: ^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$
                    if !cached_grok_mapped!("^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$", [("_tmp_registry_hive", "_tmp.registry.hive")]).extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.has_value("_tmp.registry.hive") };
            if _cond {
                // Painless script
                // Source: def name = ctx._tmp.registry.hive.toUpperCase();\nif (ctx.registry == null) {\n  ctx.registry = new HashMap();\n}\nctx.registry.hive = params.getOrDefault(name, name);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(event, cached_script!(r#"def name = ctx._tmp.registry.hive.toUpperCase();\nif (ctx.registry == null) {\n  ctx.registry = new HashMap();\n}\nctx.registry.hive = params.getOrDefault(name, name);\n"#), cached_params!("{\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_CONFIG\":\"HKCC\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_USERS\":\"HKU\"}"))?;
            }

            if let Some(v) = event.get("m365_defender.event.registry.value_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("registry.value", v)?;
            }

            let _cond = { !event.has_value("registry.value") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.previous.registry_value_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("registry.value", v)?;
            }
            }

            let _cond = { event.has_value("registry.key") && event.get_str("registry.key") != Some("") && event.has_value("registry.value") };
            if _cond {
            let v = json!(format!("{}{}{}\\{}\\{}", event.get("#registry.hive").map_or_else(String::new, painless_to_string), event.get("registry.hive").map_or_else(String::new, painless_to_string), event.get("/registry.hive").map_or_else(String::new, painless_to_string), event.get("registry.key").map_or_else(String::new, painless_to_string), event.get("registry.value").map_or_else(String::new, painless_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("registry.path", v)?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.registry.value_data") };
            if _cond {
                event.append_unique("registry.data.strings", json!(event.get("m365_defender.event.registry.value_data").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.previous.registry_value_data") };
            if _cond {
                event.append_unique("registry.data.strings", json!(event.get("m365_defender.event.previous.registry_value_data").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.registry.value_type") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.registry.value_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("registry.data.type", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("In") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false })) && !event.has_value("m365_defender.event.network_direction") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.local.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("Out") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.local.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.request.source_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("In") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.local.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("Out") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.local.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.request.source_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.remote.device_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.domain", v)?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("Out") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false })) && !event.has_value("m365_defender.event.network_direction") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("In") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.local.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.has_value("destination.ip") };
            if _cond {
            if let Some(v) = event.get("destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.address", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false })) && !event.has_value("m365_defender.event.network_direction") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("Out") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.remote.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("In") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.local.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.account.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.request.account_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.request.account_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("registry")), serde_json::Value::String(s) => s.contains("registry"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !event.has_value("user.name") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.account_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.account.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("registry")), serde_json::Value::String(s) => s.contains("registry"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !event.has_value("user.domain") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.account_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.account.sid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("event.category") && (event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("library")), serde_json::Value::String(s) => s.contains("library"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("registry")), serde_json::Value::String(s) => s.contains("registry"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) || event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("api")), serde_json::Value::String(s) => s.contains("api"), _ => false })) && !event.has_value("user.id") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.initiating_process.account_sid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false }) && !event.has_value("user.id") };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.request.account_sid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.dns.query").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.name", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.dns.qclass_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.class", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.dns.qtype_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.type", v)?;
            }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.dns.rcode_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.response_code", v)?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.dns.answers") && event.has_value("m365_defender.event.dns.ttls") };
            if _cond {
                // Painless script
                // Source: def answers = ctx.m365_defender.event.dns.answers; def ttls = ctx.m365_defender.event.dns.ttls; if (answers.isEmpty() || ttls.isEmpty()) {\n  return;\n} else if (answers.length != ttls.length) {\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('DNS answers and TTLs have a different length');\n} def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n  lst.add([\n    \"data\": answers[i],\n    \"ttl\": (long)ttls[i]\n  ])\n} if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(event, cached_script!(r#"def answers = ctx.m365_defender.event.dns.answers; def ttls = ctx.m365_defender.event.dns.ttls; if (answers.isEmpty() || ttls.isEmpty()) {\n  return;\n} else if (answers.length != ttls.length) {\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('DNS answers and TTLs have a different length');\n} def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n  lst.add([\n    \"data\": answers[i],\n    \"ttl\": (long)ttls[i]\n  ])\n} if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;"#))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected")) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.dns.header_flags").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.header_flags", v)?;
            }
            }

            if let Some(v) = event.get("m365_defender.event.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                if let Some(s) = event.get_string("network.transport") {
                    let lowered = s.to_lowercase();
                    event.set("network.transport", lowered)?;
                }
            }

            let _cond = { event.get_str("network.transport") == Some("ntlm") };
            if _cond {
                event.rename("network.transport", "network.protocol")?;
            }

            let _cond = { event.get_str("network.protocol") == Some("icmp") };
            if _cond {
                event.rename("network.protocol", "network.transport")?;
            }

            let _cond = { event.get_str("network.transport") == Some("tcpv4") || event.get_str("network.transport") == Some("tcpv6") };
            if _cond {
            event.set("network.transport", json!("tcp"))?;
            }

            let _cond = { event.get_str("network.transport") == Some("negotiate") || event.get_str("network.transport") == Some("microsoft_authentication_package_v1_0") };
            if _cond {
                if event.remove("network.transport").is_none() {
                    return Err(TransformError::FieldNotFound { path: "network.transport".into() });
                }
            }

            if let Some(v) = event.get("m365_defender.event.request.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }

            let _cond = { event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("dns")) };
            if _cond {
            event.set("network.protocol", json!("dns"))?;
            }

            let _cond = { event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("http")) };
            if _cond {
            event.set("network.protocol", json!("http"))?;
            }

            let _cond = { event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("ssl")) };
            if _cond {
            event.set("network.protocol", json!("ssl"))?;
            }

            let _cond = { event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase().contains("ftp")) };
            if _cond {
            event.set("network.protocol", json!("ftp"))?;
            }

            let _cond = { event.get_str("network.protocol") == Some("local") || event.get_str("network.protocol") == Some("unknown") };
            if _cond {
                if event.remove("network.protocol").is_none() {
                    return Err(TransformError::FieldNotFound { path: "network.protocol".into() });
                }
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("In") };
            if _cond {
            event.set("network.direction", json!("inbound"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && event.get_str("m365_defender.event.network_direction") == Some("Out") };
            if _cond {
            event.set("network.direction", json!("outbound"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")), serde_json::Value::String(s) => s.contains("network"), _ => false }) && !event.has_value("m365_defender.event.network_direction") };
            if _cond {
            event.set("network.direction", json!("unknown"))?;
            }

            let _cond = { (event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false })) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase() == "filedeleted") };
            if _cond {
            event.set("event.action", json!("deletion"))?;
            }

            let _cond = { (event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false })) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase() == "filemodified") };
            if _cond {
            event.set("event.action", json!("modification"))?;
            }

            let _cond = { (event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false })) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase() == "filerenamed") };
            if _cond {
            event.set("event.action", json!("rename"))?;
            }

            let _cond = { (event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false })) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase() == "filecreated") };
            if _cond {
            event.set("event.action", json!("creation"))?;
            }

            let _cond = { (event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("registry")), serde_json::Value::String(s) => s.contains("registry"), _ => false })) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase() == "registrykeycreated") };
            if _cond {
            event.set("event.action", json!("creation"))?;
            }

            let _cond = { (event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("registry")), serde_json::Value::String(s) => s.contains("registry"), _ => false })) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase() == "registryvalueset") };
            if _cond {
            event.set("event.action", json!("modification"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("driver")), serde_json::Value::String(s) => s.contains("driver"), _ => false }) };
            if _cond {
            event.set("event.action", json!("load"))?;
            }

            let _cond = { event.has_value("event.category") && event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")), serde_json::Value::String(s) => s.contains("process"), _ => false }) && event.has_value("m365_defender.event.action.type") && event.get_str("m365_defender.event.action.type").is_some_and(|s| s.to_lowercase() == "processcreated") };
            if _cond {
            event.set("event.action", json!("start"))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.category").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")), serde_json::Value::String(s) => s.contains("file"), _ => false })) };
            if _cond {
            if let Some(v) = event.get("m365_defender.event.action.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                if !event.has("event.action") {
                    event.set("event.action", v)?;
                }
            }
            }

            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let lowered = s.to_lowercase();
                    event.set("event.action", lowered)?;
                }
            }

            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let re = cached_regex!(" ");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("event.action", replaced)?;
                }
            }

            let _cond = { (!event.has_value("m365_defender.event.failure_reason") || event.get_str("m365_defender.event.failure_reason") == Some("")) && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("devicelogonevents")) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { (event.has_value("m365_defender.event.failure_reason") && event.get_str("m365_defender.event.failure_reason") != Some("")) && event.get_str("m365_defender.event.category").is_some_and(|s| s.to_lowercase().contains("devicelogonevents")) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            if let Some(v) = event.get("m365_defender.event.client_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.version", v)?;
            }

            if let Some(v) = event.get("m365_defender.event.device.category").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.type", v)?;
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

            let _cond = { event.has_value("m365_defender.event.azure_resource_id") };
            if _cond {
            if !event.has("cloud.provider") {
                event.set("cloud.provider", json!("azure"))?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.aws_resource_name") };
            if _cond {
            if !event.has("cloud.provider") {
                event.set("cloud.provider", json!("aws"))?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.gcp_full_resource_name") };
            if _cond {
            if !event.has("cloud.provider") {
                event.set("cloud.provider", json!("gcp"))?;
            }
            }

            let _cond = { event.has_value("m365_defender.event.remote.url") && event.get_str("m365_defender.event.remote.url") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "m365_defender.event.remote.url", "url", true, false)?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("m365_defender.event.file.origin_referrer_url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.referrer", v)?;
            }

            let _cond = { event.has_value("json.properties.NetworkAdapterName") && event.get_str("json.properties.NetworkAdapterName").is_some_and(|s| s.starts_with("{")) };
            if _cond {
                if let Some(input) = event.get_string("json.properties.NetworkAdapterName") {
                    // Grok pattern: ^{%{DATA:m365_defender.event.network.adapter_name}}$
                    if !cached_grok!("^{%{DATA:m365_defender.event.network.adapter_name}}$").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.has_value("json.properties.NetworkAdapterName") && !(event.get_str("json.properties.NetworkAdapterName").is_some_and(|s| s.starts_with("{"))) };
            if _cond {
            if let Some(v) = event.get("json.properties.NetworkAdapterName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("m365_defender.event.network.adapter_name", v)?;
            }
            }

            let _cond = { event.get("json.properties.LoggedOnUsers").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.properties.LoggedOnUsers").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append("m365_defender.event.active_users", json!(event.get("_ingest._value.UserName").map_or_else(String::new, painless_to_string)))?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("json.properties.LoggedOnUsers", Value::Array(out))?;
                }
            }

            let _cond = { event.get("m365_defender.event.active_users").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("m365_defender.event.active_users").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append("related.user", json!(event.get("_ingest._value").map_or_else(String::new, painless_to_string)))?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("m365_defender.event.active_users", Value::Array(out))?;
                }
            }

            let _cond = { (event.has_value("process.command_line") && event.get_str("process.command_line") != Some("")) || (event.has_value("process.parent.command_line") && event.get_str("process.parent.command_line") != Some("")) };
            if _cond {
                // Painless script
                // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nif (ctx.process?.command_line != null && ctx.process.command_line != '') {\n  ctx.process.args = commandLineToArgv(ctx.process.command_line);\n  ctx.process.args_count = ctx.process.args.length;\n}\nif (ctx.process?.parent?.command_line != null && ctx.process.parent.command_line != '') {\n  ctx.process.parent.args = commandLineToArgv(ctx.process.parent.command_line);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(event, cached_script!(r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nif (ctx.process?.command_line != null && ctx.process.command_line != '') {\n  ctx.process.args = commandLineToArgv(ctx.process.command_line);\n  ctx.process.args_count = ctx.process.args.length;\n}\nif (ctx.process?.parent?.command_line != null && ctx.process.parent.command_line != '') {\n  ctx.process.parent.args = commandLineToArgv(ctx.process.parent.command_line);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}"#))?;
            }

            let _cond = { event.has_value("host.ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

                if event.has("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }

                if event.has("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }

                if event.has("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }

                if event.has("destination.as.organization_name") {
                    event.rename("destination.as.organization_name", "destination.as.organization.name")?;
                }

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.id").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("user.domain").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.initiating_process.account_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("m365_defender.event.initiating_process.account_domain").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.initiating_process.account_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("m365_defender.event.initiating_process.account_name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.file.origin_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("m365_defender.event.file.origin_ip").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(Value::Array(items)) = event.get("host.ip").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, painless_to_string)))?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("host.ip", Value::Array(out))?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("m365_defender.event.ipv4_dhcp") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("m365_defender.event.ipv4_dhcp").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.ipv6_dhcp") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("m365_defender.event.ipv6_dhcp").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.md5").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.sha1").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.sha256").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.hash.md5").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.hash.sha1").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.hash.sha256").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.parent.hash.md5").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.parent.hash.sha1").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.parent.hash.sha256").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.issuer_hash") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("m365_defender.event.issuer_hash").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("m365_defender.event.signer_hash") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("m365_defender.event.signer_hash").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("m365_defender.event.folder_path");
                event.remove("m365_defender.event.md5");
                event.remove("m365_defender.event.sha1");
                event.remove("m365_defender.event.sha256");
                event.remove("m365_defender.event.file.name");
                event.remove("m365_defender.event.file.size");
                event.remove("m365_defender.event.file.origin_referrer_url");
                event.remove("m365_defender.event.device.name");
                event.remove("m365_defender.event.device.id");
                event.remove("m365_defender.event.process.version_info_company_name");
                event.remove("m365_defender.event.process.version_info_product_name");
                event.remove("m365_defender.event.process.version_info_product_version");
                event.remove("m365_defender.event.process.version_info_file_description");
                event.remove("m365_defender.event.initiating_process.file_name");
                event.remove("m365_defender.event.initiating_process.version_info_product_version");
                event.remove("m365_defender.event.initiating_process.version_info_file_description");
                event.remove("m365_defender.event.initiating_process.version_info_original_file_name");
                event.remove("m365_defender.event.initiating_process.file_size");
                event.remove("m365_defender.event.initiating_process.version_info_company_name");
                event.remove("m365_defender.event.initiating_process.version_info_product_name");
                event.remove("m365_defender.event.initiating_process.folder_path");
                event.remove("m365_defender.event.initiating_process.command_line");
                event.remove("m365_defender.event.initiating_process.md5");
                event.remove("m365_defender.event.initiating_process.sha1");
                event.remove("m365_defender.event.initiating_process.sha256");
                event.remove("m365_defender.event.initiating_process.parent_id");
                event.remove("m365_defender.event.initiating_process.id");
                event.remove("m365_defender.event.initiating_process.parent_file_name");
                event.remove("m365_defender.event.initiating_process.parent_creation_time");
                event.remove("m365_defender.event.initiating_process.signature_status");
                event.remove("m365_defender.event.registry.key");
                event.remove("m365_defender.event.registry.value_name");
                event.remove("m365_defender.event.registry.value_data");
                event.remove("m365_defender.event.public_ip.value");
                event.remove("m365_defender.event.local.ip");
                event.remove("m365_defender.event.remote.ip");
                event.remove("m365_defender.event.request.source_ip");
                event.remove("m365_defender.event.local.port");
                event.remove("m365_defender.event.remote.port");
                event.remove("m365_defender.event.request.source_port");
                event.remove("m365_defender.event.account.name");
                event.remove("m365_defender.event.account.sid");
                event.remove("m365_defender.event.certificate.expiration_time");
                event.remove("m365_defender.event.certificate.serial_number");
                event.remove("m365_defender.event.protocol");
                event.remove("m365_defender.event.request.protocol");
                event.remove("m365_defender.event.request.account_domain");
                event.remove("m365_defender.event.request.account_name");
                event.remove("m365_defender.event.os.architecture");
                event.remove("m365_defender.event.os.platform");
                event.remove("m365_defender.event.os.distribution");
                event.remove("m365_defender.event.os.version");
                event.remove("m365_defender.event.device.type");
                event.remove("m365_defender.event.account.domain");
                event.remove("m365_defender.event.mac_address");
                event.remove("m365_defender.event.client_version");
                event.remove("m365_defender.event.device.category");
                event.remove("m365_defender.event.action.type");
                event.remove("m365_defender.event.is_signed");
                event.remove("m365_defender.event.signer");
                event.remove("m365_defender.event.issuer");
                event.remove("m365_defender.event.is_trusted");
                event.remove("m365_defender.event.dns.qclass_name");
                event.remove("m365_defender.event.dns.query");
                event.remove("m365_defender.event.dns.qtype_name");
                event.remove("m365_defender.event.dns.rcode_name");
                event.remove("m365_defender.event.dns.answers");
                event.remove("m365_defender.event.dns.ttls");
                event.remove("m365_defender.event.dns.header_flags");
                event.remove("m365_defender.event.container_name");
                event.remove("m365_defender.event.container_id");
                event.remove("m365_defender.event.container_image_name");
                event.remove("m365_defender.event.kubernetes_resource");
                event.remove("m365_defender.event.kubernetes_namespace");
                event.remove("m365_defender.event.process_name");
                event.remove("m365_defender.event.parent_process_name");
                event.remove("m365_defender.event.parent_process_id");
                event.remove("m365_defender.event.process_current_working_directory");
            }

                event.remove("_tmp");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
