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

            event.set("event.kind", json!("event"))?;

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
                event.set("_ingest.on_failure_processor_tag", "json_message")?;
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
                            .get("_ingest.pipeline")
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

            if event.has_value("json._id") {
                event.rename("json._id", "prisma_cloud.incident_audit._id")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.incident_audit._id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.accountID") {
                event.rename("json.accountID", "prisma_cloud.incident_audit.account_id")?;
            }

            event.append_unique(
                "cloud.account.id",
                json!(
                    event
                        .get("prisma_cloud.incident_audit.account_id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get_str("json.acknowledged") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.acknowledged") {
                        if let Some(val) = event.get("json.acknowledged") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.acknowledged".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.incident_audit.acknowledged", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_prisma_cloud_incident_audit_acknowledged",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.appID") {
                event.rename("json.appID", "prisma_cloud.incident_audit.app.id")?;
            }

            if event.has_value("json.app") {
                event.rename("json.app", "prisma_cloud.incident_audit.app.value")?;
            }

            if event.has_value("json.audits") {
                event.rename("json.audits", "prisma_cloud.incident_audit.data")?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.app") {
                            event.rename("_ingest._value.app", "_ingest._value.app.value")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.appID") {
                            event.rename("_ingest._value.appID", "_ingest._value.app.id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.accountID") {
                            event
                                .rename("_ingest._value.accountID", "_ingest._value.account_id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "cloud.account.id",
                            json!(
                                event
                                    .get("_ingest._value.account_id")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.attackTechniques") {
                            event.rename(
                                "_ingest._value.attackTechniques",
                                "_ingest._value.attack.techniques",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.attack.techniques", |event| {
                                event.append_unique(
                                    "threat.technique.name",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.attackType") {
                            event.rename(
                                "_ingest._value.attackType",
                                "_ingest._value.attack.type",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "threat.technique.subtechnique.name",
                            json!(
                                event
                                    .get("_ingest._value.attack.type")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.container") {
                            event.rename(
                                "_ingest._value.container",
                                "_ingest._value.container.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.container.value") {
                                if let Some(val) = event.get("_ingest._value.container.value") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.container.value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.container.value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_incident_audit_data_container_to_boolean",
                            )?;
                            event.remove("_ingest._value.container.value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.containerId") {
                            event.rename(
                                "_ingest._value.containerId",
                                "_ingest._value.container.id",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.containerName") {
                            event.rename(
                                "_ingest._value.containerName",
                                "_ingest._value.container.name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                    event.append_unique(
                        "container.name",
                        json!(
                            event
                                .get("_ingest._value.container.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.count") {
                                if let Some(val) = event.get("_ingest._value.count") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.count".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.count", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_incident_audit_data_count_to_long_1",
                            )?;
                            event.remove("_ingest._value.count");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                    event.append_unique(
                        "host.domain",
                        json!(
                            event
                                .get("_ingest._value.fqdn")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                    event.append_unique(
                        "os.full",
                        json!(
                            event
                                .get("_ingest._value.os")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.fqdn")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.function") {
                            event.rename(
                                "_ingest._value.function",
                                "_ingest._value.function.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.functionID") {
                            event.rename(
                                "_ingest._value.functionID",
                                "_ingest._value.function.id",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.imageId") {
                            event.rename("_ingest._value.imageId", "_ingest._value.image.id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.imageName") {
                            event
                                .rename("_ingest._value.imageName", "_ingest._value.image.name")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "container.image.name",
                            json!(
                                event
                                    .get("_ingest._value.image.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.interactive") {
                                if let Some(val) = event.get("_ingest._value.interactive") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.interactive".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.interactive", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_incident_audit_data_interactive_to_boolean",
                            )?;
                            event.remove("_ingest._value.interactive");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.ip") {
                                if let Some(val) = event.get("_ingest._value.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_incident_audit_data_ip_to_ip",
                            )?;
                            event.remove("_ingest._value.ip");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.pid") {
                                if let Some(val) = event.get("_ingest._value.pid") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.pid".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.pid", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_incident_audit_data_pid_to_long",
                            )?;
                            event.remove("_ingest._value.pid");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.port") {
                                if let Some(val) = event.get("_ingest._value.port") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.port".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.port", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_incident_audit_data_port_to_long",
                            )?;
                            event.remove("_ingest._value.port");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.processPath") {
                            event.rename(
                                "_ingest._value.processPath",
                                "_ingest._value.process_path",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.profileId") {
                            event
                                .rename("_ingest._value.profileId", "_ingest._value.profile_id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "cloud.provider",
                            json!(
                                event
                                    .get("_ingest._value.provider")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.rawEvent") {
                            event.rename("_ingest._value.rawEvent", "_ingest._value.raw_event")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.requestID") {
                            event
                                .rename("_ingest._value.requestID", "_ingest._value.request_id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.resourceID") {
                            event.rename(
                                "_ingest._value.resourceID",
                                "_ingest._value.resource_id",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.ruleName") {
                            event.rename("_ingest._value.ruleName", "_ingest._value.rule_name")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "rule.name",
                            json!(
                                event
                                    .get("_ingest._value.rule_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.incident_audit.data").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.time")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.time", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.time".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.remove("_ingest._value.time");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.incident_audit.data",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.vmID") {
                            event.rename("_ingest._value.vmID", "_ingest._value.vm_id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        if event.has_value("_ingest._value.wildFireReportURL") {
                            event.rename(
                                "_ingest._value.wildFireReportURL",
                                "_ingest._value.wild_fire_report_url",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.incident_audit.data")
                    .is_some_and(|v| v.is_array())
                    && (!event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        })))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.incident_audit.data", |event| {
                        event.remove("_ingest._value.attack.type");
                        event.remove("_ingest._value.attack.techniques");
                        event.remove("_ingest._value.container.name");
                        event.remove("_ingest._value.image.name");
                        event.remove("_ingest._value.rule_name");
                        event.remove("_ingest._value.provider");
                        event.remove("_ingest._value.fqdn");
                        event.remove("_ingest._value.account_id");
                        event.remove("_ingest._value.os");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.category")
                    && event
                        .get_str("json.category")
                        .is_some_and(|s| s.to_lowercase().contains("malware"))
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.has_value("event.category") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            if event.has_value("json.category") {
                event.rename("json.category", "prisma_cloud.incident_audit.category")?;
            }

            if event.has_value("json.cluster") {
                event.rename("json.cluster", "prisma_cloud.incident_audit.cluster")?;
            }

            if event.has_value("json.collections") {
                event.rename(
                    "json.collections",
                    "prisma_cloud.incident_audit.collections",
                )?;
            }

            if event.has_value("json.containerID") {
                event.rename(
                    "json.containerID",
                    "prisma_cloud.incident_audit.container.id",
                )?;
            }

            if let Some(v) = event
                .get("prisma_cloud.incident_audit.container.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.id", v)?;
            }

            if event.has_value("json.containerName") {
                event.rename(
                    "json.containerName",
                    "prisma_cloud.incident_audit.container.name",
                )?;
            }

            let _cond = { event.has_value("prisma_cloud.incident_audit.container.name") };
            if _cond {
                event.append_unique(
                    "container.name",
                    json!(
                        event
                            .get("prisma_cloud.incident_audit.container.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.customRuleName") {
                event.rename(
                    "json.customRuleName",
                    "prisma_cloud.incident_audit.custom_rule_name",
                )?;
            }

            let _cond = { event.has_value("prisma_cloud.incident_audit.custom_rule_name") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("prisma_cloud.incident_audit.custom_rule_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.fqdn") {
                event.rename("json.fqdn", "prisma_cloud.incident_audit.fqdn")?;
            }

            event.append_unique(
                "host.domain",
                json!(
                    event
                        .get("prisma_cloud.incident_audit.fqdn")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.has_value("prisma_cloud.incident_audit.fqdn") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_cloud.incident_audit.fqdn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.function") {
                event.rename(
                    "json.function",
                    "prisma_cloud.incident_audit.function.value",
                )?;
            }

            if event.has_value("json.functionID") {
                event.rename("json.functionID", "prisma_cloud.incident_audit.function.id")?;
            }

            if event.has_value("json.hostname") {
                event.rename("json.hostname", "prisma_cloud.incident_audit.hostname")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.incident_audit.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("prisma_cloud.incident_audit.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_cloud.incident_audit.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.imageID") {
                event.rename("json.imageID", "prisma_cloud.incident_audit.image.id")?;
            }

            if event.has_value("json.imageName") {
                event.rename("json.imageName", "prisma_cloud.incident_audit.image.name")?;
            }

            let _cond = { event.has_value("prisma_cloud.incident_audit.image.name") };
            if _cond {
                event.append_unique(
                    "container.image.name",
                    json!(
                        event
                            .get("prisma_cloud.incident_audit.image.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.labels") {
                event.rename("json.labels", "prisma_cloud.incident_audit.labels")?;
            }

            if event.has_value("json.namespace") {
                event.rename("json.namespace", "prisma_cloud.incident_audit.namespace")?;
            }

            if event.has_value("json.profileID") {
                event.rename("json.profileID", "prisma_cloud.incident_audit.profile_id")?;
            }

            if event.has_value("json.provider") {
                event.rename("json.provider", "prisma_cloud.incident_audit.provider")?;
            }

            event.append_unique(
                "cloud.provider",
                json!(
                    event
                        .get("prisma_cloud.incident_audit.provider")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.region") {
                event.rename("json.region", "prisma_cloud.incident_audit.region")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.incident_audit.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if event.has_value("json.resourceID") {
                event.rename("json.resourceID", "prisma_cloud.incident_audit.resource_id")?;
            }

            if event.has_value("json.runtime") {
                event.rename("json.runtime", "prisma_cloud.incident_audit.runtime")?;
            }

            let _cond = { event.get_str("json.serialNum") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.serialNum") {
                        if let Some(val) = event.get("json.serialNum") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.serialNum".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.incident_audit.serial_num", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_prisma_cloud_incident_audit_serialNum",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.shouldCollect") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.shouldCollect") {
                        if let Some(val) = event.get("json.shouldCollect") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.shouldCollect".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.incident_audit.should_collect", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_prisma_cloud_incident_audit_shouldCollect",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.type") {
                event.rename("json.type", "prisma_cloud.incident_audit.type")?;
            }

            if event.has_value("json.vmID") {
                event.rename("json.vmID", "prisma_cloud.incident_audit.vm_id")?;
            }

            let _cond = { event.get_str("json.windows") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.windows") {
                        if let Some(val) = event.get("json.windows") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.windows".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.incident_audit.windows", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_prisma_cloud_incident_audit_windows",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_prisma_cloud_incident_audit_time",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("prisma_cloud.incident_audit.time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_rename_time_to_custom_name",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
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
                event.remove("prisma_cloud.incident_audit._id");
                event.remove("prisma_cloud.incident_audit.account_id");
                event.remove("prisma_cloud.incident_audit.container.id");
                event.remove("prisma_cloud.incident_audit.container.name");
                event.remove("prisma_cloud.incident_audit.custom_rule_name");
                event.remove("prisma_cloud.incident_audit.fqdn");
                event.remove("prisma_cloud.incident_audit.hostname");
                event.remove("prisma_cloud.incident_audit.image.name");
                event.remove("prisma_cloud.incident_audit.provider");
                event.remove("prisma_cloud.incident_audit.region");
                event.remove("prisma_cloud.incident_audit.time");
                event.remove("prisma_cloud.incident_audit.category");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                    json!(format!(
                        "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
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
