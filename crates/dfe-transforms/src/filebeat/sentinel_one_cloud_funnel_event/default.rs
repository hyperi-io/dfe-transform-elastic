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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("message") {
                    gsub_field(
                        event,
                        "message",
                        "message",
                        cached_regex!("\\\\\\\\\\\\\\\\"),
                        "\\\\\\\\",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
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
                            .get("_ingest.pipeline")
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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
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
                            .get("_ingest.pipeline")
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.json.put(\"k8sCluster.containerImage.value\", ctx.json['k8sCluster.containerImage']); ctx.json.remove('k8sCluster.containerImage');
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.json.put(\"k8sCluster.containerImage.value\", ctx.json['k8sCluster.containerImage']); ctx.json.remove('k8sCluster.containerImage');"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_change_name_of_k8sCluster_containerImage",
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
                                .get("_ingest.pipeline")
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
                dot_expand(event, "json", "*")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dot_expander")?;
                event.set("_ingest.on_failure_processor_tag", "dot_expander")?;
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
                            .get("_ingest.pipeline")
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
                // Painless script
                // Source: String trimQuotes(def doubleQuote, def v) {\n  if (v.startsWith(doubleQuote) && v.endsWith(doubleQuote)) {\n    v = v.substring(1, v.length() - 1);\n  }\n  return v;\n}\ndef v1 = ctx.json?.src?.process?.parent?.cmdline;\ndef v2 = ctx.json?.tgt?.process?.cmdline;\nif (v1 instanceof String && v1 != null) {\n  v1 = trimQuotes(params.double_quote, v1);\n  ctx.json.src.process.parent.put(\"cmdline\", v1);\n}\nif (v2 instanceof String && v2 != null) {\n  v2 = trimQuotes(params.double_quote, v2);\n  ctx.json.tgt.process.put(\"cmdline\", v2);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String trimQuotes(def doubleQuote, def v) {\n  if (v.startsWith(doubleQuote) && v.endsWith(doubleQuote)) {\n    v = v.substring(1, v.length() - 1);\n  }\n  return v;\n}\ndef v1 = ctx.json?.src?.process?.parent?.cmdline;\ndef v2 = ctx.json?.tgt?.process?.cmdline;\nif (v1 instanceof String && v1 != null) {\n  v1 = trimQuotes(params.double_quote, v1);\n  ctx.json.src.process.parent.put(\"cmdline\", v1);\n}\nif (v2 instanceof String && v2 != null) {\n  v2 = trimQuotes(params.double_quote, v2);\n  ctx.json.tgt.process.put(\"cmdline\", v2);\n}\n"#
                    ),
                    cached_params!("{\"double_quote\":\"\\\"\"}"),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_remove_quotes_from_begining_and_end",
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
                            .get("_ingest.pipeline")
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
                event.has_value("json.event.time") && event.get_str("json.event.time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.event.time") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => {
                                event.set("sentinel_one_cloud_funnel.event.time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.event.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_event_time")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("json.event.time") && event.get_str("json.event.time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.event.time") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.event.time".into(),
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.event.logout.tgt.user.name") {
                event.rename(
                    "json.event.logout.tgt.user.name",
                    "sentinel_one_cloud_funnel.event.logout.tgt.user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.logout.tgt.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
            }

            if event.has_value("json.event.login.tgt.domainName") {
                event.rename(
                    "json.event.login.tgt.domainName",
                    "sentinel_one_cloud_funnel.event.login.tgt.domain_name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.login.tgt.domain_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.domain", v)?;
            }

            if event.has_value("json.event.logout.tgt.domainName") {
                event.rename(
                    "json.event.logout.tgt.domainName",
                    "sentinel_one_cloud_funnel.event.logout.tgt.domain_name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.logout.tgt.domain_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.domain", v)?;
            }

            if event.has_value("json.event.login.tgt.user.name") {
                event.rename(
                    "json.event.login.tgt.user.name",
                    "sentinel_one_cloud_funnel.event.login.tgt.user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.login.tgt.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.domain") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.id") {
                event.rename("json.event.id", "sentinel_one_cloud_funnel.event.id")?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.tgt.file.name") {
                event.rename(
                    "json.tgt.file.name",
                    "sentinel_one_cloud_funnel.event.tgt.file.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.tgt.file.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if event.has_value("json.group.id") {
                event.rename("json.group.id", "sentinel_one_cloud_funnel.event.group.id")?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.group.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.id", v)?;
            }

            if event.has_value("json.endpoint.name") {
                event.rename(
                    "json.endpoint.name",
                    "sentinel_one_cloud_funnel.event.endpoint.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.endpoint.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.endpoint.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.endpoint.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os.name") {
                event.rename("json.os.name", "sentinel_one_cloud_funnel.event.os_name")?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.os_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.os.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.endpoint.os") {
                event.rename(
                    "json.endpoint.os",
                    "sentinel_one_cloud_funnel.event.endpoint.os",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.endpoint.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            let _cond = {
                ["linux", "windows", "macos", "ios", "android"].contains(
                    &event
                        .get_str("sentinel_one_cloud_funnel.event.endpoint.os")
                        .unwrap_or(""),
                )
            };
            if _cond {
                if let Some(v) = event
                    .get("sentinel_one_cloud_funnel.event.endpoint.os")
                    .cloned()
                {
                    event.set("host.os.type", v)?;
                }
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.endpoint.os") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.endpoint.os")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.endpoint.type") {
                event.rename(
                    "json.endpoint.type",
                    "sentinel_one_cloud_funnel.event.endpoint.type",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.endpoint.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.endpoint.type") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.endpoint.type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.cmdline") {
                event.rename(
                    "json.src.process.cmdline",
                    "sentinel_one_cloud_funnel.event.src.process.cmd_line",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.cmd_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if event.has_value("json.src.process.image.path") {
                event.rename(
                    "json.src.process.image.path",
                    "sentinel_one_cloud_funnel.event.src.process.image.path",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.image.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if event.has_value("json.src.process.image.md5") {
                event.rename(
                    "json.src.process.image.md5",
                    "sentinel_one_cloud_funnel.event.src.process.image.md5",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.image.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.md5", v)?;
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

            if event.has_value("json.src.process.image.sha1") {
                event.rename(
                    "json.src.process.image.sha1",
                    "sentinel_one_cloud_funnel.event.src.process.image.sha1",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.image.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha1", v)?;
            }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.image.sha256") {
                event.rename(
                    "json.src.process.image.sha256",
                    "sentinel_one_cloud_funnel.event.src.process.image.sha256",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.image.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.name") {
                event.rename(
                    "json.src.process.name",
                    "sentinel_one_cloud_funnel.event.src.process.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if event.has_value("json.src.process.parent.cmdline") {
                event.rename(
                    "json.src.process.parent.cmdline",
                    "sentinel_one_cloud_funnel.event.src.process.parent.cmd_line",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.cmd_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if event.has_value("json.osSrc.process.parent.image.path") {
                event.rename(
                    "json.osSrc.process.parent.image.path",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.path",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.image.md5") {
                event.rename(
                    "json.osSrc.process.parent.image.md5",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.md5",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.os_src_process.parent.image.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.md5", v)?;
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

            if event.has_value("json.src.process.parent.image.sha1") {
                event.rename(
                    "json.src.process.parent.image.sha1",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.sha1",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.image.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha1", v)?;
            }

            let _cond = { event.has_value("process.parent.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.parent.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.parent.image.sha256") {
                event.rename(
                    "json.src.process.parent.image.sha256",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.sha256",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.image.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha256", v)?;
            }

            let _cond = { event.has_value("process.parent.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.parent.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.parent.name") {
                event.rename(
                    "json.src.process.parent.name",
                    "sentinel_one_cloud_funnel.event.src.process.parent.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            let _cond = { event.get_str("json.src.process.parent.pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.pid") {
                        if let Some(val) = event.get("json.src.process.parent.pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.parent.pid",
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
                        "convert_json_src_process_parent_pid",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.parent.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            let _cond = { event.get_str("json.src.process.parent.rUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.rUserUid") {
                        if let Some(val) = event.get("json.src.process.parent.rUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.rUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.parent.r_user.uid",
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
                        "convert_json_src_process_parent_rUserUid",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.parent.r_user.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.real_user.id", v)?;
            }

            let _cond = { event.has_value("process.parent.real_user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.parent.real_user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.parent.rUserName") {
                event.rename(
                    "json.src.process.parent.rUserName",
                    "sentinel_one_cloud_funnel.event.src.process.parent.r_user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.r_user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.real_user.name", v)?;
            }

            let _cond = { event.has_value("process.parent.real_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.parent.real_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.src.process.parent.startTime")
                    && event.get_str("json.src.process.parent.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.src.process.parent.startTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set(
                                "sentinel_one_cloud_funnel.event.src.process.parent.start_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.src.process.parent.startTime".into(),
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
                        "date_json_src_process_parent_startTime",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.parent.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.start", v)?;
            }

            if event.has_value("json.src.process.parent.displayName") {
                event.rename(
                    "json.src.process.parent.displayName",
                    "sentinel_one_cloud_funnel.event.src.process.parent.display_name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.title", v)?;
            }

            let _cond = { event.get_str("json.src.process.parent.eUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.eUserUid") {
                        if let Some(val) = event.get("json.src.process.parent.eUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.eUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.parent.e_user.uid",
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
                        "convert_json_src_process_parent_eUserUid",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.parent.e_user.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.user.id", v)?;
            }

            let _cond = { event.has_value("process.parent.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.parent.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.parent.eUserName") {
                event.rename(
                    "json.src.process.parent.eUserName",
                    "sentinel_one_cloud_funnel.event.src.process.parent.e_user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.e_user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.user.name", v)?;
            }

            if event.has_value("json.src.process.parent.user") {
                event.rename(
                    "json.src.process.parent.user",
                    "sentinel_one_cloud_funnel.event.src.process.parent.user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.user.name", v)?;
            }

            let _cond = { event.has_value("process.parent.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.parent.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.src.process.pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.pid") {
                        if let Some(val) = event.get("json.src.process.pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.pid",
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
                        "convert_json_src_process_pid",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            let _cond = { event.get_str("json.tgt.process.pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.pid") {
                        if let Some(val) = event.get("json.tgt.process.pid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.pid",
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
                        "convert_json_tgt_process_pid",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.rUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.rUserUid") {
                        if let Some(val) = event.get("json.src.process.rUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.rUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.r_user.uid",
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
                        "convert_json_src_process_rUserUid",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.r_user.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.real_user.id", v)?;
            }

            let _cond = { event.has_value("process.real_user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.real_user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.rUserName") {
                event.rename(
                    "json.src.process.rUserName",
                    "sentinel_one_cloud_funnel.event.src.process.r_user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.r_user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.real_user.name", v)?;
            }

            let _cond = { event.has_value("process.real_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("process.real_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.src.process.startTime")
                    && event.get_str("json.src.process.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.src.process.startTime") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set(
                                "sentinel_one_cloud_funnel.event.src.process.start_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.src.process.startTime".into(),
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
                        "convert_json_src_process_startTime",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.start", v)?;
            }

            if event.has_value("json.src.process.displayName") {
                event.rename(
                    "json.src.process.displayName",
                    "sentinel_one_cloud_funnel.event.src.process.display_name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.title", v)?;
            }

            let _cond = { event.get_str("json.src.process.eUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.eUserUid") {
                        if let Some(val) = event.get("json.src.process.eUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.eUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.e_user.uid",
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
                        "convert_json_src_process_eUserUid",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.e_user.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.id", v)?;
            }

            if event.has_value("json.src.process.user") {
                event.rename(
                    "json.src.process.user",
                    "sentinel_one_cloud_funnel.event.src.process.user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.name", v)?;
            }

            if let Some(v) = event
                .get("process.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("process.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            let _cond = {
                event.get("process.user.name").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
                    serde_json::Value::String(s) => s.contains("\\"),
                    _ => false,
                })
            };
            if _cond {
                if let Some(input) = event.get_string("process.user.name") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("\\") else {
                            break 'dissect false;
                        };
                        captured.push(("user.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\\") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("user.name", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "process.user.name".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            if event.has_value("json.src.process.eUserName") {
                event.rename(
                    "json.src.process.eUserName",
                    "sentinel_one_cloud_funnel.event.src.process.e_user.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.e_user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.name", v)?;
            }

            if event.has_value("json.tiIndicator.description") {
                event.rename(
                    "json.tiIndicator.description",
                    "sentinel_one_cloud_funnel.event.ti_indicator.description",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.ti_indicator.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.description", v)?;
            }

            if event.has_value("json.tiIndicator.references") {
                event.rename(
                    "json.tiIndicator.references",
                    "sentinel_one_cloud_funnel.event.ti_indicator.references",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.ti_indicator.references")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.reference", v)?;
            }

            if event.has_value("json.tiIndicator.mitreTactics") {
                event.rename(
                    "json.tiIndicator.mitreTactics",
                    "sentinel_one_cloud_funnel.event.ti_indicator.mitre_tactics",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.ti_indicator.mitre_tactics") };
            if _cond {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.ti_indicator.mitre_tactics")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.account.id") {
                event.rename(
                    "json.account.id",
                    "sentinel_one_cloud_funnel.event.account_id",
                )?;
            }

            if event.has_value("json.agent.uuid") {
                event.rename(
                    "json.agent.uuid",
                    "sentinel_one_cloud_funnel.event.agent.uuid",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.agent.uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if event.has_value("json.agent.version") {
                event.rename(
                    "json.agent.version",
                    "sentinel_one_cloud_funnel.event.agent.version",
                )?;
            }

            if event.has_value("json.dataSource.name") {
                event.rename(
                    "json.dataSource.name",
                    "sentinel_one_cloud_funnel.event.data_source.name",
                )?;
            }

            if event.has_value("json.event.category") {
                event.rename(
                    "json.event.category",
                    "sentinel_one_cloud_funnel.event.category",
                )?;
            }

            let _cond = { event.get_str("json.event.repetitionCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.event.repetitionCount") {
                        if let Some(val) = event.get("json.event.repetitionCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.event.repetitionCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.repetition_count",
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
                        "convert_json_event_repetitionCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.event.type") {
                event.rename("json.event.type", "sentinel_one_cloud_funnel.event.type")?;
            }

            if event.has_value("json.i.scheme") {
                event.rename("json.i.scheme", "sentinel_one_cloud_funnel.event.i.scheme")?;
            }

            if event.has_value("json.i.version") {
                event.rename(
                    "json.i.version",
                    "sentinel_one_cloud_funnel.event.i.version",
                )?;
            }

            if event.has_value("json.meta.event.name") {
                event.rename(
                    "json.meta.event.name",
                    "sentinel_one_cloud_funnel.event.meta_event_name",
                )?;
            }

            if event.has_value("json.mgmt.url") {
                event.rename("json.mgmt.url", "sentinel_one_cloud_funnel.event.mgmt.url")?;
            }

            if event.has_value("json.osSrc.process.activeContent.hash") {
                event.rename(
                    "json.osSrc.process.activeContent.hash",
                    "sentinel_one_cloud_funnel.event.os_src_process.active_content.hash",
                )?;
            }

            if event.has_value("json.osSrc.process.activeContent.id") {
                event.rename(
                    "json.osSrc.process.activeContent.id",
                    "sentinel_one_cloud_funnel.event.os_src_process.active_content.id",
                )?;
            }

            if event.has_value("json.osSrc.process.activeContent.path") {
                event.rename(
                    "json.osSrc.process.activeContent.path",
                    "sentinel_one_cloud_funnel.event.os_src_process.active_content.path",
                )?;
            }

            if event.has_value("json.osSrc.process.activeContent.signedStatus") {
                event.rename(
                    "json.osSrc.process.activeContent.signedStatus",
                    "sentinel_one_cloud_funnel.event.os_src_process.active_content.signed_status",
                )?;
            }

            if event.has_value("json.osSrc.process.activeContentType") {
                event.rename(
                    "json.osSrc.process.activeContentType",
                    "sentinel_one_cloud_funnel.event.os_src_process.active_content.type",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.childProcCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.childProcCount") {
                        if let Some(val) = event.get("json.osSrc.process.childProcCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.childProcCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.child_proc_count",
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
                        "convert_json_osSrc_process_childProcCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.cmdline") {
                event.rename(
                    "json.osSrc.process.cmdline",
                    "sentinel_one_cloud_funnel.event.os_src_process.cmd_line",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.crossProcessCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.crossProcessCount") {
                        if let Some(val) = event.get("json.osSrc.process.crossProcessCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.crossProcessCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.cross_process.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_crossProcessCount",
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
                                .get("_ingest.pipeline")
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
                event.get_str("json.osSrc.process.crossProcessDupRemoteProcessHandleCount")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.crossProcessDupRemoteProcessHandleCount")
                    {
                        if let Some(val) =
                            event.get("json.osSrc.process.crossProcessDupRemoteProcessHandleCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "json.osSrc.process.crossProcessDupRemoteProcessHandleCount"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.cross_process.dup.remote_process_handle_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_crossProcessDupRemoteProcessHandleCount",
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
                                .get("_ingest.pipeline")
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
                event.get_str("json.osSrc.process.crossProcessDupThreadHandleCount") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.crossProcessDupThreadHandleCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.crossProcessDupThreadHandleCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.crossProcessDupThreadHandleCount"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.cross_process.dup.thread_handle_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_crossProcessDupThreadHandleCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.crossProcessOpenProcessCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.crossProcessOpenProcessCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.crossProcessOpenProcessCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.crossProcessOpenProcessCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.cross_process.open_process_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_osSrc_process_crossProcessOpenProcessCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.crossProcessOutOfStorylineCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.crossProcessOutOfStorylineCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.crossProcessOutOfStorylineCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.crossProcessOutOfStorylineCount"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.cross_process.out_of_storyline_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_crossProcessOutOfStorylineCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.crossProcessThreadCreateCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.crossProcessThreadCreateCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.crossProcessThreadCreateCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.crossProcessThreadCreateCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.cross_process.thread_create_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_crossProcessThreadCreateCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.displayName") {
                event.rename(
                    "json.osSrc.process.displayName",
                    "sentinel_one_cloud_funnel.event.os_src_process.display_name",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.dnsCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.dnsCount") {
                        if let Some(val) = event.get("json.osSrc.process.dnsCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.dnsCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.dns_count",
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
                        "convert_json__osSrc_process_dnsCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.image.binaryIsExecutable") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.image.binaryIsExecutable") {
                        if let Some(val) = event.get("json.osSrc.process.image.binaryIsExecutable")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.image.binaryIsExecutable".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.image.binary_is_executable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_image_binaryIsExecutable",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.image.md5") {
                event.rename(
                    "json.osSrc.process.image.md5",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.md5",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.os_src_process.image.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.image.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.osSrc.process.image.path") {
                event.rename(
                    "json.osSrc.process.image.path",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.path",
                )?;
            }

            if event.has_value("json.osSrc.process.image.sha1") {
                event.rename(
                    "json.osSrc.process.image.sha1",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.sha1",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.os_src_process.image.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.image.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.osSrc.process.image.sha256") {
                event.rename(
                    "json.osSrc.process.image.sha256",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.sha256",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.os_src_process.image.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.image.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("json.osSrc.process.indicatorBootConfigurationUpdateCount")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorBootConfigurationUpdateCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.indicatorBootConfigurationUpdateCount")
                        {
                            let converted =
                                convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                            path: "json.osSrc.process.indicatorBootConfigurationUpdateCount".into(),
                            message,
                        }
                                })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.boot_configuration_update_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorBootConfigurationUpdateCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.indicatorEvasionCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorEvasionCount") {
                        if let Some(val) = event.get("json.osSrc.process.indicatorEvasionCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorEvasionCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.evasion_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorEvasionCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.indicatorExploitationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorExploitationCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.indicatorExploitationCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorExploitationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.exploitation_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorExploitationCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.indicatorGeneral.count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorGeneral.count") {
                        if let Some(val) = event.get("json.osSrc.process.indicatorGeneral.count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorGeneral.count".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.general_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorGeneral_count",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.indicatorInfostealerCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorInfostealerCount") {
                        if let Some(val) = event.get("json.osSrc.process.indicatorInfostealerCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorInfostealerCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator_info_stealer_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorInfostealerCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.indicatorInjectionCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorInjectionCount") {
                        if let Some(val) = event.get("json.osSrc.process.indicatorInjectionCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorInjectionCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.injection_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorInjectionCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.indicatorPersistenceCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorPersistenceCount") {
                        if let Some(val) = event.get("json.osSrc.process.indicatorPersistenceCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorPersistenceCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.persistence_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorPersistenceCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.indicatorPostExploitationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorPostExploitationCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.indicatorPostExploitationCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorPostExploitationCount"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.post_exploitation_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorPostExploitationCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.indicatorRansomwareCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorRansomwareCount") {
                        if let Some(val) = event.get("json.osSrc.process.indicatorRansomwareCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorRansomwareCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.ransomware_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_indicatorRansomwareCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.indicatorReconnaissanceCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.indicatorReconnaissanceCount") {
                        if let Some(val) =
                            event.get("json.osSrc.process.indicatorReconnaissanceCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.indicatorReconnaissanceCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.indicator.reconnaissance_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_osSrc_process_indicatorReconnaissanceCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.integrityLevel") {
                event.rename(
                    "json.osSrc.process.integrityLevel",
                    "sentinel_one_cloud_funnel.event.os_src_process.integrity_level",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.isNative64Bit") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.isNative64Bit") {
                        if let Some(val) = event.get("json.osSrc.process.isNative64Bit") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.isNative64Bit".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.is_native_64_bit",
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
                        "convert_json_osSrc_process_isNative64Bit",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.isRedirectCmdProcessor") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.isRedirectCmdProcessor") {
                        if let Some(val) = event.get("json.osSrc.process.isRedirectCmdProcessor") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.isRedirectCmdProcessor".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.is_redirect_cmd_processor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_isRedirectCmdProcessor",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.isStorylineRoot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.isStorylineRoot") {
                        if let Some(val) = event.get("json.osSrc.process.isStorylineRoot") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.isStorylineRoot".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.is_storyline_root",
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
                        "convert_json_osSrc_process_isStorylineRoot",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.moduleCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.moduleCount") {
                        if let Some(val) = event.get("json.osSrc.process.moduleCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.moduleCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.module_count",
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
                        "convert_json_osSrc_process_moduleCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.name") {
                event.rename(
                    "json.osSrc.process.name",
                    "sentinel_one_cloud_funnel.event.os_src_process.name",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.netConnCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.netConnCount") {
                        if let Some(val) = event.get("json.osSrc.process.netConnCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.netConnCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.net_conn.count",
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
                        "convert_json_osSrc_process_netConnCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.netConnInCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.netConnInCount") {
                        if let Some(val) = event.get("json.osSrc.process.netConnInCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.netConnInCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.net_conn.in_count",
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
                        "convert_json_osSrc_process_netConnInCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.netConnOutCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.netConnOutCount") {
                        if let Some(val) = event.get("json.osSrc.process.netConnOutCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.netConnOutCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.net_conn.out_count",
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
                        "convert_json_osSrc_process_netConnOutCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.parent.activeContent.hash") {
                event.rename(
                    "json.osSrc.process.parent.activeContent.hash",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.active_content.hash",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.activeContent.id") {
                event.rename(
                    "json.osSrc.process.parent.activeContent.id",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.active_content.id",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.activeContent.path") {
                event.rename(
                    "json.osSrc.process.parent.activeContent.path",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.active_content.path",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.activeContent.signedStatus") {
                event.rename("json.osSrc.process.parent.activeContent.signedStatus", "sentinel_one_cloud_funnel.event.os_src_process.parent.active_content.signed_status")?;
            }

            if event.has_value("json.osSrc.process.parent.activeContentType") {
                event.rename(
                    "json.osSrc.process.parent.activeContentType",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.active_content.type",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.cmdline") {
                event.rename(
                    "json.osSrc.process.parent.cmdline",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.cmd_line",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.displayName") {
                event.rename(
                    "json.osSrc.process.parent.displayName",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.display_name",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.image.sha1") {
                event.rename(
                    "json.osSrc.process.parent.image.sha1",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.sha1",
                )?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.os_src_process.parent.image.sha1")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.parent.image.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.osSrc.process.parent.image.sha256") {
                event.rename(
                    "json.osSrc.process.parent.image.sha256",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.sha256",
                )?;
            }

            let _cond = {
                event
                    .has_value("sentinel_one_cloud_funnel.event.os_src_process.parent.image.sha256")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get(
                                "sentinel_one_cloud_funnel.event.os_src_process.parent.image.sha256"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.osSrc.process.parent.integrityLevel") {
                event.rename(
                    "json.osSrc.process.parent.integrityLevel",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.integrity_level",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.parent.isNative64Bit") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.isNative64Bit") {
                        if let Some(val) = event.get("json.osSrc.process.parent.isNative64Bit") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.isNative64Bit".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.parent.is_native_64_bit", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_parent_isNative64Bit",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.parent.isRedirectCmdProcessor") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.isRedirectCmdProcessor") {
                        if let Some(val) =
                            event.get("json.osSrc.process.parent.isRedirectCmdProcessor")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.isRedirectCmdProcessor".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.parent.is_redirect_cmd_processor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_parent_isRedirectCmdProcessor",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.parent.isStorylineRoot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.isStorylineRoot") {
                        if let Some(val) = event.get("json.osSrc.process.parent.isStorylineRoot") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.isStorylineRoot".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.parent.is_storyline_root", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_parent_isStorylineRoot",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.parent.name") {
                event.rename(
                    "json.osSrc.process.parent.name",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.name",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.parent.pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.pid") {
                        if let Some(val) = event.get("json.osSrc.process.parent.pid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.parent.pid",
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
                        "convert_json_osSrc_process_parent_pid",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.parent.publisher") {
                event.rename(
                    "json.osSrc.process.parent.publisher",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.publisher",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.reasonSignatureInvalid") {
                event.rename("json.osSrc.process.parent.reasonSignatureInvalid", "sentinel_one_cloud_funnel.event.os_src_process.parent.reason_signature_invalid")?;
            }

            let _cond = { event.get_str("json.osSrc.process.parent.sessionId") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.sessionId") {
                        if let Some(val) = event.get("json.osSrc.process.parent.sessionId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.sessionId".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.parent.session_id",
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
                        "convert_json_osSrc_process_parent_sessionId",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.parent.signedStatus") {
                event.rename(
                    "json.osSrc.process.parent.signedStatus",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.signed_status",
                )?;
            }

            let _cond = {
                event.has_value("json.osSrc.process.parent.startTime")
                    && event.get_str("json.osSrc.process.parent.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.osSrc.process.parent.startTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.parent.start_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.osSrc.process.parent.startTime".into(),
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
                        "date_json_osSrc_process_parent_startTime",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.parent.storyline.id") {
                event.rename(
                    "json.osSrc.process.parent.storyline.id",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.storyline_id",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.uid") {
                event.rename(
                    "json.osSrc.process.parent.uid",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.uid",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.user") {
                event.rename(
                    "json.osSrc.process.parent.user",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.user.name",
                )?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.os_src_process.parent.user.name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.parent.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.pid") {
                        if let Some(val) = event.get("json.osSrc.process.pid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.pid",
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
                        "convert_json_osSrc_process_pid",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.publisher") {
                event.rename(
                    "json.osSrc.process.publisher",
                    "sentinel_one_cloud_funnel.event.os_src_process.publisher",
                )?;
            }

            if event.has_value("json.osSrc.process.reasonSignatureInvalid") {
                event.rename(
                    "json.osSrc.process.reasonSignatureInvalid",
                    "sentinel_one_cloud_funnel.event.os_src_process.reason_signature_invalid",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.registryChangeCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.registryChangeCount") {
                        if let Some(val) = event.get("json.osSrc.process.registryChangeCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.registryChangeCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.registry_change_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_registryChangeCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.sessionId") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.sessionId") {
                        if let Some(val) = event.get("json.osSrc.process.sessionId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.sessionId".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.session_id",
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
                        "convert_json_osSrc_process_sessionId",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.signedStatus") {
                event.rename(
                    "json.osSrc.process.signedStatus",
                    "sentinel_one_cloud_funnel.event.os_src_process.signed_status",
                )?;
            }

            let _cond = {
                event.has_value("json.osSrc.process.startTime")
                    && event.get_str("json.osSrc.process.startTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.osSrc.process.startTime") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.start_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.osSrc.process.startTime".into(),
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
                        "date_json_osSrc_process_startTime",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.storyline.id") {
                event.rename(
                    "json.osSrc.process.storyline.id",
                    "sentinel_one_cloud_funnel.event.os_src_process.storyline_id",
                )?;
            }

            if event.has_value("json.osSrc.process.subsystem") {
                event.rename(
                    "json.osSrc.process.subsystem",
                    "sentinel_one_cloud_funnel.event.os_src_process.subsystem",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.tgtFileCreationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.tgtFileCreationCount") {
                        if let Some(val) = event.get("json.osSrc.process.tgtFileCreationCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.tgtFileCreationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.tgt_file.creation_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_tgtFileCreationCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.tgtFileDeletionCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.tgtFileDeletionCount") {
                        if let Some(val) = event.get("json.osSrc.process.tgtFileDeletionCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.tgtFileDeletionCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.tgt_file.deletion_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_tgtFileDeletionCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.osSrc.process.tgtFileModificationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.tgtFileModificationCount") {
                        if let Some(val) = event.get("json.osSrc.process.tgtFileModificationCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.tgtFileModificationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.tgt_file.modification_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_tgtFileModificationCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.uid") {
                event.rename(
                    "json.osSrc.process.uid",
                    "sentinel_one_cloud_funnel.event.os_src_process.uid",
                )?;
            }

            if event.has_value("json.osSrc.process.user") {
                event.rename(
                    "json.osSrc.process.user",
                    "sentinel_one_cloud_funnel.event.os_src_process.user.name",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.os_src_process.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.osSrc.process.verifiedStatus") {
                event.rename(
                    "json.osSrc.process.verifiedStatus",
                    "sentinel_one_cloud_funnel.event.os_src_process.verified_status",
                )?;
            }

            let _cond = {
                event.has_value("json")
                    && event.has_value("json.sca:atlantisIngestTime")
                    && event.get_str("json.sca:atlantisIngestTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.sca:atlantisIngestTime") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set(
                                "sentinel_one_cloud_funnel.event.sca.atlantis_ingest_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.sca:atlantisIngestTime".into(),
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
                        "date_json_sca_atlantisIngestTime",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.site.id") {
                event.rename("json.site.id", "sentinel_one_cloud_funnel.event.site.id")?;
            }

            if event.has_value("json.site.name") {
                event.rename(
                    "json.site.name",
                    "sentinel_one_cloud_funnel.event.site.name",
                )?;
            }

            if event.has_value("json.src.process.activeContent.hash") {
                event.rename(
                    "json.src.process.activeContent.hash",
                    "sentinel_one_cloud_funnel.event.src.process.active_content.hash",
                )?;
            }

            if event.has_value("json.src.process.activeContent.id") {
                event.rename(
                    "json.src.process.activeContent.id",
                    "sentinel_one_cloud_funnel.event.src.process.active_content.id",
                )?;
            }

            if event.has_value("json.src.process.activeContent.path") {
                event.rename(
                    "json.src.process.activeContent.path",
                    "sentinel_one_cloud_funnel.event.src.process.active_content.path",
                )?;
            }

            if event.has_value("json.src.process.activeContent.signedStatus") {
                event.rename(
                    "json.src.process.activeContent.signedStatus",
                    "sentinel_one_cloud_funnel.event.src.process.active_content.signed_status",
                )?;
            }

            if event.has_value("json.src.process.activeContentType") {
                event.rename(
                    "json.src.process.activeContentType",
                    "sentinel_one_cloud_funnel.event.src.process.active_content.type",
                )?;
            }

            let _cond = { event.get_str("json.src.process.childProcCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.childProcCount") {
                        if let Some(val) = event.get("json.src.process.childProcCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.childProcCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.child_proc_count",
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
                        "convert_json_src_process_childProcCount",
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
                                .get("_ingest.pipeline")
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
                event.get_str("json.src.process.crossProcessDupRemoteProcessHandleCount")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.crossProcessDupRemoteProcessHandleCount") {
                        if let Some(val) =
                            event.get("json.src.process.crossProcessDupRemoteProcessHandleCount")
                        {
                            let converted =
                                convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                            path: "json.src.process.crossProcessDupRemoteProcessHandleCount".into(),
                            message,
                        }
                                })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.cross_process.dup.remote_process_handle_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_crossProcessDupRemoteProcessHandleCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.crossProcessOpenProcessCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.crossProcessOpenProcessCount") {
                        if let Some(val) =
                            event.get("json.src.process.crossProcessOpenProcessCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.crossProcessOpenProcessCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.cross_process.open_process_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_crossProcessOpenProcessCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.indicatorExploitationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorExploitationCount") {
                        if let Some(val) = event.get("json.src.process.indicatorExploitationCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorExploitationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.exploitation_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorExploitationCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.indicatorInjectionCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorInjectionCount") {
                        if let Some(val) = event.get("json.src.process.indicatorInjectionCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorInjectionCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.injection_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorInjectionCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.indicatorRansomwareCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorRansomwareCount") {
                        if let Some(val) = event.get("json.src.process.indicatorRansomwareCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorRansomwareCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.ransomware_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorRansomwareCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.indicatorReconnaissanceCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorReconnaissanceCount") {
                        if let Some(val) =
                            event.get("json.src.process.indicatorReconnaissanceCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorReconnaissanceCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.reconnaissance_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorReconnaissanceCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.integrityLevel") {
                event.rename(
                    "json.src.process.integrityLevel",
                    "sentinel_one_cloud_funnel.event.src.process.integrity_level",
                )?;
            }

            if event.has_value("json.src.process.lUserName") {
                event.rename(
                    "json.src.process.lUserName",
                    "sentinel_one_cloud_funnel.event.src.process.l_user.name",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.src.process.l_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.process.l_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.src.process.lUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.lUserUid") {
                        if let Some(val) = event.get("json.src.process.lUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.lUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.l_user.uid",
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
                        "convert_json_src_process_lUserUid",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.src.process.l_user.uid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.process.l_user.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.src.process.moduleCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.moduleCount") {
                        if let Some(val) = event.get("json.src.process.moduleCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.moduleCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.module_count",
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
                        "convert_json_src_process_moduleCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.netConnInCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.netConnInCount") {
                        if let Some(val) = event.get("json.src.process.netConnInCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.netConnInCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.net_conn.in_count",
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
                        "convert_json_src_process_netConnInCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.parent.activeContent.hash") {
                event.rename(
                    "json.src.process.parent.activeContent.hash",
                    "sentinel_one_cloud_funnel.event.src.process.parent.active_content.hash",
                )?;
            }

            if event.has_value("json.src.process.parent.activeContent.id") {
                event.rename(
                    "json.src.process.parent.activeContent.id",
                    "sentinel_one_cloud_funnel.event.src.process.parent.active_content.id",
                )?;
            }

            if event.has_value("json.src.process.parent.activeContent.path") {
                event.rename(
                    "json.src.process.parent.activeContent.path",
                    "sentinel_one_cloud_funnel.event.src.process.parent.active_content.path",
                )?;
            }

            if event.has_value("json.src.process.parent.activeContent.signedStatus") {
                event.rename("json.src.process.parent.activeContent.signedStatus", "sentinel_one_cloud_funnel.event.src.process.parent.active_content.signed_status")?;
            }

            if event.has_value("json.src.process.parent.activeContentType") {
                event.rename(
                    "json.src.process.parent.activeContentType",
                    "sentinel_one_cloud_funnel.event.src.process.parent.active_content.type",
                )?;
            }

            if event.has_value("json.src.process.parent.image.md5") {
                event.rename(
                    "json.src.process.parent.image.md5",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.md5",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.src.process.parent.image.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.process.parent.image.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.parent.image.path") {
                event.rename(
                    "json.src.process.parent.image.path",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.path",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.image.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if event.has_value("json.src.process.parent.integrityLevel") {
                event.rename(
                    "json.src.process.parent.integrityLevel",
                    "sentinel_one_cloud_funnel.event.src.process.parent.integrity_level",
                )?;
            }

            let _cond = { event.get_str("json.src.process.parent.isNative64Bit") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.isNative64Bit") {
                        if let Some(val) = event.get("json.src.process.parent.isNative64Bit") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.isNative64Bit".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.parent.is_native_64_bit", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_parent_isNative64Bit",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.parent.isRedirectCmdProcessor") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.isRedirectCmdProcessor") {
                        if let Some(val) =
                            event.get("json.src.process.parent.isRedirectCmdProcessor")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.isRedirectCmdProcessor".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.parent.is_redirect_cmd_processor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_parent_isRedirectCmdProcessor",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.parent.isStorylineRoot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.isStorylineRoot") {
                        if let Some(val) = event.get("json.src.process.parent.isStorylineRoot") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.isStorylineRoot".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.parent.is_storyline_root", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_parent_isStorylineRoot",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.parent.lUserName") {
                event.rename(
                    "json.src.process.parent.lUserName",
                    "sentinel_one_cloud_funnel.event.src.process.parent.l_user.name",
                )?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.src.process.parent.l_user.name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.process.parent.l_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.src.process.parent.lUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.lUserUid") {
                        if let Some(val) = event.get("json.src.process.parent.lUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.lUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.parent.l_user.uid",
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
                        "convert_json_src_process_parent_lUserUid",
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.src.process.parent.l_user.uid")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.process.parent.l_user.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.parent.publisher") {
                event.rename(
                    "json.src.process.parent.publisher",
                    "sentinel_one_cloud_funnel.event.src.process.parent.publisher",
                )?;
            }

            if event.has_value("json.src.process.parent.reasonSignatureInvalid") {
                event.rename(
                    "json.src.process.parent.reasonSignatureInvalid",
                    "sentinel_one_cloud_funnel.event.src.process.parent.reason_signature_invalid",
                )?;
            }

            let _cond = { event.get_str("json.src.process.parent.sessionId") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.sessionId") {
                        if let Some(val) = event.get("json.src.process.parent.sessionId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.sessionId".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.parent.session_id",
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
                        "convert_json_src_process_parent_sessionId",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.parent.signedStatus") {
                event.rename(
                    "json.src.process.parent.signedStatus",
                    "sentinel_one_cloud_funnel.event.src.process.parent.signed_status",
                )?;
            }

            if event.has_value("json.src.process.parent.storyline.id") {
                event.rename(
                    "json.src.process.parent.storyline.id",
                    "sentinel_one_cloud_funnel.event.src.process.parent.storyline_id",
                )?;
            }

            if event.has_value("json.src.process.parent.uid") {
                event.rename(
                    "json.src.process.parent.uid",
                    "sentinel_one_cloud_funnel.event.src.process.parent.uid",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.parent.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.entity_id", v)?;
            }

            if event.has_value("json.src.process.parent.userSid") {
                event.rename(
                    "json.src.process.parent.userSid",
                    "sentinel_one_cloud_funnel.event.src.process.parent.user.sid",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.src.process.parent.user.sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.process.parent.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src.process.storyline.id") {
                event.rename(
                    "json.src.process.storyline.id",
                    "sentinel_one_cloud_funnel.event.src.process.storyline_id",
                )?;
            }

            if event.has_value("json.src.process.subsystem") {
                event.rename(
                    "json.src.process.subsystem",
                    "sentinel_one_cloud_funnel.event.src.process.subsystem",
                )?;
            }

            let _cond = { event.get_str("json.src.process.tgtFileCreationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.tgtFileCreationCount") {
                        if let Some(val) = event.get("json.src.process.tgtFileCreationCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.tgtFileCreationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.tgt_file.creation_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_tgtFileCreationCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.tgt.file.isSigned") {
                event.rename(
                    "json.tgt.file.isSigned",
                    "sentinel_one_cloud_funnel.event.tgt.file.is_signed",
                )?;
            }

            if event.has_value("json.tgt.process.eUserName") {
                event.rename(
                    "json.tgt.process.eUserName",
                    "sentinel_one_cloud_funnel.event.tgt.process.e_user.name",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.e_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.e_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.eUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.eUserUid") {
                        if let Some(val) = event.get("json.tgt.process.eUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.eUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.e_user.uid",
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
                        "convert_json_tgt_process_eUserUid",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.e_user.uid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.e_user.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.process.lUserName") {
                event.rename(
                    "json.tgt.process.lUserName",
                    "sentinel_one_cloud_funnel.event.tgt.process.l_user.name",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.l_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.l_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.lUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.lUserUid") {
                        if let Some(val) = event.get("json.tgt.process.lUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.lUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.l_user.uid",
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
                        "convert_json_tgt_process_lUserUid",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.l_user.uid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.l_user.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.process.rUserName") {
                event.rename(
                    "json.tgt.process.rUserName",
                    "sentinel_one_cloud_funnel.event.tgt.process.r_user.name",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.r_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.r_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.rUserUid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.rUserUid") {
                        if let Some(val) = event.get("json.tgt.process.rUserUid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.rUserUid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.r_user.uid",
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
                        "convert_json_tgt_process_rUserUid",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.r_user.uid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.r_user.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.src.process.tid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.tid") {
                        if let Some(val) = event.get("json.src.process.tid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.tid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.tid",
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
                        "convert_json_src_process_tid",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.process.tid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.thread.id", v)?;
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
                            &["ISO8601", "epoch_millis", "HH:mm:ss.SSS"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("sentinel_one_cloud_funnel.event.timestamp", parsed)?
                            }
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
                    event.set("_ingest.on_failure_processor_tag", "date_json_timestamp")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.dataSource.category") {
                event.rename(
                    "json.dataSource.category",
                    "sentinel_one_cloud_funnel.event.data_source.category",
                )?;
            }

            if event.has_value("json.mgmt.id") {
                event.rename("json.mgmt.id", "sentinel_one_cloud_funnel.event.mgmt.id")?;
            }

            if event.has_value("json.mgmt.osRevision") {
                event.rename(
                    "json.mgmt.osRevision",
                    "sentinel_one_cloud_funnel.event.mgmt.os_revision",
                )?;
            }

            if event.has_value("json.process.unique.key") {
                event.rename(
                    "json.process.unique.key",
                    "sentinel_one_cloud_funnel.event.process_unique_key",
                )?;
            }

            let _cond = {
                event.has_value("json")
                    && event.has_value("json.sca:ingestTime")
                    && event.get_str("json.sca:ingestTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.sca:ingestTime") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event
                                .set("sentinel_one_cloud_funnel.event.sca.ingest_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.sca:ingestTime".into(),
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
                        "date_json_sca.ingestTime",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.crossProcessDupThreadHandleCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.crossProcessDupThreadHandleCount") {
                        if let Some(val) =
                            event.get("json.src.process.crossProcessDupThreadHandleCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.crossProcessDupThreadHandleCount"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.cross_process.dup.thread_handle_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_crossProcessDupThreadHandleCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.dnsCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.dnsCount") {
                        if let Some(val) = event.get("json.src.process.dnsCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.dnsCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.dns_count",
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
                        "convert_json_src_process_dnsCount",
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
                                .get("_ingest.pipeline")
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
                event.get_str("json.src.process.indicatorBootConfigurationUpdateCount") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorBootConfigurationUpdateCount") {
                        if let Some(val) =
                            event.get("json.src.process.indicatorBootConfigurationUpdateCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorBootConfigurationUpdateCount"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.boot_configuration_update_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorBootConfigurationUpdateCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.indicatorEvasionCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorEvasionCount") {
                        if let Some(val) = event.get("json.src.process.indicatorEvasionCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorEvasionCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.evasion_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorEvasionCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.indicatorGeneralCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorGeneralCount") {
                        if let Some(val) = event.get("json.src.process.indicatorGeneralCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorGeneralCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.general_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorGeneralCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.indicatorInfostealerCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorInfostealerCount") {
                        if let Some(val) = event.get("json.src.process.indicatorInfostealerCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorInfostealerCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.info_stealer_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorInfostealerCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.indicatorPersistenceCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorPersistenceCount") {
                        if let Some(val) = event.get("json.src.process.indicatorPersistenceCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorPersistenceCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.persistence_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorPersistenceCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.isNative64Bit") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.isNative64Bit") {
                        if let Some(val) = event.get("json.src.process.isNative64Bit") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.isNative64Bit".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.is_native_64_bit",
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
                        "convert_json_src_process_isNative64Bit",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.isRedirectCmdProcessor") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.isRedirectCmdProcessor") {
                        if let Some(val) = event.get("json.src.process.isRedirectCmdProcessor") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.isRedirectCmdProcessor".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.is_redirect_cmd_processor", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_isRedirectCmdProcessor",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.netConnCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.netConnCount") {
                        if let Some(val) = event.get("json.src.process.netConnCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.netConnCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.net_conn.count",
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
                        "convert_json_src_process_netConnCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.netConnOutCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.netConnOutCount") {
                        if let Some(val) = event.get("json.src.process.netConnOutCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.netConnOutCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.net_conn.out_count",
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
                        "convert_json_src_process_netConnOutCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.parent.subsystem") {
                event.rename(
                    "json.src.process.parent.subsystem",
                    "sentinel_one_cloud_funnel.event.src.process.parent.subsystem",
                )?;
            }

            let _cond = { event.get_str("json.src.process.registryChangeCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.registryChangeCount") {
                        if let Some(val) = event.get("json.src.process.registryChangeCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.registryChangeCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.registry_change_count",
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
                        "convert_json_src_process_registryChangeCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.sessionId") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.sessionId") {
                        if let Some(val) = event.get("json.src.process.sessionId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.sessionId".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.session_id",
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
                        "convert_json_src_process_sessionId",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.signedStatus") {
                event.rename(
                    "json.src.process.signedStatus",
                    "sentinel_one_cloud_funnel.event.src.process.signed_status",
                )?;
            }

            let _cond = {
                event.get_str("sentinel_one_cloud_funnel.event.src.process.signed_status")
                    == Some("signed")
            };
            if _cond {
                event.set("process.code_signature.exists", json!(true))?;
            }

            let _cond = {
                event.get_str("sentinel_one_cloud_funnel.event.src.process.signed_status")
                    == Some("unsigned")
            };
            if _cond {
                event.set("process.code_signature.exists", json!(false))?;
            }

            let _cond = { event.get_str("json.src.process.tgtFileDeletionCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.tgtFileDeletionCount") {
                        if let Some(val) = event.get("json.src.process.tgtFileDeletionCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.tgtFileDeletionCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.tgt_file.deletion_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_tgtFileDeletionCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.tgtFileModificationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.tgtFileModificationCount") {
                        if let Some(val) = event.get("json.src.process.tgtFileModificationCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.tgtFileModificationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.tgt_file.modification_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_tgtFileModificationCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.uid") {
                event.rename(
                    "json.src.process.uid",
                    "sentinel_one_cloud_funnel.event.src.process.uid",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.entity_id", v)?;
            }

            if event.has_value("json.packet.id") {
                event.rename(
                    "json.packet.id",
                    "sentinel_one_cloud_funnel.event.packet_id",
                )?;
            }

            if event.has_value("json.trace.id") {
                event.rename("json.trace.id", "sentinel_one_cloud_funnel.event.trace_id")?;
            }

            let _cond = { event.get_str("json.src.process.crossProcessCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.crossProcessCount") {
                        if let Some(val) = event.get("json.src.process.crossProcessCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.crossProcessCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.cross_process.count",
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
                        "convert_json_src_process_crossProcessCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.crossProcessOutOfStorylineCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.crossProcessOutOfStorylineCount") {
                        if let Some(val) =
                            event.get("json.src.process.crossProcessOutOfStorylineCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.crossProcessOutOfStorylineCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.cross_process.out_of_storyline_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_crossProcessOutOfStorylineCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.crossProcessThreadCreateCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.crossProcessThreadCreateCount") {
                        if let Some(val) =
                            event.get("json.src.process.crossProcessThreadCreateCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.crossProcessThreadCreateCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.cross_process.thread_create_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_crossProcessThreadCreateCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.image.binaryIsExecutable") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.image.binaryIsExecutable") {
                        if let Some(val) = event.get("json.src.process.image.binaryIsExecutable") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.image.binaryIsExecutable".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.image.binary_is_executable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_image_binaryIsExecutable",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.indicatorPostExploitationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.indicatorPostExploitationCount") {
                        if let Some(val) =
                            event.get("json.src.process.indicatorPostExploitationCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.indicatorPostExploitationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.indicator.post_exploitation_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_indicatorPostExploitationCount",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.isStorylineRoot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.isStorylineRoot") {
                        if let Some(val) = event.get("json.src.process.isStorylineRoot") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.isStorylineRoot".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.is_storyline_root",
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
                        "convert_json_src_process_isStorylineRoot",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.publisher") {
                event.rename(
                    "json.src.process.publisher",
                    "sentinel_one_cloud_funnel.event.src.process.publisher",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.process.publisher")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.code_signature.subject_name", v)?;
            }

            if event.has_value("json.src.process.verifiedStatus") {
                event.rename(
                    "json.src.process.verifiedStatus",
                    "sentinel_one_cloud_funnel.event.src.process.verified_status",
                )?;
            }

            let _cond = {
                event
                    .get("sentinel_one_cloud_funnel.event.src.process.verified_status")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("verified"))
                        }
                        serde_json::Value::String(s) => s.contains("verified"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("process.code_signature.trusted", json!(true))?;
            }

            let _cond = { event.get_bool("process.code_signature.exists") == Some(true) };
            if _cond {
                if !event.has("process.code_signature.trusted") {
                    event.set("process.code_signature.trusted", json!(false))?;
                }
            }

            if event.has_value("json.driver.certificate.thumbprint") {
                event.rename(
                    "json.driver.certificate.thumbprint",
                    "sentinel_one_cloud_funnel.event.driver.certificate.thumbprint.value",
                )?;
            }

            let _cond =
                { event.get_str("json.driver.certificate.thumbprintAlgorithm") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.driver.certificate.thumbprintAlgorithm") {
                        if let Some(val) = event.get("json.driver.certificate.thumbprintAlgorithm")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.driver.certificate.thumbprintAlgorithm".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.driver.certificate.thumbprint.algorithm", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_driver_certificate_thumbprintAlgorithm",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.driver.isLoadedBeforeMonitor") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.driver.isLoadedBeforeMonitor") {
                        if let Some(val) = event.get("json.driver.isLoadedBeforeMonitor") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.driver.isLoadedBeforeMonitor".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.driver.is_loaded_before_monitor",
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
                        "convert_json_driver_isLoadedBeforeMonitor",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.driver.loadVerdict") {
                event.rename(
                    "json.driver.loadVerdict",
                    "sentinel_one_cloud_funnel.event.driver.load_verdict",
                )?;
            }

            if event.has_value("json.driver.peSha1") {
                event.rename(
                    "json.driver.peSha1",
                    "sentinel_one_cloud_funnel.event.driver.pe.sha1",
                )?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.driver.pe.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.driver.pe.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.driver.peSha256") {
                event.rename(
                    "json.driver.peSha256",
                    "sentinel_one_cloud_funnel.event.driver.pe.sha256",
                )?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.driver.pe.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.driver.pe.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.driver.startType") {
                event.rename(
                    "json.driver.startType",
                    "sentinel_one_cloud_funnel.event.driver.start_type",
                )?;
            }

            if event.has_value("json.event.dns.status") {
                event.rename(
                    "json.event.dns.status",
                    "sentinel_one_cloud_funnel.event.dns.status",
                )?;
            }

            if event.has_value("json.event.login.tgt.userSid") {
                event.rename(
                    "json.event.login.tgt.userSid",
                    "sentinel_one_cloud_funnel.event.login.tgt.user.sid",
                )?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.login.tgt.user.sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.login.tgt.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.logout.tgt.userSid") {
                event.rename(
                    "json.event.logout.tgt.userSid",
                    "sentinel_one_cloud_funnel.event.logout.tgt.user.sid",
                )?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.logout.tgt.user.sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.logout.tgt.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.logout.type") {
                event.rename(
                    "json.event.logout.type",
                    "sentinel_one_cloud_funnel.event.logout.type",
                )?;
            }

            let _cond = { event.get_str("json.event.processtermination.exitCode") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.event.processtermination.exitCode") {
                        if let Some(val) = event.get("json.event.processtermination.exitCode") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.event.processtermination.exitCode".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.process_termination.exit_code",
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
                        "convert_json_event_processtermination.exitCode",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.event.processtermination.signal") {
                event.rename(
                    "json.event.processtermination.signal",
                    "sentinel_one_cloud_funnel.event.process_termination.signal",
                )?;
            }

            if event.has_value("json.event.url.source") {
                event.rename(
                    "json.event.url.source",
                    "sentinel_one_cloud_funnel.event.url.source",
                )?;
            }

            if event.has_value("json.group.type") {
                event.rename(
                    "json.group.type",
                    "sentinel_one_cloud_funnel.event.group.type",
                )?;
            }

            if event.has_value("json.indicator.identifier") {
                event.rename(
                    "json.indicator.identifier",
                    "sentinel_one_cloud_funnel.event.indicator.identifier",
                )?;
            }

            if event.has_value("json.namedPipe.accessMode") {
                event.rename(
                    "json.namedPipe.accessMode",
                    "sentinel_one_cloud_funnel.event.named_pipe.access_mode",
                )?;
            }

            if event.has_value("json.namedPipe.connectionType") {
                event.rename(
                    "json.namedPipe.connectionType",
                    "sentinel_one_cloud_funnel.event.named_pipe.connection_type",
                )?;
            }

            let _cond = { event.get_str("json.namedPipe.isFirstInstance") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.namedPipe.isFirstInstance") {
                        if let Some(val) = event.get("json.namedPipe.isFirstInstance") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.namedPipe.isFirstInstance".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.named_pipe.is_first_instance",
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
                        "convert_json_namedPipe_isFirstInstance",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.namedPipe.isOverlapped") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.namedPipe.isOverlapped") {
                        if let Some(val) = event.get("json.namedPipe.isOverlapped") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.namedPipe.isOverlapped".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.named_pipe.is_overlapped",
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
                        "convert_json_namedPipe_isOverlapped",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.namedPipe.isWriteThrough") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.namedPipe.isWriteThrough") {
                        if let Some(val) = event.get("json.namedPipe.isWriteThrough") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.namedPipe.isWriteThrough".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.named_pipe.is_write_through",
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
                        "convert_json_namedPipe_isWriteThrough",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.namedPipe.maxInstances") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.namedPipe.maxInstances") {
                        if let Some(val) = event.get("json.namedPipe.maxInstances") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.namedPipe.maxInstances".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.named_pipe.max_instances",
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
                        "convert_json_namedPipe_maxInstances",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.namedPipe.name") {
                event.rename(
                    "json.namedPipe.name",
                    "sentinel_one_cloud_funnel.event.named_pipe.name",
                )?;
            }

            if event.has_value("json.namedPipe.readMode") {
                event.rename(
                    "json.namedPipe.readMode",
                    "sentinel_one_cloud_funnel.event.named_pipe.read_mode",
                )?;
            }

            if event.has_value("json.namedPipe.remoteClients") {
                event.rename(
                    "json.namedPipe.remoteClients",
                    "sentinel_one_cloud_funnel.event.named_pipe.remote_clients",
                )?;
            }

            if event.has_value("json.namedPipe.securityGroups") {
                event.rename(
                    "json.namedPipe.securityGroups",
                    "sentinel_one_cloud_funnel.event.named_pipe.security.groups",
                )?;
            }

            if event.has_value("json.namedPipe.securityOwner") {
                event.rename(
                    "json.namedPipe.securityOwner",
                    "sentinel_one_cloud_funnel.event.named_pipe.security.owner",
                )?;
            }

            if event.has_value("json.namedPipe.typeMode") {
                event.rename(
                    "json.namedPipe.typeMode",
                    "sentinel_one_cloud_funnel.event.named_pipe.type_mode",
                )?;
            }

            if event.has_value("json.namedPipe.waitMode") {
                event.rename(
                    "json.namedPipe.waitMode",
                    "sentinel_one_cloud_funnel.event.named_pipe.wait_mode",
                )?;
            }

            if event.has_value("json.osSrc.process.image.extension") {
                event.rename(
                    "json.osSrc.process.image.extension",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.extension",
                )?;
            }

            if event.has_value("json.osSrc.process.image.location") {
                event.rename(
                    "json.osSrc.process.image.location",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.location",
                )?;
            }

            let _cond = { event.get_str("json.osSrc.process.image.signature.isValid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.image.signature.isValid") {
                        if let Some(val) = event.get("json.osSrc.process.image.signature.isValid") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.image.signature.isValid".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.image.signature_is_valid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_image_signature_isValid",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.image.size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.image.size") {
                        if let Some(val) = event.get("json.osSrc.process.image.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.image.size".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.image.size",
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
                        "convert_json_osSrc_process_image_size",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.image.type") {
                event.rename(
                    "json.osSrc.process.image.type",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.type",
                )?;
            }

            if event.has_value("json.osSrc.process.image.uid") {
                event.rename(
                    "json.osSrc.process.image.uid",
                    "sentinel_one_cloud_funnel.event.os_src_process.image.uid",
                )?;
            }

            let _cond =
                { event.get_str("json.osSrc.process.parent.image.binaryIsExecutable") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.image.binaryIsExecutable") {
                        if let Some(val) =
                            event.get("json.osSrc.process.parent.image.binaryIsExecutable")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.image.binaryIsExecutable"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.parent.image.binary_is_executable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_parent_image_binaryIsExecutable",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.parent.image.extension") {
                event.rename(
                    "json.osSrc.process.parent.image.extension",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.extension",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.image.location") {
                event.rename(
                    "json.osSrc.process.parent.image.location",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.location",
                )?;
            }

            let _cond =
                { event.get_str("json.osSrc.process.parent.image.signature.isValid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.image.signature.isValid") {
                        if let Some(val) =
                            event.get("json.osSrc.process.parent.image.signature.isValid")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.image.signature.isValid"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.os_src_process.parent.image.signature_is_valid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_osSrc_process_parent_image_signature_isValid",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.osSrc.process.parent.image.size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.osSrc.process.parent.image.size") {
                        if let Some(val) = event.get("json.osSrc.process.parent.image.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.osSrc.process.parent.image.size".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.os_src_process.parent.image.size",
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
                        "convert_json_osSrc_process_parent_image_size",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.osSrc.process.parent.image.type") {
                event.rename(
                    "json.osSrc.process.parent.image.type",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.type",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.image.uid") {
                event.rename(
                    "json.osSrc.process.parent.image.uid",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.image.uid",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.subsystem") {
                event.rename(
                    "json.osSrc.process.parent.subsystem",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.subsystem",
                )?;
            }

            if event.has_value("json.osSrc.process.parent.userSid") {
                event.rename(
                    "json.osSrc.process.parent.userSid",
                    "sentinel_one_cloud_funnel.event.os_src_process.parent.user.sid",
                )?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.os_src_process.parent.user.sid")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.parent.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.osSrc.process.userSid") {
                event.rename(
                    "json.osSrc.process.userSid",
                    "sentinel_one_cloud_funnel.event.os_src_process.user.sid",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.os_src_process.user.sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.os_src_process.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.registry.export.path") {
                event.rename(
                    "json.registry.export.path",
                    "sentinel_one_cloud_funnel.event.registry.export_path",
                )?;
            }

            if event.has_value("json.registry.import.path") {
                event.rename(
                    "json.registry.import.path",
                    "sentinel_one_cloud_funnel.event.registry.import_path",
                )?;
            }

            if event.has_value("json.registry.owner.user") {
                event.rename(
                    "json.registry.owner.user",
                    "sentinel_one_cloud_funnel.event.registry.owner.user.name",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.registry.owner.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.registry.owner.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.registry.owner.userSid") {
                event.rename(
                    "json.registry.owner.userSid",
                    "sentinel_one_cloud_funnel.event.registry.owner.user.sid",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.registry.owner.user.sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.registry.owner.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.registry.security.info") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.registry.security.info") {
                        if let Some(val) = event.get("json.registry.security.info") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.registry.security.info".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.registry.security_info",
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
                        "convert_json_registry_security_info",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.exeModificationCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.exeModificationCount") {
                        if let Some(val) = event.get("json.src.process.exeModificationCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.exeModificationCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.exe_modification_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_exeModificationCount",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.image.description") {
                event.rename(
                    "json.src.process.image.description",
                    "sentinel_one_cloud_funnel.event.src.process.image.description",
                )?;
            }

            if event.has_value("json.src.process.image.extension") {
                event.rename(
                    "json.src.process.image.extension",
                    "sentinel_one_cloud_funnel.event.src.process.image.extension",
                )?;
            }

            if event.has_value("json.src.process.image.internalName") {
                event.rename(
                    "json.src.process.image.internalName",
                    "sentinel_one_cloud_funnel.event.src.process.image.internal_name",
                )?;
            }

            if event.has_value("json.src.process.image.location") {
                event.rename(
                    "json.src.process.image.location",
                    "sentinel_one_cloud_funnel.event.src.process.image.location",
                )?;
            }

            if event.has_value("json.src.process.image.originalFileName") {
                event.rename(
                    "json.src.process.image.originalFileName",
                    "sentinel_one_cloud_funnel.event.src.process.image.original_file_name",
                )?;
            }

            if event.has_value("json.src.process.image.productName") {
                event.rename(
                    "json.src.process.image.productName",
                    "sentinel_one_cloud_funnel.event.src.process.image.product.name",
                )?;
            }

            if event.has_value("json.src.process.image.productVersion") {
                event.rename(
                    "json.src.process.image.productVersion",
                    "sentinel_one_cloud_funnel.event.src.process.image.product.version",
                )?;
            }

            let _cond = { event.get_str("json.src.process.image.size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.image.size") {
                        if let Some(val) = event.get("json.src.process.image.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.image.size".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.image.size",
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
                        "convert_json_src_process_image_size",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.image.type") {
                event.rename(
                    "json.src.process.image.type",
                    "sentinel_one_cloud_funnel.event.src.process.image.type",
                )?;
            }

            if event.has_value("json.src.process.image.uid") {
                event.rename(
                    "json.src.process.image.uid",
                    "sentinel_one_cloud_funnel.event.src.process.image.uid",
                )?;
            }

            let _cond = { event.get_str("json.src.process.modelChildProcessCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.modelChildProcessCount") {
                        if let Some(val) = event.get("json.src.process.modelChildProcessCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.modelChildProcessCount".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.model_child_process_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_modelChildProcessCount",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.get_str("json.src.process.parent.image.binaryIsExecutable") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.image.binaryIsExecutable") {
                        if let Some(val) =
                            event.get("json.src.process.parent.image.binaryIsExecutable")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.image.binaryIsExecutable".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.parent.image.binary_is_executable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_parent_image_binaryIsExecutable",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.parent.image.extension") {
                event.rename(
                    "json.src.process.parent.image.extension",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.extension",
                )?;
            }

            if event.has_value("json.src.process.parent.image.location") {
                event.rename(
                    "json.src.process.parent.image.location",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.location",
                )?;
            }

            let _cond =
                { event.get_str("json.src.process.parent.image.signature.isValid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.image.signature.isValid") {
                        if let Some(val) =
                            event.get("json.src.process.parent.image.signature.isValid")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.image.signature.isValid".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one_cloud_funnel.event.src.process.parent.image.signature_is_valid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_process_parent_image_signature_isValid",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.process.parent.image.size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.process.parent.image.size") {
                        if let Some(val) = event.get("json.src.process.parent.image.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.process.parent.image.size".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.process.parent.image.size",
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
                        "convert_json_src_process_parent_image_size",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.src.process.parent.image.type") {
                event.rename(
                    "json.src.process.parent.image.type",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.type",
                )?;
            }

            if event.has_value("json.src.process.parent.image.uid") {
                event.rename(
                    "json.src.process.parent.image.uid",
                    "sentinel_one_cloud_funnel.event.src.process.parent.image.uid",
                )?;
            }

            if event.has_value("json.src.process.userSid") {
                event.rename(
                    "json.src.process.userSid",
                    "sentinel_one_cloud_funnel.event.src.process.user.sid",
                )?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.src.process.user.sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.process.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.task.triggerType") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.task.triggerType") {
                        if let Some(val) = event.get("json.task.triggerType") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.task.triggerType".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.task.trigger_type",
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
                        "convert_json_task_triggerType",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.tgt.file.isDirectory") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.file.isDirectory") {
                        if let Some(val) = event.get("json.tgt.file.isDirectory") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.file.isDirectory".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.file.is_directory",
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
                        "convert_json_tgt_file_isDirectory",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.tgt.file.isKernelModule") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.file.isKernelModule") {
                        if let Some(val) = event.get("json.tgt.file.isKernelModule") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.file.isKernelModule".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.file.is_kernel_module",
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
                        "convert_json_tgt_file_isKernelModule",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.tgt.file.originalFileName") {
                event.rename(
                    "json.tgt.file.originalFileName",
                    "sentinel_one_cloud_funnel.event.tgt.file.original_file_name",
                )?;
            }

            if event.has_value("json.tgt.file.owner.name") {
                event.rename(
                    "json.tgt.file.owner.name",
                    "sentinel_one_cloud_funnel.event.tgt.file.owner.name",
                )?;
            }

            if event.has_value("json.tgt.file.owner.userSid") {
                event.rename(
                    "json.tgt.file.owner.userSid",
                    "sentinel_one_cloud_funnel.event.tgt.file.owner.user_sid",
                )?;
            }

            let _cond =
                { event.has_value("sentinel_one_cloud_funnel.event.tgt.file.owner.user_sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.file.owner.user_sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tgt.file.productName") {
                event.rename(
                    "json.tgt.file.productName",
                    "sentinel_one_cloud_funnel.event.tgt.file.product.name",
                )?;
            }

            if event.has_value("json.tgt.file.productVersion") {
                event.rename(
                    "json.tgt.file.productVersion",
                    "sentinel_one_cloud_funnel.event.tgt.file.product.version",
                )?;
            }

            if event.has_value("json.tgt.file.publisher") {
                event.rename(
                    "json.tgt.file.publisher",
                    "sentinel_one_cloud_funnel.event.tgt.file.publisher",
                )?;
            }

            let _cond = { event.get_str("json.tgt.file.signature.isValid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.file.signature.isValid") {
                        if let Some(val) = event.get("json.tgt.file.signature.isValid") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.file.signature.isValid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.file.signature.is_valid",
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
                        "convert_json_tgt_file_signature_isValid",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.tgt.file.signatureInvalidReason") {
                event.rename(
                    "json.tgt.file.signatureInvalidReason",
                    "sentinel_one_cloud_funnel.event.tgt.file.signature.invalid_reason",
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.completeness.hints") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.completeness.hints") {
                        if let Some(val) = event.get("json.tgt.process.completeness.hints") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.completeness.hints".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.completeness_hints",
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
                        "convert_json_tgt_process_completeness_hints",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.tgt.process.image.extension") {
                event.rename(
                    "json.tgt.process.image.extension",
                    "sentinel_one_cloud_funnel.event.tgt.process.image.extension",
                )?;
            }

            let _cond = { event.get_str("json.tgt.process.image.size") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.tgt.process.image.size") {
                        if let Some(val) = event.get("json.tgt.process.image.size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.tgt.process.image.size".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.tgt.process.image.size",
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
                        "convert_json_tgt_process_image_size",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.tgt.process.image.uid") {
                event.rename(
                    "json.tgt.process.image.uid",
                    "sentinel_one_cloud_funnel.event.tgt.process.image.uid",
                )?;
            }

            if event.has_value("json.tgt.process.parent.image.location") {
                event.rename(
                    "json.tgt.process.parent.image.location",
                    "sentinel_one_cloud_funnel.event.tgt.process.parent.image.location",
                )?;
            }

            if event.has_value("json.tgt.process.parent.image.type") {
                event.rename(
                    "json.tgt.process.parent.image.type",
                    "sentinel_one_cloud_funnel.event.tgt.process.parent.image.type",
                )?;
            }

            if event.has_value("json.tgt.process.userSid") {
                event.rename(
                    "json.tgt.process.userSid",
                    "sentinel_one_cloud_funnel.event.tgt.process.user.sid",
                )?;
            }

            if event.has_value("json.tgt.process.uid") {
                event.rename(
                    "json.tgt.process.uid",
                    "sentinel_one_cloud_funnel.event.tgt.process.uid",
                )?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.tgt.process.user.sid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.process.user.sid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.tiIndicator.categories") {
                event.rename(
                    "json.tiIndicator.categories",
                    "sentinel_one_cloud_funnel.event.ti_indicator.categories",
                )?;
            }

            let _cond = {
                event.has_value("json.tiIndicator.creationTime")
                    && event.get_str("json.tiIndicator.creationTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.tiIndicator.creationTime") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set(
                                "sentinel_one_cloud_funnel.event.ti_indicator.creation_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.tiIndicator.creationTime".into(),
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
                        "date_json_tiIndicator_creationTime",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.tiIndicator.externalId") {
                event.rename(
                    "json.tiIndicator.externalId",
                    "sentinel_one_cloud_funnel.event.ti_indicator.external_id",
                )?;
            }

            if event.has_value("json.tiIndicator.intrusionSets") {
                event.rename(
                    "json.tiIndicator.intrusionSets",
                    "sentinel_one_cloud_funnel.event.ti_indicator.intrusion_sets",
                )?;
            }

            if event.has_value("json.tiIndicator.metadata") {
                event.rename(
                    "json.tiIndicator.metadata",
                    "sentinel_one_cloud_funnel.event.ti_indicator.metadata",
                )?;
            }

            if event.has_value("json.tiIndicator.threatActors") {
                event.rename(
                    "json.tiIndicator.threatActors",
                    "sentinel_one_cloud_funnel.event.ti_indicator.threat_actors",
                )?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("command"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-command-script"
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                    if event.has_value("json.tgt.file.sha1") {
                        event.rename(
                            "json.tgt.file.sha1",
                            "sentinel_one_cloud_funnel.event.tgt.file.sha1",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha1", v)?;
                    }
                    let _cond = { event.has_value("file.hash.sha1") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("file.hash.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.cmdScript.applicationName") {
                        event.rename(
                            "json.cmdScript.applicationName",
                            "sentinel_one_cloud_funnel.event.cmd_script.application_name",
                        )?;
                    }
                    if event.has_value("json.cmdScript.content") {
                        event.rename(
                            "json.cmdScript.content",
                            "sentinel_one_cloud_funnel.event.cmd_script.content",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.cmd_script.content")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("powershell.file.script_block_text", v)?;
                    }
                    let _cond = { event.get_str("json.cmdScript.isComplete") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.cmdScript.isComplete") {
                                if let Some(val) = event.get("json.cmdScript.isComplete") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.cmdScript.isComplete".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.cmd_script.is_complete",
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
                                "convert_json_cmdScript_isComplete",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.cmdScript.originalSize") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.cmdScript.originalSize") {
                                if let Some(val) = event.get("json.cmdScript.originalSize") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.cmdScript.originalSize".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.cmd_script.original_size",
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
                                "convert_json_cmdScript_originalSize",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.cmdScript.sha256") {
                        event.rename(
                            "json.cmdScript.sha256",
                            "sentinel_one_cloud_funnel.event.cmd_script.sha256",
                        )?;
                    }
                    let _cond =
                        { event.has_value("sentinel_one_cloud_funnel.event.cmd_script.sha256") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.cmd_script.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.src.process")
                            && event.get_str("json.src.process.crossProcessOutOfStoryline™Count")
                                != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.crossProcessOutOfStoryline™Count")
                            {
                                if let Some(val) =
                                    event.get("json.src.process.crossProcessOutOfStoryline™Count")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                path: "json.src.process.crossProcessOutOfStoryline™Count".into(),
                message,
                }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.src.process.cross_process.out_of_storyline_count", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_src_process_crossProcessOutOfStoryline™Count",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.src.process.isStoryline™Root") {
                        event.rename(
                            "json.src.process.isStoryline™Root",
                            "sentinel_one_cloud_funnel.event.src.process.is_storyline_tm_root",
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.src.process.parent")
                            && event.get_str("json.src.process.parent.isStoryline™Root") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.parent.isStoryline™Root") {
                                if let Some(val) =
                                    event.get("json.src.process.parent.isStoryline™Root")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.process.parent.isStoryline™Root"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.src.process.parent.is_storyline_tm_root", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_src_process_parent_isStoryline™Root",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.src.process.parent.Storyline™.id") {
                        event.rename(
                            "json.src.process.parent.Storyline™.id",
                            "sentinel_one_cloud_funnel.event.src.process.parent.storyline_tm_id",
                        )?;
                    }
                    if event.has_value("json.src.process.Storyline™.id") {
                        event.rename(
                            "json.src.process.Storyline™.id",
                            "sentinel_one_cloud_funnel.event.src.process.storyline_tm_id",
                        )?;
                    }
                    // End nested pipeline: "pipeline-command-script"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "pipeline-command-script",
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("cross"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-cross-process"
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                    if event.has_value("json.k8sCluster.containerImage.sha256") {
                        event.rename(
                            "json.k8sCluster.containerImage.sha256",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("container.image.hash.all", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    if event.has_value("json.tgt.process.relation") {
                        event.rename(
                            "json.tgt.process.relation",
                            "sentinel_one_cloud_funnel.event.tgt.process.relation",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.process.accessRights") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.accessRights") {
                                if let Some(val) = event.get("json.tgt.process.accessRights") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.accessRights".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.process.access_rights",
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
                                "convert_json_tgt_process_accessRights",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.activeContent.hash") {
                        event.rename(
                            "json.tgt.process.activeContent.hash",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.hash",
                        )?;
                    }
                    if event.has_value("json.tgt.process.activeContent.id") {
                        event.rename(
                            "json.tgt.process.activeContent.id",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.id",
                        )?;
                    }
                    if event.has_value("json.tgt.process.activeContent.path") {
                        event.rename(
                            "json.tgt.process.activeContent.path",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.path",
                        )?;
                    }
                    if event.has_value("json.tgt.process.activeContent.signedStatus") {
                        event.rename("json.tgt.process.activeContent.signedStatus", "sentinel_one_cloud_funnel.event.tgt.process.active_content.signed_status")?;
                    }
                    if event.has_value("json.tgt.process.activeContentType") {
                        event.rename(
                            "json.tgt.process.activeContentType",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.type",
                        )?;
                    }
                    if event.has_value("json.tgt.process.cmdline") {
                        event.rename(
                            "json.tgt.process.cmdline",
                            "sentinel_one_cloud_funnel.event.tgt.process.cmd_line",
                        )?;
                    }
                    if event.has_value("json.tgt.process.displayName") {
                        event.rename(
                            "json.tgt.process.displayName",
                            "sentinel_one_cloud_funnel.event.tgt.process.display_name",
                        )?;
                    }
                    let _cond =
                        { event.get_str("json.tgt.process.image.binaryIsExecutable") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.image.binaryIsExecutable") {
                                if let Some(val) =
                                    event.get("json.tgt.process.image.binaryIsExecutable")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.image.binaryIsExecutable"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.image.binary_is_executable", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_image_binaryIsExecutable",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.image.md5") {
                        event.rename(
                            "json.tgt.process.image.md5",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.md5",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.md5")
                    };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.image.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.image.path") {
                        event.rename(
                            "json.tgt.process.image.path",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.path",
                        )?;
                    }
                    if event.has_value("json.tgt.process.image.sha1") {
                        event.rename(
                            "json.tgt.process.image.sha1",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.sha1",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.sha1")
                    };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.image.sha256") {
                        event.rename(
                            "json.tgt.process.image.sha256",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.sha256",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.sha256")
                    };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.integrityLevel") {
                        event.rename(
                            "json.tgt.process.integrityLevel",
                            "sentinel_one_cloud_funnel.event.tgt.process.integrity_level",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.process.isNative64Bit") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.isNative64Bit") {
                                if let Some(val) = event.get("json.tgt.process.isNative64Bit") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.isNative64Bit".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.is_native_64_bit", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_isNative64Bit",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond =
                        { event.get_str("json.tgt.process.isRedirectCmdProcessor") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.isRedirectCmdProcessor") {
                                if let Some(val) =
                                    event.get("json.tgt.process.isRedirectCmdProcessor")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.isRedirectCmdProcessor"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.is_redirect_cmd_processor", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_isRedirectCmdProcessor",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.tgt.process.isStorylineRoot") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.isStorylineRoot") {
                                if let Some(val) = event.get("json.tgt.process.isStorylineRoot") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.isStorylineRoot".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.is_storyline_root", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_isStorylineRoot",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.name") {
                        event.rename(
                            "json.tgt.process.name",
                            "sentinel_one_cloud_funnel.event.tgt.process.name",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.process.pid") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.pid") {
                                if let Some(val) = event.get("json.tgt.process.pid") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.pid".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.process.pid",
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
                                "convert_json_tgt_process_pid",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.publisher") {
                        event.rename(
                            "json.tgt.process.publisher",
                            "sentinel_one_cloud_funnel.event.tgt.process.publisher",
                        )?;
                    }
                    if event.has_value("json.tgt.process.reasonSignatureInvalid") {
                        event.rename(
                            "json.tgt.process.reasonSignatureInvalid",
                            "sentinel_one_cloud_funnel.event.tgt.process.reason_signature_invalid",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.process.pid") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.sessionId") {
                                if let Some(val) = event.get("json.tgt.process.sessionId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.sessionId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.process.session_id",
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
                                "convert_json_tgt_process_sessionId",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.signedStatus") {
                        event.rename(
                            "json.tgt.process.signedStatus",
                            "sentinel_one_cloud_funnel.event.tgt.process.signed_status",
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.tgt.process.startTime")
                            && event.get_str("json.tgt.process.startTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tgt.process.startTime")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["ISO8601", "epoch_millis"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.process.start_time",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "json.tgt.process.startTime".into(),
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
                                "date_json_tgt_process_startTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.storyline.id") {
                        event.rename(
                            "json.tgt.process.storyline.id",
                            "sentinel_one_cloud_funnel.event.tgt.process.storyline_id",
                        )?;
                    }
                    if event.has_value("json.tgt.process.subsystem") {
                        event.rename(
                            "json.tgt.process.subsystem",
                            "sentinel_one_cloud_funnel.event.tgt.process.subsystem",
                        )?;
                    }
                    if event.has_value("json.tgt.process.uid") {
                        event.rename(
                            "json.tgt.process.uid",
                            "sentinel_one_cloud_funnel.event.tgt.process.uid",
                        )?;
                    }
                    if event.has_value("json.tgt.process.user") {
                        event.rename(
                            "json.tgt.process.user",
                            "sentinel_one_cloud_funnel.event.tgt.process.user.name",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                    };
                    if _cond {
                        event.append_unique(
                            "process.user.name",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                    };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.verifiedStatus") {
                        event.rename(
                            "json.tgt.process.verifiedStatus",
                            "sentinel_one_cloud_funnel.event.tgt.process.verified_status",
                        )?;
                    }
                    let _cond = { event.has_value("error.message") };
                    if _cond {
                        event.set("event.kind", json!("pipeline_error"))?;
                    }
                    let _cond = { event.has_value("error.message") };
                    if _cond {
                        event.append_unique("tags", json!("preserve_original_event"))?;
                    }
                    // End nested pipeline: "pipeline-cross-process"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-cross-process")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("dns"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-dns"
                    event.set("event.category", Value::Array(vec![json!("network")]))?;
                    event.set(
                        "event.type",
                        Value::Array(vec![json!("info"), json!("protocol")]),
                    )?;
                    event.set(
                        "event.action",
                        Value::Array(vec![json!("lookup_requested")]),
                    )?;
                    if event.has_value("json.event.dns.request") {
                        event.rename(
                            "json.event.dns.request",
                            "sentinel_one_cloud_funnel.event.dns.request",
                        )?;
                    }
                    if event.has_value("json.event.dns.response") {
                        event.rename(
                            "json.event.dns.response",
                            "sentinel_one_cloud_funnel.event.dns.response",
                        )?;
                    }
                    let _cond = {
                        (event.has_value("sentinel_one_cloud_funnel.event.dns.response")
                            && event.get_str("sentinel_one_cloud_funnel.event.dns.response")
                                != Some(""))
                            || (event.has_value("sentinel_one_cloud_funnel.event.dns.request")
                                && event.get_str("sentinel_one_cloud_funnel.event.dns.request")
                                    != Some(""))
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            // Painless script
                            // Source: def ips = new ArrayList();\ndef relatedHosts = new ArrayList();\ndef dns = new HashMap();\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.request != null && ctx.sentinel_one_cloud_funnel.event.dns.request != \"\") {\n  def request = ctx.sentinel_one_cloud_funnel.event.dns.request;\n  def question = new HashMap();\n  def parts = /\\s+/.split(request);\n\n  if (parts.length == 3) {\n    question.put(\"type\", params[parts[1]]);\n    question.put(\"name\", parts[2]);   \n    relatedHosts.add(parts[2]);\n  } else {\n    question.put(\"name\", request); \n  }\n  dns.question = question;\n}\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.response != null && ctx.sentinel_one_cloud_funnel.event.dns.response != \"\") {\n  def response = /;/.split(ctx.sentinel_one_cloud_funnel.event.dns.response);\n  def answers = new ArrayList();\n\n  for (def i = 0; i < response.length; i++) {\n    def answer = response[i];\n    if (answer == \"\") {\n      continue;\n    }\n\n    if (answer.startsWith(\"type:\")) {\n      def parts = /\\s+/.split(answer);\n      if (parts.length < 2) {\n        throw new Exception(\"unexpected event.dns.response format\");\n      }\n      if (parts.length == 3) {\n        answers.add([\n          \"type\": params[parts[1]],\n          \"data\": parts[2]\n        ]);\n        relatedHosts.add(parts[2]);\n      } else {\n        answers.add([\n          \"type\": params[parts[1]]\n        ]);\n      }\n    } else {\n      ips.add(answer);\n    }\n  }\n\n  if (answers.length > 0) {\n    dns.answers = answers;\n  }\n  if (ips.length > 0) {\n    dns.resolved_ip = ips;\n  }\n  if (relatedHosts.length > 0) {\n    if (ctx?.related == null) {\n      ctx.related = new HashMap();\n    }\n    ctx.related.hosts = relatedHosts;\n  }\n}\nif (dns != null) {\n  ctx.dns = dns;\n}
                            // TODO: Transpile Painless to Rust (2.2.3)
                            painless_exec_plan_params(
                                event,
                                cached_painless!(
                                    r#"def ips = new ArrayList();\ndef relatedHosts = new ArrayList();\ndef dns = new HashMap();\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.request != null && ctx.sentinel_one_cloud_funnel.event.dns.request != \"\") {\n  def request = ctx.sentinel_one_cloud_funnel.event.dns.request;\n  def question = new HashMap();\n  def parts = /\\s+/.split(request);\n\n  if (parts.length == 3) {\n    question.put(\"type\", params[parts[1]]);\n    question.put(\"name\", parts[2]);   \n    relatedHosts.add(parts[2]);\n  } else {\n    question.put(\"name\", request); \n  }\n  dns.question = question;\n}\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.response != null && ctx.sentinel_one_cloud_funnel.event.dns.response != \"\") {\n  def response = /;/.split(ctx.sentinel_one_cloud_funnel.event.dns.response);\n  def answers = new ArrayList();\n\n  for (def i = 0; i < response.length; i++) {\n    def answer = response[i];\n    if (answer == \"\") {\n      continue;\n    }\n\n    if (answer.startsWith(\"type:\")) {\n      def parts = /\\s+/.split(answer);\n      if (parts.length < 2) {\n        throw new Exception(\"unexpected event.dns.response format\");\n      }\n      if (parts.length == 3) {\n        answers.add([\n          \"type\": params[parts[1]],\n          \"data\": parts[2]\n        ]);\n        relatedHosts.add(parts[2]);\n      } else {\n        answers.add([\n          \"type\": params[parts[1]]\n        ]);\n      }\n    } else {\n      ips.add(answer);\n    }\n  }\n\n  if (answers.length > 0) {\n    dns.answers = answers;\n  }\n  if (ips.length > 0) {\n    dns.resolved_ip = ips;\n  }\n  if (relatedHosts.length > 0) {\n    if (ctx?.related == null) {\n      ctx.related = new HashMap();\n    }\n    ctx.related.hosts = relatedHosts;\n  }\n}\nif (dns != null) {\n  ctx.dns = dns;\n}"#
                                ),
                                cached_params!(
                                    "{\"1\":\"A\",\"2\":\"NS\",\"3\":\"MD\",\"4\":\"MF\",\"5\":\"CNAME\",\"6\":\"SOA\",\"7\":\"MB\",\"8\":\"MG\",\"9\":\"MR\",\"10\":\"NULL\",\"11\":\"WKS\",\"12\":\"PTR\",\"13\":\"HINFO\",\"14\":\"MINFO\",\"15\":\"MX\",\"16\":\"TXT\",\"17\":\"RP\",\"18\":\"AFSDB\",\"19\":\"X25\",\"20\":\"ISDN\",\"21\":\"RT\",\"22\":\"NSAP\",\"23\":\"NSAPPTR\",\"24\":\"SIG\",\"25\":\"KEY\",\"26\":\"PX\",\"27\":\"GPOS\",\"28\":\"AAAA\",\"29\":\"LOC\",\"30\":\"NXT\",\"31\":\"EID\",\"32\":\"NIMLOC\",\"33\":\"SRV\",\"34\":\"ATMA\",\"35\":\"NAPTR\",\"36\":\"KX\",\"37\":\"CERT\",\"38\":\"A6\",\"39\":\"DNAME\",\"40\":\"SINK\",\"41\":\"OPT\",\"43\":\"DS\",\"46\":\"RRSIG\",\"47\":\"NSEC\",\"48\":\"DNSKEY\",\"49\":\"DHCID\",\"100\":\"UINFO\",\"101\":\"UID\",\"102\":\"GID\",\"103\":\"UNSPEC\",\"248\":\"ADDRS\",\"249\":\"TKEY\",\"250\":\"TSIG\",\"251\":\"IXFR\",\"252\":\"AXFR\",\"253\":\"MAILB\",\"254\":\"MAILA\",\"255\":\"ANY\",\"65281\":\"WINS\",\"65282\":\"WINSR\"}"
                                ),
                            )?;
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "script")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "script_dnsFieldParsing_to_ecs",
                            )?;
                            event.append_unique("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get("dns.answers").is_some_and(|v| v.is_array()) };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "dns.answers", |event| {
                                gsub_field(
                                    event,
                                    "_ingest._value",
                                    "_ingest._value",
                                    cached_regex!(
                                        "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                                    ),
                                    "$1",
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.get("dns.resolved_ip").is_some_and(|v| v.is_array()) };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "dns.resolved_ip", |event| {
                                gsub_field(
                                    event,
                                    "_ingest._value",
                                    "_ingest._value",
                                    cached_regex!(
                                        "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                                    ),
                                    "$1",
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.get("dns.resolved_ip").is_some_and(|v| v.is_array()) };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "dns.resolved_ip", |event| {
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value") {
                                        if let Some(val) = event.get("_ingest._value") {
                                            let converted =
                                                convert_value(val, "ip").map_err(|message| {
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
                                        "convert_dns_resolved_ip_to_ip",
                                    )?;
                                    event.remove("_ingest._value");
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
                    }
                    // End nested pipeline: "pipeline-dns"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-dns")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("file"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-file"
                    event.set("event.category", Value::Array(vec![json!("file")]))?;
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("creat"))
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("creation")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("delet"))
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && (event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("change"))
                                || event
                                    .get_str("sentinel_one_cloud_funnel.event.type")
                                    .is_some_and(|s| s.to_lowercase().contains("modif"))
                                || event
                                    .get_str("sentinel_one_cloud_funnel.event.type")
                                    .is_some_and(|s| s.to_lowercase().contains("rename")))
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("change")]))?;
                    }
                    if !event.has("event.type") {
                        event.set("event.type", Value::Array(vec![json!("info")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("creat"))
                    };
                    if _cond {
                        event.set("event.action", Value::Array(vec![json!("creation")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("delet"))
                    };
                    if _cond {
                        event.set("event.action", Value::Array(vec![json!("deletion")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && (event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("change"))
                                || event
                                    .get_str("sentinel_one_cloud_funnel.event.type")
                                    .is_some_and(|s| s.to_lowercase().contains("modif")))
                    };
                    if _cond {
                        event.set("event.action", Value::Array(vec![json!("change")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("rename"))
                    };
                    if _cond {
                        event.set("event.action", Value::Array(vec![json!("rename")]))?;
                    }
                    if event.has_value("json.k8sCluster.containerId") {
                        event.rename(
                            "json.k8sCluster.containerId",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.id",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.id", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerImage.sha256") {
                        event.rename(
                            "json.k8sCluster.containerImage.sha256",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("container.image.hash.all", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    if event.has_value("json.k8sCluster.containerImage.value") {
                        event.rename(
                            "json.k8sCluster.containerImage.value",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.image.name", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerLabels") {
                        event.rename(
                            "json.k8sCluster.containerLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.labels",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.labels")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.labels", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerName") {
                        event.rename(
                            "json.k8sCluster.containerName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.name",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.name", v)?;
                    }
                    let _cond = {
                        event.has_value("json.tgt.file.creationTime")
                            && event.get_str("json.tgt.file.creationTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tgt.file.creationTime")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["ISO8601", "epoch_millis"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.file.creation_time",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "json.tgt.file.creationTime".into(),
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
                                "date_json_tgt_file_creationTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.creation_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.created", v)?;
                    }
                    if event.has_value("json.tgt.file.extension") {
                        event.rename(
                            "json.tgt.file.extension",
                            "sentinel_one_cloud_funnel.event.tgt.file.extension",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.extension")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.extension", v)?;
                    }
                    if event.has_value("json.tgt.file.md5") {
                        event.rename(
                            "json.tgt.file.md5",
                            "sentinel_one_cloud_funnel.event.tgt.file.md5",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.md5", v)?;
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
                    if event.has_value("json.tgt.file.sha1") {
                        event.rename(
                            "json.tgt.file.sha1",
                            "sentinel_one_cloud_funnel.event.tgt.file.sha1",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha1", v)?;
                    }
                    let _cond = { event.has_value("file.hash.sha1") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("file.hash.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.file.sha256") {
                        event.rename(
                            "json.tgt.file.sha256",
                            "sentinel_one_cloud_funnel.event.tgt.file.sha256",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha256", v)?;
                    }
                    let _cond = { event.has_value("file.hash.sha256") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("file.hash.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.tgt.file.modificationTime")
                            && event.get_str("json.tgt.file.modificationTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tgt.file.modificationTime")
                            {
                                match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                Some(parsed) => event.set("sentinel_one_cloud_funnel.event.tgt.file.modification_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.tgt.file.modificationTime".into(),
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
                                "date_json_tgt_file_modificationTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.modification_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.mtime", v)?;
                    }
                    if event.has_value("json.tgt.file.path") {
                        event.rename(
                            "json.tgt.file.path",
                            "sentinel_one_cloud_funnel.event.tgt.file.path",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.path", v)?;
                    }
                    if event.has_value("json.tgt.file.type") {
                        event.rename(
                            "json.tgt.file.type",
                            "sentinel_one_cloud_funnel.event.tgt.file.type",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.type")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.type", v)?;
                    }
                    let _cond = {
                        event
                            .get("sentinel_one_cloud_funnel.event.tgt.file.path")
                            .is_some_and(|v| v.is_string())
                            && event
                                .get_as_string("sentinel_one_cloud_funnel.event.tgt.file.path")
                                .is_some_and(|s| s.len() > 1)
                    };
                    if _cond {
                        // Painless script
                        // Source: def path = ctx.sentinel_one_cloud_funnel.event.tgt.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx == -1) {\n  idx = path.lastIndexOf(\"/\");\n}\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = ctx.file.name.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.type == \"file\") {\n        ctx.file.extension = ctx.file.name.substring(extIdx+1);\n    }\n}\nif (path.indexOf(':') == 1) {\n  ctx.file.drive_letter = path.substring(0, 1).toUpperCase();\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def path = ctx.sentinel_one_cloud_funnel.event.tgt.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx == -1) {\n  idx = path.lastIndexOf(\"/\");\n}\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = ctx.file.name.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.type == \"file\") {\n        ctx.file.extension = ctx.file.name.substring(extIdx+1);\n    }\n}\nif (path.indexOf(':') == 1) {\n  ctx.file.drive_letter = path.substring(0, 1).toUpperCase();\n}"#
                            ),
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.file.size") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.file.size") {
                                if let Some(val) = event.get("json.tgt.file.size") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.file.size".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.file.size",
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
                                "convert_json_tgt_file_size",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.size")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.size", v)?;
                    }
                    let _cond = { event.get_str("json.src.process.tid") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.tid") {
                                if let Some(val) = event.get("json.src.process.tid") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.process.tid".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.src.process.tid",
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
                                "convert_json_src_process_tid",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.src.process.tid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.thread.id", v)?;
                    }
                    if event.has_value("json.tgt.file.oldMd5") {
                        event.rename(
                            "json.tgt.file.oldMd5",
                            "sentinel_one_cloud_funnel.event.tgt.file.old.md5",
                        )?;
                    }
                    let _cond =
                        { event.has_value("sentinel_one_cloud_funnel.event.tgt.file.old.md5") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.file.old.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.file.oldPath") {
                        event.rename(
                            "json.tgt.file.oldPath",
                            "sentinel_one_cloud_funnel.event.tgt.file.old.path",
                        )?;
                    }
                    if event.has_value("json.tgt.file.oldSha1") {
                        event.rename(
                            "json.tgt.file.oldSha1",
                            "sentinel_one_cloud_funnel.event.tgt.file.old.sha1",
                        )?;
                    }
                    let _cond =
                        { event.has_value("sentinel_one_cloud_funnel.event.tgt.file.old.sha1") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.file.old.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.file.oldSha256") {
                        event.rename(
                            "json.tgt.file.oldSha256",
                            "sentinel_one_cloud_funnel.event.tgt.file.old.sha256",
                        )?;
                    }
                    let _cond =
                        { event.has_value("sentinel_one_cloud_funnel.event.tgt.file.old.sha256") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.file.old.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerLabels") {
                        event.rename(
                            "json.k8sCluster.controllerLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerName") {
                        event.rename(
                            "json.k8sCluster.controllerName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerType") {
                        event.rename(
                            "json.k8sCluster.controllerType",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.type",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.name") {
                        event.rename(
                            "json.k8sCluster.name",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.namespace") {
                        event.rename(
                            "json.k8sCluster.namespace",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.value",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.namespaceLabels") {
                        event.rename(
                            "json.k8sCluster.namespaceLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.nodeName") {
                        event.rename(
                            "json.k8sCluster.nodeName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.node_name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.podLabels") {
                        event.rename(
                            "json.k8sCluster.podLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.pod.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.podName") {
                        event.rename(
                            "json.k8sCluster.podName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.pod.name",
                        )?;
                    }
                    if event.has_value("json.src.process.reasonSignatureInvalid") {
                        event.rename(
                            "json.src.process.reasonSignatureInvalid",
                            "sentinel_one_cloud_funnel.event.src.process.reason_signature_invalid",
                        )?;
                    }
                    let _cond = { event.get_str("json.src.process.rpid") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.rpid") {
                                if let Some(val) = event.get("json.src.process.rpid") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.process.rpid".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.src.process.rpid",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.task.path") {
                        event.rename(
                            "json.task.path",
                            "sentinel_one_cloud_funnel.event.task.path",
                        )?;
                    }
                    if event.has_value("json.tgt.file.convictedBy") {
                        event.rename(
                            "json.tgt.file.convictedBy",
                            "sentinel_one_cloud_funnel.event.tgt.file.convicted_by",
                        )?;
                    }
                    if event.has_value("json.tgt.file.description") {
                        event.rename(
                            "json.tgt.file.description",
                            "sentinel_one_cloud_funnel.event.tgt.file.description",
                        )?;
                    }
                    if event.has_value("json.tgt.file.id") {
                        event.rename(
                            "json.tgt.file.id",
                            "sentinel_one_cloud_funnel.event.tgt.file.id",
                        )?;
                    }
                    if event.has_value("json.tgt.file.internalName") {
                        event.rename(
                            "json.tgt.file.internalName",
                            "sentinel_one_cloud_funnel.event.tgt.file.internal_name",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.file.isExecutable") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.file.isExecutable") {
                                if let Some(val) = event.get("json.tgt.file.isExecutable") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.file.isExecutable".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.file.is_executable",
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
                                "convert_json_tgt_file_isExecutable",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.file.location") {
                        event.rename(
                            "json.tgt.file.location",
                            "sentinel_one_cloud_funnel.event.tgt.file.location",
                        )?;
                    }
                    // End nested pipeline: "pipeline-file"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-file")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("indicator"))
                    && !(event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("threat")))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-indicator"
                    let _cond = { event.get_str("json.src.process.tid") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.tid") {
                                if let Some(val) = event.get("json.src.process.tid") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.process.tid".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.src.process.tid",
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
                                "convert_json_src_process_tid",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.src.process.tid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.thread.id", v)?;
                    }
                    if event.has_value("json.indicator.category") {
                        event.rename(
                            "json.indicator.category",
                            "sentinel_one_cloud_funnel.event.indicator.category",
                        )?;
                    }
                    if event.has_value("json.indicator.description") {
                        event.rename(
                            "json.indicator.description",
                            "sentinel_one_cloud_funnel.event.indicator.description",
                        )?;
                    }
                    if event.has_value("json.indicator.metadata") {
                        event.rename(
                            "json.indicator.metadata",
                            "sentinel_one_cloud_funnel.event.indicator.metadata",
                        )?;
                    }
                    if event.has_value("json.indicator.name") {
                        event.rename(
                            "json.indicator.name",
                            "sentinel_one_cloud_funnel.event.indicator.name",
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.src.process")
                            && event.get_str("json.src.process.isStoryline™Root") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.isStoryline™Root") {
                                if let Some(val) = event.get("json.src.process.isStoryline™Root")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.process.isStoryline™Root".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.src.process.is_storyline_tm_root", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_src_process_isStoryline™Root",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = {
                        event.has_value("json.src.process.parent")
                            && event.get_str("json.src.process.parent.isStoryline™Root") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.parent.isStoryline™Root") {
                                if let Some(val) =
                                    event.get("json.src.process.parent.isStoryline™Root")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.process.parent.isStoryline™Root"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.src.process.parent.is_storyline_tm_root", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "json_src_process_parent_isStoryline™Root",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.src.process.parent.Storyline™.id") {
                        event.rename(
                            "json.src.process.parent.Storyline™.id",
                            "sentinel_one_cloud_funnel.event.src.process.parent.storyline_tm_id",
                        )?;
                    }
                    if event.has_value("json.src.process.Storyline™.id") {
                        event.rename(
                            "json.src.process.Storyline™.id",
                            "sentinel_one_cloud_funnel.event.src.process.storyline_tm_id",
                        )?;
                    }
                    // End nested pipeline: "pipeline-indicator"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-indicator")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("login"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-login"
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication")]),
                    )?;
                    let _cond =
                        { event.get_str("sentinel_one_cloud_funnel.event.type") == Some("Login") };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("start")]))?;
                    }
                    let _cond =
                        { event.get_str("sentinel_one_cloud_funnel.event.type") == Some("Logout") };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("end")]))?;
                    }
                    if !event.has("event.type") {
                        event.set("event.type", Value::Array(vec![json!("info")]))?;
                    }
                    if event.has_value("json.event.login.userName") {
                        event.rename(
                            "json.event.login.userName",
                            "sentinel_one_cloud_funnel.event.login.user_name",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.login.user_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                    let _cond = { event.has_value("user.name") };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = { event.get_str("json.src.endpoint.ip.address") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.endpoint.ip.address") {
                                if let Some(val) = event.get("json.src.endpoint.ip.address") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.endpoint.ip.address".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.src.endpoint_ip_address",
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
                                "convert_json_src_endpoint_ip_address",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.src.endpoint_ip_address")
                    };
                    if _cond {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.src.endpoint_ip_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.src.endpoint_ip_address")
                    };
                    if _cond {
                        event.append_unique(
                            "host.ip",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.src.endpoint_ip_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.src.endpoint_ip_address")
                    };
                    if _cond {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.src.endpoint_ip_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.event.login.accountDomain") {
                        event.rename(
                            "json.event.login.accountDomain",
                            "sentinel_one_cloud_funnel.event.login.account.domain",
                        )?;
                    }
                    if event.has_value("json.event.login.accountName") {
                        event.rename(
                            "json.event.login.accountName",
                            "sentinel_one_cloud_funnel.event.login.account.name",
                        )?;
                    }
                    if event.has_value("json.event.login.accountSid") {
                        event.rename(
                            "json.event.login.accountSid",
                            "sentinel_one_cloud_funnel.event.login.account.sid",
                        )?;
                    }
                    if event.has_value("json.event.login.baseType") {
                        event.rename(
                            "json.event.login.baseType",
                            "sentinel_one_cloud_funnel.event.login.base_type",
                        )?;
                    }
                    if event.has_value("json.event.login.failureReason") {
                        event.rename(
                            "json.event.login.failureReason",
                            "sentinel_one_cloud_funnel.event.login.failure_reason",
                        )?;
                    }
                    let _cond =
                        { event.get_str("json.event.login.isAdministratorEquivalent") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.event.login.isAdministratorEquivalent") {
                                if let Some(val) =
                                    event.get("json.event.login.isAdministratorEquivalent")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.event.login.isAdministratorEquivalent"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.login.is_administrator_equivalent", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_event_login_isAdministratorEquivalent",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.event.login.loginIsSuccessful") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.event.login.loginIsSuccessful") {
                                if let Some(val) = event.get("json.event.login.loginIsSuccessful") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.event.login.loginIsSuccessful".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.login.is_successful",
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
                                "convert_json_event_login_loginIsSuccessful",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.event.login.sessionId") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.event.login.sessionId") {
                                if let Some(val) = event.get("json.event.login.sessionId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.event.login.sessionId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.login.session_id",
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
                                "convert_json_event_login_sessionId",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.event.login.type") {
                        event.rename(
                            "json.event.login.type",
                            "sentinel_one_cloud_funnel.event.login.type",
                        )?;
                    }
                    // End nested pipeline: "pipeline-login"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-login")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("module"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-module"
                    if event.has_value("json.module.md5") {
                        event.rename(
                            "json.module.md5",
                            "sentinel_one_cloud_funnel.event.module.md5",
                        )?;
                    }
                    let _cond = { event.has_value("sentinel_one_cloud_funnel.event.module.md5") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.module.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.module.path") {
                        event.rename(
                            "json.module.path",
                            "sentinel_one_cloud_funnel.event.module.path",
                        )?;
                    }
                    if event.has_value("json.module.sha1") {
                        event.rename(
                            "json.module.sha1",
                            "sentinel_one_cloud_funnel.event.module.sha1",
                        )?;
                    }
                    let _cond = { event.has_value("sentinel_one_cloud_funnel.event.module.sha1") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.module.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    // End nested pipeline: "pipeline-module"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-module")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("ip"))
                    && !(event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("command")))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-network-action"
                    event.set("event.category", Value::Array(vec![json!("network")]))?;
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IPConnect")
                            || event.get_str("sentinel_one_cloud_funnel.event.type")
                                == Some("IP Connect")
                    };
                    if _cond {
                        event.set(
                            "event.type",
                            Value::Array(vec![json!("start"), json!("connection")]),
                        )?;
                    }
                    let _cond = {
                        (event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IPConnect")
                            || event.get_str("sentinel_one_cloud_funnel.event.type")
                                == Some("IP Connect"))
                            && event.get_str("json.event.network.direction") == Some("OUTGOING")
                    };
                    if _cond {
                        event.set(
                            "event.action",
                            Value::Array(vec![json!("connection_attempted")]),
                        )?;
                    }
                    let _cond = {
                        (event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IPConnect")
                            || event.get_str("sentinel_one_cloud_funnel.event.type")
                                == Some("IP Connect"))
                            && event.get_str("json.event.network.direction") == Some("INCOMING")
                    };
                    if _cond {
                        event.set(
                            "event.action",
                            Value::Array(vec![json!("connection_accepted")]),
                        )?;
                    }
                    if event.has_value("json.k8sCluster.containerId") {
                        event.rename(
                            "json.k8sCluster.containerId",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.id",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.id", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerImage.sha256") {
                        event.rename(
                            "json.k8sCluster.containerImage.sha256",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("container.image.hash.all", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    if event.has_value("json.k8sCluster.containerImage.value") {
                        event.rename(
                            "json.k8sCluster.containerImage.value",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.image.name", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerLabels") {
                        event.rename(
                            "json.k8sCluster.containerLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.labels",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.labels")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.labels", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerName") {
                        event.rename(
                            "json.k8sCluster.containerName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.name",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.name", v)?;
                    }
                    let _cond = { event.get_str("json.dst.ip.address") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.dst.ip.address") {
                                if let Some(val) = event.get("json.dst.ip.address") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.dst.ip.address".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.dst.ip_address",
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
                                "convert_json_dst_ip_address",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond =
                        { event.has_value("sentinel_one_cloud_funnel.event.dst.ip_address") };
                    if _cond {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.dst.ip_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.dst.ip_address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.dst.ip_address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.address", v)?;
                    }
                    let _cond = { event.get_str("json.dst.port.number") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.dst.port.number") {
                                if let Some(val) = event.get("json.dst.port.number") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.dst.port.number".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.dst.port_number",
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
                                "convert_json_dst_port_number",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.dst.port_number")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    if event.has_value("json.event.network.connectionStatus") {
                        event.rename(
                            "json.event.network.connectionStatus",
                            "sentinel_one_cloud_funnel.event.network.connection_status",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.network.connection_status")
                            && event
                                .get_str(
                                    "sentinel_one_cloud_funnel.event.network.connection_status",
                                )
                                .is_some_and(|s| s.to_lowercase() == "blocked")
                    };
                    if _cond {
                        event.set("event.outcome", json!("unknown"))?;
                    }
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.network.connection_status")
                            != Some("")
                            && event.get_str("event.outcome") != Some("unknown")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value(
                                "sentinel_one_cloud_funnel.event.network.connection_status",
                            ) {
                                map_strings(
                                    event,
                                    "sentinel_one_cloud_funnel.event.network.connection_status",
                                    "event.outcome",
                                    str::to_lowercase,
                                )?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "lowercase")?;
                            event.set("_ingest.on_failure_processor_tag", "lowercase_sentinel_one_cloud_funnel_event_network_connection_status")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.event.network.direction") {
                        event.rename(
                            "json.event.network.direction",
                            "sentinel_one_cloud_funnel.event.network.direction",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.network.direction")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.network.direction")
                                .is_some_and(|s| s.to_lowercase() == "incoming")
                    };
                    if _cond {
                        event.set("network.direction", json!("ingress"))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.network.direction")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.network.direction")
                                .is_some_and(|s| s.to_lowercase() == "outgoing")
                    };
                    if _cond {
                        event.set("network.direction", json!("egress"))?;
                    }
                    if event.has_value("json.event.network.protocolName") {
                        event.rename(
                            "json.event.network.protocolName",
                            "sentinel_one_cloud_funnel.event.network.protocol_name",
                        )?;
                    }
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.network.protocol_name")
                            != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event
                                .has_value("sentinel_one_cloud_funnel.event.network.protocol_name")
                            {
                                map_strings(
                                    event,
                                    "sentinel_one_cloud_funnel.event.network.protocol_name",
                                    "network.protocol",
                                    str::to_lowercase,
                                )?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "lowercase")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "lowercase_sentinel_one_cloud_funnel_event_network_protocol_name",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.src.ip.address") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.ip.address") {
                                if let Some(val) = event.get("json.src.ip.address") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.ip.address".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.src.ip_address",
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
                                "convert_json_src_ip_address",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond =
                        { event.has_value("sentinel_one_cloud_funnel.event.src.ip.address") };
                    if _cond {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.src.ip_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.src.ip_address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    let _cond = { event.get_str("json.src.port.number") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.port.number") {
                                if let Some(val) = event.get("json.src.port.number") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.port.number".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.src.port_number",
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
                                "convert_json_src_port_number",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.src.port_number")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    if event.has_value("json.k8sCluster.controllerLabels") {
                        event.rename(
                            "json.k8sCluster.controllerLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerName") {
                        event.rename(
                            "json.k8sCluster.controllerName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerType") {
                        event.rename(
                            "json.k8sCluster.controllerType",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.type",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.name") {
                        event.rename(
                            "json.k8sCluster.name",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.namespace") {
                        event.rename(
                            "json.k8sCluster.namespace",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.value",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.namespaceLabels") {
                        event.rename(
                            "json.k8sCluster.namespaceLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.nodeName") {
                        event.rename(
                            "json.k8sCluster.nodeName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.node_name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.podLabels") {
                        event.rename(
                            "json.k8sCluster.podLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.pod.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.podName") {
                        event.rename(
                            "json.k8sCluster.podName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.pod.name",
                        )?;
                    }
                    let _cond = {
                        (event.has_value("source.ip") || event.has_value("source.address"))
                            && event.has_value("source.port")
                            && (event.has_value("destination.ip")
                                || event.has_value("destination.address"))
                            && event.has_value("destination.port")
                    };
                    if _cond {
                        event.append_unique("event.type", json!("connection"))?;
                    }
                    if !event.has("event.type") {
                        event.set("event.type", Value::Array(vec![json!("info")]))?;
                    }
                    // End nested pipeline: "pipeline-network-action"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "pipeline-network-action",
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("process"))
                    && !(event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("cross")))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-process"
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                            == Some("PROCESSCREATION")
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("start")]))?;
                    }
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.type") == Some("ProcessExit")
                            || event.get_str("sentinel_one_cloud_funnel.event.type")
                                == Some("ProcessTermination")
                            || event.get_str("sentinel_one_cloud_funnel.event.type")
                                == Some("Process Exit")
                            || event.get_str("sentinel_one_cloud_funnel.event.type")
                                == Some("Process Termination")
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("end")]))?;
                    }
                    if let Some(v) = event
                        .get("event.type")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.action", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerId") {
                        event.rename(
                            "json.k8sCluster.containerId",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.id",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.id", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerImage.sha256") {
                        event.rename(
                            "json.k8sCluster.containerImage.sha256",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("container.image.hash.all", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    let _cond = {
                        event.has_value(
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                        )
                    };
                    if _cond {
                        event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
                    }
                    if event.has_value("json.k8sCluster.containerImage.value") {
                        event.rename(
                            "json.k8sCluster.containerImage.value",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.image.name", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerLabels") {
                        event.rename(
                            "json.k8sCluster.containerLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.labels",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.labels")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.labels", v)?;
                    }
                    if event.has_value("json.k8sCluster.containerName") {
                        event.rename(
                            "json.k8sCluster.containerName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.container.name",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("container.name", v)?;
                    }
                    if event.has_value("json.tgt.file.sha1") {
                        event.rename(
                            "json.tgt.file.sha1",
                            "sentinel_one_cloud_funnel.event.tgt.file.sha1",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha1", v)?;
                    }
                    let _cond = { event.has_value("file.hash.sha1") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("file.hash.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.user") {
                        event.rename(
                            "json.tgt.process.user",
                            "sentinel_one_cloud_funnel.event.tgt.process.user.name",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                    };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerLabels") {
                        event.rename(
                            "json.k8sCluster.controllerLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerName") {
                        event.rename(
                            "json.k8sCluster.controllerName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.controllerType") {
                        event.rename(
                            "json.k8sCluster.controllerType",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.controller.type",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.name") {
                        event.rename(
                            "json.k8sCluster.name",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.namespace") {
                        event.rename(
                            "json.k8sCluster.namespace",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.value",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.namespaceLabels") {
                        event.rename(
                            "json.k8sCluster.namespaceLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.nodeName") {
                        event.rename(
                            "json.k8sCluster.nodeName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.node_name",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.podLabels") {
                        event.rename(
                            "json.k8sCluster.podLabels",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.pod.labels",
                        )?;
                    }
                    if event.has_value("json.k8sCluster.podName") {
                        event.rename(
                            "json.k8sCluster.podName",
                            "sentinel_one_cloud_funnel.event.k8s_cluster.pod.name",
                        )?;
                    }
                    if event.has_value("json.src.process.reasonSignatureInvalid") {
                        event.rename(
                            "json.src.process.reasonSignatureInvalid",
                            "sentinel_one_cloud_funnel.event.src.process.reason_signature_invalid",
                        )?;
                    }
                    let _cond = { event.get_str("json.src.process.rpid") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.src.process.rpid") {
                                if let Some(val) = event.get("json.src.process.rpid") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.src.process.rpid".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.src.process.rpid",
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
                                "convert_json_src_process_rpid",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.file.convictedBy") {
                        event.rename(
                            "json.tgt.file.convictedBy",
                            "sentinel_one_cloud_funnel.event.tgt.file.convicted_by",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.process.accessRights") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.accessRights") {
                                if let Some(val) = event.get("json.tgt.process.accessRights") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.accessRights".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.process.access_rights",
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
                                "convert_json_tgt_process_accessRights",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.activeContent.hash") {
                        event.rename(
                            "json.tgt.process.activeContent.hash",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.hash",
                        )?;
                    }
                    if event.has_value("json.tgt.process.activeContent.id") {
                        event.rename(
                            "json.tgt.process.activeContent.id",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.id",
                        )?;
                    }
                    if event.has_value("json.tgt.process.activeContent.path") {
                        event.rename(
                            "json.tgt.process.activeContent.path",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.path",
                        )?;
                    }
                    if event.has_value("json.tgt.process.activeContent.signedStatus") {
                        event.rename("json.tgt.process.activeContent.signedStatus", "sentinel_one_cloud_funnel.event.tgt.process.active_content.signed_status")?;
                    }
                    if event.has_value("json.tgt.process.activeContentType") {
                        event.rename(
                            "json.tgt.process.activeContentType",
                            "sentinel_one_cloud_funnel.event.tgt.process.active_content.type",
                        )?;
                    }
                    if event.has_value("json.tgt.process.cmdline") {
                        event.rename(
                            "json.tgt.process.cmdline",
                            "sentinel_one_cloud_funnel.event.tgt.process.cmd_line",
                        )?;
                    }
                    if event.has_value("json.tgt.process.displayName") {
                        event.rename(
                            "json.tgt.process.displayName",
                            "sentinel_one_cloud_funnel.event.tgt.process.display_name",
                        )?;
                    }
                    let _cond =
                        { event.get_str("json.tgt.process.image.binaryIsExecutable") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.image.binaryIsExecutable") {
                                if let Some(val) =
                                    event.get("json.tgt.process.image.binaryIsExecutable")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.image.binaryIsExecutable"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.image.binary_is_executable", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_image_binaryIsExecutable",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.image.md5") {
                        event.rename(
                            "json.tgt.process.image.md5",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.md5",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.md5")
                    };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.image.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.image.path") {
                        event.rename(
                            "json.tgt.process.image.path",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.path",
                        )?;
                    }
                    if event.has_value("json.tgt.process.image.sha1") {
                        event.rename(
                            "json.tgt.process.image.sha1",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.sha1",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.sha1")
                    };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.image.sha256") {
                        event.rename(
                            "json.tgt.process.image.sha256",
                            "sentinel_one_cloud_funnel.event.tgt.process.image.sha256",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.image.sha256")
                    };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.integrityLevel") {
                        event.rename(
                            "json.tgt.process.integrityLevel",
                            "sentinel_one_cloud_funnel.event.tgt.process.integrity_level",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.process.isNative64Bit") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.isNative64Bit") {
                                if let Some(val) = event.get("json.tgt.process.isNative64Bit") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.isNative64Bit".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.is_native_64_bit", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_isNative64Bit",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond =
                        { event.get_str("json.tgt.process.isRedirectCmdProcessor") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.isRedirectCmdProcessor") {
                                if let Some(val) =
                                    event.get("json.tgt.process.isRedirectCmdProcessor")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.isRedirectCmdProcessor"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.is_redirect_cmd_processor", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_isRedirectCmdProcessor",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.tgt.process.isStorylineRoot") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.isStorylineRoot") {
                                if let Some(val) = event.get("json.tgt.process.isStorylineRoot") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.isStorylineRoot".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.tgt.process.is_storyline_root", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tgt_process_isStorylineRoot",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.lUserName") {
                        event.rename(
                            "json.tgt.process.lUserName",
                            "sentinel_one_cloud_funnel.event.tgt.process.l_user.name",
                        )?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.l_user.name")
                    };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.l_user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.process.name") {
                        event.rename(
                            "json.tgt.process.name",
                            "sentinel_one_cloud_funnel.event.tgt.process.name",
                        )?;
                    }
                    if event.has_value("json.tgt.process.publisher") {
                        event.rename(
                            "json.tgt.process.publisher",
                            "sentinel_one_cloud_funnel.event.tgt.process.publisher",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.publisher")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.code_signature.subject_name", v)?;
                    }
                    if event.has_value("json.tgt.process.reasonSignatureInvalid") {
                        event.rename(
                            "json.tgt.process.reasonSignatureInvalid",
                            "sentinel_one_cloud_funnel.event.tgt.process.reason_signature_invalid",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.process.sessionId") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.process.sessionId") {
                                if let Some(val) = event.get("json.tgt.process.sessionId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.process.sessionId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.process.session_id",
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
                                "convert_json_tgt_process_sessionId",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.signedStatus") {
                        event.rename(
                            "json.tgt.process.signedStatus",
                            "sentinel_one_cloud_funnel.event.tgt.process.signed_status",
                        )?;
                    }
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.tgt.process.signed_status")
                            == Some("signed")
                    };
                    if _cond {
                        event.set("process.code_signature.exists", json!(true))?;
                    }
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.tgt.process.signed_status")
                            != Some("signed")
                    };
                    if _cond {
                        event.set("process.code_signature.exists", json!(false))?;
                    }
                    let _cond = {
                        event.has_value("json.tgt.process.startTime")
                            && event.get_str("json.tgt.process.startTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tgt.process.startTime")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["ISO8601", "epoch_millis"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.process.start_time",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "json.tgt.process.startTime".into(),
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
                                "date_json_tgt_process_startTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.process.storyline.id") {
                        event.rename(
                            "json.tgt.process.storyline.id",
                            "sentinel_one_cloud_funnel.event.tgt.process.storyline_id",
                        )?;
                    }
                    if event.has_value("json.tgt.process.subsystem") {
                        event.rename(
                            "json.tgt.process.subsystem",
                            "sentinel_one_cloud_funnel.event.tgt.process.subsystem",
                        )?;
                    }
                    if event.has_value("json.tgt.process.uid") {
                        event.rename(
                            "json.tgt.process.uid",
                            "sentinel_one_cloud_funnel.event.tgt.process.uid",
                        )?;
                    }
                    if event.has_value("json.tgt.process.verifiedStatus") {
                        event.rename(
                            "json.tgt.process.verifiedStatus",
                            "sentinel_one_cloud_funnel.event.tgt.process.verified_status",
                        )?;
                    }
                    let _cond = {
                        event.get_str("sentinel_one_cloud_funnel.event.tgt.process.verified_status")
                            == Some("verified")
                    };
                    if _cond {
                        event.set("process.code_signature.trusted", json!(true))?;
                    }
                    let _cond = {
                        event.get_bool("process.code_signature.exists") == Some(true)
                            && event.get_str(
                                "sentinel_one_cloud_funnel.event.tgt.process.verified_status",
                            ) != Some("verified")
                    };
                    if _cond {
                        event.set("process.code_signature.trusted", json!(false))?;
                    }
                    event.remove("process.parent");
                    event.rename("process", "process.parent")?;
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                    };
                    if _cond {
                        event.append_unique(
                            "process.user.name",
                            json!(
                                event
                                    .get("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.name", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.image.path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.executable", v)?;
                    }
                    if event.has_value("sentinel_one_cloud_funnel.event.tgt.process.pid") {
                        if let Some(val) =
                            event.get("sentinel_one_cloud_funnel.event.tgt.process.pid")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sentinel_one_cloud_funnel.event.tgt.process.pid".into(),
                                    message,
                                }
                            })?;
                            event.set("process.pid", converted)?;
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.tid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.thread.id", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.uid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.entity_id", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.cmd_line")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.command_line", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.image.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.md5", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.sha1", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.image.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.sha256", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.r_user.uid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.real_user.id", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.r_user.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.real_user.name", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.start_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.start", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.display_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.title", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.e_user.uid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.user.id", v)?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.process.user.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.user.name", v)?;
                    }
                    // End nested pipeline: "pipeline-process"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline_process")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("registry"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-registry"
                    event.set("event.category", Value::Array(vec![json!("registry")]))?;
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("create"))
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("creation")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("delet"))
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && (event
                                .get_str("sentinel_one_cloud_funnel.event.type")
                                .is_some_and(|s| s.to_lowercase().contains("change"))
                                || event
                                    .get_str("sentinel_one_cloud_funnel.event.type")
                                    .is_some_and(|s| s.to_lowercase().contains("modif")))
                    };
                    if _cond {
                        event.set("event.type", Value::Array(vec![json!("change")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && (event
                                .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                                .is_some_and(|s| s.to_lowercase().contains("regvaluecreate"))
                                || event
                                    .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                                    .is_some_and(|s| s.to_lowercase().contains("regkeycreate")))
                    };
                    if _cond {
                        event.set("event.action", Value::Array(vec![json!("creation")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && event
                                .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                                .is_some_and(|s| s.to_lowercase().contains("regvaluemodified"))
                    };
                    if _cond {
                        event.set("event.action", Value::Array(vec![json!("modification")]))?;
                    }
                    let _cond = {
                        event.has_value("sentinel_one_cloud_funnel.event.type")
                            && (event
                                .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                                .is_some_and(|s| s.to_lowercase().contains("regvaluedelete"))
                                || event
                                    .get_str("sentinel_one_cloud_funnel.event.meta_event_name")
                                    .is_some_and(|s| s.to_lowercase().contains("regkeydelete")))
                    };
                    if _cond {
                        event.set("event.action", Value::Array(vec![json!("deletion")]))?;
                    }
                    if event.has_value("json.registry.keyPath") {
                        event.rename(
                            "json.registry.keyPath",
                            "sentinel_one_cloud_funnel.event.registry.key.path",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.registry.key.path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("registry.path", v)?;
                    }
                    let _cond = {
                        event.get("registry.path").is_some_and(|v| v.is_string())
                            && event.get_str("registry.path") != Some("")
                    };
                    if _cond {
                        // Painless script
                        // Source: def idx = ctx.registry.path.lastIndexOf('\\\\');\nif (idx >= 0) {\n  ctx.registry.key = ctx.registry.path.substring(0, idx);\n  ctx.registry.value = ctx.registry.path.substring(idx+1);\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def idx = ctx.registry.path.lastIndexOf('\\\\');\nif (idx >= 0) {\n  ctx.registry.key = ctx.registry.path.substring(0, idx);\n  ctx.registry.value = ctx.registry.path.substring(idx+1);\n}"#
                            ),
                        )?;
                    }
                    if event.has_value("json.registry.value") {
                        event.rename(
                            "json.registry.value",
                            "sentinel_one_cloud_funnel.event.registry.val",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.registry.val")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("registry.data.strings", v)?;
                    }
                    if event.has_value("json.registry.valueType") {
                        event.rename(
                            "json.registry.valueType",
                            "sentinel_one_cloud_funnel.event.registry.value.type",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.registry.value.type")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("registry.data.type", v)?;
                    }
                    if event.has_value("json.registry.keyUid") {
                        event.rename(
                            "json.registry.keyUid",
                            "sentinel_one_cloud_funnel.event.registry.key.uid",
                        )?;
                    }
                    if event.has_value("json.registry.oldValue") {
                        event.rename(
                            "json.registry.oldValue",
                            "sentinel_one_cloud_funnel.event.registry.old_value.detail",
                        )?;
                    }
                    let _cond = { event.get_str("json.registry.oldValueFullSize") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.registry.oldValueFullSize") {
                                if let Some(val) = event.get("json.registry.oldValueFullSize") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.registry.oldValueFullSize".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.registry.old_value.full_size", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_registry_oldValueFullSize",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.registry.oldValueIsComplete") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.registry.oldValueIsComplete") {
                                if let Some(val) = event.get("json.registry.oldValueIsComplete") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.registry.oldValueIsComplete".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.registry.old_value.is_complete", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_registry_oldValueIsComplete",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.registry.oldValueType") {
                        event.rename(
                            "json.registry.oldValueType",
                            "sentinel_one_cloud_funnel.event.registry.old_value.type",
                        )?;
                    }
                    let _cond = { event.get_str("json.registry.valueFullSize") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.registry.valueFullSize") {
                                if let Some(val) = event.get("json.registry.valueFullSize") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.registry.valueFullSize".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.registry.value.full_size",
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
                                "convert_json_registry_valueFullSize",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = { event.get_str("json.registry.valueIsComplete") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.registry.valueIsComplete") {
                                if let Some(val) = event.get("json.registry.valueIsComplete") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.registry.valueIsComplete".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.registry.value.is_complete", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_registry_valueIsComplete",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.registry.valueType") {
                        event.rename(
                            "json.registry.valueType",
                            "sentinel_one_cloud_funnel.event.registry.value.type",
                        )?;
                    }
                    // End nested pipeline: "pipeline-registry"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-registry")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("schedule"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-scheduled-task"
                    let _cond = {
                        event.has_value("json.tgt.file.creationTime")
                            && event.get_str("json.tgt.file.creationTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tgt.file.creationTime")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["ISO8601", "epoch_millis"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.file.creation_time",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "json.tgt.file.creationTime".into(),
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
                                "date_json_tgt_file_creationTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.creation_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.created", v)?;
                    }
                    if event.has_value("json.tgt.file.extension") {
                        event.rename(
                            "json.tgt.file.extension",
                            "sentinel_one_cloud_funnel.event.tgt.file.extension",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.extension")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.extension", v)?;
                    }
                    if event.has_value("json.tgt.file.md5") {
                        event.rename(
                            "json.tgt.file.md5",
                            "sentinel_one_cloud_funnel.event.tgt.file.md5",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.md5", v)?;
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
                    if event.has_value("json.tgt.file.sha1") {
                        event.rename(
                            "json.tgt.file.sha1",
                            "sentinel_one_cloud_funnel.event.tgt.file.sha1",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha1", v)?;
                    }
                    let _cond = { event.has_value("file.hash.sha1") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("file.hash.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if event.has_value("json.tgt.file.sha256") {
                        event.rename(
                            "json.tgt.file.sha256",
                            "sentinel_one_cloud_funnel.event.tgt.file.sha256",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha256", v)?;
                    }
                    let _cond = { event.has_value("file.hash.sha256") };
                    if _cond {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("file.hash.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.tgt.file.modificationTime")
                            && event.get_str("json.tgt.file.modificationTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tgt.file.modificationTime")
                            {
                                match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                Some(parsed) => event.set("sentinel_one_cloud_funnel.event.tgt.file.modification_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.tgt.file.modificationTime".into(),
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
                                "date_json_tgt_file_modificationTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.modification_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.mtime", v)?;
                    }
                    if event.has_value("json.tgt.file.path") {
                        event.rename(
                            "json.tgt.file.path",
                            "sentinel_one_cloud_funnel.event.tgt.file.path",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.path", v)?;
                    }
                    let _cond = { event.get_str("json.tgt.file.size") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.file.size") {
                                if let Some(val) = event.get("json.tgt.file.size") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.file.size".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.file.size",
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
                                "convert_json_tgt_file_size",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.size")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.size", v)?;
                    }
                    if event.has_value("json.tgt.file.type") {
                        event.rename(
                            "json.tgt.file.type",
                            "sentinel_one_cloud_funnel.event.tgt.file.type",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.tgt.file.type")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.type", v)?;
                    }
                    if event.has_value("json.task.name") {
                        event.rename(
                            "json.task.name",
                            "sentinel_one_cloud_funnel.event.task.name",
                        )?;
                    }
                    if event.has_value("json.task.path") {
                        event.rename(
                            "json.task.path",
                            "sentinel_one_cloud_funnel.event.task.path",
                        )?;
                    }
                    if event.has_value("json.tgt.file.description") {
                        event.rename(
                            "json.tgt.file.description",
                            "sentinel_one_cloud_funnel.event.tgt.file.description",
                        )?;
                    }
                    if event.has_value("json.tgt.file.id") {
                        event.rename(
                            "json.tgt.file.id",
                            "sentinel_one_cloud_funnel.event.tgt.file.id",
                        )?;
                    }
                    if event.has_value("json.tgt.file.internalName") {
                        event.rename(
                            "json.tgt.file.internalName",
                            "sentinel_one_cloud_funnel.event.tgt.file.internal_name",
                        )?;
                    }
                    let _cond = { event.get_str("json.tgt.file.isExecutable") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tgt.file.isExecutable") {
                                if let Some(val) = event.get("json.tgt.file.isExecutable") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tgt.file.isExecutable".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "sentinel_one_cloud_funnel.event.tgt.file.is_executable",
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
                                "convert_json_tgt_file_isExecutable",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tgt.file.location") {
                        event.rename(
                            "json.tgt.file.location",
                            "sentinel_one_cloud_funnel.event.tgt.file.location",
                        )?;
                    }
                    // End nested pipeline: "pipeline-scheduled-task"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "pipeline-scheduled-task",
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("threat"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-threat-intelligence-indicator"
                    event.set("event.category", Value::Array(vec![json!("threat")]))?;
                    event.set("event.type", Value::Array(vec![json!("indicator")]))?;
                    if event.has_value("json.tiIndicator.addedBy") {
                        event.rename(
                            "json.tiIndicator.addedBy",
                            "sentinel_one_cloud_funnel.event.ti_indicator.added_by",
                        )?;
                    }
                    if event.has_value("json.tiIndicator.comparisonMethod") {
                        event.rename(
                            "json.tiIndicator.comparisonMethod",
                            "sentinel_one_cloud_funnel.event.ti_indicator.comparison_method",
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.tiIndicator.modificationTime")
                            && event.get_str("json.tiIndicator.modificationTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tiIndicator.modificationTime")
                            {
                                match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                Some(parsed) => event.set("sentinel_one_cloud_funnel.event.ti_indicator.modification_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.tiIndicator.modificationTime".into(),
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
                                "date_json_tiIndicator_modificationTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tiIndicator.name") {
                        event.rename(
                            "json.tiIndicator.name",
                            "sentinel_one_cloud_funnel.event.ti_indicator.name",
                        )?;
                    }
                    if event.has_value("json.tiindicator.originalEvent.id") {
                        event.rename(
                            "json.tiindicator.originalEvent.id",
                            "sentinel_one_cloud_funnel.event.ti_indicator.original_event.id",
                        )?;
                    }
                    let _cond =
                        { event.get_str("json.tiindicator.originalEvent.index") != Some("") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("json.tiindicator.originalEvent.index") {
                                if let Some(val) = event.get("json.tiindicator.originalEvent.index")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "json.tiindicator.originalEvent.index".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("sentinel_one_cloud_funnel.event.ti_indicator.original_event.index", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_json_tiindicator_originalEvent_index",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = {
                        event.has_value("json.tiindicator.originalEvent.time")
                            && event.get_str("json.tiindicator.originalEvent.time") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tiindicator.originalEvent.time")
                            {
                                match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                Some(parsed) => event.set("sentinel_one_cloud_funnel.event.ti_indicator.original_event.time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.tiindicator.originalEvent.time".into(),
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
                                "date_json_tiindicator_originalEvent_time",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tiindicator.originalEvent.traceId") {
                        event.rename(
                            "json.tiindicator.originalEvent.traceId",
                            "sentinel_one_cloud_funnel.event.ti_indicator.original_event.trace_id",
                        )?;
                    }
                    if event.has_value("json.tiIndicator.source") {
                        event.rename(
                            "json.tiIndicator.source",
                            "sentinel_one_cloud_funnel.event.ti_indicator.source",
                        )?;
                    }
                    if event.has_value("json.tiIndicator.type") {
                        event.rename(
                            "json.tiIndicator.type",
                            "sentinel_one_cloud_funnel.event.ti_indicator.type",
                        )?;
                    }
                    if event.has_value("json.tiIndicator.uid") {
                        event.rename(
                            "json.tiIndicator.uid",
                            "sentinel_one_cloud_funnel.event.ti_indicator.uid",
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.tiIndicator.uploadTime")
                            && event.get_str("json.tiIndicator.uploadTime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tiIndicator.uploadTime")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["ISO8601", "epoch_millis"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "sentinel_one_cloud_funnel.event.ti_indicator.upload_time",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "json.tiIndicator.uploadTime".into(),
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
                                "date_json_tiIndicator_uploadTime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = {
                        event.has_value("json.tiIndicator.validUntil")
                            && event.get_str("json.tiIndicator.validUntil") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.tiIndicator.validUntil")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["ISO8601", "epoch_millis"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "sentinel_one_cloud_funnel.event.ti_indicator.valid_until",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "json.tiIndicator.validUntil".into(),
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
                                "date_json_tiIndicator_validUntil",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.tiIndicator.value") {
                        event.rename(
                            "json.tiIndicator.value",
                            "sentinel_one_cloud_funnel.event.ti_indicator.value",
                        )?;
                    }
                    // End nested pipeline: "pipeline-threat-intelligence-indicator"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "pipeline-threat-intelligence-indicator",
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
                                .get("_ingest.pipeline")
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
                event.has_value("sentinel_one_cloud_funnel.event.category")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("url"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Begin nested pipeline: "pipeline-url"
                    if event.has_value("json.url.address") {
                        event.rename(
                            "json.url.address",
                            "sentinel_one_cloud_funnel.event.url.address",
                        )?;
                    }
                    if let Some(v) = event
                        .get("sentinel_one_cloud_funnel.event.url.address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.original", v)?;
                    }
                    if event.has_value("url.original") {
                        uri_parts(event, "url.original", "url", true, false)?;
                    }
                    if let Some(v) = event
                        .get("url.domain")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.address", v)?;
                    }
                    let _cond = { event.has_value("destination.address") };
                    if _cond {
                        if let Some(domain) = event.get_string("destination.address") {
                            // Public suffix list lookup for registered domain extraction.
                            // A failed lookup writes NO target field, which is what
                            // Elasticsearch does.
                            if let Some(rd) = registered_domain_lookup(&domain) {
                                event.set("destination.domain", json!(domain))?;
                                if let Some(registered) = rd.registered_domain {
                                    event
                                        .set("destination.registered_domain", json!(registered))?;
                                }
                                event.set(
                                    "destination.top_level_domain",
                                    json!(rd.top_level_domain),
                                )?;
                                if let Some(sub) = rd.subdomain {
                                    event.set("destination.subdomain", json!(sub))?;
                                }
                            }
                        }
                    }
                    if event.has_value("json.event.url.action") {
                        event.rename(
                            "json.event.url.action",
                            "sentinel_one_cloud_funnel.event.url.action",
                        )?;
                    }
                    // End nested pipeline: "pipeline-url"
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "pipeline")?;
                    event.set("_ingest.on_failure_processor_tag", "pipeline-url")?;
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
                                .get("_ingest.pipeline")
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
                (event.has_value("process.command_line")
                    && event.get_str("process.command_line") != Some(""))
                    || (event.has_value("process.parent.command_line")
                        && event.get_str("process.parent.command_line") != Some(""))
            };
            if _cond {
                // Painless script
                // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\ndef cmd = ctx.process?.command_line;\nif (cmd != null && cmd != \"\") {\n  ctx.process.args = commandLineToArgv(cmd);\n  ctx.process.args_count = ctx.process.args.length;\n}\n\ndef parentCmd = ctx.process?.parent?.command_line;\nif (parentCmd != null && parentCmd != \"\") {\n  ctx.process.parent.args = commandLineToArgv(parentCmd);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\ndef cmd = ctx.process?.command_line;\nif (cmd != null && cmd != \"\") {\n  ctx.process.args = commandLineToArgv(cmd);\n  ctx.process.args_count = ctx.process.args.length;\n}\n\ndef parentCmd = ctx.process?.parent?.command_line;\nif (parentCmd != null && parentCmd != \"\") {\n  ctx.process.parent.args = commandLineToArgv(parentCmd);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}"#
                    ),
                )?;
            }

            event.remove("json");

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
                event.remove("sentinel_one_cloud_funnel.event.dst.ip_address");
                event.remove("sentinel_one_cloud_funnel.event.dst.port_number");
                event.remove("sentinel_one_cloud_funnel.event.endpoint.name");
                event.remove("sentinel_one_cloud_funnel.event.endpoint.os");
                event.remove("sentinel_one_cloud_funnel.event.endpoint.type");
                event.remove("sentinel_one_cloud_funnel.event.group.id");
                event.remove("sentinel_one_cloud_funnel.event.id");
                event.remove("sentinel_one_cloud_funnel.event.k8s_cluster.container.id");
                event.remove("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256");
                event.remove("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value");
                event.remove("sentinel_one_cloud_funnel.event.k8s_cluster.container.labels");
                event.remove("sentinel_one_cloud_funnel.event.k8s_cluster.container.name");
                event.remove("sentinel_one_cloud_funnel.event.login.tgt.domain_name");
                event.remove("sentinel_one_cloud_funnel.event.login.tgt.user.name");
                event.remove("sentinel_one_cloud_funnel.event.login.user_name");
                event.remove("sentinel_one_cloud_funnel.event.logout.tgt.domain_name");
                event.remove("sentinel_one_cloud_funnel.event.logout.tgt.user.name");
                event.remove("sentinel_one_cloud_funnel.event.network.protocol_name");
                event.remove("sentinel_one_cloud_funnel.event.os.name");
                event.remove("sentinel_one_cloud_funnel.event.os_src_process.parent.image.md5");
                event.remove("sentinel_one_cloud_funnel.event.registry.key.path");
                event.remove("sentinel_one_cloud_funnel.event.registry.val");
                event.remove("sentinel_one_cloud_funnel.event.src.endpoint.ip.address");
                event.remove("sentinel_one_cloud_funnel.event.src.ip.address");
                event.remove("sentinel_one_cloud_funnel.event.src.port.number");
                event.remove("sentinel_one_cloud_funnel.event.src.process.cmd_line");
                event.remove("sentinel_one_cloud_funnel.event.src.process.display_name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.e_user.name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.e_user.uid");
                event.remove("sentinel_one_cloud_funnel.event.src.process.image.md5");
                event.remove("sentinel_one_cloud_funnel.event.src.process.image.sha1");
                event.remove("sentinel_one_cloud_funnel.event.src.process.image.sha256");
                event.remove("sentinel_one_cloud_funnel.event.src.process.name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.cmd_line");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.display_name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.e_user.name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.e_user.uid");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.image.sha1");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.image.sha256");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.pid");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.r_user.name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.r_user.uid");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.start_time");
                event.remove("sentinel_one_cloud_funnel.event.src.process.parent.user.name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.pid");
                event.remove("sentinel_one_cloud_funnel.event.src.process.r_user.name");
                event.remove("sentinel_one_cloud_funnel.event.src.process.r_user.uid");
                event.remove("sentinel_one_cloud_funnel.event.src.process.start_time");
                event.remove("sentinel_one_cloud_funnel.event.src.process.tid");
                event.remove("sentinel_one_cloud_funnel.event.src.process.user.name");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.creation_time");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.extension");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.md5");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.modification_time");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.name");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.path");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.sha1");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.sha256");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.size");
                event.remove("sentinel_one_cloud_funnel.event.tgt.file.type");
                event.remove("sentinel_one_cloud_funnel.event.tgt.process.user.name");
                event.remove("sentinel_one_cloud_funnel.event.ti_indicator.description");
                event.remove("sentinel_one_cloud_funnel.event.ti_indicator.mitre_tactics");
                event.remove("sentinel_one_cloud_funnel.event.ti_indicator.references");
                event.remove("sentinel_one_cloud_funnel.event.time");
                event.remove("sentinel_one_cloud_funnel.event.url.address");
            }

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

            let _cond = {
                event.get("_conf.reroute").is_some_and(|v| {
                    match (
                        v,
                        event
                            .get_str("sentinel_one_cloud_funnel.event.category")
                            .map(|s| s.to_lowercase()),
                    ) {
                        (serde_json::Value::Array(a), Some(n)) => {
                            a.iter().any(|x| x.as_str() == Some(n.as_str()))
                        }
                        (serde_json::Value::String(s), Some(n)) => s.contains(n.as_str()),
                        _ => false,
                    }
                })
            };
            if _cond {
                event.set("sentinel_one_cloud_funnel.event.rerouted", json!(true))?;
            }

            event.remove("_conf");

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
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
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
