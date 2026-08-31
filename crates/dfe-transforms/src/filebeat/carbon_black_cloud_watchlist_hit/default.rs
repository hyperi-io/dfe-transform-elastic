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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.create_time") && event.get_str("json.create_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.create_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.create_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.create_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.create_time".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.severity") {
                    if let Some(val) = event.get("json.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severity".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.severity").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.severity".into(),
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
                        event.set("carbon_black_cloud.watchlist_hit.schema", converted)?;
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

            if event.has_value("json.device_os_version") {
                event.rename("json.device_os_version", "host.os.version")?;
            }

            if event.has_value("json.device_name") {
                event.rename("json.device_name", "host.hostname")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("host.hostname") {
                    if let Some(input) = event.get_string("host.hostname") {
                        // Grok pattern: ^(%{DATA:user.domain})\\\\(%{GREEDYDATA:host.hostname})$
                        if !cached_grok!("^(%{DATA:user.domain})\\\\(%{GREEDYDATA:host.hostname})$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
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

            if event.has_value("json.process_cmdline") {
                event.rename("json.process_cmdline", "process.command_line")?;
            }

            if event.has_value("json.process_guid") {
                event.rename("json.process_guid", "process.entity_id")?;
            }

            if event.has_value("json.process_path") {
                event.rename("json.process_path", "process.executable")?;
            }

            // on_failure: 2 handler(s)
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
                if event.remove("json.process_pid").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.process_pid".into(),
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

            if event.has_value("json.parent_cmdline") {
                event.rename("json.parent_cmdline", "process.parent.command_line")?;
            }

            if event.has_value("json.parent_guid") {
                event.rename("json.parent_guid", "process.parent.entity_id")?;
            }

            if event.has_value("json.parent_path") {
                event.rename("json.parent_path", "process.parent.executable")?;
            }

            // on_failure: 2 handler(s)
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
                if event.remove("json.parent_pid").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.parent_pid".into(),
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

            let _cond = { event.has_value("json.parent_username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("json.parent_username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.process_username") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("json.process_username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

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

            let _cond = { event.has_value("host.hostname") };
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

            // Painless script
            // Source: void mapHashField(def ctx, def hashes, def key) {\n    for (hash in hashes) {\n        if (hash.length() == 32) {ctx['json'][key + '_md5'] = hash;}\n        if (hash.length() == 64) {ctx['json'][key + '_sha256'] = hash;}\n    }\n}\nif (ctx.json?.process_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.process_hash, 'process_hash');\n}\nif (ctx.json?.parent_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.parent_hash, 'parent_hash');\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void mapHashField(def ctx, def hashes, def key) {\n    for (hash in hashes) {\n        if (hash.length() == 32) {ctx['json'][key + '_md5'] = hash;}\n        if (hash.length() == 64) {ctx['json'][key + '_sha256'] = hash;}\n    }\n}\nif (ctx.json?.process_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.process_hash, 'process_hash');\n}\nif (ctx.json?.parent_hash instanceof List) {\n    mapHashField(ctx, ctx.json?.parent_hash, 'parent_hash');\n}\n"#
                ),
            )?;

            if event.has_value("json.process_hash_md5") {
                event.rename("json.process_hash_md5", "process.hash.md5")?;
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

            if event.has_value("json.process_hash_sha256") {
                event.rename("json.process_hash_sha256", "process.hash.sha256")?;
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

            if event.has_value("json.parent_hash_md5") {
                event.rename("json.parent_hash_md5", "process.parent.hash.md5")?;
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

            if event.has_value("json.parent_hash_sha256") {
                event.rename("json.parent_hash_sha256", "process.parent.hash.sha256")?;
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

            if event.has_value("json.device_os") {
                event.rename(
                    "json.device_os",
                    "carbon_black_cloud.watchlist_hit.device.os",
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
                            "carbon_black_cloud.watchlist_hit.device.internal_ip",
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
                            "carbon_black_cloud.watchlist_hit.device.external_ip",
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

            let _cond = { event.has_value("carbon_black_cloud.watchlist_hit.device.internal_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.watchlist_hit.device.internal_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("carbon_black_cloud.watchlist_hit.device.external_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.watchlist_hit.device.external_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("carbon_black_cloud.watchlist_hit.device.internal_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.watchlist_hit.device.internal_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("carbon_black_cloud.watchlist_hit.device.external_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("carbon_black_cloud.watchlist_hit.device.external_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.ioc_hit") {
                event.rename("json.ioc_hit", "carbon_black_cloud.watchlist_hit.ioc.hit")?;
            }

            if event.has_value("json.ioc_id") {
                event.rename("json.ioc_id", "carbon_black_cloud.watchlist_hit.ioc.id")?;
            }

            if event.has_value("json.org_key") {
                event.rename(
                    "json.org_key",
                    "carbon_black_cloud.watchlist_hit.organization_key",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.parent_publisher") {
                    foreach_array(event, "json.parent_publisher", |event| {
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

            if event.has_value("json.parent_publisher") {
                event.rename(
                    "json.parent_publisher",
                    "carbon_black_cloud.watchlist_hit.process.parent.publisher",
                )?;
            }

            if event.has_value("json.parent_reputation") {
                event.rename(
                    "json.parent_reputation",
                    "carbon_black_cloud.watchlist_hit.process.parent.reputation",
                )?;
            }

            if event.has_value("json.parent_username") {
                event.rename(
                    "json.parent_username",
                    "carbon_black_cloud.watchlist_hit.process.parent.username",
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
                    "carbon_black_cloud.watchlist_hit.process.publisher",
                )?;
            }

            if event.has_value("json.process_reputation") {
                event.rename(
                    "json.process_reputation",
                    "carbon_black_cloud.watchlist_hit.process.reputation",
                )?;
            }

            if event.has_value("json.process_username") {
                event.rename(
                    "json.process_username",
                    "carbon_black_cloud.watchlist_hit.process.username",
                )?;
            }

            if event.has_value("json.report_id") {
                event.rename(
                    "json.report_id",
                    "carbon_black_cloud.watchlist_hit.report.id",
                )?;
            }

            if event.has_value("json.report_name") {
                event.rename(
                    "json.report_name",
                    "carbon_black_cloud.watchlist_hit.report.name",
                )?;
            }

            if event.has_value("json.report_tags") {
                event.rename(
                    "json.report_tags",
                    "carbon_black_cloud.watchlist_hit.report.tags",
                )?;
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            event.remove("json.process_pid");
            event.remove("json.parent_pid");
            event.remove("json.severity");
            event.remove("json.create_time");
            event.remove("json.device_id");
            event.remove("json.schema");
            event.remove("json.process_hash");
            event.remove("json.parent_hash");
            event.remove("json.device_external_ip");
            event.remove("json.device_internal_ip");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.carbon_black_cloud.watchlist_hit[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.carbon_black_cloud.watchlist_hit[m.getKey()] = m.getValue();\n}\n"#
                    ),
                )?;
            }

            event.remove("json");

            // Painless script
            // Source: if (ctx?.related?.user != null) {\n  ctx.related.user = new HashSet(ctx.related.user)\n}\nif (ctx?.related?.hash != null) {\n  def hashes = new HashSet(ctx.related.hash);\n  def hash = new ArrayList();\n  for (def h: hashes) {\n    hash.add(h);\n  }\n  Collections.sort(hash);\n  ctx.related.hash = hash;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx?.related?.user != null) {\n  ctx.related.user = new HashSet(ctx.related.user)\n}\nif (ctx?.related?.hash != null) {\n  def hashes = new HashSet(ctx.related.hash);\n  def hash = new ArrayList();\n  for (def h: hashes) {\n    hash.add(h);\n  }\n  Collections.sort(hash);\n  ctx.related.hash = hash;\n}\n"#
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
