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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.ts") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.data") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("email"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Proofpoint"))?;

            event.set("observer.product", json!("Proofpoint On Demand"))?;

            event.set("observer.type", json!("mail-gateway"))?;

            if event.has("json.data") {
                event.rename("json.data", "proofpoint_on_demand.mail.data")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.data")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has("json.id") {
                event.rename("json.id", "proofpoint_on_demand.mail.id")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has("json.metadata.origin.data.agent") {
                event.rename(
                    "json.metadata.origin.data.agent",
                    "proofpoint_on_demand.mail.metadata.origin.data.agent",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.metadata.origin.data.agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.mail.metadata.origin.data.agent") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.mail.metadata.origin.data.agent")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.metadata.origin.data.cid") {
                event.rename(
                    "json.metadata.origin.data.cid",
                    "proofpoint_on_demand.mail.metadata.origin.data.cid",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.metadata.origin.data.cid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if event.has("json.metadata.origin.data.version") {
                event.rename(
                    "json.metadata.origin.data.version",
                    "proofpoint_on_demand.mail.metadata.origin.data.version",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.metadata.origin.data.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            if event.has("json.pps.agent") {
                event.rename("json.pps.agent", "proofpoint_on_demand.mail.pps.agent")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.pps.agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.mail.pps.agent") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_on_demand.mail.pps.agent")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.pps.cid") {
                event.rename("json.pps.cid", "proofpoint_on_demand.mail.pps.cid")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.pps.cid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if event.has("json.pps.version") {
                event.rename("json.pps.version", "proofpoint_on_demand.mail.pps.version")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.pps.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            if event.has("json.sm.auth") {
                event.rename("json.sm.auth", "proofpoint_on_demand.mail.sm.auth")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.sm.class") {
                    if let Some(val) = event.get("json.sm.class") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sm.class".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.mail.sm.class", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_sm_class_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.sm.ctladdr") {
                event.rename("json.sm.ctladdr", "proofpoint_on_demand.mail.sm.ctladdr")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("proofpoint_on_demand.mail.sm.ctladdr") {
                    // Grok pattern: ^<%{EMAILADDRESS:_temp.address}> \\(%{NUMBER}/%{NUMBER}\\)$
                    if !cached_grok!("^<%{EMAILADDRESS:_temp.address}> \\(%{NUMBER}/%{NUMBER}\\)$")
                        .extract_into(&input, event)?
                    {}
                }
                Ok(())
            })();

            let _cond = { !event.has_value("json.sm.from") && event.has_value("_temp.address") };
            if _cond {
                if let Some(v) = event
                    .get("_temp.address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.sender.address", v)?;
                }
            }

            let _cond = { !event.has_value("json.sm.from") && event.has_value("_temp.address") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("_temp.address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.address") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("_temp.address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.sm.daemon") {
                event.rename("json.sm.daemon", "proofpoint_on_demand.mail.sm.daemon")?;
            }

            if event.has("json.sm.delay") {
                event.rename("json.sm.delay", "proofpoint_on_demand.mail.sm.delay")?;
            }

            if event.has("json.sm.dsn") {
                event.rename("json.sm.dsn", "proofpoint_on_demand.mail.sm.dsn")?;
            }

            if event.has("json.sm.mailer") {
                event.rename("json.sm.mailer", "proofpoint_on_demand.mail.sm.mailer")?;
            }

            if event.has("json.sm.from") {
                event.rename("json.sm.from", "proofpoint_on_demand.mail.sm.from")?;
            }

            if event.has("json.sm.msgid") {
                event.rename("json.sm.msgid", "proofpoint_on_demand.mail.sm.msgid")?;
            }

            if event.has("json.sm.to") {
                event.rename("json.sm.to", "proofpoint_on_demand.mail.sm.to")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def removeUnwantedCharacters(def input) {\n  if (input.startsWith(\"<\") && input.endsWith(\">\")) {\n    String trimmed = input.substring(1, input.length() - 1);\n    return trimmed;\n  } else {\n    return input;\n  }\n}\nif (ctx.proofpoint_on_demand?.mail?.sm?.from != null) {\n  ctx.proofpoint_on_demand.mail.sm.from = removeUnwantedCharacters(ctx.proofpoint_on_demand.mail.sm.from);\n}\nif (ctx.proofpoint_on_demand?.mail?.sm?.msgid != null) {\n  ctx.proofpoint_on_demand.mail.sm.msgid = removeUnwantedCharacters(ctx.proofpoint_on_demand.mail.sm.msgid);\n}\nList toAddresses = new ArrayList();\nif (ctx.proofpoint_on_demand?.mail?.sm?.to instanceof List) {\n  for (address in ctx.proofpoint_on_demand.mail.sm.to) {\n    toAddresses.add(removeUnwantedCharacters(address));\n  }\n  ctx.proofpoint_on_demand.mail.sm.to = toAddresses;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def removeUnwantedCharacters(def input) {\n  if (input.startsWith(\"<\") && input.endsWith(\">\")) {\n    String trimmed = input.substring(1, input.length() - 1);\n    return trimmed;\n  } else {\n    return input;\n  }\n}\nif (ctx.proofpoint_on_demand?.mail?.sm?.from != null) {\n  ctx.proofpoint_on_demand.mail.sm.from = removeUnwantedCharacters(ctx.proofpoint_on_demand.mail.sm.from);\n}\nif (ctx.proofpoint_on_demand?.mail?.sm?.msgid != null) {\n  ctx.proofpoint_on_demand.mail.sm.msgid = removeUnwantedCharacters(ctx.proofpoint_on_demand.mail.sm.msgid);\n}\nList toAddresses = new ArrayList();\nif (ctx.proofpoint_on_demand?.mail?.sm?.to instanceof List) {\n  for (address in ctx.proofpoint_on_demand.mail.sm.to) {\n    toAddresses.add(removeUnwantedCharacters(address));\n  }\n  ctx.proofpoint_on_demand.mail.sm.to = toAddresses;\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_remove_unwanted_characters",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("proofpoint_on_demand.mail.sm.from") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("proofpoint_on_demand.mail.sm.from")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint_on_demand.mail.sm.from") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_on_demand.mail.sm.from")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.sm.msgid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.mail.sm.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "proofpoint_on_demand.mail.sm.to", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("proofpoint_on_demand.mail.sm.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "proofpoint_on_demand.mail.sm.to", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.sm.nrcpts") {
                    if let Some(val) = event.get("json.sm.nrcpts") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sm.nrcpts".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.mail.sm.nrcpts", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_sm_nrcpts_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                if event.has_value("json.sm.pri") {
                    if let Some(val) = event.get("json.sm.pri") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sm.pri".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.mail.sm.priority", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sm_pri_to_long")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.sm.proto") {
                event.rename("json.sm.proto", "proofpoint_on_demand.mail.sm.protocol")?;
            }

            if let Some(v) = event
                .get("proofpoint_on_demand.mail.sm.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }

            if event.has("json.sm.qid") {
                event.rename("json.sm.qid", "proofpoint_on_demand.mail.sm.qid")?;
            }

            if event.has("json.sm.relay") {
                event.rename("json.sm.relay", "proofpoint_on_demand.mail.sm.relay")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.sm.sizeBytes") {
                    if let Some(val) = event.get("json.sm.sizeBytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sm.sizeBytes".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint_on_demand.mail.sm.size_bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_sm_sizeBytes_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.sm.stat") {
                event.rename("json.sm.stat", "proofpoint_on_demand.mail.sm.status")?;
            }

            let _cond = {
                event.has_value("proofpoint_on_demand.mail.sm.status")
                    && event
                        .get_str("proofpoint_on_demand.mail.sm.status")
                        .is_some_and(|s| s.to_lowercase().contains("sent"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("proofpoint_on_demand.mail.sm.status")
                    && (event
                        .get_str("proofpoint_on_demand.mail.sm.status")
                        .is_some_and(|s| s.to_lowercase().contains("service unavailable"))
                        || event
                            .get_str("proofpoint_on_demand.mail.sm.status")
                            .is_some_and(|s| s.to_lowercase().contains("connection refused")))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has("json.sm.tls.verify") {
                event.rename(
                    "json.sm.tls.verify",
                    "proofpoint_on_demand.mail.sm.tls.verify",
                )?;
            }

            if event.has("json.sm.xdelay") {
                event.rename("json.sm.xdelay", "proofpoint_on_demand.mail.sm.xdelay")?;
            }

            if event.has("json.tls") {
                event.rename("json.tls", "proofpoint_on_demand.mail.tls")?;
            }

            let _cond = { event.get_str("proofpoint_on_demand.mail.tls.cipher") != Some("NONE") };
            if _cond {
                if let Some(v) = event
                    .get("proofpoint_on_demand.mail.tls.cipher")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("tls.cipher", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("proofpoint_on_demand.mail.tls.version") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("v") else {
                            break 'dissect false;
                        };
                        captured.push(("tls.version_protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("v") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("tls.version", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("tls.version_protocol") {
                if let Some(s) = event.get_string("tls.version_protocol") {
                    let lowered = s.to_lowercase();
                    event.set("tls.version_protocol", lowered)?;
                }
            }

            let _cond = { event.has_value("json.ts") && event.get_str("json.ts") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ts") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("proofpoint_on_demand.mail.ts", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ts")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                .get("proofpoint_on_demand.mail.ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
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
                event.remove("proofpoint_on_demand.mail.data");
                event.remove("proofpoint_on_demand.mail.id");
                event.remove("proofpoint_on_demand.mail.sm.from");
                event.remove("proofpoint_on_demand.mail.sm.msgid");
                event.remove("proofpoint_on_demand.mail.sm.protocol");
                event.remove("proofpoint_on_demand.mail.ts");
                event.remove("proofpoint_on_demand.mail.to");
                event.remove("proofpoint_on_demand.mail.tls.cipher");
            }

            event.remove("json");
            event.remove("_temp");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
