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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("email"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Sublime Security"))?;

            event.set("observer.product", json!("Sublime Security"))?;

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    if event.has_value("_ingest._value.content_id") {
                        event.rename("_ingest._value.content_id", "_ingest._value.content.id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    if event.has_value("_ingest._value.content_transfer_encoding") {
                        event.rename(
                            "_ingest._value.content_transfer_encoding",
                            "_ingest._value.content.transfer_encoding",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    if event.has_value("_ingest._value.content_type") {
                        event
                            .rename("_ingest._value.content_type", "_ingest._value.content.type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    if event.has_value("_ingest._value.file_name") {
                        event.rename("_ingest._value.file_name", "_ingest._value.file.name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    if event.has_value("_ingest._value.file_type") {
                        event.rename("_ingest._value.file_type", "_ingest._value.file.type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    if event.has_value("_ingest._value.file_extension") {
                        event.rename(
                            "_ingest._value.file_extension",
                            "_ingest._value.file.extension",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.attachments", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.size") {
                            if let Some(val) = event.get("_ingest._value.size") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.size".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_attachments_size_to_long",
                        )?;
                        event.remove("_ingest._value.size");
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

            if event.has_value("json.attachments") {
                event.rename(
                    "json.attachments",
                    "sublime_security.email_message.attachments",
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.email_message.attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def attachmentList = new ArrayList();\nfor (attachment in ctx.sublime_security.email_message.attachments) {\n  def object = new HashMap();\n  object.put('file', new HashMap());\n  object.file.put('hash', new HashMap());\n  object.file.put('mime_type', attachment.content.type);\n  object.file.put('extension', attachment.file.extension);\n  object.file.put('name', attachment.file.name);\n  object.file.put('size', attachment.size);\n  object.file.hash.put('md5', attachment.md5);\n  object.file.hash.put('sha1', attachment.sha1);\n  object.file.hash.put('sha256', attachment.sha256);\n  attachmentList.add(object);\n}\nctx.put('email',new HashMap());\nctx.email.attachments = attachmentList;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def attachmentList = new ArrayList();\nfor (attachment in ctx.sublime_security.email_message.attachments) {\n  def object = new HashMap();\n  object.put('file', new HashMap());\n  object.file.put('hash', new HashMap());\n  object.file.put('mime_type', attachment.content.type);\n  object.file.put('extension', attachment.file.extension);\n  object.file.put('name', attachment.file.name);\n  object.file.put('size', attachment.size);\n  object.file.hash.put('md5', attachment.md5);\n  object.file.hash.put('sha1', attachment.sha1);\n  object.file.hash.put('sha256', attachment.sha256);\n  attachmentList.add(object);\n}\nctx.put('email',new HashMap());\nctx.email.attachments = attachmentList;"#
                    ),
                )?;
            }

            let _cond = { event.get("email.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: for (attachment in ctx.email.attachments) {\n  if (attachment.file.extension != null && attachment.file.extension.startsWith('.')) {\n    String extension = attachment.file.extension.substring(1);\n    attachment.file.extension = extension;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (attachment in ctx.email.attachments) {\n  if (attachment.file.extension != null && attachment.file.extension.startsWith('.')) {\n    String extension = attachment.file.extension.substring(1);\n    attachment.file.extension = extension;\n  }\n}"#
                    ),
                )?;
            }

            if event.has_value("json.body.current_thread.text") {
                event.rename(
                    "json.body.current_thread.text",
                    "sublime_security.email_message.body.current_thread.text",
                )?;
            }

            if event.has_value("json.body.html.charset") {
                event.rename(
                    "json.body.html.charset",
                    "sublime_security.email_message.body.html.charset",
                )?;
            }

            if event.has_value("json.body.html.content_transfer_encoding") {
                event.rename(
                    "json.body.html.content_transfer_encoding",
                    "sublime_security.email_message.body.html.content_transfer_encoding",
                )?;
            }

            if event.has_value("json.body.html.display_text") {
                event.rename(
                    "json.body.html.display_text",
                    "sublime_security.email_message.body.html.display_text",
                )?;
            }

            if event.has_value("json.body.html.inner_text") {
                event.rename(
                    "json.body.html.inner_text",
                    "sublime_security.email_message.body.html.inner_text",
                )?;
            }

            if event.has_value("json.body.html.raw") {
                event.rename(
                    "json.body.html.raw",
                    "sublime_security.email_message.body.html.raw",
                )?;
            }

            let _cond = { event.get("json.body.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.ips", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.ip") {
                            if let Some(val) = event.get("_ingest._value.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
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
                            "convert_body_ips_ip_to_ip",
                        )?;
                        event.remove("_ingest._value.ip");
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

            let _cond = { event.get("json.body.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.ips", |event| {
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
            }

            if event.has_value("json.body.ips") {
                event.rename("json.body.ips", "sublime_security.email_message.body.ips")?;
            }

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.display_url.domain.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.display_url.port") {
                            if let Some(val) = event.get("_ingest._value.display_url.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.display_url.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.display_url.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_body_links_display_url_port_to_long",
                        )?;
                        event.remove("_ingest._value.display_url.port");
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

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.display_url.domain.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.display_url.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.display_url.domain.valid") {
                            if let Some(val) = event.get("_ingest._value.display_url.domain.valid")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.display_url.domain.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.display_url.domain.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_body_links_display_url_domain_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.display_url.domain.valid");
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

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.href_url.domain.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.href_url.domain.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.href_url.port") {
                            if let Some(val) = event.get("_ingest._value.href_url.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.href_url.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.href_url.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_body_links_href_url_port_to_long",
                        )?;
                        event.remove("_ingest._value.href_url.port");
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

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.href_url.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.href_url.domain.valid") {
                            if let Some(val) = event.get("_ingest._value.href_url.domain.valid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.href_url.domain.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.href_url.domain.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_body_links_href_url_domain_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.href_url.domain.valid");
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

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.body.links", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.mismatched") {
                            if let Some(val) = event.get("_ingest._value.mismatched") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.mismatched".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.mismatched", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_body_links_mismatched_to_boolean",
                        )?;
                        event.remove("_ingest._value.mismatched");
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

            let _cond = { event.get("json.body.links").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def links = new ArrayList();\nfor (link in ctx.json.body.links) {\n  def object = new HashMap();\n  if(link?.href_url != null) {\n    object.put('domain', link.href_url.domain.domain);\n    object.put('subdomain', link.href_url.domain.subdomain);\n    object.put('top_level_domain', link.href_url.domain.tld);\n    object.put('fragment', link.href_url.fragment);\n    object.put('password', link.href_url.password);\n    object.put('path', link.href_url.path);\n    object.put('port', link.href_url.port);\n    object.put('query', link.href_url.query_params);\n    object.put('scheme', link.href_url.scheme);\n    object.put('full', link.href_url.url);\n    object.put('username', link.href_url.username);\n    links.add(object);\n  }\n}\nctx.put('url',new HashMap());\nctx.url = links;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def links = new ArrayList();\nfor (link in ctx.json.body.links) {\n  def object = new HashMap();\n  if(link?.href_url != null) {\n    object.put('domain', link.href_url.domain.domain);\n    object.put('subdomain', link.href_url.domain.subdomain);\n    object.put('top_level_domain', link.href_url.domain.tld);\n    object.put('fragment', link.href_url.fragment);\n    object.put('password', link.href_url.password);\n    object.put('path', link.href_url.path);\n    object.put('port', link.href_url.port);\n    object.put('query', link.href_url.query_params);\n    object.put('scheme', link.href_url.scheme);\n    object.put('full', link.href_url.url);\n    object.put('username', link.href_url.username);\n    links.add(object);\n  }\n}\nctx.put('url',new HashMap());\nctx.url = links;"#
                    ),
                )?;
            }

            if event.has_value("json.body.links") {
                event.rename(
                    "json.body.links",
                    "sublime_security.email_message.body.links",
                )?;
            }

            if event.has_value("json.body.plain.charset") {
                event.rename(
                    "json.body.plain.charset",
                    "sublime_security.email_message.body.plain.charset",
                )?;
            }

            if event.has_value("json.body.plain.content_transfer_encoding") {
                event.rename(
                    "json.body.plain.content_transfer_encoding",
                    "sublime_security.email_message.body.plain.content_transfer_encoding",
                )?;
            }

            if event.has_value("json.body.plain.raw") {
                event.rename(
                    "json.body.plain.raw",
                    "sublime_security.email_message.body.plain.raw",
                )?;
            }

            if event.has_value("json._errors") {
                event.rename("json._errors", "sublime_security.email_message.errors")?;
            }

            let _cond = {
                event.has_value("json.external.created_at")
                    && event.get_str("json.external.created_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.external.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "sublime_security.email_message.external.created_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.external.created_at".into(),
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
                        "date_external_created_at",
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
                .get("sublime_security.email_message.external.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.origination_timestamp", v)?;
            }

            if event.has_value("json.external.message_id") {
                event.rename(
                    "json.external.message_id",
                    "sublime_security.email_message.external.message_id",
                )?;
            }

            if event.has_value("json.external.route_type") {
                event.rename(
                    "json.external.route_type",
                    "sublime_security.email_message.external.route_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.external.spam") {
                    if let Some(val) = event.get("json.external.spam") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.external.spam".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.external.spam", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_external_spam_to_boolean",
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
                if event.has_value("json.external.spam_folder") {
                    if let Some(val) = event.get("json.external.spam_folder") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.external.spam_folder".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.external.spam_folder",
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
                    "convert_external_spam_folder_to_boolean",
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

            if event.has_value("json.external.thread_id") {
                event.rename(
                    "json.external.thread_id",
                    "sublime_security.email_message.external.thread_id",
                )?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.action") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.action",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.action",
                )?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.disposition") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.disposition",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.disposition",
                )?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.from.domain") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.from.domain",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.from.domain",
                )?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.auth_summary.dmarc.details.from.domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.auth_summary.dmarc.details.from.punycode") {
                event.rename("json.headers.auth_summary.dmarc.details.from.punycode", "sublime_security.email_message.headers.auth_summary.dmarc.details.from.punycode")?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.from.root_domain") {
                event.rename("json.headers.auth_summary.dmarc.details.from.root_domain", "sublime_security.email_message.headers.auth_summary.dmarc.details.from.root_domain")?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.auth_summary.dmarc.details.from.root_domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.auth_summary.dmarc.details.from.sld") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.from.sld",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.from.sld",
                )?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.from.subdomain") {
                event.rename("json.headers.auth_summary.dmarc.details.from.subdomain", "sublime_security.email_message.headers.auth_summary.dmarc.details.from.subdomain")?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.from.tld") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.from.tld",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.from.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.auth_summary.dmarc.details.from.valid") {
                    if let Some(val) =
                        event.get("json.headers.auth_summary.dmarc.details.from.valid")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.dmarc.details.from.valid".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.headers.auth_summary.dmarc.details.from.valid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_headers_auth_summary_dmarc_details_from_valid_to_boolean",
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

            if event.has_value("json.headers.auth_summary.dmarc.details.policy") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.policy",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.policy",
                )?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.sub_policy") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.sub_policy",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.sub_policy",
                )?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.verdict") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.verdict",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.verdict",
                )?;
            }

            if event.has_value("json.headers.auth_summary.dmarc.details.version") {
                event.rename(
                    "json.headers.auth_summary.dmarc.details.version",
                    "sublime_security.email_message.headers.auth_summary.dmarc.details.version",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.auth_summary.dmarc.pass") {
                    if let Some(val) = event.get("json.headers.auth_summary.dmarc.pass") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.dmarc.pass".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.auth_summary.dmarc.pass",
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
                    "convert_headers_auth_summary_dmarc_pass_to_boolean",
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
                if event.has_value("json.headers.auth_summary.dmarc.received_hop") {
                    if let Some(val) = event.get("json.headers.auth_summary.dmarc.received_hop") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.dmarc.received_hop".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.headers.auth_summary.dmarc.received_hop", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_headers_auth_summary_dmarc_received_hop_to_long",
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
                event.has_value("json.headers.auth_summary.spf.details.client_ip.ip")
                    && event.get_str("json.headers.auth_summary.spf.details.client_ip.ip")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.headers.auth_summary.spf.details.client_ip.ip") {
                        if let Some(val) =
                            event.get("json.headers.auth_summary.spf.details.client_ip.ip")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.headers.auth_summary.spf.details.client_ip.ip"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sublime_security.email_message.headers.auth_summary.spf.details.client_ip.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_headers_auth_summary_spf_details_client_ip_ip_to_ip",
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

            event.append_unique("related.ip", json!(event.get("sublime_security.email_message.headers.auth_summary.spf.details.client_ip.ip").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.auth_summary.spf.details.description") {
                event.rename(
                    "json.headers.auth_summary.spf.details.description",
                    "sublime_security.email_message.headers.auth_summary.spf.details.description",
                )?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.designator") {
                event.rename(
                    "json.headers.auth_summary.spf.details.designator",
                    "sublime_security.email_message.headers.auth_summary.spf.details.designator",
                )?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.helo.domain") {
                event.rename(
                    "json.headers.auth_summary.spf.details.helo.domain",
                    "sublime_security.email_message.headers.auth_summary.spf.details.helo.domain",
                )?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.auth_summary.spf.details.helo.domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.auth_summary.spf.details.helo.punycode") {
                event.rename(
                    "json.headers.auth_summary.spf.details.helo.punycode",
                    "sublime_security.email_message.headers.auth_summary.spf.details.helo.punycode",
                )?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.helo.root_domain") {
                event.rename("json.headers.auth_summary.spf.details.helo.root_domain", "sublime_security.email_message.headers.auth_summary.spf.details.helo.root_domain")?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.auth_summary.spf.details.helo.root_domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.auth_summary.spf.details.helo.sld") {
                event.rename(
                    "json.headers.auth_summary.spf.details.helo.sld",
                    "sublime_security.email_message.headers.auth_summary.spf.details.helo.sld",
                )?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.helo.subdomain") {
                event.rename("json.headers.auth_summary.spf.details.helo.subdomain", "sublime_security.email_message.headers.auth_summary.spf.details.helo.subdomain")?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.helo.tld") {
                event.rename(
                    "json.headers.auth_summary.spf.details.helo.tld",
                    "sublime_security.email_message.headers.auth_summary.spf.details.helo.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.auth_summary.spf.details.helo.valid") {
                    if let Some(val) = event.get("json.headers.auth_summary.spf.details.helo.valid")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.spf.details.helo.valid".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.headers.auth_summary.spf.details.helo.valid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_headers_auth_summary_spf_details_helo_valid_to_boolean",
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

            if event.has_value("json.headers.auth_summary.spf.details.server.domain") {
                event.rename(
                    "json.headers.auth_summary.spf.details.server.domain",
                    "sublime_security.email_message.headers.auth_summary.spf.details.server.domain",
                )?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.auth_summary.spf.details.server.domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.auth_summary.spf.details.server.punycode") {
                event.rename("json.headers.auth_summary.spf.details.server.punycode", "sublime_security.email_message.headers.auth_summary.spf.details.server.punycode")?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.server.root_domain") {
                event.rename("json.headers.auth_summary.spf.details.server.root_domain", "sublime_security.email_message.headers.auth_summary.spf.details.server.root_domain")?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.auth_summary.spf.details.server.root_domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.auth_summary.spf.details.server.sld") {
                event.rename(
                    "json.headers.auth_summary.spf.details.server.sld",
                    "sublime_security.email_message.headers.auth_summary.spf.details.server.sld",
                )?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.server.subdomain") {
                event.rename("json.headers.auth_summary.spf.details.server.subdomain", "sublime_security.email_message.headers.auth_summary.spf.details.server.subdomain")?;
            }

            if event.has_value("json.headers.auth_summary.spf.details.server.tld") {
                event.rename(
                    "json.headers.auth_summary.spf.details.server.tld",
                    "sublime_security.email_message.headers.auth_summary.spf.details.server.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.auth_summary.spf.details.server.valid") {
                    if let Some(val) =
                        event.get("json.headers.auth_summary.spf.details.server.valid")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.spf.details.server.valid".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.headers.auth_summary.spf.details.server.valid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_headers_auth_summary_spf_details_server_valid_to_boolean",
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

            if event.has_value("json.headers.auth_summary.spf.details.verdict") {
                event.rename(
                    "json.headers.auth_summary.spf.details.verdict",
                    "sublime_security.email_message.headers.auth_summary.spf.details.verdict",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.auth_summary.spf.error") {
                    if let Some(val) = event.get("json.headers.auth_summary.spf.error") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.spf.error".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.auth_summary.spf.error",
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
                    "convert_headers_auth_summary_spf_error_to_boolean",
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
                if event.has_value("json.headers.auth_summary.spf.pass") {
                    if let Some(val) = event.get("json.headers.auth_summary.spf.pass") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.spf.pass".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.auth_summary.spf.pass",
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
                    "convert_headers_auth_summary_spf_pass_to_boolean",
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
                if event.has_value("json.headers.auth_summary.spf.received_hop") {
                    if let Some(val) = event.get("json.headers.auth_summary.spf.received_hop") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.auth_summary.spf.received_hop".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.auth_summary.spf.received_hop",
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
                    "convert_headers_auth_summary_spf_received_hop_to_long",
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
                event.has_value("json.headers.date")
                    && event.get_str("json.headers.date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.headers.date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("sublime_security.email_message.headers.date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.headers.date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_headers_date")?;
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

            if event.has_value("json.headers.date_original_offset") {
                event.rename(
                    "json.headers.date_original_offset",
                    "sublime_security.email_message.headers.date_original_offset",
                )?;
            }

            if event.has_value("json.headers.delivered_to.domain.domain") {
                event.rename(
                    "json.headers.delivered_to.domain.domain",
                    "sublime_security.email_message.headers.delivered_to.domain.domain",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.headers.delivered_to.domain.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if event.has_value("json.headers.delivered_to.domain.punycode") {
                event.rename(
                    "json.headers.delivered_to.domain.punycode",
                    "sublime_security.email_message.headers.delivered_to.domain.punycode",
                )?;
            }

            if event.has_value("json.headers.delivered_to.domain.root_domain") {
                event.rename(
                    "json.headers.delivered_to.domain.root_domain",
                    "sublime_security.email_message.headers.delivered_to.domain.root_domain",
                )?;
            }

            if event.has_value("json.headers.delivered_to.domain.sld") {
                event.rename(
                    "json.headers.delivered_to.domain.sld",
                    "sublime_security.email_message.headers.delivered_to.domain.sld",
                )?;
            }

            if event.has_value("json.headers.delivered_to.domain.subdomain") {
                event.rename(
                    "json.headers.delivered_to.domain.subdomain",
                    "sublime_security.email_message.headers.delivered_to.domain.subdomain",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.headers.delivered_to.domain.subdomain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.subdomain", v)?;
            }

            if event.has_value("json.headers.delivered_to.domain.tld") {
                event.rename(
                    "json.headers.delivered_to.domain.tld",
                    "sublime_security.email_message.headers.delivered_to.domain.tld",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.headers.delivered_to.domain.tld")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.top_level_domain", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.delivered_to.domain.valid") {
                    if let Some(val) = event.get("json.headers.delivered_to.domain.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.delivered_to.domain.valid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.delivered_to.domain.valid",
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
                    "convert_headers_delivered_to_domain_valid_to_boolean",
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

            if event.has_value("json.headers.delivered_to.email") {
                event.rename(
                    "json.headers.delivered_to.email",
                    "sublime_security.email_message.headers.delivered_to.email",
                )?;
            }

            if event.has_value("json.headers.delivered_to.local_part") {
                event.rename(
                    "json.headers.delivered_to.local_part",
                    "sublime_security.email_message.headers.delivered_to.local_part",
                )?;
            }

            let _cond = {
                event
                    .get("json.headers.domains")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.domains", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.headers.domains")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.domains", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.headers.domains")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.domains", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.valid") {
                            if let Some(val) = event.get("_ingest._value.valid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_headers_domains_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.valid");
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

            if event.has_value("json.headers.domains") {
                event.rename(
                    "json.headers.domains",
                    "sublime_security.email_message.headers.domains",
                )?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "_ingest._value.authentication_results.dmarc_details.from.valid",
                        ) {
                            if let Some(val) = event.get(
                                "_ingest._value.authentication_results.dmarc_details.from.valid",
                            ) {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                    path: "_ingest._value.authentication_results.dmarc_details.from.valid".into(),
                    message,
                    }
                                    })?;
                                event.set("_ingest._value.authentication_results.dmarc_details.from.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_headers_hops_authentication_results_dmarc_details_from_valid_to_boolean")?;
                        event.remove(
                            "_ingest._value.authentication_results.dmarc_details.from.valid",
                        );
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.authentication_results.server.valid") {
                            if let Some(val) =
                                event.get("_ingest._value.authentication_results.server.valid")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path:
                                                "_ingest._value.authentication_results.server.valid"
                                                    .into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "_ingest._value.authentication_results.server.valid",
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
                            "convert_headers_hops_authentication_results_server_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.authentication_results.server.valid");
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "_ingest._value.fields", |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.position") {
                                    if let Some(val) = event.get("_ingest._value.position") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.position".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.position", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_headers_hops_fields_position_to_long",
                                )?;
                                event.remove("_ingest._value.position");
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
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def hops = ctx.json.headers.hops;\nfor (int i = 0; i < hops.size(); i++) {\n  def hop = hops[i];\n  if(hop.fields instanceof List) {\n    for (def field : hop.fields) {\n      def lowercaseName = field.name.toLowerCase();\n      field[lowercaseName] = field.value;\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def hops = ctx.json.headers.hops;\nfor (int i = 0; i < hops.size(); i++) {\n  def hop = hops[i];\n  if(hop.fields instanceof List) {\n    for (def field : hop.fields) {\n      def lowercaseName = field.name.toLowerCase();\n      field[lowercaseName] = field.value;\n    }\n  }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.authentication_results.dkim_details.body_hash")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.authentication_results.dmarc_details.from.domain").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.authentication_results.dmarc_details.from.root_domain").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.authentication_results.server.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.authentication_results.server.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.index") {
                            if let Some(val) = event.get("_ingest._value.index") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.index".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.index", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_headers_hops_index_to_long",
                        )?;
                        event.remove("_ingest._value.index");
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.headers.hops").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.received.time")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.received.time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.received.time".into(),
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
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_headers_hops_received_time",
                                )?;
                                event.remove("_ingest._value.received.time");
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
                            "json.headers.hops",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.received_spf.client_ip.ip") {
                            if let Some(val) = event.get("_ingest._value.received_spf.client_ip.ip")
                            {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.received_spf.client_ip.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.received_spf.client_ip.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_headers_hops_received_spf_client_ip_ip_to_ip",
                        )?;
                        event.remove("_ingest._value.received_spf.client_ip.ip");
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.received_spf.client_ip.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.received_spf.helo.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.received_spf.helo.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.received_spf.helo.valid") {
                            if let Some(val) = event.get("_ingest._value.received_spf.helo.valid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.received_spf.helo.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.received_spf.helo.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_headers_hops_received_spf_helo_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.received_spf.helo.valid");
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.received_spf.server.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.received_spf.server.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.received_spf.server.valid") {
                            if let Some(val) = event.get("_ingest._value.received_spf.server.valid")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.received_spf.server.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.received_spf.server.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_headers_hops_received_spf_server_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.received_spf.server.valid");
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "_ingest._value.authentication_results.spf_details.client_ip.ip",
                        ) {
                            if let Some(val) = event.get(
                                "_ingest._value.authentication_results.spf_details.client_ip.ip",
                            ) {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                    path: "_ingest._value.authentication_results.spf_details.client_ip.ip".into(),
                    message,
                    }
                                })?;
                                event.set("_ingest._value.authentication_results.spf_details.client_ip.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_headers_hops_authentication_results_spf_details_client_ip_ip_to_ip")?;
                        event.remove(
                            "_ingest._value.authentication_results.spf_details.client_ip.ip",
                        );
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get(
                                    "_ingest._value.authentication_results.spf_details.client_ip.ip"
                                )
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get(
                                    "_ingest._value.authentication_results.spf_details.helo.domain"
                                )
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.authentication_results.spf_details.helo.root_domain").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "_ingest._value.authentication_results.spf_details.helo.valid",
                        ) {
                            if let Some(val) = event
                                .get("_ingest._value.authentication_results.spf_details.helo.valid")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                    path: "_ingest._value.authentication_results.spf_details.helo.valid".into(),
                    message,
                    }
                                    })?;
                                event.set(
                                    "_ingest._value.authentication_results.spf_details.helo.valid",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_headers_hops_authentication_results_spf_details_helo_valid_to_boolean")?;
                        event
                            .remove("_ingest._value.authentication_results.spf_details.helo.valid");
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

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.authentication_results.spf_details.server.domain").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.authentication_results.spf_details.server.root_domain").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.headers.hops").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.hops", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "_ingest._value.authentication_results.spf_details.server.valid",
                        ) {
                            if let Some(val) = event.get(
                                "_ingest._value.authentication_results.spf_details.server.valid",
                            ) {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                    path: "_ingest._value.authentication_results.spf_details.server.valid".into(),
                    message,
                    }
                                    })?;
                                event.set("_ingest._value.authentication_results.spf_details.server.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_headers_hops_authentication_results_spf_details_server_valid_to_boolean")?;
                        event.remove(
                            "_ingest._value.authentication_results.spf_details.server.valid",
                        );
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

            if event.has_value("json.headers.hops") {
                event.rename(
                    "json.headers.hops",
                    "sublime_security.email_message.headers.hops",
                )?;
            }

            if event.has_value("json.headers.in_reply_to") {
                event.rename(
                    "json.headers.in_reply_to",
                    "sublime_security.email_message.headers.in_reply_to",
                )?;
            }

            let _cond = { event.get("json.headers.ips").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.headers.ips", |event| {
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
            }

            if event.has_value("json.headers.ips") {
                event.rename(
                    "json.headers.ips",
                    "sublime_security.email_message.headers.ips",
                )?;
            }

            if event.has_value("json.headers.mailer") {
                event.rename(
                    "json.headers.mailer",
                    "sublime_security.email_message.headers.mailer",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.headers.mailer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.x_mailer", v)?;
            }

            if event.has_value("email.x_mailer") {
                if let Some(ua_str) = event.get_string("email.x_mailer") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            if event.has_value("json.headers.message_id") {
                event.rename(
                    "json.headers.message_id",
                    "sublime_security.email_message.headers.message_id",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.headers.message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            if event.has_value("json.headers.references") {
                event.rename(
                    "json.headers.references",
                    "sublime_security.email_message.headers.references",
                )?;
            }

            let _cond = {
                event
                    .get("json.headers.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.reply_to", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.headers.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.reply_to", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.headers.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.reply_to", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.email.domain.valid") {
                            if let Some(val) = event.get("_ingest._value.email.domain.valid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.email.domain.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.email.domain.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_headers_reply_to_email_domain_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.email.domain.valid");
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
                    .get("json.headers.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.reply_to", |event| {
                    event.append_unique(
                        "email.reply_to.address",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.headers.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.reply_to", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.headers.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.headers.reply_to", |event| {
                    if event.has_value("_ingest._value.email.email") {
                        event.rename("_ingest._value.email.email", "_ingest._value.email.value")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.headers.reply_to") {
                event.rename(
                    "json.headers.reply_to",
                    "sublime_security.email_message.headers.reply_to",
                )?;
            }

            if event.has_value("json.headers.return_path.domain.domain") {
                event.rename(
                    "json.headers.return_path.domain.domain",
                    "sublime_security.email_message.headers.return_path.domain.domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.headers.return_path.domain.domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.headers.return_path.domain.punycode") {
                event.rename(
                    "json.headers.return_path.domain.punycode",
                    "sublime_security.email_message.headers.return_path.domain.punycode",
                )?;
            }

            if event.has_value("json.headers.return_path.domain.root_domain") {
                event.rename(
                    "json.headers.return_path.domain.root_domain",
                    "sublime_security.email_message.headers.return_path.domain.root_domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get(
                            "sublime_security.email_message.headers.return_path.domain.root_domain"
                        )
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.headers.return_path.domain.sld") {
                event.rename(
                    "json.headers.return_path.domain.sld",
                    "sublime_security.email_message.headers.return_path.domain.sld",
                )?;
            }

            if event.has_value("json.headers.return_path.domain.subdomain") {
                event.rename(
                    "json.headers.return_path.domain.subdomain",
                    "sublime_security.email_message.headers.return_path.domain.subdomain",
                )?;
            }

            if event.has_value("json.headers.return_path.domain.tld") {
                event.rename(
                    "json.headers.return_path.domain.tld",
                    "sublime_security.email_message.headers.return_path.domain.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.return_path.domain.valid") {
                    if let Some(val) = event.get("json.headers.return_path.domain.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.return_path.domain.valid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.return_path.domain.valid",
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
                    "convert_headers_return_path_domain_valid_to_boolean",
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

            if event.has_value("json.headers.return_path.email") {
                event.rename(
                    "json.headers.return_path.email",
                    "sublime_security.email_message.headers.return_path.email",
                )?;
            }

            if event.has_value("json.headers.return_path.local_part") {
                event.rename(
                    "json.headers.return_path.local_part",
                    "sublime_security.email_message.headers.return_path.local_part",
                )?;
            }

            if event.has_value("json.headers.x_authenticated_domain.domain") {
                event.rename(
                    "json.headers.x_authenticated_domain.domain",
                    "sublime_security.email_message.headers.x_authenticated_domain.domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.headers.x_authenticated_domain.domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.headers.x_authenticated_domain.punycode") {
                event.rename(
                    "json.headers.x_authenticated_domain.punycode",
                    "sublime_security.email_message.headers.x_authenticated_domain.punycode",
                )?;
            }

            if event.has_value("json.headers.x_authenticated_domain.root_domain") {
                event.rename(
                    "json.headers.x_authenticated_domain.root_domain",
                    "sublime_security.email_message.headers.x_authenticated_domain.root_domain",
                )?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.x_authenticated_domain.root_domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.x_authenticated_domain.sld") {
                event.rename(
                    "json.headers.x_authenticated_domain.sld",
                    "sublime_security.email_message.headers.x_authenticated_domain.sld",
                )?;
            }

            if event.has_value("json.headers.x_authenticated_domain.subdomain") {
                event.rename(
                    "json.headers.x_authenticated_domain.subdomain",
                    "sublime_security.email_message.headers.x_authenticated_domain.subdomain",
                )?;
            }

            if event.has_value("json.headers.x_authenticated_domain.tld") {
                event.rename(
                    "json.headers.x_authenticated_domain.tld",
                    "sublime_security.email_message.headers.x_authenticated_domain.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.x_authenticated_domain.valid") {
                    if let Some(val) = event.get("json.headers.x_authenticated_domain.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.x_authenticated_domain.valid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.x_authenticated_domain.valid",
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
                    "convert_headers_x_authenticated_domain_valid_to_boolean",
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

            if event.has_value("json.headers.x_authenticated_sender.domain.domain") {
                event.rename(
                    "json.headers.x_authenticated_sender.domain.domain",
                    "sublime_security.email_message.headers.x_authenticated_sender.domain.domain",
                )?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.x_authenticated_sender.domain.domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.x_authenticated_sender.domain.punycode") {
                event.rename(
                    "json.headers.x_authenticated_sender.domain.punycode",
                    "sublime_security.email_message.headers.x_authenticated_sender.domain.punycode",
                )?;
            }

            if event.has_value("json.headers.x_authenticated_sender.domain.root_domain") {
                event.rename("json.headers.x_authenticated_sender.domain.root_domain", "sublime_security.email_message.headers.x_authenticated_sender.domain.root_domain")?;
            }

            event.append_unique("related.hosts", json!(event.get("sublime_security.email_message.headers.x_authenticated_sender.domain.root_domain").map_or_else(String::new, template_to_string)))?;

            if event.has_value("json.headers.x_authenticated_sender.domain.sld") {
                event.rename(
                    "json.headers.x_authenticated_sender.domain.sld",
                    "sublime_security.email_message.headers.x_authenticated_sender.domain.sld",
                )?;
            }

            if event.has_value("json.headers.x_authenticated_sender.domain.subdomain") {
                event.rename("json.headers.x_authenticated_sender.domain.subdomain", "sublime_security.email_message.headers.x_authenticated_sender.domain.subdomain")?;
            }

            if event.has_value("json.headers.x_authenticated_sender.domain.tld") {
                event.rename(
                    "json.headers.x_authenticated_sender.domain.tld",
                    "sublime_security.email_message.headers.x_authenticated_sender.domain.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.x_authenticated_sender.domain.valid") {
                    if let Some(val) = event.get("json.headers.x_authenticated_sender.domain.valid")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.x_authenticated_sender.domain.valid".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.headers.x_authenticated_sender.domain.valid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_headers_x_authenticated_sender_domain_valid_to_boolean",
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

            if event.has_value("json.headers.x_authenticated_sender.email") {
                event.rename(
                    "json.headers.x_authenticated_sender.email",
                    "sublime_security.email_message.headers.x_authenticated_sender.email",
                )?;
            }

            if event.has_value("json.headers.x_authenticated_sender.local_part") {
                event.rename(
                    "json.headers.x_authenticated_sender.local_part",
                    "sublime_security.email_message.headers.x_authenticated_sender.local_part",
                )?;
            }

            let _cond = {
                event.has_value("json.headers.x_client_ip.ip")
                    && event.get_str("json.headers.x_client_ip.ip") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.headers.x_client_ip.ip") {
                        if let Some(val) = event.get("json.headers.x_client_ip.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.headers.x_client_ip.ip".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sublime_security.email_message.headers.x_client_ip.ip",
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
                        "convert_headers_x_client_ip_ip_to_ip",
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
                .get("sublime_security.email_message.headers.x_client_ip.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("sublime_security.email_message.headers.x_client_ip.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event.has_value("json.headers.x_originating_ip.ip")
                    && event.get_str("json.headers.x_originating_ip.ip") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.headers.x_originating_ip.ip") {
                        if let Some(val) = event.get("json.headers.x_originating_ip.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.headers.x_originating_ip.ip".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sublime_security.email_message.headers.x_originating_ip.ip",
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
                        "convert_headers_x_originating_ip_ip_to_ip",
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

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("sublime_security.email_message.headers.x_originating_ip.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.headers.x_secure_server_account") {
                event.rename(
                    "json.headers.x_secure_server_account",
                    "sublime_security.email_message.headers.x_secure_server_account",
                )?;
            }

            if event.has_value("json.headers.x_sender.domain.domain") {
                event.rename(
                    "json.headers.x_sender.domain.domain",
                    "sublime_security.email_message.headers.x_sender.domain.domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.headers.x_sender.domain.domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.headers.x_sender.domain.punycode") {
                event.rename(
                    "json.headers.x_sender.domain.punycode",
                    "sublime_security.email_message.headers.x_sender.domain.punycode",
                )?;
            }

            if event.has_value("json.headers.x_sender.domain.root_domain") {
                event.rename(
                    "json.headers.x_sender.domain.root_domain",
                    "sublime_security.email_message.headers.x_sender.domain.root_domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.headers.x_sender.domain.root_domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.headers.x_sender.domain.sld") {
                event.rename(
                    "json.headers.x_sender.domain.sld",
                    "sublime_security.email_message.headers.x_sender.domain.sld",
                )?;
            }

            if event.has_value("json.headers.x_sender.domain.subdomain") {
                event.rename(
                    "json.headers.x_sender.domain.subdomain",
                    "sublime_security.email_message.headers.x_sender.domain.subdomain",
                )?;
            }

            if event.has_value("json.headers.x_sender.domain.tld") {
                event.rename(
                    "json.headers.x_sender.domain.tld",
                    "sublime_security.email_message.headers.x_sender.domain.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.headers.x_sender.domain.valid") {
                    if let Some(val) = event.get("json.headers.x_sender.domain.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.headers.x_sender.domain.valid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.headers.x_sender.domain.valid",
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
                    "convert_headers_x_sender_domain_valid_to_boolean",
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

            if event.has_value("json.headers.x_sender.email") {
                event.rename(
                    "json.headers.x_sender.email",
                    "sublime_security.email_message.headers.x_sender.email",
                )?;
            }

            if event.has_value("json.headers.x_sender.local_part") {
                event.rename(
                    "json.headers.x_sender.local_part",
                    "sublime_security.email_message.headers.x_sender.local_part",
                )?;
            }

            if event.has_value("json.mailbox.display_name") {
                event.rename(
                    "json.mailbox.display_name",
                    "sublime_security.email_message.mailbox.display_name",
                )?;
            }

            if event.has_value("json.mailbox.email.domain.domain") {
                event.rename(
                    "json.mailbox.email.domain.domain",
                    "sublime_security.email_message.mailbox.email.domain.domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.mailbox.email.domain.domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.mailbox.email.domain.punycode") {
                event.rename(
                    "json.mailbox.email.domain.punycode",
                    "sublime_security.email_message.mailbox.email.domain.punycode",
                )?;
            }

            if event.has_value("json.mailbox.email.domain.root_domain") {
                event.rename(
                    "json.mailbox.email.domain.root_domain",
                    "sublime_security.email_message.mailbox.email.domain.root_domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.mailbox.email.domain.root_domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.mailbox.email.domain.sld") {
                event.rename(
                    "json.mailbox.email.domain.sld",
                    "sublime_security.email_message.mailbox.email.domain.sld",
                )?;
            }

            if event.has_value("json.mailbox.email.domain.subdomain") {
                event.rename(
                    "json.mailbox.email.domain.subdomain",
                    "sublime_security.email_message.mailbox.email.domain.subdomain",
                )?;
            }

            if event.has_value("json.mailbox.email.domain.tld") {
                event.rename(
                    "json.mailbox.email.domain.tld",
                    "sublime_security.email_message.mailbox.email.domain.tld",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.mailbox.email.domain.valid") {
                    if let Some(val) = event.get("json.mailbox.email.domain.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.mailbox.email.domain.valid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.mailbox.email.domain.valid",
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
                    "convert_mailbox_email_domain_valid_to_boolean",
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

            if event.has_value("json.mailbox.email.email") {
                event.rename(
                    "json.mailbox.email.email",
                    "sublime_security.email_message.mailbox.email.value",
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("sublime_security.email_message.mailbox.email.value")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.mailbox.email.local_part") {
                event.rename(
                    "json.mailbox.email.local_part",
                    "sublime_security.email_message.mailbox.email.local_part",
                )?;
            }

            if event.has_value("json._meta.canonical_id") {
                event.rename(
                    "json._meta.canonical_id",
                    "sublime_security.email_message.meta.canonical_id",
                )?;
            }

            let _cond = {
                event.has_value("json._meta.created_at")
                    && event.get_str("json._meta.created_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json._meta.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("sublime_security.email_message.meta.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json._meta.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_meta_created_at")?;
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
                .get("sublime_security.email_message.meta.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("json._meta.effective_at")
                    && event.get_str("json._meta.effective_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json._meta.effective_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("sublime_security.email_message.meta.effective_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json._meta.effective_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_meta_effective_at")?;
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

            if event.has_value("json._meta.id") {
                event.rename("json._meta.id", "sublime_security.email_message.meta.id")?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.meta.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event
                    .get("json.recipients.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.bcc", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.bcc", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.bcc", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.email.domain.valid") {
                            if let Some(val) = event.get("_ingest._value.email.domain.valid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.email.domain.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.email.domain.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_recipients_bcc_email_domain_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.email.domain.valid");
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
                    .get("json.recipients.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.bcc", |event| {
                    event.append_unique(
                        "email.bcc.address",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.bcc", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.bcc", |event| {
                    if event.has_value("_ingest._value.email.email") {
                        event.rename("_ingest._value.email.email", "_ingest._value.email.value")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.recipients.bcc") {
                event.rename(
                    "json.recipients.bcc",
                    "sublime_security.email_message.recipients.bcc",
                )?;
            }

            let _cond = {
                event
                    .get("json.recipients.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.cc", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.cc", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.cc", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.email.domain.valid") {
                            if let Some(val) = event.get("_ingest._value.email.domain.valid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.email.domain.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.email.domain.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_recipients_cc_email_domain_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.email.domain.valid");
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
                    .get("json.recipients.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.cc", |event| {
                    event.append_unique(
                        "email.cc.address",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.cc", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.cc", |event| {
                    if event.has_value("_ingest._value.email.email") {
                        event.rename("_ingest._value.email.email", "_ingest._value.email.value")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.recipients.cc") {
                event.rename(
                    "json.recipients.cc",
                    "sublime_security.email_message.recipients.cc",
                )?;
            }

            let _cond = {
                event
                    .get("json.recipients.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.to", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.to", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.email.domain.root_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.to", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.email.domain.valid") {
                            if let Some(val) = event.get("_ingest._value.email.domain.valid") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.email.domain.valid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.email.domain.valid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_recipients_to_email_domain_valid_to_boolean",
                        )?;
                        event.remove("_ingest._value.email.domain.valid");
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
                    .get("json.recipients.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.to", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.to", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.recipients.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.recipients.to", |event| {
                    if event.has_value("_ingest._value.email.email") {
                        event.rename("_ingest._value.email.email", "_ingest._value.email.value")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.recipients.to") {
                event.rename(
                    "json.recipients.to",
                    "sublime_security.email_message.recipients.to",
                )?;
            }

            if event.has_value("json.sender.display_name") {
                event.rename(
                    "json.sender.display_name",
                    "sublime_security.email_message.sender.display_name",
                )?;
            }

            if event.has_value("json.sender.email.domain.domain") {
                event.rename(
                    "json.sender.email.domain.domain",
                    "sublime_security.email_message.sender.email.domain.domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.sender.email.domain.domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(v) = event
                .get("sublime_security.email_message.sender.email.domain.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            if event.has_value("json.sender.email.domain.punycode") {
                event.rename(
                    "json.sender.email.domain.punycode",
                    "sublime_security.email_message.sender.email.domain.punycode",
                )?;
            }

            if event.has_value("json.sender.email.domain.root_domain") {
                event.rename(
                    "json.sender.email.domain.root_domain",
                    "sublime_security.email_message.sender.email.domain.root_domain",
                )?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("sublime_security.email_message.sender.email.domain.root_domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.sender.email.domain.sld") {
                event.rename(
                    "json.sender.email.domain.sld",
                    "sublime_security.email_message.sender.email.domain.sld",
                )?;
            }

            if event.has_value("json.sender.email.domain.subdomain") {
                event.rename(
                    "json.sender.email.domain.subdomain",
                    "sublime_security.email_message.sender.email.domain.subdomain",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.sender.email.domain.subdomain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.subdomain", v)?;
            }

            if event.has_value("json.sender.email.domain.tld") {
                event.rename(
                    "json.sender.email.domain.tld",
                    "sublime_security.email_message.sender.email.domain.tld",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.sender.email.domain.tld")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.top_level_domain", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.sender.email.domain.valid") {
                    if let Some(val) = event.get("json.sender.email.domain.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sender.email.domain.valid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sublime_security.email_message.sender.email.domain.valid",
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
                    "convert_sender_email_domain_valid_to_boolean",
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

            if event.has_value("json.sender.email.email") {
                event.rename(
                    "json.sender.email.email",
                    "sublime_security.email_message.sender.email.value",
                )?;
            }

            let _cond = { event.has_value("sublime_security.email_message.sender.email.value") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("sublime_security.email_message.sender.email.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("sublime_security.email_message.sender.email.value")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.sender.email.local_part") {
                event.rename(
                    "json.sender.email.local_part",
                    "sublime_security.email_message.sender.email.local_part",
                )?;
            }

            if event.has_value("json.subject.subject") {
                event.rename(
                    "json.subject.subject",
                    "sublime_security.email_message.subject.subject",
                )?;
            }

            if let Some(v) = event
                .get("sublime_security.email_message.subject.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.type.inbound") {
                    if let Some(val) = event.get("json.type.inbound") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.type.inbound".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.type.inbound", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_type_inbound_to_boolean",
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

            let _cond =
                { event.get_bool("sublime_security.email_message.type.inbound") == Some(true) };
            if _cond {
                let v = json!("inbound");
                if !painless_is_empty_value(&v) {
                    event.set("email.direction", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.type.internal") {
                    if let Some(val) = event.get("json.type.internal") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.type.internal".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.type.internal", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_type_internal_to_boolean",
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

            let _cond =
                { event.get_bool("sublime_security.email_message.type.internal") == Some(true) };
            if _cond {
                let v = json!("internal");
                if !painless_is_empty_value(&v) {
                    event.set("email.direction", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.type.outbound") {
                    if let Some(val) = event.get("json.type.outbound") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.type.outbound".into(),
                                message,
                            }
                        })?;
                        event.set("sublime_security.email_message.type.outbound", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_type_outbound_to_boolean",
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

            let _cond =
                { event.get_bool("sublime_security.email_message.type.outbound") == Some(true) };
            if _cond {
                let v = json!("outbound");
                if !painless_is_empty_value(&v) {
                    event.set("email.direction", v)?;
                }
            }

            let _cond = {
                event
                    .get("sublime_security.email_message.attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.email_message.attachments",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.content.type");
                            event.remove("_ingest._value.file.extension");
                            event.remove("_ingest._value.file.name");
                            event.remove("_ingest._value.md5");
                            event.remove("_ingest._value.sha1");
                            event.remove("_ingest._value.sha256");
                            event.remove("_ingest._value.size");
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.email_message.body.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.email_message.body.links",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.href_url.domain.domain");
                            event.remove("_ingest._value.href_url.domain.subdomain");
                            event.remove("_ingest._value.href_url.domain.tld");
                            event.remove("_ingest._value.href_url.fragment");
                            event.remove("_ingest._value.href_url.password");
                            event.remove("_ingest._value.href_url.path");
                            event.remove("_ingest._value.href_url.port");
                            event.remove("_ingest._value.href_url.query_params");
                            event.remove("_ingest._value.href_url.scheme");
                            event.remove("_ingest._value.href_url.url");
                            event.remove("_ingest._value.href_url.username");
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.email_message.headers.reply_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.email_message.headers.reply_to",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.email.value");
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.email_message.recipients.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.email_message.recipients.bcc",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.email.value");
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.email_message.recipients.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.email_message.recipients.cc",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.email.value");
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("sublime_security.email_message.recipients.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "sublime_security.email_message.recipients.to",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.email.value");
                        }
                        Ok(())
                    },
                )?;
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
                event.remove("sublime_security.email_message.external.created_at");
                event.remove("sublime_security.email_message.headers.delivered_to.domain.domain");
                event
                    .remove("sublime_security.email_message.headers.delivered_to.domain.subdomain");
                event.remove("sublime_security.email_message.headers.delivered_to.domain.tld");
                event.remove("sublime_security.email_message.headers.mailer");
                event.remove("sublime_security.email_message.headers.message_id");
                event.remove("sublime_security.email_message.headers.x_client_ip.ip");
                event.remove("sublime_security.email_message.meta.created_at");
                event.remove("sublime_security.email_message.meta.id");
                event.remove("sublime_security.email_message.sender.email.domain.domain");
                event.remove("sublime_security.email_message.sender.email.domain.subdomain");
                event.remove("sublime_security.email_message.sender.email.domain.tld");
                event.remove("sublime_security.email_message.sender.email.value");
                event.remove("sublime_security.email_message.subject.subject");
            }

            event.remove("json");

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
