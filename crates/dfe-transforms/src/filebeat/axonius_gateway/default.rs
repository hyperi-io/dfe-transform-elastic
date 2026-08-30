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
            event.set("ecs.version", json!("9.3.0"))?;

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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "axonius.gateway")?;

            event.set("event.kind", json!("event"))?;

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("axonius.gateway.default") {
                    if let Some(val) = event.get("axonius.gateway.default") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.gateway.default".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.gateway.default", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_axonius_gateway_default_to_boolean_c2d04259",
                )?;
                if event.remove("axonius.gateway.default").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "axonius.gateway.default".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("axonius.gateway.email_when_connected") {
                    if let Some(val) = event.get("axonius.gateway.email_when_connected") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.gateway.email_when_connected".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.gateway.email_when_connected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_axonius_gateway_email_when_connected_to_boolean_26cb764f",
                )?;
                if event
                    .remove("axonius.gateway.email_when_connected")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "axonius.gateway.email_when_connected".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("axonius.gateway.email_when_disconnected") {
                    if let Some(val) = event.get("axonius.gateway.email_when_disconnected") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.gateway.email_when_disconnected".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.gateway.email_when_disconnected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_axonius_gateway_email_when_disconnected_to_boolean_297af2e9",
                )?;
                if event
                    .remove("axonius.gateway.email_when_disconnected")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "axonius.gateway.email_when_disconnected".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("axonius.gateway.tunnel_proxy_settings.enabled") {
                    if let Some(val) = event.get("axonius.gateway.tunnel_proxy_settings.enabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "axonius.gateway.tunnel_proxy_settings.enabled".into(),
                                message,
                            }
                        })?;
                        event.set("axonius.gateway.tunnel_proxy_settings.enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_axonius_gateway_tunnel_proxy_settings_enabled_to_boolean_1b5ece04",
                )?;
                if event
                    .remove("axonius.gateway.tunnel_proxy_settings.enabled")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "axonius.gateway.tunnel_proxy_settings.enabled".into(),
                    });
                }
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

            if event.has_value("axonius.gateway.tunnel_proxy_settings.tunnel_proxy_port") {
                if let Some(val) =
                    event.get("axonius.gateway.tunnel_proxy_settings.tunnel_proxy_port")
                {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "axonius.gateway.tunnel_proxy_settings.tunnel_proxy_port".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "axonius.gateway.tunnel_proxy_settings.tunnel_proxy_port",
                        converted,
                    )?;
                }
            }

            let _cond = { event.get_str("axonius.gateway.dns_server") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("axonius.gateway.dns_server") {
                        if let Some(val) = event.get("axonius.gateway.dns_server") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "axonius.gateway.dns_server".into(),
                                    message,
                                }
                            })?;
                            event.set("axonius.gateway.dns_server", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_axonius_gateway_dns_server_to_ip_33adc7e4",
                    )?;
                    if event.remove("axonius.gateway.dns_server").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "axonius.gateway.dns_server".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("axonius.gateway.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event
                    .get("axonius.gateway.email_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "axonius.gateway.email_recipients", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("axonius.gateway.status")
                    && event
                        .get_str("axonius.gateway.status")
                        .is_some_and(|s| s.to_lowercase() == "success")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("axonius.gateway.status")
                    && event
                        .get_str("axonius.gateway.status")
                        .is_some_and(|s| s.to_lowercase() == "error")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond =
                { event.has_value("axonius.gateway.tunnel_proxy_settings.tunnel_proxy_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("axonius.gateway.tunnel_proxy_settings.tunnel_proxy_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("axonius.gateway.dns_server") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("axonius.gateway.dns_server")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("axonius.gateway.email_recipients");
            event.remove("axonius.gateway.id");

            // Painless script
            // Source: void handleMap(Map map) {\nmap.values().removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\nmap.values().removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nvoid handleList(List list) {\nlist.removeIf(v -> {\n\tif (v instanceof Map) {\n\thandleMap(v);\n\t} else if (v instanceof List) {\n\thandleList(v);\n\t}\n\treturn v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n});\n}\nhandleMap(ctx);"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
