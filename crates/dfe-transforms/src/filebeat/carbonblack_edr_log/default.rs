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

            let _cond = { !event.has_value("json") };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("json object is missing from event").to_string(),
                });
            }

            let _cond = {
                event.has_value("json.docs") && event.get("json.docs").is_some_and(|v| v.is_array()) && event.get("json.docs").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("docs array has more than one entry, this is unsupported. Use CB Event Forwarder as source of events").to_string(),
                });
            }

            let _cond = { event.has_value("json.docs") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def docs = ctx.json.docs;\nif (docs instanceof List && docs.size() > 0) {\n  ctx.json[\"doc\"] = docs[0];\n} else if (docs instanceof Map) {\n  ctx.json[\"doc\"] = docs;\n} else {\n  throw new Exception(\"Unexpected type\");\n}\nctx.json.remove(\"docs\");
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def docs = ctx.json.docs;\nif (docs instanceof List && docs.size() > 0) {\n  ctx.json[\"doc\"] = docs[0];\n} else if (docs instanceof Map) {\n  ctx.json[\"doc\"] = docs;\n} else {\n  throw new Exception(\"Unexpected type\");\n}\nctx.json.remove(\"docs\");"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed extracting docs field: {}",
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.compressed_size") {
                    if let Some(val) = event.get("json.compressed_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.compressed_size".into(),
                                message,
                            }
                        })?;
                        event.set("json.compressed_size", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.emet_timestamp") {
                    if let Some(val) = event.get("json.emet_timestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.emet_timestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.emet_timestamp", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event_timestamp") {
                    if let Some(val) = event.get("json.event_timestamp") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event_timestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.event_timestamp", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.feed_id") {
                    if let Some(val) = event.get("json.feed_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.feed_id".into(),
                                message,
                            }
                        })?;
                        event.set("json.feed_id", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.parent_create_time") {
                    if let Some(val) = event.get("json.parent_create_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.parent_create_time".into(),
                                message,
                            }
                        })?;
                        event.set("json.parent_create_time", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.pid") {
                    if let Some(val) = event.get("json.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.pid".into(),
                                message,
                            }
                        })?;
                        event.set("json.pid", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.doc.process_pid") {
                    if let Some(val) = event.get("json.doc.process_pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.doc.process_pid".into(),
                                message,
                            }
                        })?;
                        event.set("json.doc.process_pid", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.doc.parent_pid") {
                    if let Some(val) = event.get("json.doc.parent_pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.doc.parent_pid".into(),
                                message,
                            }
                        })?;
                        event.set("json.doc.parent_pid", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.port") {
                    if let Some(val) = event.get("json.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.port".into(),
                                message,
                            }
                        })?;
                        event.set("json.port", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.requested_access") {
                    if let Some(val) = event.get("json.requested_access") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.requested_access".into(),
                                message,
                            }
                        })?;
                        event.set("json.requested_access", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.size") {
                    if let Some(val) = event.get("json.size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.size".into(),
                                message,
                            }
                        })?;
                        event.set("json.size", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.doc.orig_mod_len") {
                    if let Some(val) = event.get("json.doc.orig_mod_len") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.doc.orig_mod_len".into(),
                                message,
                            }
                        })?;
                        event.set("json.doc.orig_mod_len", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.scores.alliance_score_virustotal") {
                    if let Some(val) = event.get("json.scores.alliance_score_virustotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.scores.alliance_score_virustotal".into(),
                                message,
                            }
                        })?;
                        event.set("json.scores.alliance_score_virustotal", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.target_create_time") {
                    if let Some(val) = event.get("json.target_create_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.target_create_time".into(),
                                message,
                            }
                        })?;
                        event.set("json.target_create_time", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.target_pid") {
                    if let Some(val) = event.get("json.target_pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.target_pid".into(),
                                message,
                            }
                        })?;
                        event.set("json.target_pid", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.timestamp") {
                    if let Some(val) = event.get("json.timestamp") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.timestamp", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("host.name") };
            if _cond {
                event.set("_tmp.forwarded", json!(true))?;
            }

            // Painless script
            // Source: void removeEmptyStr(Map m) {\n  if (m != null) m.entrySet().removeIf( e -> e.value == \"\");\n}\nremoveEmptyStr(ctx.json);\nremoveEmptyStr(ctx.json.doc);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void removeEmptyStr(Map m) {\n  if (m != null) m.entrySet().removeIf( e -> e.value == \"\");\n}\nremoveEmptyStr(ctx.json);\nremoveEmptyStr(ctx.json.doc);"#
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.rename("json.type", "event.action")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "rename")?;
                event.set("event.action", json!("unknown"))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // Painless script
            // Source: def clone(def ref) {\n  if (ref == null) return ref;\n  if (ref instanceof Map) {\n    ref = ref.entrySet().stream().collect(\n            Collectors.toMap(\n              e -> e.getKey(),\n              e -> clone(e.getValue())\n            )\n          );\n  } else if (ref instanceof List) {\n    ref = ref.stream().map(e -> clone(e)).collect(\n            Collectors.toList()\n          );\n  }\n  return ref;\n}\ndef event = ctx.event;\nif (event == null) {\n  event = new HashMap();\n  ctx[\"event\"] = event;\n}\ndef type = ctx.event.action;\ndef fields = params[type] != null? params[type] : params[\"unknown\"];\nfields.forEach( (k, v) -> {\n  event[k] = clone(v);\n});
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def clone(def ref) {\n  if (ref == null) return ref;\n  if (ref instanceof Map) {\n    ref = ref.entrySet().stream().collect(\n            Collectors.toMap(\n              e -> e.getKey(),\n              e -> clone(e.getValue())\n            )\n          );\n  } else if (ref instanceof List) {\n    ref = ref.stream().map(e -> clone(e)).collect(\n            Collectors.toList()\n          );\n  }\n  return ref;\n}\ndef event = ctx.event;\nif (event == null) {\n  event = new HashMap();\n  ctx[\"event\"] = event;\n}\ndef type = ctx.event.action;\ndef fields = params[type] != null? params[type] : params[\"unknown\"];\nfields.forEach( (k, v) -> {\n  event[k] = clone(v);\n});"#
                ),
                cached_params!(
                    "{\"alert.watchlist.hit.ingress.host\":{\"kind\":\"alert\",\"category\":[\"host\"],\"type\":[\"info\"]},\"alert.watchlist.hit.ingress.binary\":{\"kind\":\"alert\",\"category\":[\"file\"],\"type\":[\"info\"]},\"alert.watchlist.hit.ingress.process\":{\"kind\":\"alert\",\"category\":[\"process\"],\"type\":[\"info\"]},\"alert.watchlist.hit.query.binary\":{\"kind\":\"alert\",\"category\":[\"file\"],\"type\":[\"info\"]},\"alert.watchlist.hit.query.process\":{\"kind\":\"alert\",\"category\":[\"process\"],\"type\":[\"info\"]},\"binaryinfo.host.observed\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"info\"]},\"binaryinfo.group.observed\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"info\"]},\"binaryinfo.observed\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"info\"]},\"binarystore.file.added\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"creation\"]},\"feed.ingress.hit.host\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"info\"]},\"feed.ingress.hit.binary\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"info\"]},\"feed.ingress.hit.process\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"feed.query.hit.binary\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"info\"]},\"feed.query.hit.process\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"feed.storage.hit.binary\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"info\"]},\"feed.storage.hit.process\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"watchlist.hit.process\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"watchlist.hit.binary\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"info\"]},\"watchlist.storage.hit.binary\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"info\"]},\"watchlist.storage.hit.process\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"ingress.event.regmod\":{\"kind\":\"event\",\"category\":[\"registry\"],\"type\":[\"change\"]},\"ingress.event.filemod\":{\"kind\":\"event\",\"category\":[\"file\"],\"type\":[\"change\"]},\"ingress.event.netconn\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\",\"start\"]},\"ingress.event.module\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"start\",\"info\"]},\"ingress.event.childproc\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"start\",\"info\"]},\"ingress.event.process\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"ingress.event.crossprocopen\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"ingress.event.remotethread\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\"]},\"ingress.event.emetmitigation\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\",\"end\"]},\"ingress.event.processblock\":{\"kind\":\"event\",\"category\":[\"process\"],\"type\":[\"info\",\"end\"]},\"ingress.event.tamper\":{\"kind\":\"event\",\"category\":[\"process\",\"driver\"],\"type\":[\"info\"]},\"unknown\":{\"kind\":\"event\"}}"
                ),
            )?;

            event.set("observer.vendor", json!("VMWare"))?;

            event.set("observer.product", json!("Carbon Black EDR"))?;

            event.set("observer.type", json!("edr"))?;

            if event.has_value("json.cb_version") {
                event.rename("json.cb_version", "observer.version")?;
            }

            if event.has_value("json.cb_server") {
                event.rename("json.cb_server", "observer.name")?;
            }

            let _cond = { !event.has_value("observer.name") };
            if _cond {
                if event.has_value("json.server_name") {
                    event.rename("json.server_name", "observer.name")?;
                }
            }

            if event.has_value("json.ioc_attrs") {
                event.rename("json.ioc_attrs", "json.ioc_attr")?;
            }

            let _cond = {
                event.has_value("json.ioc_attr")
                    && event.get("json.ioc_attr").is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "json.ioc_attr", "json.ioc_attr")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to parse string field ioc_attr as JSON (value:{}): {}",
                            event
                                .get("json.ioc_attr")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    if event.remove("json.ioc_attr").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.ioc_attr".into(),
                        });
                    }
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
                if event.has_value("json.timestamp") {
                    if let Some(val) = event.get("json.timestamp") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message,
                            }
                        })?;
                        event.set("_tmp.timestamp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "failed to convert numeric timestamp (value: {}): {}",
                        event
                            .get("json.timestamp")
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

            let _cond = { !event.has_value("_tmp.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.event_timestamp") {
                        if let Some(val) = event.get("json.event_timestamp") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.event_timestamp".into(),
                                    message,
                                }
                            })?;
                            event.set("_tmp.timestamp", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to convert numeric event_timestamp (value: {}): {}",
                            event
                                .get("json.event_timestamp")
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

            let _cond = { !event.has_value("_tmp.timestamp") };
            if _cond {
                let v = json!(
                    event
                        .get("json.doc.timestamp")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("_tmp.timestamp", v)?;
                }
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse timestamp (value: {}): {}",
                            event
                                .get("_tmp.timestamp")
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

            if event.has_value("json.watchlist_id") {
                if let Some(val) = event.get("json.watchlist_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.watchlist_id".into(),
                            message,
                        }
                    })?;
                    event.set("rule.id", converted)?;
                }
            }

            if event.has_value("json.watchlist_name") {
                event.rename("json.watchlist_name", "rule.name")?;
            }

            // Painless script
            // Source: def ep = ctx.json.doc?.endpoint;\nif (ep != null && !(ep instanceof List)) {\n  ctx.json.doc.endpoint = [ ep ];\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def ep = ctx.json.doc?.endpoint;\nif (ep != null && !(ep instanceof List)) {\n  ctx.json.doc.endpoint = [ ep ];\n}"#
                ),
            )?;

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if event.has_value("json.doc.hostname") {
                    event.rename("json.doc.hostname", "host.name")?;
                }
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if event.has_value("json.hostname") {
                    event.rename("json.hostname", "host.name")?;
                }
            }

            let _cond = {
                !event.has_value("host.name")
                    && !event.has_value("json.doc.hostname")
                    && event.has_value("json.doc.endpoint")
            };
            if _cond {
                if event.has_value("json.doc.endpoint") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.doc.endpoint").cloned();
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
                                if let Some(input) = event.get_string("_ingest._value") {
                                    // Grok pattern: ^(?P<host_name>(?:[^|]*))(?:|)
                                    if !cached_grok_mapped!(
                                        "^(?P<host_name>(?:[^|]*))(?:|)",
                                        [("host_name", "host.name")]
                                    )
                                    .extract_into(&input, event)?
                                    {
                                        return Err(TransformError::GrokNoMatch { value: input });
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
                                "json.doc.endpoint",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            if event.has_value("json.doc.digsig_subject") {
                event.rename(
                    "json.doc.digsig_subject",
                    "file.code_signature.subject_name",
                )?;
            }

            if event.has_value("json.doc.digsig_status") {
                event.rename("json.doc.digsig_status", "file.code_signature.status")?;
            }

            let _cond = { event.has_value("file_signature") };
            if _cond {
                event.set("file.code_signature.exists", json!(true))?;
            }

            let _cond = { event.has_value("_tmp.forwarded") };
            if _cond {
                if event.has_value("json.doc.os_type") {
                    map_strings(event, "json.doc.os_type", "host.os.type", str::to_lowercase)?;
                }
            }

            let _cond = {
                event.has_value("_tmp.forwarded") && event.get_str("host.os.type") == Some("osx")
            };
            if _cond {
                event.set("host.os.type", json!("macos"))?;
            }

            let _cond = {
                event.has_value("_tmp.forwarded")
                    && event.has_value("json.doc.os_type")
                    && !(["windows", "linux", "macos"]
                        .contains(&event.get_str("host.os.type").unwrap_or("")))
            };
            if _cond {
                if event.remove("host.os.type").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "host.os.type".into(),
                    });
                }
            }

            let _cond = { event.has_value("_tmp.forwarded") };
            if _cond {
                if event.has_value("json.doc.os_name") {
                    event.rename("json.doc.os_name", "host.os.name")?;
                }
            }

            let _cond = {
                event.get_bool("json.doc.is_executable_image") == Some(true)
                    || event.get_str("json.doc.is_executable_image") == Some("true")
            };
            if _cond {
                event.append("file.attributes", json!("executable"))?;
            }

            if event.has_value("json.doc.md5") {
                map_strings(event, "json.doc.md5", "file.hash.md5", str::to_lowercase)?;
            }

            if event.has_value("json.doc.observed_filename") {
                foreach_array(event, "json.doc.observed_filename", |event| {
                    event.set(
                        "file.path",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.file_path") {
                event.rename("json.file_path", "file.path")?;
            }

            let _cond = { event.get_str("event.action") == Some("ingress.event.regmod") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.path") {
                        if let Some(input) = event.get_string("json.path") {
                            // Grok pattern: (?i)\\\\registry\\\\%{GREEDYDATA:registry.path}
                            if !cached_grok!("(?i)\\\\registry\\\\%{GREEDYDATA:registry.path}")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.doc.orig_mod_len") {
                event.rename("json.doc.orig_mod_len", "file.size")?;
            }

            let _cond = { !event.has_value("file.size") };
            if _cond {
                if event.has_value("json.size") {
                    event.rename("json.size", "file.size")?;
                }
            }

            if event.has_value("json.doc.cmdline") {
                event.rename("json.doc.cmdline", "process.command_line")?;
            }

            if event.has_value("json.doc.path") {
                event.rename("json.doc.path", "process.executable")?;
            }

            if event.has_value("json.doc.process_md5") {
                map_strings(
                    event,
                    "json.doc.process_md5",
                    "process.hash.md5",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.doc.process_name") {
                event.rename("json.doc.process_name", "process.name")?;
            }

            if event.has_value("json.doc.process_pid") {
                event.rename("json.doc.process_pid", "process.pid")?;
            }

            if event.has_value("json.doc.unique_id") {
                event.rename("json.doc.unique_id", "process.entity_id")?;
            }

            let _cond = { event.has_value("json.doc.start") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.doc.start") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                            Some(parsed) => event.set("process.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.doc.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse process start timestamp (value: {}): {}",
                            event
                                .get("doc.start")
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

            if event.has_value("json.doc.parent_name") {
                event.rename("json.doc.parent_name", "process.parent.name")?;
            }

            if event.has_value("json.doc.parent_pid") {
                event.rename("json.doc.parent_pid", "process.parent.pid")?;
            }

            if event.has_value("json.doc.parent_unique_id") {
                event.rename("json.doc.parent_unique_id", "process.parent.entity_id")?;
            }

            if event.has_value("json.doc.parent_md5") {
                map_strings(
                    event,
                    "json.doc.parent_md5",
                    "process.parent.hash.md5",
                    str::to_lowercase,
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.doc.is_64bit") {
                    if let Some(val) = event.get("json.doc.is_64bit") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.doc.is_64bit".into(),
                                message,
                            }
                        })?;
                        event.set("json.doc.is_64bit", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.doc.is_64bit").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.doc.is_64bit".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_bool("json.doc.is_64bit") == Some(true) };
            if _cond {
                event.set("file.pe.architecture", json!("x64"))?;
            }

            let _cond = { event.get_bool("json.doc.is_64bit") == Some(false) };
            if _cond {
                event.set("file.pe.architecture", json!("x86"))?;
            }

            if event.has_value("json.utf8_file_description") {
                event.rename("json.utf8_file_description", "file.pe.description")?;
            }

            if event.has_value("json.utf8_company_name") {
                event.rename("json.utf8_company_name", "file.pe.company")?;
            }

            if event.has_value("json.utf8_product") {
                event.rename("json.utf8_product", "file.pe.product_name")?;
            }

            if event.has_value("json.utf8_product_name") {
                event.rename("json.utf8_product_name", "file.pe.product")?;
            }

            if event.has_value("json.utf8_original_file_name") {
                event.rename("json.utf8_original_file_name", "file.pe.original_file_name")?;
            }

            if event.has_value("json.utf8_file_version") {
                event.rename("json.utf8_file_version", "file.pe.file_version")?;
            }

            let _cond = { event.has_value("json.ioc_type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: void _set(Map base, def path, def value) {\n  if (path.length == 0) return;\n  for (int i=0; i<path.length-1; i++) {\n    String c = path[i];\n    if (base[c] == null) base[c] = new HashMap();\n    base = base[c];\n  }\n  base[path[path.length-1]] = value;\n} void set(Map base, String path, def value) {\n  _set(base, path.splitOnToken(\".\"), value);\n} def mapping = params[ctx.json.ioc_type.toLowerCase()]; if (mapping == null) return; set(ctx, \"threat.indicator.type\", mapping.type); def value = ctx.json.ioc_value; if (value == null) return; set(ctx, mapping.target, value); ctx[\"_tmp_ioc_done\"] = true;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"void _set(Map base, def path, def value) {\n  if (path.length == 0) return;\n  for (int i=0; i<path.length-1; i++) {\n    String c = path[i];\n    if (base[c] == null) base[c] = new HashMap();\n    base = base[c];\n  }\n  base[path[path.length-1]] = value;\n} void set(Map base, String path, def value) {\n  _set(base, path.splitOnToken(\".\"), value);\n} def mapping = params[ctx.json.ioc_type.toLowerCase()]; if (mapping == null) return; set(ctx, \"threat.indicator.type\", mapping.type); def value = ctx.json.ioc_value; if (value == null) return; set(ctx, mapping.target, value); ctx[\"_tmp_ioc_done\"] = true;\n"#
                        ),
                        cached_params!(
                            "{\"dns\":{\"type\":\"domain-name\",\"target\":\"threat.indicator.url.domain\"},\"ipv4\":{\"type\":\"ipv4-addr\",\"target\":\"threat.indicator.ip\"},\"ipv6\":{\"type\":\"ipv6-addr\",\"target\":\"threat.indicator.ip\"},\"md5\":{\"type\":\"file\",\"target\":\"threat.indicator.file.hash.md5\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Unable to determine indicator type from \"{}\": {}",
                            event
                                .get("json.ioc_type")
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

            let _cond = { event.get_bool("_tmp_ioc_done") == Some(true) };
            if _cond {
                if event.remove("_tmp_ioc_done").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_tmp_ioc_done".into(),
                    });
                }
                if event.remove("json.ioc_type").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ioc_type".into(),
                    });
                }
                if event.remove("json.ioc_value").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ioc_value".into(),
                    });
                }
            }

            if event.has_value("threat.indicator.file.hash.md5") {
                map_strings(
                    event,
                    "threat.indicator.file.hash.md5",
                    "threat.indicator.file.hash.md5",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.ioc_attr.port") {
                if let Some(val) = event.get("json.ioc_attr.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.ioc_attr.port".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.port", converted)?;
                }
            }

            if event.has_value("json.ioc_attr.direction") {
                map_strings(
                    event,
                    "json.ioc_attr.direction",
                    "network.direction",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.ioc_attr.protocol") {
                map_strings(
                    event,
                    "json.ioc_attr.protocol",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.protocol") {
                if let Some(val) = event.get("json.protocol") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.protocol".into(),
                            message,
                        }
                    })?;
                    event.set("network.iana_number", converted)?;
                }
            }

            if let Some(v) = event
                .get("json.ja3")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.ja3", v)?;
            }

            if let Some(v) = event
                .get("json.ja3s")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.ja3s", v)?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.parent.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.server.ja3s") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("tls.server.ja3s")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.client.ja3") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("tls.client.ja3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("message");
            event.remove("_tmp");
            event.remove("json.doc.is_executable_image");
            event.remove("json.doc.start");
            event.remove("json.doc.process_md5");
            event.remove("json.doc.parent_md5");
            event.remove("json.doc.md5");
            event.remove("json.ioc_attr.port");
            event.remove("json.ioc_attr.direction");
            event.remove("json.ioc_attr.protocol");
            event.remove("json.watchlist_id");

            if event.has_value("json") {
                event.rename("json", "carbonblack.edr")?;
            }

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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
