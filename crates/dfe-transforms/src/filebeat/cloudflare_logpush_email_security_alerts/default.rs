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

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            event.set(
                "event.category",
                Value::Array(vec![json!("email"), json!("network")]),
            )?;

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                event.get_bool("_conf.enable_deduplication") == Some(false)
                    && event.get_str("input.type") == Some("aws-s3")
            };
            if _cond {
                event.remove("_id");
            }

            let _cond = {
                event.has_value("json.Timestamp") && event.get_str("json.Timestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.Timestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Timestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.Timestamp", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Timestamp")
                    && event.get("json.Timestamp").is_some_and(|v| v.is_number())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: long t = (long)(ctx.json.Timestamp);\nif (t > (long)(1e18)) {\n  ctx.json.Timestamp = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Timestamp = t*(long)(1e3)\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"long t = (long)(ctx.json.Timestamp);\nif (t > (long)(1e18)) {\n  ctx.json.Timestamp = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Timestamp = t*(long)(1e3)\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "painless_Timestamp_to_milli",
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
                event.has_value("json.Timestamp") && event.get_str("json.Timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Timestamp".into(),
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
                        "date_json_Timestamp_70be028a",
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
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloudflare_logpush.email_security_alerts.timestamp", v)?;
            }

            if event.has_value("json.AlertID") {
                event.rename(
                    "json.AlertID",
                    "cloudflare_logpush.email_security_alerts.alert_id",
                )?;
            }

            if event.has_value("json.AlertReasons") {
                event.rename(
                    "json.AlertReasons",
                    "cloudflare_logpush.email_security_alerts.alert_reasons",
                )?;
            }

            if event.has_value("json.Attachments") {
                event.rename(
                    "json.Attachments",
                    "cloudflare_logpush.email_security_alerts.attachments",
                )?;
            }

            if event.has_value("json.CC") {
                event.rename("json.CC", "cloudflare_logpush.email_security_alerts.cc")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.email_security_alerts.cc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.cc.address", v)?;
            }

            if event.has_value("json.CCName") {
                event.rename(
                    "json.CCName",
                    "cloudflare_logpush.email_security_alerts.cc_name",
                )?;
            }

            if event.has_value("json.FinalDisposition") {
                event.rename(
                    "json.FinalDisposition",
                    "cloudflare_logpush.email_security_alerts.final_disposition",
                )?;
            }

            if event.has_value("json.From") {
                event.rename("json.From", "cloudflare_logpush.email_security_alerts.from")?;
            }

            let _cond = { event.has_value("cloudflare_logpush.email_security_alerts.from") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.FromName") {
                event.rename(
                    "json.FromName",
                    "cloudflare_logpush.email_security_alerts.from_name",
                )?;
            }

            if event.has_value("json.Links") {
                event.rename(
                    "json.Links",
                    "cloudflare_logpush.email_security_alerts.links",
                )?;
            }

            if event.has_value("json.MessageDeliveryMode") {
                event.rename(
                    "json.MessageDeliveryMode",
                    "cloudflare_logpush.email_security_alerts.message_delivery_mode",
                )?;
            }

            if event.has_value("json.MessageID") {
                event.rename(
                    "json.MessageID",
                    "cloudflare_logpush.email_security_alerts.message_id",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.email_security_alerts.message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            if event.has_value("json.Origin") {
                event.rename(
                    "json.Origin",
                    "cloudflare_logpush.email_security_alerts.origin",
                )?;
            }

            if event.has_value("json.OriginalSender") {
                event.rename(
                    "json.OriginalSender",
                    "cloudflare_logpush.email_security_alerts.original_sender",
                )?;
            }

            if event.has_value("json.ReplyTo") {
                event.rename(
                    "json.ReplyTo",
                    "cloudflare_logpush.email_security_alerts.reply_to",
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.email_security_alerts.reply_to") };
            if _cond {
                event.append_unique(
                    "email.reply_to.address",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.reply_to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.ReplyToName") {
                event.rename(
                    "json.ReplyToName",
                    "cloudflare_logpush.email_security_alerts.reply_to_name",
                )?;
            }

            if event.has_value("json.SMTPEnvelopeFrom") {
                event.rename(
                    "json.SMTPEnvelopeFrom",
                    "cloudflare_logpush.email_security_alerts.smtp_envelope_from",
                )?;
            }

            if event.has_value("json.SMTPEnvelopeTo") {
                event.rename(
                    "json.SMTPEnvelopeTo",
                    "cloudflare_logpush.email_security_alerts.smtp_envelope_to",
                )?;
            }

            let _cond = {
                event.has_value("json.SMTPHeloServerIP")
                    && event.get_str("json.SMTPHeloServerIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SMTPHeloServerIP") {
                        if let Some(val) = event.get("json.SMTPHeloServerIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SMTPHeloServerIP".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.email_security_alerts.smtp_helo_server_ip",
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
                        "convert_smtp_helo_server_ip",
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

            if event.has_value("json.SMTPHeloServerIPAsName") {
                event.rename(
                    "json.SMTPHeloServerIPAsName",
                    "cloudflare_logpush.email_security_alerts.smtp_helo_server_ip_as_name",
                )?;
            }

            if event.has_value("json.SMTPHeloServerIPAsNumber") {
                event.rename(
                    "json.SMTPHeloServerIPAsNumber",
                    "cloudflare_logpush.email_security_alerts.smtp_helo_server_ip_as_number",
                )?;
            }

            if event.has_value("json.SMTPHeloServerIPGeo") {
                event.rename(
                    "json.SMTPHeloServerIPGeo",
                    "cloudflare_logpush.email_security_alerts.smtp_helo_server_ip_geo",
                )?;
            }

            if event.has_value("json.SMTPHeloServerName") {
                event.rename(
                    "json.SMTPHeloServerName",
                    "cloudflare_logpush.email_security_alerts.smtp_helo_server_name",
                )?;
            }

            if event.has_value("json.Subject") {
                event.rename(
                    "json.Subject",
                    "cloudflare_logpush.email_security_alerts.subject",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.email_security_alerts.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if event.has_value("json.ThreatCategories") {
                event.rename(
                    "json.ThreatCategories",
                    "cloudflare_logpush.email_security_alerts.threat_categories",
                )?;
            }

            if event.has_value("json.To") {
                event.rename("json.To", "cloudflare_logpush.email_security_alerts.to")?;
            }

            let _cond = {
                event
                    .get("cloudflare_logpush.email_security_alerts.to")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("cloudflare_logpush.email_security_alerts.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.email_security_alerts.to",
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

            if event.has_value("json.ToName") {
                event.rename(
                    "json.ToName",
                    "cloudflare_logpush.email_security_alerts.to_name",
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.email_security_alerts.final_disposition")
                    && event.get_str("cloudflare_logpush.email_security_alerts.final_disposition")
                        != Some("unset")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.email_security_alerts.from")
                    && event.get_str("cloudflare_logpush.email_security_alerts.from") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.email_security_alerts.from_name")
                    && event.get_str("cloudflare_logpush.email_security_alerts.from_name")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.from_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.email_security_alerts.smtp_envelope_from")
                    && event.get_str("cloudflare_logpush.email_security_alerts.smtp_envelope_from")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.smtp_envelope_from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.email_security_alerts.original_sender") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.original_sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.email_security_alerts.reply_to_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.reply_to_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.email_security_alerts.reply_to") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.reply_to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("cloudflare_logpush.email_security_alerts.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.email_security_alerts.to",
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
                    .get("cloudflare_logpush.email_security_alerts.smtp_envelope_to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.email_security_alerts.smtp_envelope_to",
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
                    .get("cloudflare_logpush.email_security_alerts.to_name")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.email_security_alerts.to_name",
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
                    .get("cloudflare_logpush.email_security_alerts.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.email_security_alerts.cc",
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
                    .get("cloudflare_logpush.email_security_alerts.cc_name")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cloudflare_logpush.email_security_alerts.cc_name",
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

            if let Some(v) = event
                .get("cloudflare_logpush.email_security_alerts.smtp_helo_server_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.address", v)?;
            }

            let _cond = {
                event.has_value("server.address") && event.get_str("server.address") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("server.address") {
                        if let Some(val) = event.get("server.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.address".into(),
                                    message,
                                }
                            })?;
                            event.set("_tmp.server_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_smtp_helo_server_name_ip",
                    )?;
                    if let Some(v) = event
                        .get("server.address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.domain", v)?;
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
                .get("cloudflare_logpush.email_security_alerts.links")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("_tmp.links", v)?;
            }

            let _cond = { event.get("_tmp.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "_tmp.links", |event| {
                    if event.has_value("_ingest._value") {
                        uri_parts(event, "_ingest._value", "_ingest._value", true, false)?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("_tmp.links").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "_tmp.links", |event| {
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
                event.get("related.user").is_some_and(|v| v.is_array())
                    || (event.has_value("server.domain"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def domains = new HashSet();\nif (ctx.related?.hosts instanceof List) {\n  for (def h: ctx.related.hosts) {\n    domains.add(h);\n  }\n}\nif (ctx.server?.domain != null) {\n  domains.add(ctx.server.domain)\n}\nif (ctx.related?.user instanceof List) {\n  for (def u: ctx.related.user) {\n    if (u.length() < 3) {\n      continue;\n    }\n    def parts = u.splitOnToken('@');\n    if (parts.length != 2) {\n      continue;\n    }\n    domains.add(parts[1]);\n  }\n}\nctx.related.hosts = domains;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def domains = new HashSet();\nif (ctx.related?.hosts instanceof List) {\n  for (def h: ctx.related.hosts) {\n    domains.add(h);\n  }\n}\nif (ctx.server?.domain != null) {\n  domains.add(ctx.server.domain)\n}\nif (ctx.related?.user instanceof List) {\n  for (def u: ctx.related.user) {\n    if (u.length() < 3) {\n      continue;\n    }\n    def parts = u.splitOnToken('@');\n    if (parts.length != 2) {\n      continue;\n    }\n    domains.add(parts[1]);\n  }\n}\nctx.related.hosts = domains;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_45dae815")?;
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
                .get("cloudflare_logpush.email_security_alerts.smtp_helo_server_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.ip", v)?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.email_security_alerts.smtp_helo_server_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.email_security_alerts.smtp_helo_server_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                if event.has_value("server.ip") {
                    if let Some(ip_str) = event.get_string("server.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("server.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("server.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("server.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("server.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("server.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("server.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("server.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("server.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                if event.has_value("server.ip") {
                    if let Some(ip_str) = event.get_string("server.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("server.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("server.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("server.as.asn") {
                event.rename("server.as.asn", "server.as.number")?;
            }

            if event.has_value("server.as.organization_name") {
                event.rename("server.as.organization_name", "server.as.organization.name")?;
            }

            let _cond = {
                event
                    .get("cloudflare_logpush.email_security_alerts.attachments")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def files = [];\ndef emailAttachments = [];\ndef hashes = new HashSet();\nfor (def a: ctx.cloudflare_logpush.email_security_alerts.attachments) {\n  def f = [:];\n  def efa = [:];\n  if (a.Name != null) {\n    f.name = a.Name;\n  }\n  if (a.ContentTypeComputed != null) {\n    f.mime_type = a.ContentTypeComputed;\n  }\n  def fh = [:];\n  if (a.Md5 != null) {\n    fh.md5 = a.Md5;\n  }\n  if (a.Sha1 != null) {\n    fh.sha1 = a.Sha1;\n  }\n  if (a.Sha256 != null) {\n    fh.sha256 = a.Sha256;\n  }\n  if (a.Sha384 != null) {\n    fh.sha384 = a.Sha384;\n  }\n  if (a.Sha512 != null) {\n    fh.sha512 = a.Sha512;\n  }\n  if (a.Ssdeep != null) {\n    fh.ssdeep = a.Ssdeep;\n  }\n  if (fh.size() != 0) {\n    f.hash = fh;\n  }\n  files.add(f);\n  efa.put('file', f);\n  emailAttachments.add(efa);\n  for (def h: fh.entrySet()) {\n    hashes.add(h.getValue());\n  }\n}\nctx.file = files;\nif (emailAttachments.size() != 0) {\n  if (ctx.email == null) {\n    ctx.email = [:];\n  }\n  ctx.email.attachments = emailAttachments;\n}\nctx.related = ctx.related ?: [:];\nctx.related.hash = hashes;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def files = [];\ndef emailAttachments = [];\ndef hashes = new HashSet();\nfor (def a: ctx.cloudflare_logpush.email_security_alerts.attachments) {\n  def f = [:];\n  def efa = [:];\n  if (a.Name != null) {\n    f.name = a.Name;\n  }\n  if (a.ContentTypeComputed != null) {\n    f.mime_type = a.ContentTypeComputed;\n  }\n  def fh = [:];\n  if (a.Md5 != null) {\n    fh.md5 = a.Md5;\n  }\n  if (a.Sha1 != null) {\n    fh.sha1 = a.Sha1;\n  }\n  if (a.Sha256 != null) {\n    fh.sha256 = a.Sha256;\n  }\n  if (a.Sha384 != null) {\n    fh.sha384 = a.Sha384;\n  }\n  if (a.Sha512 != null) {\n    fh.sha512 = a.Sha512;\n  }\n  if (a.Ssdeep != null) {\n    fh.ssdeep = a.Ssdeep;\n  }\n  if (fh.size() != 0) {\n    f.hash = fh;\n  }\n  files.add(f);\n  efa.put('file', f);\n  emailAttachments.add(efa);\n  for (def h: fh.entrySet()) {\n    hashes.add(h.getValue());\n  }\n}\nctx.file = files;\nif (emailAttachments.size() != 0) {\n  if (ctx.email == null) {\n    ctx.email = [:];\n  }\n  ctx.email.attachments = emailAttachments;\n}\nctx.related = ctx.related ?: [:];\nctx.related.hash = hashes;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_218079c2")?;
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

            event.remove("json");
            event.remove("_tmp");
            event.remove("_conf");

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
                event.remove("cloudflare_logpush.email_security_alerts.timestamp");
                event.remove("cloudflare_logpush.email_security_alerts.smtp_helo_server_ip");
                event.remove("cloudflare_logpush.email_security_alerts.from");
                event.remove("cloudflare_logpush.email_security_alerts.to");
                event.remove("cloudflare_logpush.email_security_alerts.cc");
                event.remove("cloudflare_logpush.email_security_alerts.subject");
                event.remove("cloudflare_logpush.email_security_alerts.message_id");
                event.remove("cloudflare_logpush.email_security_alerts.reply_to");
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
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
