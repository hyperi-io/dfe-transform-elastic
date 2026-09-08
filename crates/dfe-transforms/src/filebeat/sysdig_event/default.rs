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
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == '<NA>' || v == '{}' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == '<NA>' || v == '{}' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx.json);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        empty_collections: true,
                        prune_lists: true,
                        sentinels: vec!["<NA>".into(), "{}".into()],
                        ..DropPolicy::none()
                    },
                    Some("json"),
                );
            }

            event.set("observer.vendor", json!("Sysdig"))?;

            event.set("observer.product", json!("Sysdig Secure"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.afterEventNs") {
                            if let Some(val) = event.get("_ingest._value.afterEventNs") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.afterEventNs".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.after_event_ns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_actions_afterEventNs_to_long",
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.beforeEventNs") {
                            if let Some(val) = event.get("_ingest._value.beforeEventNs") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.beforeEventNs".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.before_event_ns", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_actions_beforeEventNs_to_long",
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.isSuccessful") {
                            if let Some(val) = event.get("_ingest._value.isSuccessful") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.isSuccessful".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_successful", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_actions_isSuccessful_to_boolean",
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    if event.has_value("_ingest._value.errMsg") {
                        event.rename("_ingest._value.errMsg", "_ingest._value.err_msg")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.actions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.actions", |event| {
                    event.remove("_ingest._value.afterEventNs");
                    event.remove("_ingest._value.beforeEventNs");
                    event.remove("_ingest._value.isSuccessful");
                    Ok(())
                })?;
            }

            if event.has_value("json.actions") {
                event.rename("json.actions", "sysdig.event.actions")?;
            }

            if event.has_value("json.category") {
                event.rename("json.category", "sysdig.event.category")?;
            }

            if event.has_value("json.content.clusterName") {
                event.rename(
                    "json.content.clusterName",
                    "sysdig.event.content.cluster_name",
                )?;
            }

            if event.has_value("json.content.command") {
                event.rename("json.content.command", "sysdig.event.content.command")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.content.detectedClassProbability") {
                    if let Some(val) = event.get("json.content.detectedClassProbability") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.content.detectedClassProbability".into(),
                                message,
                            }
                        })?;
                        event.set("sysdig.event.content.detected_class_probability", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_content_detectedClassProbability_to_double",
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

            if event.has_value("json.content.exe") {
                event.rename("json.content.exe", "sysdig.event.content.exe")?;
            }

            if event.has_value("json.content.fields") {
                event.rename("json.content.fields", "sysdig.event.content.fields")?;
            }

            let _cond = {
                event
                    .get("sysdig.event.content.fields")
                    .is_some_and(|v| v.is_object())
                    && event
                        .get("sysdig.event.content.fields")
                        .and_then(|v| v.get("proc.pid.ts"))
                        .is_some_and(|v| !v.is_null())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    dot_expand(event, "sysdig.event.content.fields", "proc.pid.ts")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dot_expander")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dot_expander_proc_pid_ts",
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
            }

            if event.has_value("sysdig.event.content.fields.proc.pid.ts") {
                event.rename(
                    "sysdig.event.content.fields.proc.pid.ts",
                    "sysdig.event.content.fields.proc.pid_ts",
                )?;
            }

            let _cond = {
                event
                    .get("sysdig.event.content.fields")
                    .is_some_and(|v| v.is_object())
                    && event
                        .get("sysdig.event.content.fields")
                        .and_then(|v| v.get("proc.ppid.ts"))
                        .is_some_and(|v| !v.is_null())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    dot_expand(event, "sysdig.event.content.fields", "proc.ppid.ts")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dot_expander")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dot_expander_proc_ppid_ts",
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
            }

            if event.has_value("sysdig.event.content.fields.proc.ppid.ts") {
                event.rename(
                    "sysdig.event.content.fields.proc.ppid.ts",
                    "sysdig.event.content.fields.proc.ppid_ts",
                )?;
            }

            let _cond = {
                event
                    .get("sysdig.event.content.fields")
                    .is_some_and(|v| v.is_object())
                    && (event
                        .get("sysdig.event.content.fields")
                        .and_then(|v| v.get("proc.pid.ts"))
                        .is_some_and(|v| !v.is_null())
                        || event
                            .get("sysdig.event.content.fields")
                            .and_then(|v| v.get("proc.ppid.ts"))
                            .is_some_and(|v| !v.is_null())
                        || event.has_value("sysdig.event.content.fields.proc.pid_ts")
                        || event.has_value("sysdig.event.content.fields.proc.ppid_ts"))
            };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: if (ctx.sysdig.event.content.fields.proc instanceof Map) {\n  def proc = ctx.sysdig.event.content.fields.proc;\n  if (proc.containsKey('pid') && proc.pid instanceof Map && proc.pid.size() == 0) {\n    proc.remove('pid');\n  }\n  if (proc.containsKey('ppid') && proc.ppid instanceof Map && proc.ppid.size() == 0) {\n    proc.remove('ppid');\n  }\n}\n
                remove_empty_child_maps(
                    event,
                    &RemoveEmptyChildMaps::new(
                        "sysdig.event.content.fields.proc".into(),
                        vec!["pid".into(), "ppid".into()],
                    ),
                );
            }

            let _cond = {
                event
                    .get("sysdig.event.content.fields")
                    .is_some_and(|v| v.is_object())
                    && event
                        .get("sysdig.event.content.fields")
                        .and_then(|v| v.get("ct.user"))
                        .is_some_and(|v| !v.is_null())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    dot_expand(event, "sysdig.event.content.fields", "ct.user")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dot_expander")?;
                    event.set("_ingest.on_failure_processor_tag", "dot_expander_ct_user")?;
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

            if event.has_value("sysdig.event.content.fields.ct.user") {
                event.rename(
                    "sysdig.event.content.fields.ct.user",
                    "sysdig.event.content.fields.ct.user.value",
                )?;
            }

            let _cond = {
                event
                    .get("sysdig.event.content.fields")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    dot_expand(event, "sysdig.event.content.fields", "*")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dot_expander")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dot_expander_content_fields",
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
            }

            if event.has_value("sysdig.event.content.fields.aws.accountId") {
                if let Some(val) = event.get("sysdig.event.content.fields.aws.accountId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.content.fields.aws.accountId".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.fields.aws.account_id", converted)?;
                }
            }

            if event.has_value("sysdig.event.content.fields.aws.eventName") {
                event.rename(
                    "sysdig.event.content.fields.aws.eventName",
                    "sysdig.event.content.fields.aws.event_name",
                )?;
            }

            let _cond = { event.get_str("sysdig.event.content.fields.aws.source_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sysdig.event.content.fields.aws.sourceIP") {
                        if let Some(val) = event.get("sysdig.event.content.fields.aws.sourceIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sysdig.event.content.fields.aws.sourceIP".into(),
                                    message,
                                }
                            })?;
                            event.set("sysdig.event.content.fields.aws.source_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_content_fields_aws_sourceIP_to_ip",
                    )?;
                    if event.has_value("sysdig.event.content.fields.aws.sourceIP") {
                        event.rename(
                            "sysdig.event.content.fields.aws.sourceIP",
                            "sysdig.event.content.fields.aws.source_domain",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("sysdig.event.content.fields.aws.source_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sysdig.event.content.fields.aws.source_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.aws.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.content.fields.aws.user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.container.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.id", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.container.image.repository")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.image.name", v)?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.container.image.tag") };
            if _cond {
                event.append_unique(
                    "container.image.tag",
                    json!(
                        event
                            .get("sysdig.event.content.fields.container.image.tag")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.container.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sysdig.event.content.fields.container.privileged") {
                    if let Some(val) = event.get("sysdig.event.content.fields.container.privileged")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "sysdig.event.content.fields.container.privileged".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sysdig.event.content.fields.container.privileged",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_content_fields_container_privileged_to_boolean",
                )?;
                if event
                    .remove("sysdig.event.content.fields.container.privileged")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "sysdig.event.content.fields.container.privileged".into(),
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

            if let Some(v) = event
                .get("sysdig.event.content.fields.container.privileged")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.security_context.privileged", v)?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.ct.request.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sysdig.event.content.fields.ct.request.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("sysdig.event.content.fields.ct.srcip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sysdig.event.content.fields.ct.srcip") {
                        if let Some(val) = event.get("sysdig.event.content.fields.ct.srcip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sysdig.event.content.fields.ct.srcip".into(),
                                    message,
                                }
                            })?;
                            event.set("sysdig.event.content.fields.ct.srcip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_content_fields_ct_srcip_to_ip",
                    )?;
                    if event.has_value("sysdig.event.content.fields.ct.srcip") {
                        event.rename(
                            "sysdig.event.content.fields.ct.srcip",
                            "sysdig.event.content.fields.ct.srcdomain",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("sysdig.event.content.fields.ct.srcip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sysdig.event.content.fields.ct.srcip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.ct.user.value") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.content.fields.ct.user.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("sysdig.event.content.fields.ct.user.accountid") {
                if let Some(val) = event.get("sysdig.event.content.fields.ct.user.accountid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.content.fields.ct.user.accountid".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.fields.ct.user.accountid", converted)?;
                }
            }

            let _cond = { event.has_value("sysdig.event.content.fields.ct.user.accountid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.content.fields.ct.user.accountid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("sysdig.event.content.fields.evt.res")
                    && event
                        .get_str("sysdig.event.content.fields.evt.res")
                        .is_some_and(|s| s.eq_ignore_ascii_case("success"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("sysdig.event.content.fields.evt.res")
                    && event
                        .get_str("sysdig.event.content.fields.evt.res")
                        .is_some_and(|s| s.eq_ignore_ascii_case("failure"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("sysdig.event.content.fields.fd.sip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sysdig.event.content.fields.fd.sip") {
                        if let Some(val) = event.get("sysdig.event.content.fields.fd.sip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sysdig.event.content.fields.fd.sip".into(),
                                    message,
                                }
                            })?;
                            event.set("sysdig.event.content.fields.fd.sip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_content_fields_fd_sip_to_ip",
                    )?;
                    if event.remove("sysdig.event.content.fields.fd.sip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "sysdig.event.content.fields.fd.sip".into(),
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

            let _cond = { event.has_value("sysdig.event.content.fields.fd.sip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sysdig.event.content.fields.fd.sip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sysdig.event.content.fields.fd.sport") {
                    if let Some(val) = event.get("sysdig.event.content.fields.fd.sport") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sysdig.event.content.fields.fd.sport".into(),
                                message,
                            }
                        })?;
                        event.set("sysdig.event.content.fields.fd.sport", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_content_fields_fd_sport_to_long",
                )?;
                if event
                    .remove("sysdig.event.content.fields.fd.sport")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "sysdig.event.content.fields.fd.sport".into(),
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

            if event.has_value("sysdig.event.content.fields.group.gid") {
                if let Some(val) = event.get("sysdig.event.content.fields.group.gid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.content.fields.group.gid".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.fields.group.gid", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.group.gid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.id", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.group.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.name", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.acmdline[2]") {
                event.rename(
                    "sysdig.event.content.fields.proc.acmdline[2]",
                    "sysdig.event.content.fields.proc.acmdline_2",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.acmdline_2")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.command_line", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.acmdline[3]") {
                event.rename(
                    "sysdig.event.content.fields.proc.acmdline[3]",
                    "sysdig.event.content.fields.proc.acmdline_3",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.acmdline_3")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.parent.command_line", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.acmdline[4]") {
                event.rename(
                    "sysdig.event.content.fields.proc.acmdline[4]",
                    "sysdig.event.content.fields.proc.acmdline_4",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.acmdline_4")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.parent.parent.command_line", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.aexepath[2]") {
                event.rename(
                    "sysdig.event.content.fields.proc.aexepath[2]",
                    "sysdig.event.content.fields.proc.aexepath_2",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.aexepath_2")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.executable", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.aexepath[3]") {
                event.rename(
                    "sysdig.event.content.fields.proc.aexepath[3]",
                    "sysdig.event.content.fields.proc.aexepath_3",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.aexepath_3")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.parent.executable", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.aexepath[4]") {
                event.rename(
                    "sysdig.event.content.fields.proc.aexepath[4]",
                    "sysdig.event.content.fields.proc.aexepath_4",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.aexepath_4")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.parent.parent.executable", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.aname[2]") {
                event.rename(
                    "sysdig.event.content.fields.proc.aname[2]",
                    "sysdig.event.content.fields.proc.aname_2",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.aname_2")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.name", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.aname[3]") {
                event.rename(
                    "sysdig.event.content.fields.proc.aname[3]",
                    "sysdig.event.content.fields.proc.aname_3",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.aname_3")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.parent.name", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.aname[4]") {
                event.rename(
                    "sysdig.event.content.fields.proc.aname[4]",
                    "sysdig.event.content.fields.proc.aname_4",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.aname_4")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.parent.parent.parent.name", v)?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.proc.args") };
            if _cond {
                event.append_unique(
                    "process.args",
                    json!(
                        event
                            .get("sysdig.event.content.fields.proc.args")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.cmdline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.cwd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.working_directory", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.exepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.proc.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sysdig.event.content.fields.proc.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.pcmdline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.pexepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sysdig.event.content.fields.proc.pid") {
                    if let Some(val) = event.get("sysdig.event.content.fields.proc.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sysdig.event.content.fields.proc.pid".into(),
                                message,
                            }
                        })?;
                        event.set("sysdig.event.content.fields.proc.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_content_fields_proc_pid_to_long",
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

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def parseDate(def rawtimestamp) {\n  long timestamp;\n  if (rawtimestamp instanceof String) {\n    timestamp = Long.parseLong(rawtimestamp);\n  } else if (rawtimestamp instanceof long) {\n    timestamp = (long) rawtimestamp;\n  }\n  if (String.valueOf(timestamp).length() == 19) {\n    long epoch = timestamp / 1000000000L;\n    long seconds = timestamp % 1000000000L;\n    return Instant.ofEpochSecond(epoch, seconds).atZone(ZoneOffset.UTC);\n  }\n  return '';\n} if (ctx.json?.timestamp != null) {\n  ctx.json.timestamp = parseDate(ctx.json.timestamp);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.pid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.pid_ts = parseDate(ctx.sysdig.event.content.fields.proc.pid_ts);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.ppid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.ppid_ts = parseDate(ctx.sysdig.event.content.fields.proc.ppid_ts);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def parseDate(def rawtimestamp) {\n  long timestamp;\n  if (rawtimestamp instanceof String) {\n    timestamp = Long.parseLong(rawtimestamp);\n  } else if (rawtimestamp instanceof long) {\n    timestamp = (long) rawtimestamp;\n  }\n  if (String.valueOf(timestamp).length() == 19) {\n    long epoch = timestamp / 1000000000L;\n    long seconds = timestamp % 1000000000L;\n    return Instant.ofEpochSecond(epoch, seconds).atZone(ZoneOffset.UTC);\n  }\n  return '';\n} if (ctx.json?.timestamp != null) {\n  ctx.json.timestamp = parseDate(ctx.json.timestamp);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.pid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.pid_ts = parseDate(ctx.sysdig.event.content.fields.proc.pid_ts);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.ppid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.ppid_ts = parseDate(ctx.sysdig.event.content.fields.proc.ppid_ts);\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_parse_date_fields",
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

            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            None,
                            Some("strict_date_optional_time_nanos"),
                        ) {
                            Some(parsed) => event.set("sysdig.event.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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
                .get("sysdig.event.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("sysdig.event.content.fields.proc.pid_ts")
                    && event.get_str("sysdig.event.content.fields.proc.pid_ts") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sysdig.event.content.fields.proc.pid_ts")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            None,
                            Some("strict_date_optional_time_nanos"),
                        ) {
                            Some(parsed) => {
                                event.set("sysdig.event.content.fields.proc.pid_ts", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sysdig.event.content.fields.proc.pid_ts".into(),
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
                        "date_content_fields_proc_pid_ts",
                    )?;
                    if event
                        .remove("sysdig.event.content.fields.proc.pid_ts")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "sysdig.event.content.fields.proc.pid_ts".into(),
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
                .get("sysdig.event.content.fields.proc.pid_ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.start", v)?;
            }

            let _cond = {
                event.has_value("sysdig.event.content.fields.proc.ppid_ts")
                    && event.get_str("sysdig.event.content.fields.proc.ppid_ts") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sysdig.event.content.fields.proc.ppid_ts")
                    {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            None,
                            Some("strict_date_optional_time_nanos"),
                        ) {
                            Some(parsed) => {
                                event.set("sysdig.event.content.fields.proc.ppid_ts", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sysdig.event.content.fields.proc.ppid_ts".into(),
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
                        "date_content_fields_proc_ppid_ts",
                    )?;
                    if event
                        .remove("sysdig.event.content.fields.proc.ppid_ts")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "sysdig.event.content.fields.proc.ppid_ts".into(),
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
                .get("sysdig.event.content.fields.proc.ppid_ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.start", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.pname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sysdig.event.content.fields.proc.ppid") {
                    if let Some(val) = event.get("sysdig.event.content.fields.proc.ppid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sysdig.event.content.fields.proc.ppid".into(),
                                message,
                            }
                        })?;
                        event.set("sysdig.event.content.fields.proc.ppid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_content_fields_proc_ppid_to_long",
                )?;
                if event
                    .remove("sysdig.event.content.fields.proc.ppid")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "sysdig.event.content.fields.proc.ppid".into(),
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

            if let Some(v) = event
                .get("sysdig.event.content.fields.proc.ppid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            if event.has_value("sysdig.event.content.fields.proc.sid") {
                if let Some(val) = event.get("sysdig.event.content.fields.proc.sid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.content.fields.proc.sid".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.fields.proc.sid", converted)?;
                }
            }

            let _cond = { event.has_value("sysdig.event.content.fields.user.loginname") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.content.fields.user.loginname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("sysdig.event.content.fields.user.loginuid") {
                if let Some(val) = event.get("sysdig.event.content.fields.user.loginuid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.content.fields.user.loginuid".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.fields.user.loginuid", converted)?;
                }
            }

            let _cond = { event.has_value("sysdig.event.content.fields.user.loginuid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.content.fields.user.loginuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.content.fields.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("sysdig.event.content.fields.user.uid") {
                if let Some(val) = event.get("sysdig.event.content.fields.user.uid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.content.fields.user.uid".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.fields.user.uid", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.event.content.fields.user.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("sysdig.event.content.fields.user.uid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.content.fields.user.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.content.integrationId") {
                event.rename(
                    "json.content.integrationId",
                    "sysdig.event.content.integration_id",
                )?;
            }

            if event.has_value("json.content.integrationType") {
                event.rename(
                    "json.content.integrationType",
                    "sysdig.event.content.integration_type",
                )?;
            }

            if event.has_value("json.content.namespace") {
                event.rename("json.content.namespace", "sysdig.event.content.namespace")?;
            }

            if event.has_value("json.content.origin") {
                event.rename("json.content.origin", "sysdig.event.content.origin")?;
            }

            if event.has_value("json.content.output") {
                event.rename("json.content.output", "sysdig.event.content.output")?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.output")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.content.policyId") {
                if let Some(val) = event.get("json.content.policyId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.content.policyId".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.policy_id", converted)?;
                }
            }

            if event.has_value("json.content.policyNotificationChannelIds") {
                if let Some(val) = event.get("json.content.policyNotificationChannelIds") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.content.policyNotificationChannelIds".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "sysdig.event.content.policy_notification_channel_ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.content.policyOrigin") {
                event.rename(
                    "json.content.policyOrigin",
                    "sysdig.event.content.policy_origin",
                )?;
            }

            let _cond = { event.has_value("sysdig.event.content.policy_origin") };
            if _cond {
                event.append_unique(
                    "rule.author",
                    json!(
                        event
                            .get("sysdig.event.content.policy_origin")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.content.policyVersion") {
                if let Some(val) = event.get("json.content.policyVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.content.policyVersion".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.content.policy_version", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.event.content.policy_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.version", v)?;
            }

            if event.has_value("json.content.priority") {
                event.rename("json.content.priority", "sysdig.event.content.priority")?;
            }

            if event.has_value("json.content.resourceKind") {
                event.rename(
                    "json.content.resourceKind",
                    "sysdig.event.content.resource_kind",
                )?;
            }

            if event.has_value("json.content.resourceName") {
                event.rename(
                    "json.content.resourceName",
                    "sysdig.event.content.resource_name",
                )?;
            }

            if event.has_value("json.content.ruleName") {
                event.rename("json.content.ruleName", "sysdig.event.content.rule_name")?;
            }

            if let Some(v) = event
                .get("sysdig.event.content.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.content.ruleSubType") {
                    if let Some(val) = event.get("json.content.ruleSubType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.content.ruleSubType".into(),
                                message,
                            }
                        })?;
                        event.set("sysdig.event.content.rule_sub_type", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_content_ruleSubType_to_long",
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

            if event.has_value("json.content.ruleTags") {
                event.rename("json.content.ruleTags", "sysdig.event.content.rule_tags")?;
            }

            let _cond = {
                event
                    .get("sysdig.event.content.rule_tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.threat = ctx.threat ?: [:]; ctx.threat.tactic = ctx.threat.tactic ?: [:]; ctx.threat.technique = ctx.threat.technique ?: [:]; ctx.threat.technique.subtechnique = ctx.threat.technique.subtechnique ?: [:]; ctx.threat.tactic.id = ctx.threat.tactic.id ?: []; ctx.threat.technique.id = ctx.threat.technique.id ?: []; ctx.threat.technique.subtechnique.id = ctx.threat.technique.subtechnique.id ?: []; ctx.threat.tactic.name = ctx.threat.tactic.name ?: []; ctx.threat.technique.name = ctx.threat.technique.name ?: []; ctx.threat.technique.subtechnique.name = ctx.threat.technique.subtechnique.name ?: []; Pattern pattern = /MITRE_(T[A-Z]\\d{4})_(\\w+)|MITRE_(T\\d{4}(?:\\.\\d{3})?)_(\\w+)/; for (int i = 0; i < ctx.sysdig.event.content.rule_tags.size(); i++) {\n  def tag = ctx.sysdig.event.content.rule_tags[i];\n  def matcher = pattern.matcher(tag);\n  if (matcher.find()) {\n    if (matcher.group(1) != null && matcher.group(2) != null) {\n      ctx.threat.tactic.id.add(matcher.group(1));\n      ctx.threat.tactic.name.add(matcher.group(2));\n    } else if (matcher.group(3) != null && matcher.group(4) != null) {\n      if (matcher.group(3).contains('.')) {\n        ctx.threat.technique.subtechnique.id.add(matcher.group(3));\n        ctx.threat.technique.subtechnique.name.add(matcher.group(4));\n      } else {\n        ctx.threat.technique.id.add(matcher.group(3));\n        ctx.threat.technique.name.add(matcher.group(4));\n      }\n    }\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.threat = ctx.threat ?: [:]; ctx.threat.tactic = ctx.threat.tactic ?: [:]; ctx.threat.technique = ctx.threat.technique ?: [:]; ctx.threat.technique.subtechnique = ctx.threat.technique.subtechnique ?: [:]; ctx.threat.tactic.id = ctx.threat.tactic.id ?: []; ctx.threat.technique.id = ctx.threat.technique.id ?: []; ctx.threat.technique.subtechnique.id = ctx.threat.technique.subtechnique.id ?: []; ctx.threat.tactic.name = ctx.threat.tactic.name ?: []; ctx.threat.technique.name = ctx.threat.technique.name ?: []; ctx.threat.technique.subtechnique.name = ctx.threat.technique.subtechnique.name ?: []; Pattern pattern = /MITRE_(T[A-Z]\\d{4})_(\\w+)|MITRE_(T\\d{4}(?:\\.\\d{3})?)_(\\w+)/; for (int i = 0; i < ctx.sysdig.event.content.rule_tags.size(); i++) {\n  def tag = ctx.sysdig.event.content.rule_tags[i];\n  def matcher = pattern.matcher(tag);\n  if (matcher.find()) {\n    if (matcher.group(1) != null && matcher.group(2) != null) {\n      ctx.threat.tactic.id.add(matcher.group(1));\n      ctx.threat.tactic.name.add(matcher.group(2));\n    } else if (matcher.group(3) != null && matcher.group(4) != null) {\n      if (matcher.group(3).contains('.')) {\n        ctx.threat.technique.subtechnique.id.add(matcher.group(3));\n        ctx.threat.technique.subtechnique.name.add(matcher.group(4));\n      } else {\n        ctx.threat.technique.id.add(matcher.group(3));\n        ctx.threat.technique.name.add(matcher.group(4));\n      }\n    }\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_set_threat_*")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.content.ruleType") {
                    if let Some(val) = event.get("json.content.ruleType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.content.ruleType".into(),
                                message,
                            }
                        })?;
                        event.set("sysdig.event.content.rule_type", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_content_ruleType_to_long",
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

            if event.has_value("json.content.runBook") {
                event.rename("json.content.runBook", "sysdig.event.content.run_book")?;
            }

            if event.has_value("json.content.scanResult") {
                event.rename(
                    "json.content.scanResult",
                    "sysdig.event.content.scan_result",
                )?;
            }

            let _cond = {
                event
                    .get("json.content.sequence")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.sequence", |event| {
                    if event.has_value("_ingest._value.eventId") {
                        event.rename("_ingest._value.eventId", "_ingest._value.event_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.sequence")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.sequence", |event| {
                    if event.has_value("_ingest._value.eventName") {
                        event.rename("_ingest._value.eventName", "_ingest._value.event_name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.sequence")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.sequence", |event| {
                    if event.has_value("_ingest._value.ingestionId") {
                        event
                            .rename("_ingest._value.ingestionId", "_ingest._value.ingestion_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.sequence")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.sequence", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.sourceIpAddress") {
                            if let Some(val) = event.get("_ingest._value.sourceIpAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.sourceIpAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.source_ip_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_content_sequence_sourceIpAddress_to_ip",
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.sequence")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.sequence", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.source_ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.sequence")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.sequence", |event| {
                    if event.has_value("_ingest._value.subIngestionId") {
                        event.rename(
                            "_ingest._value.subIngestionId",
                            "_ingest._value.sub_ingestion_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.sequence")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.sequence", |event| {
                    event.remove("_ingest._value.sourceIpAddress");
                    Ok(())
                })?;
            }

            if event.has_value("json.content.sequence") {
                event.rename("json.content.sequence", "sysdig.event.content.sequence")?;
            }

            let _cond = {
                event
                    .get("json.content.stats")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.stats", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.count") {
                            if let Some(val) = event.get("_ingest._value.count") {
                                let converted = convert_value(val, "long").map_err(|message| {
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
                            "convert_content_stats_to_long",
                        )?;
                        event.remove("_ingest._value.count");
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
                    Ok(())
                })?;
            }

            if event.has_value("json.content.stats") {
                event.rename("json.content.stats", "sysdig.event.content.stats")?;
            }

            if event.has_value("json.content.type") {
                event.rename("json.content.type", "sysdig.event.content.type")?;
            }

            let _cond = {
                event
                    .get("json.content.zones")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.zones", |event| {
                    if event.has_value("_ingest._value.zoneId") {
                        if let Some(val) = event.get("_ingest._value.zoneId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.zoneId".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.zones")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.zones", |event| {
                    if event.has_value("_ingest._value.zoneName") {
                        event.rename("_ingest._value.zoneName", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.content.zones")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.content.zones", |event| {
                    event.remove("_ingest._value.zoneId");
                    Ok(())
                })?;
            }

            if event.has_value("json.content.zones") {
                event.rename("json.content.zones", "sysdig.event.content.zones")?;
            }

            if event.has_value("json.description") {
                event.rename("json.description", "sysdig.event.description")?;
            }

            if let Some(v) = event
                .get("sysdig.event.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            if event.has_value("json.engine") {
                event.rename("json.engine", "sysdig.event.engine")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "sysdig.event.id")?;
            }

            if let Some(v) = event
                .get("sysdig.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.labels") {
                event.rename("json.labels", "sysdig.event.labels")?;
            }

            let _cond = {
                event
                    .get("sysdig.event.labels")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    dot_expand(event, "sysdig.event.labels", "*")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dot_expander")?;
                    event.set("_ingest.on_failure_processor_tag", "dot_expander_labels")?;
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

            if event.has_value("sysdig.event.labels.aws.accountId") {
                if let Some(val) = event.get("sysdig.event.labels.aws.accountId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.labels.aws.accountId".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.labels.aws.account_id", converted)?;
                }
            }

            let _cond = { event.has_value("sysdig.event.labels.aws.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.labels.aws.user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("sysdig.event.labels.azure.instanceId") {
                if let Some(val) = event.get("sysdig.event.labels.azure.instanceId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.labels.azure.instanceId".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.labels.azure.instance_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.event.labels.azure.instance_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if event.has_value("sysdig.event.labels.azure.instanceName") {
                event.rename(
                    "sysdig.event.labels.azure.instanceName",
                    "sysdig.event.labels.azure.instance_name",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.azure.instance_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.name", v)?;
            }

            if event.has_value("sysdig.event.labels.azure.instanceSize") {
                event.rename(
                    "sysdig.event.labels.azure.instanceSize",
                    "sysdig.event.labels.azure.instance_size",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.azure.instance_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.machine.type", v)?;
            }

            if event.has_value("sysdig.event.labels.azure.subscriptionId") {
                event.rename(
                    "sysdig.event.labels.azure.subscriptionId",
                    "sysdig.event.labels.azure.subscription_id",
                )?;
            }

            if event.has_value("sysdig.event.labels.cloudProvider.account.id") {
                if let Some(val) = event.get("sysdig.event.labels.cloudProvider.account.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.labels.cloudProvider.account.id".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.labels.cloud_provider.account.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.event.labels.cloud_provider.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if event.has_value("sysdig.event.labels.cloudProvider.name") {
                event.rename(
                    "sysdig.event.labels.cloudProvider.name",
                    "sysdig.event.labels.cloud_provider.name",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.cloud_provider.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has_value("sysdig.event.labels.cloudProvider.region") {
                event.rename(
                    "sysdig.event.labels.cloudProvider.region",
                    "sysdig.event.labels.cloud_provider.region",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.cloud_provider.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if event.has_value("sysdig.event.labels.cloudProvider.user") {
                event.rename(
                    "sysdig.event.labels.cloudProvider.user",
                    "sysdig.event.labels.cloud_provider.user",
                )?;
            }

            let _cond = { event.has_value("sysdig.event.labels.cloud_provider.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sysdig.event.labels.cloud_provider.user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sysdig.event.labels.container.image.digest") };
            if _cond {
                event.append_unique(
                    "container.image.hash.all",
                    json!(
                        event
                            .get("sysdig.event.labels.container.image.digest")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("sysdig.event.labels.container.image.digest") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.related = ctx.related ?: [:]; ctx.related.hash = ctx.related.hash ?: []; String digest = ctx.sysdig.event.labels.container.image.digest; int index = digest.indexOf(':'); String hash = digest.substring(index + 1); ctx.related.hash.add(hash);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.related = ctx.related ?: [:]; ctx.related.hash = ctx.related.hash ?: []; String digest = ctx.sysdig.event.labels.container.image.digest; int index = digest.indexOf(':'); String hash = digest.substring(index + 1); ctx.related.hash.add(hash);"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_append_container_image_hash_to_related_hash",
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
            }

            if event.has_value("sysdig.event.labels.gcp.availabilityZone") {
                event.rename(
                    "sysdig.event.labels.gcp.availabilityZone",
                    "sysdig.event.labels.gcp.availability_zone",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.gcp.availability_zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.availability_zone", v)?;
            }

            if event.has_value("sysdig.event.labels.gcp.instanceId") {
                if let Some(val) = event.get("sysdig.event.labels.gcp.instanceId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.labels.gcp.instanceId".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.labels.gcp.instance_id", converted)?;
                }
            }

            let _cond = { !event.has_value("cloud.instance.id") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.event.labels.gcp.instance_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            if event.has_value("sysdig.event.labels.gcp.instanceName") {
                event.rename(
                    "sysdig.event.labels.gcp.instanceName",
                    "sysdig.event.labels.gcp.instance_name",
                )?;
            }

            let _cond = { !event.has_value("cloud.instance.name") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.event.labels.gcp.instance_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            if event.has_value("sysdig.event.labels.gcp.machineType") {
                event.rename(
                    "sysdig.event.labels.gcp.machineType",
                    "sysdig.event.labels.gcp.machine_type",
                )?;
            }

            let _cond = { !event.has_value("cloud.machine.type") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.event.labels.gcp.machine_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.machine.type", v)?;
                }
            }

            if event.has_value("sysdig.event.labels.gcp.projectId") {
                if let Some(val) = event.get("sysdig.event.labels.gcp.projectId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.event.labels.gcp.projectId".into(),
                            message,
                        }
                    })?;
                    event.set("sysdig.event.labels.gcp.project_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.event.labels.gcp.project_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.project.id", v)?;
            }

            if event.has_value("sysdig.event.labels.gcp.projectName") {
                event.rename(
                    "sysdig.event.labels.gcp.projectName",
                    "sysdig.event.labels.gcp.project_name",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.gcp.project_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.project.name", v)?;
            }

            if event.has_value("sysdig.event.labels.host.hostName") {
                event.rename(
                    "sysdig.event.labels.host.hostName",
                    "sysdig.event.labels.host.host_name",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.host.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.host.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("sysdig.event.labels.host.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sysdig.event.labels.host.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("sysdig.event.labels.host.mac") {
                map_strings(
                    event,
                    "sysdig.event.labels.host.mac",
                    "sysdig.event.labels.host.mac",
                    str::to_uppercase,
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sysdig.event.labels.host.mac") {
                    gsub_field(
                        event,
                        "sysdig.event.labels.host.mac",
                        "sysdig.event.labels.host.mac",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_labels_host_mac")?;
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

            let _cond = { event.has_value("sysdig.event.labels.host.mac") };
            if _cond {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("sysdig.event.labels.host.mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.kubernetes.cluster.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.cluster.name", v)?;
            }

            if event.has_value("sysdig.event.labels.kubernetes.cronJob.name") {
                event.rename(
                    "sysdig.event.labels.kubernetes.cronJob.name",
                    "sysdig.event.labels.kubernetes.cron_job.name",
                )?;
            }

            if event.has_value("sysdig.event.labels.kubernetes.daemonSet.name") {
                event.rename(
                    "sysdig.event.labels.kubernetes.daemonSet.name",
                    "sysdig.event.labels.kubernetes.daemon_set.name",
                )?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.kubernetes.namespace.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.namespace", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.kubernetes.pod.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.event.labels.kubernetes.workload.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.parent.type", v)?;
            }

            let _cond = { event.get_str("sysdig.event.labels.source.ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sysdig.event.labels.source.ip") {
                        if let Some(val) = event.get("sysdig.event.labels.source.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sysdig.event.labels.source.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("sysdig.event.labels.source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_labels_source_ip",
                    )?;
                    if event.has_value("sysdig.event.labels.source.ip") {
                        event.rename(
                            "sysdig.event.labels.source.ip",
                            "sysdig.event.labels.source.domain",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("sysdig.event.labels.source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("sysdig.event.labels.source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sysdig.event.labels.source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "sysdig.event.name")?;
            }

            if let Some(v) = event
                .get("sysdig.event.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            if event.has_value("json.originator") {
                event.rename("json.originator", "sysdig.event.originator")?;
            }

            if event.has_value("json.rawEventCategory") {
                event.rename("json.rawEventCategory", "sysdig.event.raw_event_category")?;
            }

            if event.has_value("json.rawEventOriginator") {
                event.rename(
                    "json.rawEventOriginator",
                    "sysdig.event.raw_event_originator",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.severity") {
                    if let Some(val) = event.get("json.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severity".into(),
                                message,
                            }
                        })?;
                        event.set("sysdig.event.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_severity_to_long",
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

            if let Some(v) = event
                .get("sysdig.event.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            let _cond = { event.has_value("event.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String severity_value; if (ctx.event.severity >= 0 && ctx.event.severity <= 3) {\n  severity_value = \"High\";\n} else if (ctx.event.severity >= 4 && ctx.event.severity <= 5) {\n  severity_value = \"Medium\";\n} else if (ctx.event.severity == 6) {\n  severity_value = \"Low\";\n} else if (ctx.event.severity == 7) {\n  severity_value = \"Info\";\n} if (severity_value != null) {\n  ctx.sysdig.event.severity_value = severity_value;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String severity_value; if (ctx.event.severity >= 0 && ctx.event.severity <= 3) {\n  severity_value = \"High\";\n} else if (ctx.event.severity >= 4 && ctx.event.severity <= 5) {\n  severity_value = \"Medium\";\n} else if (ctx.event.severity == 6) {\n  severity_value = \"Low\";\n} else if (ctx.event.severity == 7) {\n  severity_value = \"Info\";\n} if (severity_value != null) {\n  ctx.sysdig.event.severity_value = severity_value;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_set_severity_value",
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
            }

            if event.has_value("json.source") {
                event.rename("json.source", "sysdig.event.source")?;
            }

            if let Some(v) = event
                .get("sysdig.event.source")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.sourceDetails.subType") {
                event.rename(
                    "json.sourceDetails.subType",
                    "sysdig.event.source_details.sub_type",
                )?;
            }

            if event.has_value("json.sourceDetails.type") {
                event.rename(
                    "json.sourceDetails.type",
                    "sysdig.event.source_details.type",
                )?;
            }

            let _cond = {
                event.has_value("sysdig.labels.kubernetes.cluster.name")
                    || event.has_value("sysdig.labels.kubernetes.namespace.name")
                    || event.has_value("sysdig.labels.kubernetes.pod.name")
                    || event.has_value("sysdig.labels.kubernetes.workload.type")
            };
            if _cond {
                event.set("orchestrator.type", json!("kubernetes"))?;
            }

            let _cond = {
                event.has_value("process.name")
                    || event.has_value("process.pid")
                    || event.has_value("process.command_line")
                    || event.has_value("process.executable")
            };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

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
                event.remove("sysdig.event.content.fields.container.id");
                event.remove("sysdig.event.content.fields.container.image.repository");
                event.remove("sysdig.event.content.fields.container.image.tag");
                event.remove("sysdig.event.content.fields.container.name");
                event.remove("sysdig.event.content.fields.container.privileged");
                event.remove("sysdig.event.content.fields.group.gid");
                event.remove("sysdig.event.content.fields.group.name");
                event.remove("sysdig.event.content.fields.proc.acmdline[2]");
                event.remove("sysdig.event.content.fields.proc.acmdline[3]");
                event.remove("sysdig.event.content.fields.proc.acmdline[4]");
                event.remove("sysdig.event.content.fields.proc.aexepath[2]");
                event.remove("sysdig.event.content.fields.proc.aexepath[3]");
                event.remove("sysdig.event.content.fields.proc.aexepath[4]");
                event.remove("sysdig.event.content.fields.proc.aname[2]");
                event.remove("sysdig.event.content.fields.proc.aname[3]");
                event.remove("sysdig.event.content.fields.proc.aname[4]");
                event.remove("sysdig.event.content.fields.proc.cmdline");
                event.remove("sysdig.event.content.fields.proc.cwd");
                event.remove("sysdig.event.content.fields.proc.exepath");
                event.remove("sysdig.event.content.fields.proc.hash.sha256");
                event.remove("sysdig.event.content.fields.proc.name");
                event.remove("sysdig.event.content.fields.proc.pcmdline");
                event.remove("sysdig.event.content.fields.proc.pexepath");
                event.remove("sysdig.event.content.fields.proc.pid");
                event.remove("sysdig.event.content.fields.proc.pid_ts");
                event.remove("sysdig.event.content.fields.proc.pname");
                event.remove("sysdig.event.content.fields.proc.ppid");
                event.remove("sysdig.event.content.fields.proc.ppid_ts");
                event.remove("sysdig.event.content.fields.user.name");
                event.remove("sysdig.event.content.fields.user.uid");
                event.remove("sysdig.event.content.output");
                event.remove("sysdig.event.content.policy_origin");
                event.remove("sysdig.event.content.policy_version");
                event.remove("sysdig.event.content.rule_name");
                event.remove("sysdig.event.description");
                event.remove("sysdig.event.id");
                event.remove("sysdig.event.labels.cloud_provider.account.id");
                event.remove("sysdig.event.labels.cloud_provider.name");
                event.remove("sysdig.event.labels.cloud_provider.region");
                event.remove("sysdig.event.labels.container.image.digest");
                event.remove("sysdig.event.labels.host.host_name");
                event.remove("sysdig.event.labels.host.mac");
                event.remove("sysdig.event.labels.kubernetes.cluster.name");
                event.remove("sysdig.event.labels.kubernetes.namespace.name");
                event.remove("sysdig.event.labels.kubernetes.pod.name");
                event.remove("sysdig.event.labels.kubernetes.workload.type");
                event.remove("sysdig.event.labels.source.ip");
                event.remove("sysdig.event.name");
                event.remove("sysdig.event.severity");
                event.remove("sysdig.event.source");
                event.remove("sysdig.event.timestamp");
            }

            event.remove("json");
            event.remove("sysdig.event.content.fields.aws.accountId");
            event.remove("sysdig.event.content.fields.aws.sourceIP");
            event.remove("sysdig.event.labels.aws.accountId");
            event.remove("sysdig.event.labels.azure.instanceId");
            event.remove("sysdig.event.labels.cloudProvider.account.id");
            event.remove("sysdig.event.labels.gcp.instanceId");
            event.remove("sysdig.event.labels.gcp.projectId");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' || v == '{}' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' || v == '{}' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["<NA>".into(), "-1".into(), "{}".into()],
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
