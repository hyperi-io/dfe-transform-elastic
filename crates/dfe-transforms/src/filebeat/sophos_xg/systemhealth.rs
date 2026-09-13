// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `systemhealth` pipeline.
pub struct Systemhealth;

impl Transform for Systemhealth {
    fn name(&self) -> &str {
        "systemhealth"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if event.has_value("sophos.xg.idle") {
                    event.rename("sophos.xg.idle", "sophos.xg.idle_cpu")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.idle_cpu") {
                gsub_field(event, "sophos.xg.idle_cpu", "sophos.xg.idle_cpu", cached_regex!("%$"), "")?;
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.idle_cpu") {
                if let Some(val) = event.get("sophos.xg.idle_cpu") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.idle_cpu".into(),
                            message,
                        })?;
                    event.set("sophos.xg.idle_cpu", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_idle_cpu_db95818d")?;
                        if event.remove("sophos.xg.idle_cpu").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.idle_cpu".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("sophos.xg.system") {
                    event.rename("sophos.xg.system", "sophos.xg.system_cpu")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.system_cpu") {
                gsub_field(event, "sophos.xg.system_cpu", "sophos.xg.system_cpu", cached_regex!("%$"), "")?;
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.system_cpu") {
                if let Some(val) = event.get("sophos.xg.system_cpu") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.system_cpu".into(),
                            message,
                        })?;
                    event.set("sophos.xg.system_cpu", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_system_cpu_ec01adad")?;
                        if event.remove("sophos.xg.system_cpu").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.system_cpu".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("sophos.xg.user") {
                    event.rename("sophos.xg.user", "sophos.xg.user_cpu")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.user_cpu") {
                gsub_field(event, "sophos.xg.user_cpu", "sophos.xg.user_cpu", cached_regex!("%$"), "")?;
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.user_cpu") {
                if let Some(val) = event.get("sophos.xg.user_cpu") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.user_cpu".into(),
                            message,
                        })?;
                    event.set("sophos.xg.user_cpu", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_user_cpu_c2564909")?;
                        if event.remove("sophos.xg.user_cpu").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.user_cpu".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.used") {
                if let Some(val) = event.get("sophos.xg.used") {
                    let converted = convert_value(val, "integer")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.used".into(),
                            message,
                        })?;
                    event.set("sophos.xg.used", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_used_8dbe6b69")?;
                        if event.remove("sophos.xg.used").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.used".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.total_memory") {
                if let Some(val) = event.get("sophos.xg.total_memory") {
                    let converted = convert_value(val, "integer")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.total_memory".into(),
                            message,
                        })?;
                    event.set("sophos.xg.total_memory", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_total_memory_e8eb0879")?;
                        if event.remove("sophos.xg.total_memory").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.total_memory".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.free") {
                if let Some(val) = event.get("sophos.xg.free") {
                    let converted = convert_value(val, "integer")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.free".into(),
                            message,
                        })?;
                    event.set("sophos.xg.free", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_free_6cba3b75")?;
                        if event.remove("sophos.xg.free").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.free".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.configuration") {
                gsub_field(event, "sophos.xg.configuration", "sophos.xg.configuration", cached_regex!("%$"), "")?;
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.configuration") {
                if let Some(val) = event.get("sophos.xg.configuration") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.configuration".into(),
                            message,
                        })?;
                    event.set("sophos.xg.configuration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_configuration_e9a7a27f")?;
                        if event.remove("sophos.xg.configuration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.configuration".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.reports") {
                gsub_field(event, "sophos.xg.reports", "sophos.xg.reports", cached_regex!("%$"), "")?;
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.reports") {
                if let Some(val) = event.get("sophos.xg.reports") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.reports".into(),
                            message,
                        })?;
                    event.set("sophos.xg.reports", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_reports_1dfbe06d")?;
                        if event.remove("sophos.xg.reports").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.reports".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.temp") {
                gsub_field(event, "sophos.xg.temp", "sophos.xg.temp", cached_regex!("%$"), "")?;
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.temp") {
                if let Some(val) = event.get("sophos.xg.temp") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.temp".into(),
                            message,
                        })?;
                    event.set("sophos.xg.temp", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_temp_9bb56e85")?;
                        if event.remove("sophos.xg.temp").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.temp".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.signature") {
                gsub_field(event, "sophos.xg.signature", "sophos.xg.signature", cached_regex!("%$"), "")?;
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.signature") {
                if let Some(val) = event.get("sophos.xg.signature") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.signature".into(),
                            message,
                        })?;
                    event.set("sophos.xg.signature", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_signature_f1ca0097")?;
                        if event.remove("sophos.xg.signature").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.signature".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.users") {
                if let Some(val) = event.get("sophos.xg.users") {
                    let converted = convert_value(val, "integer")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.users".into(),
                            message,
                        })?;
                    event.set("sophos.xg.users", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_users_389670cf")?;
                        if event.remove("sophos.xg.users").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.users".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.transmittedkbits") {
                if let Some(val) = event.get("sophos.xg.transmittedkbits") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.transmittedkbits".into(),
                            message,
                        })?;
                    event.set("sophos.xg.transmittedkbits", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_transmittedkbits_01bd9d3d")?;
                        if event.remove("sophos.xg.transmittedkbits").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.transmittedkbits".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.receivedkbits") {
                if let Some(val) = event.get("sophos.xg.receivedkbits") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.receivedkbits".into(),
                            message,
                        })?;
                    event.set("sophos.xg.receivedkbits", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_receivedkbits_0f95c677")?;
                        if event.remove("sophos.xg.receivedkbits").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.receivedkbits".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.collisions") {
                if let Some(val) = event.get("sophos.xg.collisions") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.collisions".into(),
                            message,
                        })?;
                    event.set("sophos.xg.collisions", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_collisions_68b4412d")?;
                        if event.remove("sophos.xg.collisions").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.collisions".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.receiveddrops") {
                if let Some(val) = event.get("sophos.xg.receiveddrops") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.receiveddrops".into(),
                            message,
                        })?;
                    event.set("sophos.xg.receiveddrops", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_receiveddrops_50710095")?;
                        if event.remove("sophos.xg.receiveddrops").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.receiveddrops".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.transmitteddrops") {
                if let Some(val) = event.get("sophos.xg.transmitteddrops") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.transmitteddrops".into(),
                            message,
                        })?;
                    event.set("sophos.xg.transmitteddrops", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_transmitteddrops_c502223d")?;
                        if event.remove("sophos.xg.transmitteddrops").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.transmitteddrops".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
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
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
