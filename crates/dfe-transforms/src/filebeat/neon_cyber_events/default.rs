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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

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

            parse_json_field(event, "event.original", "json")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.id".into(),
                    });
                }
                if let Some(v) = event.get("json.inserted_at") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.inserted_at".into(),
                    });
                }
                if let Some(v) = event.get("json.updated_at") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.updated_at".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("neon_cyber.events.event_timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event_timestamp".into(),
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
                    "date_event_timestamp_bc171e42",
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
                if let Some(date_str) = event.get_as_string("json.event_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event_timestamp".into(),
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
                    "date_event_timestamp_28033f70",
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

            event.rename("json.id", "neon_cyber.events.id")?;

            event.rename("json.deployment_id", "neon_cyber.events.deployment_id")?;

            event.rename("json.client_id", "neon_cyber.events.client_id")?;

            event.rename("json.registration_id", "neon_cyber.events.registration_id")?;

            event.rename("json.event_type", "neon_cyber.events.event_type")?;

            if event.has_value("json.arch") {
                event.rename("json.arch", "host.architecture")?;
            }

            if event.has_value("json.os") {
                event.rename("json.os", "host.os.platform")?;
            }

            if event.has_value("json.latitude") {
                event.rename("json.latitude", "host.geo.location.lat")?;
            }

            if event.has_value("json.longitude") {
                event.rename("json.longitude", "host.geo.location.lon")?;
            }

            if event.has_value("json.ip") {
                event.rename("json.ip", "client.nat.ip")?;
            }

            if event.has_value("json.ip_latitude") {
                event.rename("json.ip_latitude", "client.geo.location.lat")?;
            }

            if event.has_value("json.ip_longitude") {
                event.rename("json.ip_longitude", "client.geo.location.lon")?;
            }

            if event.has_value("json.city") {
                event.rename("json.city", "client.geo.city_name")?;
            }

            if event.has_value("json.country") {
                event.rename("json.country", "client.geo.country_name")?;
            }

            if event.has_value("json.postal_code") {
                event.rename("json.postal_code", "client.geo.postal_code")?;
            }

            if event.has_value("json.region_code") {
                event.rename("json.region_code", "client.geo.region_iso_code")?;
            }

            if event.has_value("json.region_name") {
                event.rename("json.region_name", "client.geo.region_name")?;
            }

            if event.has_value("json.asn") {
                event.rename("json.asn", "client.as.number")?;
            }

            if event.has_value("json.asn_isp") {
                event.rename("json.asn_isp", "client.as.organization.name")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "user_agent.name")?;
            }

            if event.has_value("json.version") {
                event.rename("json.version", "user_agent.version")?;
            }

            if event.has_value("json.ua") {
                event.rename("json.ua", "user_agent.original")?;
            }

            if event.has_value("json.url") {
                event.rename("json.url", "neon_cyber.events.url")?;
            }

            if event.has_value("neon_cyber.detections.url") {
                uri_parts(event, "neon_cyber.detections.url", "url", true, false)?;
            }

            if event.has_value("json.agent") {
                event.rename("json.agent", "neon_cyber.events.agent")?;
            }

            if event.has_value("json.display") {
                event.rename("json.display", "neon_cyber.events.display")?;
            }

            if event.has_value("json.tab_id") {
                event.rename("json.tab_id", "neon_cyber.events.tab_id")?;
            }

            if event.has_value("json.frame_id") {
                event.rename("json.frame_id", "neon_cyber.events.frame_id")?;
            }

            if event.has_value("json.parent_frame_id") {
                event.rename("json.parent_frame_id", "neon_cyber.events.parent_frame_id")?;
            }

            if event.has_value("json.download_id") {
                event.rename("json.download_id", "neon_cyber.events.download_id")?;
            }

            if event.has_value("json.filename") {
                event.rename("json.filename", "neon_cyber.events.filename")?;
            }

            if event.has_value("json.mime") {
                event.rename("json.mime", "neon_cyber.events.mime")?;
            }

            if event.has_value("json.incognito") {
                event.rename("json.incognito", "neon_cyber.events.incognito")?;
            }

            if event.has_value("json.referrer") {
                event.rename("json.referrer", "neon_cyber.events.referrer")?;
            }

            if event.has_value("json.danger") {
                event.rename("json.danger", "neon_cyber.events.danger")?;
            }

            if event.has_value("json.total_bytes") {
                event.rename("json.total_bytes", "neon_cyber.events.total_bytes")?;
            }

            if event.has_value("json.description") {
                event.rename("json.description", "neon_cyber.events.description")?;
            }

            if event.has_value("json.catalog_id") {
                event.rename("json.catalog_id", "neon_cyber.events.catalog_id")?;
            }

            if event.has_value("json.catalog_name") {
                event.rename("json.catalog_name", "neon_cyber.events.catalog_name")?;
            }

            if event.has_value("json.domains") {
                event.rename("json.domains", "neon_cyber.events.domains")?;
            }

            let _cond = { event.has_value("json.start_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.start_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("neon_cyber.events.start_timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.start_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.end_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.end_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("neon_cyber.events.end_timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.end_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.cumulative") {
                event.rename("json.cumulative", "neon_cyber.events.cumulative")?;
            }

            if event.has_value("json.auth_method") {
                event.rename("json.auth_method", "neon_cyber.events.auth_method")?;
            }

            if event.has_value("json.login") {
                event.rename("json.login", "neon_cyber.events.login")?;
            }

            if event.has_value("json.mfa") {
                event.rename("json.mfa", "neon_cyber.events.mfa")?;
            }

            if event.has_value("json.email") {
                event.rename("json.email", "neon_cyber.events.email")?;
            }

            if event.has_value("json.autofill") {
                event.rename("json.autofill", "neon_cyber.events.autofill")?;
            }

            if event.has_value("json.filehash") {
                event.rename("json.filehash", "neon_cyber.events.filehash")?;
            }

            let _cond = { event.has_value("json.extensions") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("json.extensions", "neon_cyber.events.extensions")?;
                    Ok(())
                })();
            }

            if event.has_value("json.ext_id") {
                event.rename("json.ext_id", "neon_cyber.events.ext_id")?;
            }

            if event.has_value("json.ext_name") {
                event.rename("json.ext_name", "neon_cyber.events.ext_name")?;
            }

            if event.has_value("json.ext_enabled") {
                event.rename("json.ext_enabled", "neon_cyber.events.ext_enabled")?;
            }

            if event.has_value("json.ext_description") {
                event.rename("json.ext_description", "neon_cyber.events.ext_description")?;
            }

            if event.has_value("json.ext_install_type") {
                event.rename(
                    "json.ext_install_type",
                    "neon_cyber.events.ext_install_type",
                )?;
            }

            if event.has_value("json.ext_host_permissions") {
                event.rename(
                    "json.ext_host_permissions",
                    "neon_cyber.events.ext_host_permissions",
                )?;
            }

            if event.has_value("json.ext_permissions") {
                event.rename("json.ext_permissions", "neon_cyber.events.ext_permissions")?;
            }

            if event.remove("json.event_timestamp").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json.event_timestamp".into(),
                });
            }

            if event.remove("json.inserted_at").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json.inserted_at".into(),
                });
            }

            if event.remove("json.updated_at").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json.updated_at".into(),
                });
            }

            let _cond = { event.has_value("json.start_timestamp") };
            if _cond {
                if event.remove("json.start_timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.start_timestamp".into(),
                    });
                }
            }

            let _cond = { event.has_value("json.end_timestamp") };
            if _cond {
                if event.remove("json.end_timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.end_timestamp".into(),
                    });
                }
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n    if (object == null || object == '') {\n    return true;\n    } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n    } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n    }\n    return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n    if (object == null || object == '') {\n    return true;\n    } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n    } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n    }\n    return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
