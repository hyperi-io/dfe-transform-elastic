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
            event.set("event.module", json!("checkpoint_harmony_endpoint"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("alert"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.category", Value::Array(vec![json!("malware")]))?;

            event.set(
                "event.dataset",
                json!("checkpoint_harmony_endpoint.urlfiltering"),
            )?;

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

            parse_json_field(event, "event.original", "json")?;

            let _cond = { event.get_str("json.event.reason") == Some("polling") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            if event.has_value("json.action") {
                event.rename("json.action", "event.action")?;
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

            let _cond = {
                event
                    .get("json.src_user_name")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.src_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("json.src_user_name")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.src_user_name", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("json.process_username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.process_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json.src") && event.get_str("json.src") != Some("0.0.0.0") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.src")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json.dst") && event.get_str("json.dst") != Some("0.0.0.0") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.dst")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.src_machine_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("json.src_machine_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.file_md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("json.file_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.file_sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("json.file_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(date_str) = event.get_as_string("json.time") {
                match parse_date_out(&date_str, &["strict_date_optional_time_nanos"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("json.host_type") {
                event.rename("json.host_type", "host.type")?;
            }

            let _cond = { event.has_value("json.src") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("json.src")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src_machine_name") {
                event.rename_over("json.src_machine_name", "host.hostname")?;
            }

            if let Some(v) = event
                .get("host.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("json.os_name") };
            if _cond {
                event.rename_over("json.os_name", "host.os.name")?;
            }

            let _cond = { event.has_value("json.os_version") };
            if _cond {
                event.rename_over("json.os_version", "host.os.version")?;
            }

            let _cond = { event.has_value("host.os.name") && event.has_value("host.os.version") };
            if _cond {
                // Painless script
                // Source: if (ctx.host.os.name instanceof List && ctx.host.os.name.size() == 1)\n    ctx.host.os.name = ctx.host.os.name[0];\nif (ctx.host.os.version instanceof List && ctx.host.os.version.size() == 1)\n    ctx.host.os.version = ctx.host.os.version[0];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.host.os.name instanceof List && ctx.host.os.name.size() == 1)\n    ctx.host.os.name = ctx.host.os.name[0];\nif (ctx.host.os.version instanceof List && ctx.host.os.version.size() == 1)\n    ctx.host.os.version = ctx.host.os.version[0];"#
                    ),
                )?;
            }

            if event.has_value("json.domain") {
                event.rename("json.domain", "user.domain")?;
            }

            let _cond = { event.has_value("json.src_user_name") };
            if _cond {
                if event.has_value("json.src_user_name") {
                    event.rename("json.src_user_name", "user.name")?;
                }
            }

            if event.has_value("json.user_sid") {
                event.rename("json.user_sid", "user.id")?;
            }

            let _cond = {
                event.has_value("json.resource")
                    && event.get_str("json.protection_type").is_some_and(|s| {
                        [
                            "http emulation",
                            "url reputation",
                            "url filtering",
                            "phishing",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.rename("json.resource", "url.original")?;
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                // Painless script
                // Source: if (ctx.url.original instanceof List && ctx.url.original.size() == 1)\n    ctx.url.original = ctx.url.original[0];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.url.original instanceof List && ctx.url.original.size() == 1)\n    ctx.url.original = ctx.url.original[0];"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("url.original") {
                    if !uri_parts(event, "url.original", "url", true, false)?
                        && event
                            .get_str("url.original")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "url.original".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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

            if event.has_value("json.protection_name") {
                event.rename("json.protection_name", "rule.name")?;
            }

            if event.has_value("json") {
                event.rename("json", "checkpoint_harmony_endpoint.urlfiltering")?;
            }

            if event.has_value("checkpoint_harmony_endpoint.urlfiltering.policy_date") {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.policy_date",
                    "checkpoint_harmony_endpoint.urlfiltering.policy.date",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.urlfiltering.policy_name") {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.policy_name",
                    "checkpoint_harmony_endpoint.urlfiltering.policy.name",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.urlfiltering.policy_number") {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.policy_number",
                    "checkpoint_harmony_endpoint.urlfiltering.policy.number",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.urlfiltering.client_name") {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.client_name",
                    "checkpoint_harmony_endpoint.urlfiltering.client.name",
                )?;
            }

            let _cond =
                { event.has_value("checkpoint_harmony_endpoint.urlfiltering.client_version") };
            if _cond {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.client_version",
                    "checkpoint_harmony_endpoint.urlfiltering.client.version",
                )?;
            }

            let _cond =
                { event.has_value("checkpoint_harmony_endpoint.urlfiltering.client.version") };
            if _cond {
                // Painless script
                // Source: if (ctx.checkpoint_harmony_endpoint.urlfiltering.client.version instanceof List && ctx.checkpoint_harmony_endpoint.urlfiltering.client.version.size() == 1)\n    ctx.checkpoint_harmony_endpoint.urlfiltering.client.version = ctx.checkpoint_harmony_endpoint.urlfiltering.client.version[0];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.checkpoint_harmony_endpoint.urlfiltering.client.version instanceof List && ctx.checkpoint_harmony_endpoint.urlfiltering.client.version.size() == 1)\n    ctx.checkpoint_harmony_endpoint.urlfiltering.client.version = ctx.checkpoint_harmony_endpoint.urlfiltering.client.version[0];"#
                    ),
                )?;
            }

            let _cond =
                { event.has_value("checkpoint_harmony_endpoint.urlfiltering.web_client_type") };
            if _cond {
                // Painless script
                // Source: if (ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type instanceof List && ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type.size() == 1)\n    ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type = ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type[0];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type instanceof List && ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type.size() == 1)\n    ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type = ctx.checkpoint_harmony_endpoint.urlfiltering.web_client_type[0];"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("checkpoint_harmony_endpoint.urlfiltering.web_client_type")
                    == Some(" ")
            };
            if _cond {
                event.remove("checkpoint_harmony_endpoint.urlfiltering.web_client_type");
            }

            if event.has_value("checkpoint_harmony_endpoint.urlfiltering.app_id") {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.app_id",
                    "checkpoint_harmony_endpoint.urlfiltering.app.id",
                )?;
            }

            let _cond =
                { event.has_value("checkpoint_harmony_endpoint.urlfiltering.app_properties") };
            if _cond {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.app_properties",
                    "checkpoint_harmony_endpoint.urlfiltering.app.properties",
                )?;
            }

            let _cond =
                { event.has_value("checkpoint_harmony_endpoint.urlfiltering.app.properties") };
            if _cond {
                // Painless script
                // Source: if (ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties instanceof List && ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties.size() == 1)\n    ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties = ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties[0];
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties instanceof List && ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties.size() == 1)\n    ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties = ctx.checkpoint_harmony_endpoint.urlfiltering.app.properties[0];"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("checkpoint_harmony_endpoint.urlfiltering.app.properties")
                    == Some(" ")
            };
            if _cond {
                event.remove("checkpoint_harmony_endpoint.urlfiltering.app.properties");
            }

            let _cond = {
                event.has_value("checkpoint_harmony_endpoint.urlfiltering.dst")
                    && event.get_str("checkpoint_harmony_endpoint.urlfiltering.dst")
                        != Some("0.0.0.0")
            };
            if _cond {
                if event.has_value("checkpoint_harmony_endpoint.urlfiltering.dst") {
                    event.rename(
                        "checkpoint_harmony_endpoint.urlfiltering.dst",
                        "destination.ip",
                    )?;
                }
            }

            if event.has_value("checkpoint_harmony_endpoint.urlfiltering.product") {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.product",
                    "checkpoint_harmony_endpoint.urlfiltering.product.name",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.urlfiltering.product_family") {
                event.rename(
                    "checkpoint_harmony_endpoint.urlfiltering.product_family",
                    "checkpoint_harmony_endpoint.urlfiltering.product.family",
                )?;
            }

            event.remove("json");

            event.remove("checkpoint_harmony_endpoint.urlfiltering.orig");
            event.remove("checkpoint_harmony_endpoint.urlfiltering.os_name");
            event.remove("checkpoint_harmony_endpoint.urlfiltering.os_version");
            event.remove("checkpoint_harmony_endpoint.urlfiltering.client_version");
            event.remove("checkpoint_harmony_endpoint.urlfiltering.product_family");
            event.remove("checkpoint_harmony_endpoint.urlfiltering.time");
            event.remove("checkpoint_harmony_endpoint.urlfiltering.dst");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' ||(v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["<NA>".into(), "-1".into()],
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
