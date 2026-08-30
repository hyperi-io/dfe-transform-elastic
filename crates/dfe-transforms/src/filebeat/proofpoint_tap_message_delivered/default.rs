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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = {
                event.get("json.messagesDelivered").is_some_and(|v| v.is_array()) && event.get("json.messagesDelivered").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.has_value("json.messageTime") && event.get_str("json.messageTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.messageTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.messageTime".into(),
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
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("email"))?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.senderIP") {
                    if let Some(val) = event.get("json.senderIP") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.senderIP".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.senderIP").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.senderIP".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = { event.get("json.messageParts").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.messageParts", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.md5")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.messageParts").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.messageParts", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.sha256")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.ccAddresses") {
                event.rename("json.ccAddresses", "email.cc.address")?;
            }

            let _cond = { event.get("email.cc.address").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "email.cc.address",
                    Value::Array(vec![json!(
                        event
                            .get("email.cc.address")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("email.delivery_timestamp", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.fromAddress") {
                event.rename("json.fromAddress", "email.from.address")?;
            }

            let _cond = {
                event
                    .get("email.from.address")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "email.from.address",
                    Value::Array(vec![json!(
                        event
                            .get("email.from.address")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            if event.has_value("json.messageID") {
                event.rename("json.messageID", "email.message_id")?;
            }

            if event.has_value("email.message_id") {
                gsub_field(
                    event,
                    "email.message_id",
                    "email.message_id",
                    cached_regex!("<|>"),
                    "",
                )?;
            }

            if event.has_value("json.replyToAddress") {
                event.rename("json.replyToAddress", "email.reply_to.address")?;
            }

            if event.has_value("json.sender") {
                event.rename("json.sender", "email.sender.address")?;
            }

            if event.has_value("json.subject") {
                event.rename("json.subject", "email.subject")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.toAddresses").cloned() {
                    event.set("email.to.address", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get("email.to.address").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "email.to.address",
                    Value::Array(vec![json!(
                        event
                            .get("email.to.address")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = { event.get("json.recipient").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.recipient", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "email.to.address",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.xmailer") {
                event.rename("json.xmailer", "email.x_mailer")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.messageParts").cloned() {
                    event.set("email.attachments", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get("email.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "email.attachments", |event| {
                        event.remove("_ingest._value.disposition");
                        event.remove("_ingest._value.oContentType");
                        event.remove("_ingest._value.sandboxStatus");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("email.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "email.attachments", |event| {
                        if event.has_value("_ingest._value.contentType") {
                            event.rename(
                                "_ingest._value.contentType",
                                "_ingest._value.file.mime_type",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("email.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "email.attachments", |event| {
                        if event.has_value("_ingest._value.md5") {
                            event.rename("_ingest._value.md5", "_ingest._value.file.hash.md5")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("email.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "email.attachments", |event| {
                        if event.has_value("_ingest._value.sha256") {
                            event.rename(
                                "_ingest._value.sha256",
                                "_ingest._value.file.hash.sha256",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("email.attachments").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "email.attachments", |event| {
                        if event.has_value("_ingest._value.filename") {
                            event.rename("_ingest._value.filename", "_ingest._value.file.name")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.threatsInfoMap instanceof List) {\n    for (artifact in ctx.json.threatsInfoMap) {\n        def flag = true;\n        def str = artifact.threat.toLowerCase();\n        if (str?.length() == 64) {\n            for (int i = 0; i < str.length(); i++) {\n                def ch = str.charAt(i);\n                if ((ch < (char)'0' || ch > (char)'9') && (ch < (char)'a' || ch > (char)'f')) {\n                    flag = false;\n                    break;\n                }\n            }\n            if (flag && !ctx['related']['hash'].contains(str)) {\n                ctx['related']['hash'].add(str);\n            }\n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json?.threatsInfoMap instanceof List) {\n    for (artifact in ctx.json.threatsInfoMap) {\n        def flag = true;\n        def str = artifact.threat.toLowerCase();\n        if (str?.length() == 64) {\n            for (int i = 0; i < str.length(); i++) {\n                def ch = str.charAt(i);\n                if ((ch < (char)'0' || ch > (char)'9') && (ch < (char)'a' || ch > (char)'f')) {\n                    flag = false;\n                    break;\n                }\n            }\n            if (flag && !ctx['related']['hash'].contains(str)) {\n                ctx['related']['hash'].add(str);\n            }\n        }\n    }\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.toAddresses") {
                event.rename(
                    "json.toAddresses",
                    "proofpoint_tap.message_delivered.to_addresses",
                )?;
            }

            if event.has_value("json.recipient") {
                event.rename(
                    "json.recipient",
                    "proofpoint_tap.message_delivered.recipient",
                )?;
            }

            if event.has_value("json.cluster") {
                event.rename("json.cluster", "proofpoint_tap.message_delivered.cluster")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.completelyRewritten") {
                    if let Some(val) = event.get("json.completelyRewritten") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.completelyRewritten".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "proofpoint_tap.message_delivered.completely_rewritten",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.GUID") {
                event.rename("json.GUID", "proofpoint_tap.guid")?;
            }

            if event.has_value("json.headerFrom") {
                event.rename(
                    "json.headerFrom",
                    "proofpoint_tap.message_delivered.header.from",
                )?;
            }

            if event.has_value("proofpoint_tap.message_delivered.header.from") {
                gsub_field(
                    event,
                    "proofpoint_tap.message_delivered.header.from",
                    "proofpoint_tap.message_delivered.header.from",
                    cached_regex!("<|>"),
                    "",
                )?;
            }

            if event.has_value("json.headerReplyTo") {
                event.rename(
                    "json.headerReplyTo",
                    "proofpoint_tap.message_delivered.header.replyto",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.impostorScore") {
                    if let Some(val) = event.get("json.impostorScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.impostorScore".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_tap.message_delivered.impostor_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.impostorScore").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.impostorScore".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.malwareScore") {
                    if let Some(val) = event.get("json.malwareScore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.malwareScore".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_tap.message_delivered.malware_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.malwareScore").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.malwareScore".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.messageParts") {
                event.rename(
                    "json.messageParts",
                    "proofpoint_tap.message_delivered.message_parts",
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.message_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.message_parts",
                        |event| {
                            event.remove("_ingest._value.contentType");
                            event.remove("_ingest._value.filename");
                            event.remove("_ingest._value.md5");
                            event.remove("_ingest._value.sha256");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.message_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.message_parts",
                        |event| {
                            if event.has_value("_ingest._value.oContentType") {
                                event.rename(
                                    "_ingest._value.oContentType",
                                    "_ingest._value.o_content_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.message_parts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.message_parts",
                        |event| {
                            if event.has_value("_ingest._value.sandboxStatus") {
                                event.rename(
                                    "_ingest._value.sandboxStatus",
                                    "_ingest._value.sandbox_status",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.messageSize") {
                    if let Some(val) = event.get("json.messageSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.messageSize".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_tap.message_delivered.message_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.messageSize").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.messageSize".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.modulesRun") {
                event.rename(
                    "json.modulesRun",
                    "proofpoint_tap.message_delivered.modules_run",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.phishScore") {
                    if let Some(val) = event.get("json.phishScore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.phishScore".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_tap.message_delivered.phish_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.phishScore").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.phishScore".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.policyRoutes") {
                event.rename(
                    "json.policyRoutes",
                    "proofpoint_tap.message_delivered.policy_routes",
                )?;
            }

            if event.has_value("json.QID") {
                event.rename("json.QID", "proofpoint_tap.message_delivered.qid")?;
            }

            if event.has_value("json.quarantineFolder") {
                event.rename(
                    "json.quarantineFolder",
                    "proofpoint_tap.message_delivered.quarantine.folder",
                )?;
            }

            if event.has_value("json.quarantineRule") {
                event.rename(
                    "json.quarantineRule",
                    "proofpoint_tap.message_delivered.quarantine.rule",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.spamScore") {
                    if let Some(val) = event.get("json.spamScore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.spamScore".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_tap.message_delivered.spam_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.spamScore").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.spamScore".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.threatsInfoMap") {
                event.rename(
                    "json.threatsInfoMap",
                    "proofpoint_tap.message_delivered.threat_info_map",
                )?;
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.threat_info_map",
                        |event| {
                            if event.has_value("_ingest._value.campaignId") {
                                event.rename(
                                    "_ingest._value.campaignId",
                                    "_ingest._value.campaign_id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.threat_info_map",
                        |event| {
                            if event.has_value("_ingest._value.threat") {
                                event.rename(
                                    "_ingest._value.threat",
                                    "_ingest._value.threat.artifact",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.threat_info_map",
                        |event| {
                            if event.has_value("_ingest._value.threatID") {
                                event.rename(
                                    "_ingest._value.threatID",
                                    "_ingest._value.threat.id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.threat_info_map",
                        |event| {
                            if event.has_value("_ingest._value.threatStatus") {
                                event.rename(
                                    "_ingest._value.threatStatus",
                                    "_ingest._value.threat.status",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("proofpoint_tap.message_delivered.threat_info_map")
                            .cloned();
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
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.threatTime")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.threat.time", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.threatTime".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })();
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
                                "proofpoint_tap.message_delivered.threat_info_map",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.threat_info_map",
                        |event| {
                            event.remove("_ingest._value.threatTime");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.threat_info_map",
                        |event| {
                            if event.has_value("_ingest._value.threatType") {
                                event.rename(
                                    "_ingest._value.threatType",
                                    "_ingest._value.threat.type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "proofpoint_tap.message_delivered.threat_info_map",
                        |event| {
                            if event.has_value("_ingest._value.threatUrl") {
                                event.rename(
                                    "_ingest._value.threatUrl",
                                    "_ingest._value.threat.url",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint_tap.message_delivered.threat_info_map")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def ts = Instant.parse(ctx['@timestamp']);\nfor (item in ctx.proofpoint_tap.message_delivered.threat_info_map) {\n  if (item?.threat?.time instanceof String && Instant.parse(item.threat.time).isAfter(ts)) {\n    ctx['@timestamp'] = item.threat.time;\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def ts = Instant.parse(ctx['@timestamp']);\nfor (item in ctx.proofpoint_tap.message_delivered.threat_info_map) {\n  if (item?.threat?.time instanceof String && Instant.parse(item.threat.time).isAfter(ts)) {\n    ctx['@timestamp'] = item.threat.time;\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
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
