// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_cybox` pipeline.
pub struct PipelineObjectCybox;

impl Transform for PipelineObjectCybox {
    fn name(&self) -> &str {
        "pipeline_object_cybox"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.direction_id") {
                    if let Some(val) = event.get("_ingest._value.direction_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.direction_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.direction_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_emails_direction_id_to_string")?;
                    event.remove("_ingest._value.direction_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet();\n  if (ctx.email != null && ctx.email.direction != null) {\n      var = ctx.email.direction;\n  } else {\n    if (ctx.email == null)\n    {\n      ctx.email = new HashMap();\n    }\n  }\n\nfor (def email : ctx.ses.cybox.emails) {\n  def direction = email.direction_id;\n  if (params.containsKey(direction.toString())) {\n    def type = params.get(direction.toString());\n    var.add(type);\n  }\n}\nctx.email.put('direction', var)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet();\n  if (ctx.email != null && ctx.email.direction != null) {\n      var = ctx.email.direction;\n  } else {\n    if (ctx.email == null)\n    {\n      ctx.email = new HashMap();\n    }\n  }\n\nfor (def email : ctx.ses.cybox.emails) {\n  def direction = email.direction_id;\n  if (params.containsKey(direction.toString())) {\n    def type = params.get(direction.toString());\n    var.add(type);\n  }\n}\nctx.email.put('direction', var)"#), cached_params!("{\"0\":\"unknown\",\"1\":\"inbound\",\"2\":\"outbound\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_email_direction")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    event.append_unique("email.from.address", json!(event.get("_ingest._value.header_from").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    event.append_unique("email.subject", json!(event.get("_ingest._value.header_subject").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    foreach_array(event, "_ingest._value.header_to", |event| {
                    event.append_unique("email.to.address", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.sender_ip") {
                    if let Some(val) = event.get("_ingest._value.sender_ip") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.sender_ip".into(),
                    message,
                    })?;
                    event.set("_ingest._value.sender_ip", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_emails_sender_ip_to_ip")?;
                    event.remove("_ingest._value.sender_ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    let _cond = { event.has_value("ses.container.networks") };
                    if _cond {
                    event.append_unique("related.ip", json!(event.get("_ingest._value.sender_ip").map_or_else(String::new, template_to_string)))?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.size") {
                    if let Some(val) = event.get("_ingest._value.size") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.size".into(),
                    message,
                    })?;
                    event.set("_ingest._value.size", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_emails_size_to_long")?;
                    event.remove("_ingest._value.size");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.files").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.accessed") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                        Some(parsed) => event.set("_ingest._value.accessed", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.accessed".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cybox_files_accessed")?;
                        event.remove("_ingest._value.accessed");
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
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.files", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.accessed", json!(event.get("_ingest._value.accessed").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.attribute_ids == null || !file.containsKey('attribute_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.attribute_ids.length; j++) {\n    if (file.attribute_ids[j] != null) {\n      new_ids.add(file.attribute_ids[j].toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.attribute_ids = new_ids;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.attribute_ids == null || !file.containsKey('attribute_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.attribute_ids.length; j++) {\n    if (file.attribute_ids[j] != null) {\n      new_ids.add(file.attribute_ids[j].toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.attribute_ids = new_ids;\n  }\n}"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_convert_attribute_ids")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def file : ctx.ses.cybox.files) {\n  if (file.attribute_ids == null) {\n    continue;\n  }\n  for (def id : file.attribute_ids) {\n    def type = params[id.toString()];\n    if (type != null) {\n      var.add(type);\n    }\n  }\n} ctx.file.put('attributes', var)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet();\n  if (ctx.file != null && ctx.file.attributes != null) {\n      var = ctx.file.attributes;\n  } else {\n    if (ctx.file == null)\n    {\n      ctx.file = new HashMap();\n    }\n  }\nfor (def file : ctx.ses.cybox.files) {\n  if (file.attribute_ids == null) {\n    continue;\n  }\n  for (def id : file.attribute_ids) {\n    def type = params[id.toString()];\n    if (type != null) {\n      var.add(type);\n    }\n  }\n} ctx.file.put('attributes', var)"#), cached_params!("{\"1\":\"archive\",\"2\":\"compressed\",\"3\":\"directory\",\"4\":\"encrypted\",\"5\":\"hidden\",\"8\":\"readonly\",\"11\":\"system\",\"16\":\"execute\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_file_attributes")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.attributes") {
                    if let Some(val) = event.get("_ingest._value.attributes") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.attributes".into(),
                    message,
                    })?;
                    event.set("_ingest._value.attributes", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_attributes_to_long")?;
                    event.remove("_ingest._value.attributes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.confidentiality_id") {
                    if let Some(val) = event.get("_ingest._value.confidentiality_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.confidentiality_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.confidentiality_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_confidentiality_id_to_string")?;
                    event.remove("_ingest._value.confidentiality_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.content_type.family_id") {
                    if let Some(val) = event.get("_ingest._value.content_type.family_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.content_type.family_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.content_type.family_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_content_type_family_id_to_string")?;
                    event.remove("_ingest._value.content_type.family_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.content_type.type_id") {
                    if let Some(val) = event.get("_ingest._value.content_type.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.content_type.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.content_type.type_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_content_type_type_id_to_string")?;
                    event.remove("_ingest._value.content_type.type_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.files").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.created") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                        Some(parsed) => event.set("_ingest._value.created", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.created".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cybox_files_created")?;
                        event.remove("_ingest._value.created");
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
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.files", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.created", json!(event.get("_ingest._value.created").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.is_system") {
                    if let Some(val) = event.get("_ingest._value.is_system") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.is_system".into(),
                    message,
                    })?;
                    event.set("_ingest._value.is_system", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_is_system_to_boolean")?;
                    event.remove("_ingest._value.is_system");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.hash.md5", json!(event.get("_ingest._value.md5").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.mime_type", json!(event.get("_ingest._value.mime_type").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.files").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.modified") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                        Some(parsed) => event.set("_ingest._value.modified", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.modified".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cybox_files_modified")?;
                        event.remove("_ingest._value.modified");
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
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.files", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.directory", json!(event.get("_ingest._value.folder").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.path", json!(event.get("_ingest._value.path").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.rep_discovered_band") {
                    if let Some(val) = event.get("_ingest._value.rep_discovered_band") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.rep_discovered_band".into(),
                    message,
                    })?;
                    event.set("_ingest._value.rep_discovered_band", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_discovered_band_to_long")?;
                    event.remove("_ingest._value.rep_discovered_band");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.files").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.rep_discovered_date") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                        Some(parsed) => event.set("_ingest._value.rep_discovered_date", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.rep_discovered_date".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cybox_files_rep_discovered_date")?;
                        event.remove("_ingest._value.rep_discovered_date");
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
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.files", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.rep_prevalence") {
                    if let Some(val) = event.get("_ingest._value.rep_prevalence") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.rep_prevalence".into(),
                    message,
                    })?;
                    event.set("_ingest._value.rep_prevalence", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_prevalence_to_long")?;
                    event.remove("_ingest._value.rep_prevalence");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.rep_prevalence_band") {
                    if let Some(val) = event.get("_ingest._value.rep_prevalence_band") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.rep_prevalence_band".into(),
                    message,
                    })?;
                    event.set("_ingest._value.rep_prevalence_band", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_prevalence_band_to_long")?;
                    event.remove("_ingest._value.rep_prevalence_band");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.rep_score") {
                    if let Some(val) = event.get("_ingest._value.rep_score") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.rep_score".into(),
                    message,
                    })?;
                    event.set("_ingest._value.rep_score", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_score_to_long")?;
                    event.remove("_ingest._value.rep_score");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.rep_score_band") {
                    if let Some(val) = event.get("_ingest._value.rep_score_band") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.rep_score_band".into(),
                    message,
                    })?;
                    event.set("_ingest._value.rep_score_band", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_rep_score_band_to_long")?;
                    event.remove("_ingest._value.rep_score_band");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.hash.sha1", json!(event.get("_ingest._value.sha1").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.files").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.signature_created_date") {
                        match parse_date_out(&date_str, &["UNIX_MS", "ISO8601"], None, None) {
                        Some(parsed) => event.set("_ingest._value.signature_created_date", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.signature_created_date".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cybox_files_signature_created_date")?;
                        event.remove("_ingest._value.signature_created_date");
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
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.files", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.x509.issuer.distinguished_name", json!(event.get("_ingest._value.signature_issuer").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.signature_level_id") {
                    if let Some(val) = event.get("_ingest._value.signature_level_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.signature_level_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.signature_level_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_signature_level_id_to_string")?;
                    event.remove("_ingest._value.signature_level_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.x509.serial_number", json!(event.get("_ingest._value.signature_serial_number").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.signature_value") {
                    if let Some(val) = event.get("_ingest._value.signature_value") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.signature_value".into(),
                    message,
                    })?;
                    event.set("_ingest._value.signature_value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_signature_value_to_long")?;
                    event.remove("_ingest._value.signature_value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.signature_value_ids == null || !file.containsKey('signature_value_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.signature_value_ids.length; j++) {\n    def value_id = file.signature_value_ids[j];\n    if (value_id != null) {\n      new_ids.add(value_id.toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.signature_value_ids = new_ids;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"for (int i = 0; i < ctx.ses.cybox.files.length; i++) {\n  def file = ctx.ses.cybox.files[i];\n  if (file.signature_value_ids == null || !file.containsKey('signature_value_ids')) {\n    continue;\n  }\n  def new_ids = [];\n  for (int j = 0; j < file.signature_value_ids.length; j++) {\n    def value_id = file.signature_value_ids[j];\n    if (value_id != null) {\n      new_ids.add(value_id.toString());\n    }\n  }\n  if (new_ids.length != 0) {\n    file.signature_value_ids = new_ids;\n  }\n}"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_convert_signature_value_ids")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.size") {
                    if let Some(val) = event.get("_ingest._value.size") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.size".into(),
                    message,
                    })?;
                    event.set("_ingest._value.size", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_size_to_long")?;
                    event.remove("_ingest._value.size");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.size", json!(event.get("_ingest._value.size").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.size_compressed") {
                    if let Some(val) = event.get("_ingest._value.size_compressed") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.size_compressed".into(),
                    message,
                    })?;
                    event.set("_ingest._value.size_compressed", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_size_compressed_to_long")?;
                    event.remove("_ingest._value.size_compressed");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.src_ip") {
                    if let Some(val) = event.get("_ingest._value.src_ip") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.src_ip".into(),
                    message,
                    })?;
                    event.set("_ingest._value.src_ip", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_src_ip_to_ip")?;
                    event.remove("_ingest._value.src_ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.files").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        let _cond = { event.has_value("ses.cybox.files") };
                        if _cond {
                        event.append_unique("related.ip", json!(event.get("_ingest._value.src_ip").map_or_else(String::new, template_to_string)))?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.files", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.type_id") {
                    if let Some(val) = event.get("_ingest._value.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.type_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_type_id_to_string")?;
                    event.remove("_ingest._value.type_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def var = new HashSet(); if (ctx.file != null && ctx.file.type != null) {\n    var = ctx.file.type;\n} else {\n  if (ctx.file == null)\n  {\n    ctx.file = new HashMap();\n  }\n}\nfor (def file : ctx.ses.cybox.files) {\n  if (file.type_id == null) {\n    continue;\n  }\n  def type = params[file.type_id.toString()];\n  if (type != null) {\n      var.add(type);\n  }  \n}\nctx.file.put('type', var); 
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def var = new HashSet(); if (ctx.file != null && ctx.file.type != null) {\n    var = ctx.file.type;\n} else {\n  if (ctx.file == null)\n  {\n    ctx.file = new HashMap();\n  }\n}\nfor (def file : ctx.ses.cybox.files) {\n  if (file.type_id == null) {\n    continue;\n  }\n  def type = params[file.type_id.toString()];\n  if (type != null) {\n      var.add(type);\n  }  \n}\nctx.file.put('type', var); "#), cached_params!("{\"1\":\"file\",\"2\":\"dir\",\"6\":\"symlink\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_add_file_type")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    if event.has_value("_ingest._value.url.category_ids") {
                    foreach_array(event, "_ingest._value.url.category_ids", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_category_ids_to_string")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.url.port") {
                    if let Some(val) = event.get("_ingest._value.url.port") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.url.port".into(),
                    message,
                    })?;
                    event.set("_ingest._value.url.port", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_port_to_long")?;
                    event.remove("_ingest._value.url.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    if event.has_value("_ingest._value.url.referrer_category_ids") {
                    foreach_array(event, "_ingest._value.url.referrer_category_ids", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_referrer_category_ids_to_string")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.url.rep_score_id") {
                    if let Some(val) = event.get("_ingest._value.url.rep_score_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.url.rep_score_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.url.rep_score_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_files_url_rep_score_id_to_string")?;
                    event.remove("_ingest._value.url.rep_score_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.ipv4s").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.ipv4s", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_ipv4s_to_ip")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.ipv4s").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.ipv4s").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        let _cond = { event.has_value("ses.cybox.ipv4s") };
                        if _cond {
                        event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.ipv4s", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.ipv6s").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.ipv6s", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_ipv6s_to_ip")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.ipv6s").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("ses.cybox.ipv6s").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        let _cond = { event.has_value("ses.cybox.ipv6s") };
                        if _cond {
                        event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("ses.cybox.ipv6s", Value::Array(out))?;
                }
            }

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    foreach_array(event, "_ingest._value.category_ids", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_category_ids_to_string")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                })?;
            }

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    event.append_unique("url.path", json!(event.get("_ingest._value.path").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.port") {
                    if let Some(val) = event.get("_ingest._value.port") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.port".into(),
                    message,
                    })?;
                    event.set("_ingest._value.port", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_port_to_long")?;
                    event.remove("_ingest._value.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    event.append_unique("url.port", json!(event.get("_ingest._value.port").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("url.port").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "url.port", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_url_port_to_long")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    event.append_unique("url.query", json!(event.get("_ingest._value.query").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    foreach_array(event, "_ingest._value.referrer_category_ids", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_referrer_category_ids_to_string")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                })?;
            }

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.rep_score_id") {
                    if let Some(val) = event.get("_ingest._value.rep_score_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.rep_score_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.rep_score_id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cybox_urls_rep_score_id_to_string")?;
                    event.remove("_ingest._value.rep_score_id");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    event.append_unique("url.scheme", json!(event.get("_ingest._value.scheme").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    event.append_unique("url.full", json!(event.get("_ingest._value.text").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.md5").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.sha1").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.sha2").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("file.hash.sha256", json!(event.get("_ingest._value.sha2").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.parent_sha2").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.src_name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.emails").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.emails", |event| {
                    let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                    if _cond {
                    event.remove("_ingest._value.header_from");
                    event.remove("_ingest._value.header_subject");
                    event.remove("_ingest._value.header_to");
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.files").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.files", |event| {
                    let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                    if _cond {
                    event.remove("_ingest._value.accessed");
                    event.remove("_ingest._value.created");
                    event.remove("_ingest._value.md5");
                    event.remove("_ingest._value.sha1");
                    event.remove("_ingest._value.sha2");
                    event.remove("_ingest._value.mime_type");
                    event.remove("_ingest._value.name");
                    event.remove("_ingest._value.folder");
                    event.remove("_ingest._value.path");
                    event.remove("_ingest._value.size");
                    event.remove("_ingest._value.signature_issuer");
                    event.remove("_ingest._value.signature_serial_number");
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("ses.cybox.urls").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "ses.cybox.urls", |event| {
                    let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                    if _cond {
                    event.remove("_ingest._value.text");
                    event.remove("_ingest._value.path");
                    event.remove("_ingest._value.port");
                    event.remove("_ingest._value.query");
                    event.remove("_ingest._value.scheme");
                    }
                    Ok(())
                })?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
