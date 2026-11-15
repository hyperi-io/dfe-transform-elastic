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

            event.set("event.category", Value::Array(vec![json!("malware")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set(
                "event.dataset",
                json!("checkpoint_harmony_endpoint.forensics"),
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

            let _cond = {
                event.has_value("json.proxy_src_ip")
                    && event.get_str("json.proxy_src_ip") != Some("0.0.0.0")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.proxy_src_ip")
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

            let _cond =
                { event.has_value("json.src") && event.get_str("json.src") != Some("0.0.0.0") };
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "9e8e3ec533b117be2ce709d23d6a427dfcd970667e786cc0b6ca1a6155f90097",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"host.os.name","target":"host.os.name","only_if_unset":false,"only_if_single":true,"consumes_array":false},{"array":"host.os.version","target":"host.os.version","only_if_unset":false,"only_if_single":true,"consumes_array":false}]}}]}"#
                    ),
                )?;
            }

            if event.has_value("json.domain") {
                event.rename("json.domain", "user.domain")?;
            }

            if event.has_value("json.src_user_name") {
                event.rename("json.src_user_name", "user.name")?;
            }

            if event.has_value("json.user_sid") {
                event.rename("json.user_sid", "user.id")?;
            }

            let _cond = { event.has_value("json.file_type") };
            if _cond {
                if event.has_value("json.file_type") {
                    event.rename("json.file_type", "file.type")?;
                }
            }

            let _cond = { event.has_value("json.file_name") };
            if _cond {
                event.rename("json.file_name", "file.name")?;
            }

            let _cond = { event.has_value("file.name") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "5e76f0fca7e93974af2669ce25615ed26d54ef4be061b0a5a12bbf0c4c144369",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"file.name","target":"file.name","only_if_unset":false,"only_if_single":false,"consumes_array":false}]}}]}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("file.name") == Some(" ") };
            if _cond {
                event.remove("file.name");
            }

            if event.has_value("json.file_size") {
                event.rename("json.file_size", "file.size")?;
            }

            let _cond = {
                event.has_value("json.resource")
                    && !(event.get_str("json.protection_type").is_some_and(|s| {
                        [
                            "http emulation",
                            "url reputation",
                            "url filtering",
                            "phishing",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    }))
            };
            if _cond {
                event.rename("json.resource", "file.path")?;
            }

            let _cond = { event.has_value("file.path") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "8cf9f518c5b8ed9935e7da0744d25c03b8390dabe0baa6427401fc0ea41c61d4",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"file.path","target":"file.path","only_if_unset":false,"only_if_single":true,"consumes_array":false}]}}]}"#
                    ),
                )?;
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "cedd2657f48613f5eeda466f73d0cd26a4f6f09fe352b0181800b37c7da98efe",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"url.original","target":"url.original","only_if_unset":false,"only_if_single":true,"consumes_array":false}]}}]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("json.file_md5") };
            if _cond {
                if event.has_value("json.file_md5") {
                    event.rename("json.file_md5", "file.hash.md5")?;
                }
            }

            let _cond = { event.has_value("json.file_sha1") };
            if _cond {
                if event.has_value("json.file_sha1") {
                    event.rename("json.file_sha1", "file.hash.sha1")?;
                }
            }

            if event.has_value("json.protection_name") {
                event.rename("json.protection_name", "rule.name")?;
            }

            if event.has_value("json") {
                event.rename("json", "checkpoint_harmony_endpoint.forensics")?;
            }

            if event.has_value("checkpoint_harmony_endpoint.forensics.policy_date") {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.policy_date",
                    "checkpoint_harmony_endpoint.forensics.policy.date",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.forensics.policy_name") {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.policy_name",
                    "checkpoint_harmony_endpoint.forensics.policy.name",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.forensics.policy_number") {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.policy_number",
                    "checkpoint_harmony_endpoint.forensics.policy.number",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.forensics.client_name") {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.client_name",
                    "checkpoint_harmony_endpoint.forensics.client.name",
                )?;
            }

            let _cond = { event.has_value("checkpoint_harmony_endpoint.forensics.client_version") };
            if _cond {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.client_version",
                    "checkpoint_harmony_endpoint.forensics.client.version",
                )?;
            }

            let _cond = { event.has_value("checkpoint_harmony_endpoint.forensics.client.version") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "247971e779f9db71d3329a11e35d8a8bf055a3eaba323e797a7a93ff7d2ecd86",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"checkpoint_harmony_endpoint.forensics.client.version","target":"checkpoint_harmony_endpoint.forensics.client.version","only_if_unset":false,"only_if_single":true,"consumes_array":false}]}}]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint_harmony_endpoint.forensics.malware_action") };
            if _cond {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.malware_action",
                    "checkpoint_harmony_endpoint.forensics.malware.action",
                )?;
            }

            let _cond = { event.has_value("checkpoint_harmony_endpoint.forensics.malware.action") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "854f1a6976521a7f62075d20882eab403d6fb80722e04e00f219d76dd43600b3",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"checkpoint_harmony_endpoint.forensics.malware.action","target":"checkpoint_harmony_endpoint.forensics.malware.action","only_if_unset":false,"only_if_single":true,"consumes_array":false}]}}]}"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("checkpoint_harmony_endpoint.forensics.malware.action") == Some(" ")
            };
            if _cond {
                event.remove("checkpoint_harmony_endpoint.forensics.malware.action");
            }

            let _cond =
                { event.has_value("checkpoint_harmony_endpoint.forensics.web_client_type") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "151c650a43290012787a27f0c7595bfba3a4ff330b34ca0de5718edff432f080",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"checkpoint_harmony_endpoint.forensics.web_client_type","target":"checkpoint_harmony_endpoint.forensics.web_client_type","only_if_unset":false,"only_if_single":true,"consumes_array":false}]}}]}"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("checkpoint_harmony_endpoint.forensics.web_client_type") == Some(" ")
            };
            if _cond {
                event.remove("checkpoint_harmony_endpoint.forensics.web_client_type");
            }

            if event.has_value("checkpoint_harmony_endpoint.forensics.product") {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.product",
                    "checkpoint_harmony_endpoint.forensics.product.name",
                )?;
            }

            if event.has_value("checkpoint_harmony_endpoint.forensics.product_family") {
                event.rename(
                    "checkpoint_harmony_endpoint.forensics.product_family",
                    "checkpoint_harmony_endpoint.forensics.product.family",
                )?;
            }

            event.remove("json");

            event.remove("checkpoint_harmony_endpoint.forensics.client_version");
            event.remove("checkpoint_harmony_endpoint.forensics.malware_action");
            event.remove("checkpoint_harmony_endpoint.forensics.product_family");
            event.remove("checkpoint_harmony_endpoint.forensics.resource");
            event.remove("checkpoint_harmony_endpoint.forensics.orig");
            event.remove("checkpoint_harmony_endpoint.forensics.os_name");
            event.remove("checkpoint_harmony_endpoint.forensics.os_version");
            event.remove("checkpoint_harmony_endpoint.forensics.time");

            // The pipeline's script, emitted as the runtime's equivalent
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
