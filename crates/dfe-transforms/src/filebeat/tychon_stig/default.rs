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
            // Begin nested pipeline: "common_init"
            parse_json_field(event, "message", "tychon")?;
            // Painless script, resolved to its runners at generation time
            // Source: def keys = new ArrayList(ctx.tychon.keySet());\nfor (key in keys) {\n  if (ctx.tychon[key] == \"\" || ctx.tychon[key] == null) {\n    ctx.tychon.remove(key);\n  }\n}\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    shallow: true,
                    ..DropPolicy::none()
                },
                Some("tychon"),
            );
            dot_expand(event, "tychon", "*")?;
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
            // Painless script
            // Source: if (ctx.tychon?.script?.current_duration != null)\n{\n  ctx.tychon.script.current_duration =\n    (long) Double.parseDouble(ctx.tychon.script.current_duration.toString());\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.tychon?.script?.current_duration != null)\n{\n  ctx.tychon.script.current_duration =\n    (long) Double.parseDouble(ctx.tychon.script.current_duration.toString());\n}\n"#
                ),
            )?;
            event.set("ecs.version", json!("8.17.0"))?;
            event.set("event.module", json!("tychon"))?;
            event.set("event.kind", json!("state"))?;
            event.append("event.type", json!("info"))?;
            // End nested pipeline: "common_init"

            // Begin nested pipeline: "common_host"
            if event.has_value("tychon.host.mac") {
                gsub_field(
                    event,
                    "tychon.host.mac",
                    "tychon.host.mac",
                    cached_regex!(":"),
                    "-",
                )?;
            }
            let _cond = { event.get("tychon.host.mac").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("tychon.host.mac") {
                    if let Some(s) = event.get_string("tychon.host.mac") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("tychon.host.mac", Value::Array(parts))?;
                    }
                }
            }
            let _cond = { event.get("tychon.host.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("tychon.host.ip") {
                    if let Some(s) = event.get_string("tychon.host.ip") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("tychon.host.ip", Value::Array(parts))?;
                    }
                }
            }
            let _cond = { event.get("tychon.host.ipv4").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("tychon.host.ipv4") {
                    if let Some(s) = event.get_string("tychon.host.ipv4") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("tychon.host.ipv4", Value::Array(parts))?;
                    }
                }
            }
            let _cond = { event.get("tychon.host.ipv6").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("tychon.host.ipv6") {
                    if let Some(s) = event.get_string("tychon.host.ipv6") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("tychon.host.ipv6", Value::Array(parts))?;
                    }
                }
            }
            if event.has_value("tychon.host.uptime") {
                if let Some(val) = event.get("tychon.host.uptime") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "tychon.host.uptime".into(),
                            message,
                        }
                    })?;
                    event.set("tychon.host.uptime", converted)?;
                }
            }
            if event.has_value("tychon.host.uptime") {
                gsub_field(
                    event,
                    "tychon.host.uptime",
                    "tychon.host.uptime",
                    cached_regex!("\\.\\d+$"),
                    "",
                )?;
            }
            if event.has_value("tychon.host.uptime") {
                if let Some(val) = event.get("tychon.host.uptime") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "tychon.host.uptime".into(),
                            message,
                        }
                    })?;
                    event.set("tychon.host.uptime", converted)?;
                }
            }
            let _cond = { event.get_bool("tychon.host.cloud.hosted") != Some(true) };
            if _cond {
                event.remove("tychon.host.cloud");
            }
            let _cond = { event.get("tychon.host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "tychon.host.ip", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }
            let _cond = { event.get("tychon.host.ipv4").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "tychon.host.ipv4", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }
            let _cond = { event.get("tychon.host.ipv6").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "tychon.host.ipv6", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }
            let _cond = { event.has_value("tychon.host.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("tychon.host.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            let _cond = { event.has_value("tychon.host.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("tychon.host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            let _cond = { event.has_value("tychon.host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("tychon.host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            if let Some(v) = event
                .get("tychon.host.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.ip", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.mac")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.mac", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.os.family")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.family", v)?;
            }
            // Painless script, resolved to its runners at generation time
            // Source: def value = ctx.tychon.host?.os?.family?.toLowerCase();\nif (['linux', 'macos', 'unix', 'windows', 'ios', 'android'].contains(value)) {\n  if (ctx.host == null) {\n    ctx.host = [:];\n  }\n  if (ctx.host.os == null) {\n    ctx.host.os = [:];\n  }\n  ctx.host.os.type = value;\n}\n
            allowed_value_copy(
                event,
                &AllowedValueCopy::new(
                    "tychon.host.os.family",
                    true,
                    vec![
                        "linux".into(),
                        "macos".into(),
                        "unix".into(),
                        "windows".into(),
                        "ios".into(),
                        "android".into(),
                    ],
                    "host.os.type",
                ),
            );
            if let Some(v) = event
                .get("tychon.host.os.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.os.platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }
            if let Some(v) = event
                .get("tychon.host.uptime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.uptime", v)?;
            }
            // End nested pipeline: "common_host"

            // Begin nested pipeline: "rest"
            if event.has_value("tychon.rule.test_result") {
                event.rename("tychon.rule.test_result", "tychon.rule.result")?;
            }
            if event.has_value("tychon.rule.weight") {
                if let Some(val) = event.get("tychon.rule.weight") {
                    let converted = convert_value(val, "float").map_err(|message| {
                        TransformError::ParseError {
                            path: "tychon.rule.weight".into(),
                            message,
                        }
                    })?;
                    event.set("tychon.rule.weight", converted)?;
                }
            }
            event.set("tychon.rule.result_score", json!(0))?;
            let _cond = { event.get_str("tychon.rule.result") == Some("fail") };
            if _cond {
                event.set("tychon.rule.result_score", json!(10))?;
            }
            event.set(
                "event.category",
                Value::Array(vec![json!("vulnerability"), json!("configuration")]),
            )?;
            let _cond = { event.has_value("tychon.benchmark.hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("tychon.benchmark.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            if let Some(v) = event
                .get("tychon.rule.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }
            if let Some(v) = event
                .get("tychon.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }
            // End nested pipeline: "rest"

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
