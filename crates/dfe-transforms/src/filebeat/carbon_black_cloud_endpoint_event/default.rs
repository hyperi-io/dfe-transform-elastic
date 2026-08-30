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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(v) = event.get("json.device_timestamp").cloned() {
                    event.set("_temp_.device_timestamp", v)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "set")?;
                if event.remove("json.device_timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.device_timestamp".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            let _cond = {
                event.has_value("json.device_timestamp")
                    && event.get_str("json.device_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.device_timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd HH:mm:ss Z z",
                                "yyyy-MM-dd HH:mm:ss.S Z z",
                                "yyyy-MM-dd HH:mm:ss.SS Z z",
                                "yyyy-MM-dd HH:mm:ss.SSS Z z",
                                "yyyy-MM-dd HH:mm:ss.SSSS Z z",
                                "yyyy-MM-dd HH:mm:ss.SSSSS Z z",
                                "yyyy-MM-dd HH:mm:ss.SSSSSS Z z",
                                "yyyy-MM-dd HH:mm:ss.SSSSSSS Z z",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.device_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.device_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.device_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            let _cond = { event.has_value("json.type") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def mapping = params.type_mapping[ctx.json.type];\nif (mapping == null) {\n  return;\n}\nctx.event.category = mapping.category;\nctx.event.type = mapping.type;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def mapping = params.type_mapping[ctx.json.type];\nif (mapping == null) {\n  return;\n}\nctx.event.category = mapping.category;\nctx.event.type = mapping.type;"#
                        ),
                        cached_params!(
                            "{\"type_mapping\":{\"endpoint.event.crossproc\":{\"category\":[\"process\"],\"type\":[\"info\"]},\"endpoint.event.filemod\":{\"category\":[\"file\"],\"type\":[\"change\"]},\"endpoint.event.fileless_scriptload\":{\"category\":[\"process\"],\"type\":[\"start\"]},\"endpoint.event.moduleload\":{\"category\":[\"process\"],\"type\":[\"start\"]},\"endpoint.event.netconn\":{\"category\":[\"network\"],\"type\":[\"connection\"]},\"endpoint.event.netconn_proxy\":{\"category\":[\"network\"],\"type\":[\"connection\"]},\"endpoint.event.procstart\":{\"category\":[\"process\"],\"type\":[\"start\"]},\"endpoint.event.regmod\":{\"category\":[\"registry\"],\"type\":[\"change\"]},\"endpoint.event.scriptload\":{\"category\":[\"process\",\"file\"],\"type\":[\"start\",\"access\"]}}}"
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.action") {
                event.rename("json.action", "event.action")?;
            }

            if event.has_value("json.event_id") {
                event.rename("json.event_id", "event.id")?;
            }

            if event.has_value("json.event_description") {
                event.rename("json.event_description", "event.reason")?;
            }

            if event.has_value("json.filemod_name") {
                event.rename("json.filemod_name", "file.path")?;
            }

            if event.has_value("json.modload_name") {
                event.rename("json.modload_name", "dll.path")?;
            }

            let _cond = { event.get_str("json.netconn_protocol") == Some("PROTO_UDP") };
            if _cond {
                event.set("network.transport", json!("udp"))?;
            }

            let _cond = { event.get_str("json.netconn_protocol") == Some("PROTO_TCP") };
            if _cond {
                event.set("network.transport", json!("tcp"))?;
            }

            let _cond = { event.get_bool("json.netconn_inbound") == Some(true) };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = { event.get_bool("json.netconn_inbound") == Some(false) };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.remote_port") {
                    if let Some(val) = event.get("json.remote_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.remote_port".into(),
                                message,
                            }
                        })?;
                        event.set("json.remote_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.remote_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.remote_port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.remote_ip") {
                    if let Some(val) = event.get("json.remote_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.remote_ip".into(),
                                message,
                            }
                        })?;
                        event.set("json.remote_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.remote_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.remote_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.local_port") {
                    if let Some(val) = event.get("json.local_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.local_port".into(),
                                message,
                            }
                        })?;
                        event.set("json.local_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.local_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.local_port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.local_ip") {
                    if let Some(val) = event.get("json.local_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.local_ip".into(),
                                message,
                            }
                        })?;
                        event.set("json.local_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.local_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.local_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // Painless script
            // Source: // These allocations may be futile, but will be cleaned up in the postamble.\nif (ctx.client == null) {\n  ctx.client = new HashMap();\n}\nif (ctx.source == null) {\n  ctx.source = new HashMap();\n}\nif (ctx.destination == null) {\n  ctx.destination = new HashMap();\n}\n// Nulls inserted into the document will be cleaned up in the postamble.\nctx.client.ip = ctx.json.local_ip;\nctx.client.port = ctx.json.local_port;\nif (ctx.json?.netconn_inbound == true) {\n  ctx.destination.ip = ctx.json?.local_ip;\n  ctx.destination.port = ctx.json?.local_port;\n  ctx.source.ip = ctx.json?.remote_ip;\n  ctx.source.port = ctx.json?.remote_port;\n  ctx.source.domain = ctx.json?.netconn_domain;\n} else {\n  ctx.source.ip = ctx.json?.local_ip;\n  ctx.source.port = ctx.json?.local_port;\n  ctx.destination.ip = ctx.json?.remote_ip;\n  ctx.destination.port = ctx.json?.remote_port;\n  ctx.destination.domain = ctx.json?.netconn_domain;\n}     \n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// These allocations may be futile, but will be cleaned up in the postamble.\nif (ctx.client == null) {\n  ctx.client = new HashMap();\n}\nif (ctx.source == null) {\n  ctx.source = new HashMap();\n}\nif (ctx.destination == null) {\n  ctx.destination = new HashMap();\n}\n// Nulls inserted into the document will be cleaned up in the postamble.\nctx.client.ip = ctx.json.local_ip;\nctx.client.port = ctx.json.local_port;\nif (ctx.json?.netconn_inbound == true) {\n  ctx.destination.ip = ctx.json?.local_ip;\n  ctx.destination.port = ctx.json?.local_port;\n  ctx.source.ip = ctx.json?.remote_ip;\n  ctx.source.port = ctx.json?.remote_port;\n  ctx.source.domain = ctx.json?.netconn_domain;\n} else {\n  ctx.source.ip = ctx.json?.local_ip;\n  ctx.source.port = ctx.json?.local_port;\n  ctx.destination.ip = ctx.json?.remote_ip;\n  ctx.destination.port = ctx.json?.remote_port;\n  ctx.destination.domain = ctx.json?.netconn_domain;\n}     \n"#
                ),
            )?;

            let _cond = { event.has_value("source.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.device_id") {
                    if let Some(val) = event.get("json.device_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.device_id".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.device_os") == Some("WINDOWS") };
            if _cond {
                event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { event.get_str("json.device_os") == Some("LINUX") };
            if _cond {
                event.set("host.os.type", json!("linux"))?;
            }

            let _cond = { event.get_str("json.device_os") == Some("MAC") };
            if _cond {
                event.set("host.os.type", json!("macos"))?;
            }

            if event.has_value("json.device_name") {
                event.rename("json.device_name", "host.hostname")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("host.hostname") {
                    if let Some(input) = event.get_string("host.hostname") {
                        // Grok pattern: ^(%{DATA:user.domain})\\\\(%{GREEDYDATA:host.hostname})$
                        let _ = cached_grok!(
                            "^(%{DATA:user.domain})\\\\(%{GREEDYDATA:host.hostname})$"
                        )
                        .extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "host.name",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("user.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.device_group") {
                event.rename("json.device_group", "host.os.family")?;
            }

            if event.has_value("json.device_group") {
                event.rename("json.device_group", "host.os.family")?;
            }

            if event.has_value("json.regmod_name") {
                event.rename("json.regmod_name", "registry.path")?;
            }

            // Painless script
            // Source: void mapHashField(def ctx, def hashes, def key) {\n    for (hash in hashes) {\n        if (hash.length() == 32) {ctx.json[key + '_md5'] = hash;}\n        if (hash.length() == 64) {ctx.json[key + '_sha256'] = hash;}\n    }\n}\nif (ctx.json?.process_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.process_hash, 'process_hash');\n}\nif (ctx.json?.parent_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.parent_hash, 'parent_hash');\n}\nif (ctx.json?.filemod_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.filemod_hash, 'filemod_hash');\n}\nif (ctx.json?.childproc_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.childproc_hash, 'childproc_hash');\n}\nif (ctx.json?.crossproc_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.crossproc_hash, 'crossproc_hash');\n}\nif (ctx.json?.scriptload_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.scriptload_hash, 'scriptload_hash');\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void mapHashField(def ctx, def hashes, def key) {\n    for (hash in hashes) {\n        if (hash.length() == 32) {ctx.json[key + '_md5'] = hash;}\n        if (hash.length() == 64) {ctx.json[key + '_sha256'] = hash;}\n    }\n}\nif (ctx.json?.process_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.process_hash, 'process_hash');\n}\nif (ctx.json?.parent_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.parent_hash, 'parent_hash');\n}\nif (ctx.json?.filemod_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.filemod_hash, 'filemod_hash');\n}\nif (ctx.json?.childproc_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.childproc_hash, 'childproc_hash');\n}\nif (ctx.json?.crossproc_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.crossproc_hash, 'crossproc_hash');\n}\nif (ctx.json?.scriptload_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.scriptload_hash, 'scriptload_hash');\n}\n"#
                ),
            )?;

            let _cond = {
                event.has_value("json.type")
                    && event.get_str("json.type") != Some("endpoint.event.procstart")
            };
            if _cond {
                // Begin nested pipeline: "process_other"
                if event.has_value("json.parent_cmdline") {
                    event.rename("json.parent_cmdline", "process.parent.command_line")?;
                }
                if event.has_value("json.parent_path") {
                    event.rename("json.parent_path", "process.parent.executable")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.parent_pid") {
                        if let Some(val) = event.get("json.parent_pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.parent_pid".into(),
                                    message,
                                }
                            })?;
                            event.set("process.parent.pid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.parent_guid") {
                    event.rename("json.parent_guid", "process.parent.entity_id")?;
                }
                if event.has_value("json.parent_reputation") {
                    event.rename(
                        "json.parent_reputation",
                        "carbon_black_cloud.endpoint_event.process.parent.reputation",
                    )?;
                }
                if event.has_value("json.parent_hash_md5") {
                    event.rename("json.parent_hash_md5", "process.parent.hash.md5")?;
                }
                if event.has_value("json.parent_hash_sha256") {
                    event.rename("json.parent_hash_sha256", "process.parent.hash.sha256")?;
                }
                if event.has_value("json.process_cmdline") {
                    event.rename("json.process_cmdline", "process.command_line")?;
                }
                if event.has_value("json.process_path") {
                    event.rename("json.process_path", "process.executable")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.process_pid") {
                        if let Some(val) = event.get("json.process_pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.process_pid".into(),
                                    message,
                                }
                            })?;
                            event.set("process.pid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.process_guid") {
                    event.rename("json.process_guid", "process.entity_id")?;
                }
                if event.has_value("json.process_username") {
                    event.rename(
                        "json.process_username",
                        "carbon_black_cloud.endpoint_event.process.username",
                    )?;
                }
                if event.has_value("json.process_reputation") {
                    event.rename(
                        "json.process_reputation",
                        "carbon_black_cloud.endpoint_event.process.reputation",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.process_publisher") {
                        foreach_array(event, "json.process_publisher", |event| {
                            if event.has_value("_ingest._value.state") {
                                if let Some(s) = event.get_string("_ingest._value.state") {
                                    let mut parts: Vec<Value> = cached_regex!(" \\| ")
                                        .split(&s)
                                        .into_iter()
                                        .map(|p| json!(p))
                                        .collect();
                                    while parts.last().and_then(Value::as_str) == Some("") {
                                        parts.pop();
                                    }
                                    event.set("_ingest._value.state", Value::Array(parts))?;
                                }
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
                if event.has_value("json.process_publisher") {
                    event.rename(
                        "json.process_publisher",
                        "carbon_black_cloud.endpoint_event.process.publisher",
                    )?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.process_duration") {
                        if let Some(val) = event.get("json.process_duration") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.process_duration".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "carbon_black_cloud.endpoint_event.process.duration",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    if event.remove("json.process_duration").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.process_duration".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.process_terminated") {
                        if let Some(val) = event.get("json.process_terminated") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.process_terminated".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "carbon_black_cloud.endpoint_event.process.terminated",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.process_hash_md5") {
                    event.rename("json.process_hash_md5", "process.hash.md5")?;
                }
                if event.has_value("json.process_hash_sha256") {
                    event.rename("json.process_hash_sha256", "process.hash.sha256")?;
                }
                // End nested pipeline: "process_other"
            }

            let _cond = { event.get_str("json.type") == Some("endpoint.event.procstart") };
            if _cond {
                // Begin nested pipeline: "process_procstart"
                if event.has_value("json.parent_cmdline") {
                    event.rename(
                        "json.parent_cmdline",
                        "carbon_black_cloud.endpoint_event.process.grandparent.command_line",
                    )?;
                }
                if event.has_value("json.parent_path") {
                    event.rename(
                        "json.parent_path",
                        "carbon_black_cloud.endpoint_event.process.grandparent.executable",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.parent_pid") {
                        if let Some(val) = event.get("json.parent_pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.parent_pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "carbon_black_cloud.endpoint_event.process.grandparent.pid",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.parent_guid") {
                    event.rename(
                        "json.parent_guid",
                        "carbon_black_cloud.endpoint_event.process.grandparent.entity_id",
                    )?;
                }
                if event.has_value("json.parent_reputation") {
                    event.rename(
                        "json.parent_reputation",
                        "carbon_black_cloud.endpoint_event.process.grandparent.reputation",
                    )?;
                }
                if event.has_value("json.parent_hash_md5") {
                    event.rename(
                        "json.parent_hash_md5",
                        "carbon_black_cloud.endpoint_event.process.grandparent.hash.md5",
                    )?;
                }
                if event.has_value("json.parent_hash_sha256") {
                    event.rename(
                        "json.parent_hash_sha256",
                        "carbon_black_cloud.endpoint_event.process.grandparent.hash.sha256",
                    )?;
                }
                let _cond = {
                    event
                        .has_value("carbon_black_cloud.endpoint_event.process.grandparent.hash.md5")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique("related.hash", json!(event.get("carbon_black_cloud.endpoint_event.process.grandparent.hash.md5").map_or_else(String::new, template_to_string)))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value(
                        "carbon_black_cloud.endpoint_event.process.grandparent.hash.sha256",
                    )
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique("related.hash", json!(event.get("carbon_black_cloud.endpoint_event.process.grandparent.hash.sha256").map_or_else(String::new, template_to_string)))?;
                        Ok(())
                    })();
                }
                if event.has_value("json.process_cmdline") {
                    event.rename("json.process_cmdline", "process.parent.command_line")?;
                }
                if event.has_value("json.process_path") {
                    event.rename("json.process_path", "process.parent.executable")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.process_pid") {
                        if let Some(val) = event.get("json.process_pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.process_pid".into(),
                                    message,
                                }
                            })?;
                            event.set("process.parent.pid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.process_guid") {
                    event.rename("json.process_guid", "process.parent.entity_id")?;
                }
                if event.has_value("json.process_username") {
                    event.rename(
                        "json.process_username",
                        "carbon_black_cloud.endpoint_event.process.parent.username",
                    )?;
                }
                if event.has_value("json.process_reputation") {
                    event.rename(
                        "json.process_reputation",
                        "carbon_black_cloud.endpoint_event.process.parent.reputation",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.process_publisher") {
                        foreach_array(event, "json.process_publisher", |event| {
                            if event.has_value("_ingest._value.state") {
                                if let Some(s) = event.get_string("_ingest._value.state") {
                                    let mut parts: Vec<Value> = cached_regex!(" \\| ")
                                        .split(&s)
                                        .into_iter()
                                        .map(|p| json!(p))
                                        .collect();
                                    while parts.last().and_then(Value::as_str) == Some("") {
                                        parts.pop();
                                    }
                                    event.set("_ingest._value.state", Value::Array(parts))?;
                                }
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
                if event.has_value("json.process_publisher") {
                    event.rename(
                        "json.process_publisher",
                        "carbon_black_cloud.endpoint_event.process.parent.publisher",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.process_duration") {
                        if let Some(val) = event.get("json.process_duration") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.process_duration".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "carbon_black_cloud.endpoint_event.process.parent.duration",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.process_terminated") {
                        if let Some(val) = event.get("json.process_terminated") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.process_terminated".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "carbon_black_cloud.endpoint_event.process.parent.terminated",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.process_hash_md5") {
                    event.rename("json.process_hash_md5", "process.parent.hash.md5")?;
                }
                if event.has_value("json.process_hash_sha256") {
                    event.rename("json.process_hash_sha256", "process.parent.hash.sha256")?;
                }
                let _cond = {
                    event.has_value("carbon_black_cloud.endpoint_event.process.parent.username")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get(
                                        "carbon_black_cloud.endpoint_event.process.parent.username"
                                    )
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("json.childproc_name") {
                    event.rename("json.childproc_name", "process.executable")?;
                }
                if event.has_value("json.childproc_username") {
                    event.rename("json.childproc_username", "process.user.name")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.childproc_pid") {
                        if let Some(val) = event.get("json.childproc_pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.childproc_pid".into(),
                                    message,
                                }
                            })?;
                            event.set("process.pid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.childproc_guid") {
                    event.rename("json.childproc_guid", "process.entity_id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.childproc_publisher") {
                        foreach_array(event, "json.childproc_publisher", |event| {
                            if event.has_value("_ingest._value.state") {
                                if let Some(s) = event.get_string("_ingest._value.state") {
                                    let mut parts: Vec<Value> = cached_regex!(" \\| ")
                                        .split(&s)
                                        .into_iter()
                                        .map(|p| json!(p))
                                        .collect();
                                    while parts.last().and_then(Value::as_str) == Some("") {
                                        parts.pop();
                                    }
                                    event.set("_ingest._value.state", Value::Array(parts))?;
                                }
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
                if event.has_value("json.childproc_publisher") {
                    event.rename(
                        "json.childproc_publisher",
                        "carbon_black_cloud.endpoint_event.process.publisher",
                    )?;
                }
                if event.has_value("json.childproc_reputation") {
                    event.rename(
                        "json.childproc_reputation",
                        "carbon_black_cloud.endpoint_event.process.reputation",
                    )?;
                }
                if event.has_value("json.childproc_hash_md5") {
                    event.rename("json.childproc_hash_md5", "process.hash.md5")?;
                }
                if event.has_value("json.childproc_hash_sha256") {
                    event.rename("json.childproc_hash_sha256", "process.hash.sha256")?;
                }
                // End nested pipeline: "process_procstart"
            }

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("process.parent.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("process.parent.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.backend_timestamp") {
                event.rename(
                    "json.backend_timestamp",
                    "carbon_black_cloud.endpoint_event.backend.timestamp",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(v) = event
                    .get("_temp_.device_timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("carbon_black_cloud.endpoint_event.device.timestamp", v)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "set")?;
                if event
                    .remove("carbon_black_cloud.endpoint_event.device.timestamp")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "carbon_black_cloud.endpoint_event.device.timestamp".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.create_time") {
                event.rename(
                    "json.create_time",
                    "carbon_black_cloud.endpoint_event.create_time",
                )?;
            }

            if event.has_value("json.device_os") {
                event.rename(
                    "json.device_os",
                    "carbon_black_cloud.endpoint_event.device.os",
                )?;
            }

            if event.has_value("json.org_key") {
                event.rename(
                    "json.org_key",
                    "carbon_black_cloud.endpoint_event.organization_key",
                )?;
            }

            let _cond = { event.has_value("process.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("process.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("carbon_black_cloud.endpoint_event.process.username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.process.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("carbon_black_cloud.endpoint_event.process.parent.username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.process.parent.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.target_cmdline") {
                event.rename(
                    "json.target_cmdline",
                    "carbon_black_cloud.endpoint_event.target_cmdline",
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "carbon_black_cloud.endpoint_event.type")?;
            }

            if event.has_value("json.crossproc_action") {
                event.rename(
                    "json.crossproc_action",
                    "carbon_black_cloud.endpoint_event.crossproc.action",
                )?;
            }

            if event.has_value("json.crossproc_api") {
                event.rename(
                    "json.crossproc_api",
                    "carbon_black_cloud.endpoint_event.crossproc.api",
                )?;
            }

            if event.has_value("json.crossproc_guid") {
                event.rename(
                    "json.crossproc_guid",
                    "carbon_black_cloud.endpoint_event.crossproc.guid",
                )?;
            }

            if event.has_value("json.crossproc_name") {
                event.rename(
                    "json.crossproc_name",
                    "carbon_black_cloud.endpoint_event.crossproc.name",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.crossproc_target") {
                    if let Some(val) = event.get("json.crossproc_target") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.crossproc_target".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.endpoint_event.crossproc.target",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.crossproc_target").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.crossproc_target".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.crossproc_reputation") {
                event.rename(
                    "json.crossproc_reputation",
                    "carbon_black_cloud.endpoint_event.crossproc.reputation",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.crossproc_publisher") {
                    foreach_array(event, "json.crossproc_publisher", |event| {
                        if event.has_value("_ingest._value.state") {
                            if let Some(s) = event.get_string("_ingest._value.state") {
                                let mut parts: Vec<Value> = cached_regex!(" \\| ")
                                    .split(&s)
                                    .into_iter()
                                    .map(|p| json!(p))
                                    .collect();
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                                event.set("_ingest._value.state", Value::Array(parts))?;
                            }
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.crossproc_publisher") {
                event.rename(
                    "json.crossproc_publisher",
                    "carbon_black_cloud.endpoint_event.crossproc.publisher",
                )?;
            }

            if event.has_value("json.crossproc_hash_md5") {
                event.rename(
                    "json.crossproc_hash_md5",
                    "carbon_black_cloud.endpoint_event.crossproc.hash.md5",
                )?;
            }

            let _cond = { event.has_value("carbon_black_cloud.endpoint_event.crossproc.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.crossproc.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.crossproc_hash_sha256") {
                event.rename(
                    "json.crossproc_hash_sha256",
                    "carbon_black_cloud.endpoint_event.crossproc.hash.sha256",
                )?;
            }

            let _cond =
                { event.has_value("carbon_black_cloud.endpoint_event.crossproc.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.crossproc.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.filemod_hash_md5") {
                event.rename("json.filemod_hash_md5", "file.hash.md5")?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.filemod_hash_sha256") {
                event.rename("json.filemod_hash_sha256", "file.hash.sha256")?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.fileless_scriptload_cmdline") {
                event.rename(
                    "json.fileless_scriptload_cmdline",
                    "carbon_black_cloud.endpoint_event.fileless_scriptload.cmdline",
                )?;
            }

            if event.has_value("json.fileless_scriptload_cmdline_length") {
                event.rename(
                    "json.fileless_scriptload_cmdline_length",
                    "carbon_black_cloud.endpoint_event.fileless_scriptload.cmdline_length",
                )?;
            }

            if event.has_value("json.fileless_scriptload_hash_md5") {
                event.rename(
                    "json.fileless_scriptload_hash_md5",
                    "carbon_black_cloud.endpoint_event.fileless_scriptload.hash.md5",
                )?;
            }

            let _cond = {
                event.has_value("carbon_black_cloud.endpoint_event.fileless_scriptload.hash.md5")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get(
                                    "carbon_black_cloud.endpoint_event.fileless_scriptload.hash.md5"
                                )
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.fileless_scriptload_hash_sha256") {
                event.rename(
                    "json.fileless_scriptload_hash_sha256",
                    "carbon_black_cloud.endpoint_event.fileless_scriptload.hash.sha256",
                )?;
            }

            let _cond = {
                event.has_value("carbon_black_cloud.endpoint_event.fileless_scriptload.hash.sha256")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("related.hash", json!(event.get("carbon_black_cloud.endpoint_event.fileless_scriptload.hash.sha256").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })();
            }

            if event.has_value("json.modload_md5") {
                event.rename("json.modload_md5", "dll.hash.md5")?;
            }

            let _cond = { event.has_value("dll.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("dll.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.modload_sha256") {
                event.rename("json.modload_sha256", "dll.hash.sha256")?;
            }

            let _cond = { event.has_value("dll.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("dll.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.modload_effective_reputation") {
                event.rename(
                    "json.modload_effective_reputation",
                    "carbon_black_cloud.endpoint_event.modload.effective_reputation",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.modload_count") {
                    if let Some(val) = event.get("json.modload_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.modload_count".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.endpoint_event.modload.count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.modload_count").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.modload_count".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.modload_publisher") {
                    foreach_array(event, "json.modload_publisher", |event| {
                        if event.has_value("_ingest._value.state") {
                            if let Some(s) = event.get_string("_ingest._value.state") {
                                let mut parts: Vec<Value> = cached_regex!(" \\| ")
                                    .split(&s)
                                    .into_iter()
                                    .map(|p| json!(p))
                                    .collect();
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                                event.set("_ingest._value.state", Value::Array(parts))?;
                            }
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.modload_publisher") {
                event.rename(
                    "json.modload_publisher",
                    "carbon_black_cloud.endpoint_event.modload.publisher",
                )?;
            }

            if event.has_value("json.netconn_proxy_domain") {
                event.rename(
                    "json.netconn_proxy_domain",
                    "carbon_black_cloud.endpoint_event.netconn.proxy.domain",
                )?;
            }

            if event.has_value("json.netconn_proxy_port") {
                event.rename(
                    "json.netconn_proxy_port",
                    "carbon_black_cloud.endpoint_event.netconn.proxy.port",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_proxy_ip") {
                    if let Some(val) = event.get("json.netconn_proxy_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_proxy_ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.endpoint_event.netconn.proxy.ip",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_proxy_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_proxy_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            let _cond = { event.has_value("carbon_black_cloud.endpoint_event.netconn.proxy.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.netconn.proxy.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.scriptload_name") {
                event.rename(
                    "json.scriptload_name",
                    "carbon_black_cloud.endpoint_event.scriptload.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.scriptload_publisher") {
                    foreach_array(event, "json.scriptload_publisher", |event| {
                        if event.has_value("_ingest._value.state") {
                            if let Some(s) = event.get_string("_ingest._value.state") {
                                let mut parts: Vec<Value> = cached_regex!(" \\| ")
                                    .split(&s)
                                    .into_iter()
                                    .map(|p| json!(p))
                                    .collect();
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                                event.set("_ingest._value.state", Value::Array(parts))?;
                            }
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            if event.has_value("json.scriptload_publisher") {
                event.rename(
                    "json.scriptload_publisher",
                    "carbon_black_cloud.endpoint_event.scriptload.publisher",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.scriptload_count") {
                    if let Some(val) = event.get("json.scriptload_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.scriptload_count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.endpoint_event.scriptload.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.scriptload_count").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.scriptload_count".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.scriptload_hash_md5") {
                event.rename(
                    "json.scriptload_hash_md5",
                    "carbon_black_cloud.endpoint_event.scriptload.hash.md5",
                )?;
            }

            let _cond =
                { event.has_value("carbon_black_cloud.endpoint_event.scriptload.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.scriptload.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.scriptload_hash_sha256") {
                event.rename(
                    "json.scriptload_hash_sha256",
                    "carbon_black_cloud.endpoint_event.scriptload.hash.sha256",
                )?;
            }

            let _cond =
                { event.has_value("carbon_black_cloud.endpoint_event.scriptload.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.scriptload.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.scriptload_effective_reputation") {
                event.rename(
                    "json.scriptload_effective_reputation",
                    "carbon_black_cloud.endpoint_event.scriptload.effective_reputation",
                )?;
            }

            if event.has_value("json.scriptload_reputation") {
                event.rename(
                    "json.scriptload_reputation",
                    "carbon_black_cloud.endpoint_event.scriptload.reputation",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.device_internal_ip") {
                    if let Some(val) = event.get("json.device_internal_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.device_internal_ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.endpoint_event.device.internal_ip",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.device_internal_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.device_internal_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            let _cond = { event.has_value("carbon_black_cloud.endpoint_event.device.internal_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.device.internal_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("carbon_black_cloud.endpoint_event.device.internal_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.device.internal_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.device_external_ip") {
                    if let Some(val) = event.get("json.device_external_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.device_external_ip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.endpoint_event.device.external_ip",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.device_external_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.device_external_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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
                if event.has_value("json.schema") {
                    if let Some(val) = event.get("json.schema") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.schema".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.endpoint_event.schema", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.schema").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.schema".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            let _cond = { event.has_value("carbon_black_cloud.endpoint_event.device.external_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.device.external_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("carbon_black_cloud.endpoint_event.device.external_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.endpoint_event.device.external_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.remove("json.remote_port");
            event.remove("json.local_port");
            event.remove("json.process_pid");
            event.remove("json.parent_pid");
            event.remove("json.process_duration");
            event.remove("json.modload_count");
            event.remove("json.crossproc_target");
            event.remove("json.childproc_pid");
            event.remove("json.scriptload_count");
            event.remove("json.process_terminated");
            event.remove("json.create_time");
            event.remove("json.schema");
            event.remove("json.device_id");
            event.remove("json.process_hash");
            event.remove("json.parent_hash");
            event.remove("json.crossproc_hash");
            event.remove("json.filemod_hash");
            event.remove("json.childproc_hash");
            event.remove("json.modload_hash");
            event.remove("json.scriptload_hash");
            event.remove("json.netconn_domain");
            event.remove("json.netconn_inbound");
            event.remove("json.netconn_protocol");
            event.remove("json.remote_ip");
            event.remove("json.local_ip");
            event.remove("json.device_external_ip");
            event.remove("json.device_internal_ip");
            event.remove("json.netconn_proxy_ip");
            event.remove("json.device_timestamp");
            event.remove("_temp_");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.carbon_black_cloud.endpoint_event[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.carbon_black_cloud.endpoint_event[m.getKey()] = m.getValue();\n}\n"#
                    ),
                )?;
            }

            event.remove("json");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            // Painless script
            // Source: if (ctx?.related?.user != null) {\n  ctx.related.user = new HashSet(ctx.related.user)\n}\nif (ctx?.related?.ip != null) {\n  ctx.related.ip = new HashSet(ctx.related.ip)\n}\nif (ctx?.related?.hash != null) {\n  def hashes = new HashSet(ctx.related.hash);\n  def hash = new ArrayList();\n  for (def h: hashes) {\n    hash.add(h);\n  }\n  Collections.sort(hash);\n  ctx.related.hash = hash;\n}\nif (ctx.related?.hash != null) {\n  def hashes = new HashSet(ctx.related.hash);\n  def hash = new ArrayList();\n  for (def h: hashes) {\n    hash.add(h);\n  }\n  Collections.sort(hash);\n  ctx.related.hash = hash;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx?.related?.user != null) {\n  ctx.related.user = new HashSet(ctx.related.user)\n}\nif (ctx?.related?.ip != null) {\n  ctx.related.ip = new HashSet(ctx.related.ip)\n}\nif (ctx?.related?.hash != null) {\n  def hashes = new HashSet(ctx.related.hash);\n  def hash = new ArrayList();\n  for (def h: hashes) {\n    hash.add(h);\n  }\n  Collections.sort(hash);\n  ctx.related.hash = hash;\n}\nif (ctx.related?.hash != null) {\n  def hashes = new HashSet(ctx.related.hash);\n  def hash = new ArrayList();\n  for (def h: hashes) {\n    hash.add(h);\n  }\n  Collections.sort(hash);\n  ctx.related.hash = hash;\n}\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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
            }
        }

        Ok(TransformResult::Continue)
    }
}
