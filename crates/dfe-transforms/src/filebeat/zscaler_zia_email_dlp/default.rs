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
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.get_str("input.type") == Some("http_endpoint") };
            if _cond {
                event.remove("json");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "zscaler_zia.email_dlp")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_parse_event_original",
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

            let _cond = {
                event.has_value("zscaler_zia.email_dlp")
                    && event.get_bool("_conf.strict_fields") == Some(true)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.zscaler_zia.email_dlp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message == null ? [] : ctx.error.message;\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.email_dlp.version == null ? 'null' : ctx.zscaler_zia.email_dlp.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.zscaler_zia.email_dlp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message == null ? [] : ctx.error.message;\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.email_dlp.version == null ? 'null' : ctx.zscaler_zia.email_dlp.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}"#
                        ),
                        cached_params!(
                            "{\"data_stream\":\"email_dlp\",\"expect\":{\"version\":\"v1\"},\"pkg_version\":\"4.1.0\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "check_template_version")?;
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

            if event.has_value("zscaler_zia.email_dlp.record_id") {
                if let Some(val) = event.get("zscaler_zia.email_dlp.record_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.email_dlp.record_id".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.email_dlp.record_id", converted)?;
                }
            }

            if event.has_value("zscaler_zia.email_dlp.dlp.identifier") {
                if let Some(val) = event.get("zscaler_zia.email_dlp.dlp.identifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.email_dlp.dlp.identifier".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.email_dlp.dlp.identifier", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  if (s.length() == 0) return;\n  List out = new ArrayList();\n  int from = 0;\n  int n = s.length();\n  for (int i = 0; i < n; i++) {\n    if (s.charAt(i) == (char)'|') {\n      out.add(s.substring(from, i));\n      from = i + 1;\n    }\n  }\n  out.add(s.substring(from, n));\n  m.put(key, out);\n}\ndef ed = ctx.zscaler_zia?.email_dlp;\nif (ed == null) return;\nsplitStr(ed, 'severity');\nsplitStr(ed, 'actions');\nif (ed.rule instanceof Map) {\n  splitStr(ed.rule, 'labels');\n}\nif (ed.dlp instanceof Map) {\n  splitStr(ed.dlp, 'dict_names');\n  splitStr(ed.dlp, 'dict_counts');\n  splitStr(ed.dlp, 'engine_names');\n}\nif (ed.email instanceof Map) {\n  splitStr(ed.email, 'triggered_recipients');\n  splitStr(ed.email, 'other_recipients');\n  splitStr(ed.email, 'triggered_recipient_domains');\n  splitStr(ed.email, 'other_recipient_domains');\n  if (ed.email.attachments instanceof Map) {\n    splitStr(ed.email.attachments, 'file_names');\n    splitStr(ed.email.attachments, 'md5s');\n    splitStr(ed.email.attachments, 'sizes');\n    splitStr(ed.email.attachments, 'file_types');\n    splitStr(ed.email.attachments, 'doc_types');\n    splitStr(ed.email.attachments, 'doc_subtypes');\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  if (s.length() == 0) return;\n  List out = new ArrayList();\n  int from = 0;\n  int n = s.length();\n  for (int i = 0; i < n; i++) {\n    if (s.charAt(i) == (char)'|') {\n      out.add(s.substring(from, i));\n      from = i + 1;\n    }\n  }\n  out.add(s.substring(from, n));\n  m.put(key, out);\n}\ndef ed = ctx.zscaler_zia?.email_dlp;\nif (ed == null) return;\nsplitStr(ed, 'severity');\nsplitStr(ed, 'actions');\nif (ed.rule instanceof Map) {\n  splitStr(ed.rule, 'labels');\n}\nif (ed.dlp instanceof Map) {\n  splitStr(ed.dlp, 'dict_names');\n  splitStr(ed.dlp, 'dict_counts');\n  splitStr(ed.dlp, 'engine_names');\n}\nif (ed.email instanceof Map) {\n  splitStr(ed.email, 'triggered_recipients');\n  splitStr(ed.email, 'other_recipients');\n  splitStr(ed.email, 'triggered_recipient_domains');\n  splitStr(ed.email, 'other_recipient_domains');\n  if (ed.email.attachments instanceof Map) {\n    splitStr(ed.email.attachments, 'file_names');\n    splitStr(ed.email.attachments, 'md5s');\n    splitStr(ed.email.attachments, 'sizes');\n    splitStr(ed.email.attachments, 'file_types');\n    splitStr(ed.email.attachments, 'doc_types');\n    splitStr(ed.email.attachments, 'doc_subtypes');\n  }\n}"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_split_pipe_delimited_multi_value_fields",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("zscaler_zia.email_dlp.dlp.scan_time") {
                    if let Some(val) = event.get("zscaler_zia.email_dlp.dlp.scan_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.email_dlp.dlp.scan_time".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.email_dlp.dlp.scan_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_dlp_scan_time_to_long",
                )?;
                event.remove("zscaler_zia.email_dlp.dlp.scan_time");
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
                event
                    .get("zscaler_zia.email_dlp.dlp.dict_counts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("zscaler_zia.email_dlp.dlp.dict_counts").cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_dlp_dict_count_element_to_long",
                            )?;
                            if event.remove("_ingest._value").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("zscaler_zia.email_dlp.dlp.dict_counts", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.dlp")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def dlp = ctx.zscaler_zia.email_dlp.dlp;\ndef names = dlp.dict_names instanceof List ? dlp.dict_names : null;\ndef counts = dlp.dict_counts instanceof List ? dlp.dict_counts : null;\nif (names == null) return;\ndef out = new ArrayList();\nfor (int i = 0; i < names.size(); i++) {\n  def name = names.get(i);\n  if (!(name instanceof String) || name == '' || name == 'None') continue;\n  def item = new HashMap();\n  item.put('name', name);\n  if (counts != null && i < counts.size()) item.put('count', counts.get(i));\n  out.add(item);\n}\nif (out.size() > 0) ctx.zscaler_zia.email_dlp.dlp.dictionaries = out;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def dlp = ctx.zscaler_zia.email_dlp.dlp;\ndef names = dlp.dict_names instanceof List ? dlp.dict_names : null;\ndef counts = dlp.dict_counts instanceof List ? dlp.dict_counts : null;\nif (names == null) return;\ndef out = new ArrayList();\nfor (int i = 0; i < names.size(); i++) {\n  def name = names.get(i);\n  if (!(name instanceof String) || name == '' || name == 'None') continue;\n  def item = new HashMap();\n  item.put('name', name);\n  if (counts != null && i < counts.size()) item.put('count', counts.get(i));\n  out.add(item);\n}\nif (out.size() > 0) ctx.zscaler_zia.email_dlp.dlp.dictionaries = out;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_dlp_dictionaries",
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

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.email.attachments.sizes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event
                    .get("zscaler_zia.email_dlp.email.attachments.sizes")
                    .cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_email_attachment_size_element_to_long",
                            )?;
                            if event.remove("_ingest._value").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set(
                        "zscaler_zia.email_dlp.email.attachments.sizes",
                        Value::Array(out),
                    )?;
                }
            }

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.email.mail_sent_epoch")
                    && event.get_str("zscaler_zia.email_dlp.email.mail_sent_epoch") != Some("")
                    && event.get_str("zscaler_zia.email_dlp.email.mail_sent_epoch") != Some("0")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.email_dlp.email.mail_sent_epoch")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], None, None) {
                            event.set("zscaler_zia.email_dlp.email.mail_sent_epoch", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_mail_sent_epoch_to_timestamp",
                    )?;
                    event.remove("zscaler_zia.email_dlp.email.mail_sent_epoch");
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

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.email.mail_sent_time")
                    && event.get_str("zscaler_zia.email_dlp.email.mail_sent_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.email_dlp.email.mail_sent_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.email_dlp.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.email_dlp.email.mail_sent_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_email_mail_sent_time_string_to_date",
                    )?;
                    event.remove("zscaler_zia.email_dlp.email.mail_sent_time");
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

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.email.zs_rcv_time")
                    && event.get_str("zscaler_zia.email_dlp.email.zs_rcv_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.email_dlp.email.zs_rcv_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.email_dlp.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.email_dlp.email.zs_rcv_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_email_zs_rcv_time_string_to_date",
                    )?;
                    event.remove("zscaler_zia.email_dlp.email.zs_rcv_time");
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

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.email.zs_sent_time")
                    && event.get_str("zscaler_zia.email_dlp.email.zs_sent_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.email_dlp.email.zs_sent_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.email_dlp.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.email_dlp.email.zs_sent_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_email_zs_sent_time_string_to_date",
                    )?;
                    event.remove("zscaler_zia.email_dlp.email.zs_sent_time");
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

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.time")
                    && event.get_str("zscaler_zia.email_dlp.time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zscaler_zia.email_dlp.time") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.email_dlp.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.email_dlp.time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_nanolog_time_string_to_date",
                    )?;
                    event.remove("zscaler_zia.email_dlp.time");
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

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.feed_time")
                    && event.get_str("zscaler_zia.email_dlp.feed_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zscaler_zia.email_dlp.feed_time") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.email_dlp.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.email_dlp.feed_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_feed_time_string_to_date",
                    )?;
                    event.remove("zscaler_zia.email_dlp.feed_time");
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
                .get("zscaler_zia.email_dlp.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("email"))?;

            let _cond = { event.get_str("zscaler_zia.email_dlp.log_type") == Some("DLP Incident") };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.tz")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { event.has_value("zscaler_zia.email_dlp.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\ndef raw = ctx.zscaler_zia.email_dlp.severity;\nList vals;\nif (raw instanceof List) {\n  vals = (List) raw;\n} else if (raw instanceof String) {\n  vals = new ArrayList();\n  vals.add(raw);\n} else {\n  return;\n}\nint maxSev = 0;\nfor (def v : vals) {\n  if (!(v instanceof String)) continue;\n  String t = ((String) v).toLowerCase();\n  int cur = 0;\n  if (t.contains('high')) {\n    cur = 73;\n  } else if (t.contains('medium')) {\n    cur = 47;\n  } else if (t.contains('low') || t.contains('information')) {\n    cur = 21;\n  }\n  if (cur > maxSev) maxSev = cur;\n}\nif (maxSev > 0) ctx.event.severity = maxSev;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\ndef raw = ctx.zscaler_zia.email_dlp.severity;\nList vals;\nif (raw instanceof List) {\n  vals = (List) raw;\n} else if (raw instanceof String) {\n  vals = new ArrayList();\n  vals.add(raw);\n} else {\n  return;\n}\nint maxSev = 0;\nfor (def v : vals) {\n  if (!(v instanceof String)) continue;\n  String t = ((String) v).toLowerCase();\n  int cur = 0;\n  if (t.contains('high')) {\n    cur = 73;\n  } else if (t.contains('medium')) {\n    cur = 47;\n  } else if (t.contains('low') || t.contains('information')) {\n    cur = 21;\n  }\n  if (cur > maxSev) maxSev = cur;\n}\nif (maxSev > 0) ctx.event.severity = maxSev;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.actions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "zscaler_zia.email_dlp.actions", |event| {
                    event.append_unique(
                        "event.action",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            event.set("event.provider", json!("Zscaler"))?;

            event.set("observer.vendor", json!("Zscaler"))?;

            event.set("observer.product", json!("Zscaler ZIA"))?;

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.datacenter.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.datacenter.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.datacenter.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.city_name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.datacenter.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.country_iso_code", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.company.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.rule.labels")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "zscaler_zia.email_dlp.rule.labels", |event| {
                    event.append_unique(
                        "rule.name",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.dlp.engine_names")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.email.attachments")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def att = ctx.zscaler_zia.email_dlp.email.attachments;\ndef names = att.file_names instanceof List ? att.file_names : null;\ndef sizes = att.sizes instanceof List ? att.sizes : null;\ndef md5s = att.md5s instanceof List ? att.md5s : null;\ndef types = att.file_types instanceof List ? att.file_types : null;\nint n = 0;\nif (names != null && names.size() > n) n = names.size();\nif (sizes != null && sizes.size() > n) n = sizes.size();\nif (md5s != null && md5s.size() > n) n = md5s.size();\nif (types != null && types.size() > n) n = types.size();\nif (n == 0) return;\nif (ctx.email == null) ctx.email = [:];\nif (ctx.email.attachments == null) ctx.email.attachments = new ArrayList();\nfor (int i = 0; i < n; i++) {\n  def file = new HashMap();\n  if (names != null && i < names.size()) file.put('name', names.get(i));\n  if (sizes != null && i < sizes.size()) file.put('size', sizes.get(i));\n  if (md5s != null && i < md5s.size()) {\n    def hash = new HashMap();\n    hash.put('md5', md5s.get(i));\n    file.put('hash', hash);\n  }\n  if (types != null && i < types.size()) file.put('extension', types.get(i));\n  def item = new HashMap();\n  item.put('file', file);\n  ctx.email.attachments.add(item);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def att = ctx.zscaler_zia.email_dlp.email.attachments;\ndef names = att.file_names instanceof List ? att.file_names : null;\ndef sizes = att.sizes instanceof List ? att.sizes : null;\ndef md5s = att.md5s instanceof List ? att.md5s : null;\ndef types = att.file_types instanceof List ? att.file_types : null;\nint n = 0;\nif (names != null && names.size() > n) n = names.size();\nif (sizes != null && sizes.size() > n) n = sizes.size();\nif (md5s != null && md5s.size() > n) n = md5s.size();\nif (types != null && types.size() > n) n = types.size();\nif (n == 0) return;\nif (ctx.email == null) ctx.email = [:];\nif (ctx.email.attachments == null) ctx.email.attachments = new ArrayList();\nfor (int i = 0; i < n; i++) {\n  def file = new HashMap();\n  if (names != null && i < names.size()) file.put('name', names.get(i));\n  if (sizes != null && i < sizes.size()) file.put('size', sizes.get(i));\n  if (md5s != null && i < md5s.size()) {\n    def hash = new HashMap();\n    hash.put('md5', md5s.get(i));\n    file.put('hash', hash);\n  }\n  if (types != null && i < types.size()) file.put('extension', types.get(i));\n  def item = new HashMap();\n  item.put('file', file);\n  ctx.email.attachments.add(item);\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_zip_email_attachments_into_ecs",
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

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.email.message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.email.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.email.mail_sent_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.origination_timestamp", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.email.zs_rcv_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.delivery_timestamp", v)?;
            }

            let _cond = { event.has_value("zscaler_zia.email_dlp.sender") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("zscaler_zia.email_dlp.sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.application.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.x_mailer", v)?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.email.triggered_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.email_dlp.email.triggered_recipients",
                    |event| {
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.email.other_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.email_dlp.email.other_recipients",
                    |event| {
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.email_dlp.owner")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.owner")
                    && event
                        .get("zscaler_zia.email_dlp.owner")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(v) = event
                    .get("zscaler_zia.email_dlp.owner")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("zscaler_zia.email_dlp.owner")
                    && event
                        .get("zscaler_zia.email_dlp.owner")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("zscaler_zia.email_dlp.owner") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_owner_to_user_domain",
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

            let _cond = { event.has_value("zscaler_zia.email_dlp.owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.email_dlp.owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.email_dlp.sender") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.email_dlp.sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.email.triggered_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.email_dlp.email.triggered_recipients",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.email.other_recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.email_dlp.email.other_recipients",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.email_dlp.email.attachments.md5s")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "zscaler_zia.email_dlp.email.attachments.md5s",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            event.remove("zscaler_zia.email_dlp.time");
            event.remove("zscaler_zia.email_dlp.tz");
            event.remove("zscaler_zia.email_dlp.actions");
            event.remove("zscaler_zia.email_dlp.datacenter.name");
            event.remove("zscaler_zia.email_dlp.datacenter.city");
            event.remove("zscaler_zia.email_dlp.datacenter.country");
            event.remove("zscaler_zia.email_dlp.company.name");
            event.remove("zscaler_zia.email_dlp.rule.labels");
            event.remove("zscaler_zia.email_dlp.dlp.engine_names");
            event.remove("zscaler_zia.email_dlp.email.mail_sent_time");
            event.remove("zscaler_zia.email_dlp.email.zs_rcv_time");
            event.remove("zscaler_zia.email_dlp.email.message_id");
            event.remove("zscaler_zia.email_dlp.email.subject");
            event.remove("zscaler_zia.email_dlp.sender");
            event.remove("zscaler_zia.email_dlp.application.name");
            event.remove("zscaler_zia.email_dlp.owner");
            event.remove("_conf");

            // Painless script
            // Source: boolean dropScalar(Object v) {\n  return v == null || v == '' || v == '0' || v == 'N/A'\n    || v == 'None' || v == 'Unknown' || v == 'Unknown Host' || v == 'Unknown URL';\n}\nvoid handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropScalar(Object v) {\n  return v == null || v == '' || v == '0' || v == 'N/A'\n    || v == 'None' || v == 'Unknown' || v == 'Unknown Host' || v == 'Unknown URL';\n}\nvoid handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nhandleMap(ctx);"#
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
