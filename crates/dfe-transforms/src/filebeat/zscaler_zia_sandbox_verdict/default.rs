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
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("zscaler_zia.sandbox_verdict", parsed)?;
                }
            }

            let _cond = {
                event.has_value("zscaler_zia.sandbox_verdict")
                    && event.get_bool("_conf.strict_fields") == Some(true)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.zscaler_zia.sandbox_verdict.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.sandbox_verdict.version == null ? 'null' : ctx.zscaler_zia.sandbox_verdict.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_params(
                        event,
                        cached_script!(
                            r#"if (ctx.zscaler_zia.sandbox_verdict.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.sandbox_verdict.version == null ? 'null' : ctx.zscaler_zia.sandbox_verdict.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}"#
                        ),
                        cached_params!(
                            "{\"data_stream\":\"sandbox_verdict\",\"expect\":{\"version\":\"v1\"},\"pkg_version\":\"4.2.0\"}"
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

            if event.has_value("zscaler_zia.sandbox_verdict.record_id") {
                if let Some(val) = event.get("zscaler_zia.sandbox_verdict.record_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zscaler_zia.sandbox_verdict.record_id".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.sandbox_verdict.record_id", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  if (s.length() == 0) return;\n  List out = new ArrayList();\n  int from = 0;\n  int n = s.length();\n  for (int i = 0; i < n; i++) {\n    if (s.charAt(i) == (char)'|') {\n      out.add(s.substring(from, i));\n      from = i + 1;\n    }\n  }\n  out.add(s.substring(from, n));\n  m.put(key, out);\n}\ndef sv = ctx.zscaler_zia?.sandbox_verdict;\nif (sv == null) return;\nif (sv.threat instanceof Map) {\n  if (sv.threat.tactic instanceof Map) {\n    splitStr(sv.threat.tactic, 'id');\n  }\n  if (sv.threat.technique instanceof Map) {\n    splitStr(sv.threat.technique, 'id');\n  }\n}\nif (sv.file instanceof Map && sv.file.hash instanceof Map) {\n  splitStr(sv.file.hash, 'children_md5');\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  if (s.length() == 0) return;\n  List out = new ArrayList();\n  int from = 0;\n  int n = s.length();\n  for (int i = 0; i < n; i++) {\n    if (s.charAt(i) == (char)'|') {\n      out.add(s.substring(from, i));\n      from = i + 1;\n    }\n  }\n  out.add(s.substring(from, n));\n  m.put(key, out);\n}\ndef sv = ctx.zscaler_zia?.sandbox_verdict;\nif (sv == null) return;\nif (sv.threat instanceof Map) {\n  if (sv.threat.tactic instanceof Map) {\n    splitStr(sv.threat.tactic, 'id');\n  }\n  if (sv.threat.technique instanceof Map) {\n    splitStr(sv.threat.technique, 'id');\n  }\n}\nif (sv.file instanceof Map && sv.file.hash instanceof Map) {\n  splitStr(sv.file.hash, 'children_md5');\n}"#
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

            let _cond = {
                event.has_value("zscaler_zia.sandbox_verdict.time")
                    && event.get_str("zscaler_zia.sandbox_verdict.time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zscaler_zia.sandbox_verdict.time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.sandbox_verdict.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.sandbox_verdict.time", parsed)?;
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
                    event.remove("zscaler_zia.sandbox_verdict.time");
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

            let _cond = {
                event.has_value("zscaler_zia.sandbox_verdict.event_time")
                    && event.get_str("zscaler_zia.sandbox_verdict.event_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.sandbox_verdict.event_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.sandbox_verdict.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.sandbox_verdict.event_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_event_time_string_to_date",
                    )?;
                    event.remove("zscaler_zia.sandbox_verdict.event_time");
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

            let _cond = {
                event.has_value("zscaler_zia.sandbox_verdict.analysis_completed_time")
                    && event.get_str("zscaler_zia.sandbox_verdict.analysis_completed_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.sandbox_verdict.analysis_completed_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.sandbox_verdict.tz"),
                            None,
                        ) {
                            event.set(
                                "zscaler_zia.sandbox_verdict.analysis_completed_time",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_analysis_completed_time_string_to_date",
                    )?;
                    event.remove("zscaler_zia.sandbox_verdict.analysis_completed_time");
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

            let _cond = {
                event.has_value("zscaler_zia.sandbox_verdict.feed_time")
                    && event.get_str("zscaler_zia.sandbox_verdict.feed_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.sandbox_verdict.feed_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.sandbox_verdict.tz"),
                            None,
                        ) {
                            event.set("zscaler_zia.sandbox_verdict.feed_time", parsed)?;
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
                    event.remove("zscaler_zia.sandbox_verdict.feed_time");
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
                .get("zscaler_zia.sandbox_verdict.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("zscaler_zia.sandbox_verdict.threat.indicator.name")
                    && event.get_str("zscaler_zia.sandbox_verdict.threat.indicator.name")
                        != Some("None")
                    && !(event
                        .get_str("zscaler_zia.sandbox_verdict.verdict")
                        .is_some_and(|s| s.to_lowercase().contains("benign")))
            };
            if _cond {
                event.append_unique("event.category", json!("malware"))?;
            }

            event.append_unique("event.type", json!("info"))?;

            event.set("event.provider", json!("Zscaler"))?;

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.tz")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.record_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            event.set("observer.vendor", json!("Zscaler"))?;

            event.set("observer.product", json!("Zscaler ZIA"))?;

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.datacenter.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.datacenter.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.datacenter.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.city_name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.datacenter.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.geo.country_iso_code", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.company.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.file.hash.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.file.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.file.extension")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.extension", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.file.ba_md5_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.original", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.threat.indicator.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.threat.tactic.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.tactic.id", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.sandbox_verdict.threat.technique.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.technique.id", v)?;
            }

            let _cond = { event.has_value("zscaler_zia.sandbox_verdict.file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("zscaler_zia.sandbox_verdict.file.hash.md5")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.sandbox_verdict.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("zscaler_zia.sandbox_verdict.file.hash.sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.sandbox_verdict.file.hash.children_md5")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("zscaler_zia.sandbox_verdict.file.hash.children_md5") {
                    if let Some(Value::Array(items)) = event
                        .get("zscaler_zia.sandbox_verdict.file.hash.children_md5")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "zscaler_zia.sandbox_verdict.file.hash.children_md5",
                            Value::Array(out),
                        )?;
                    }
                }
            }

            event.remove("zscaler_zia.sandbox_verdict.time");
            event.remove("zscaler_zia.sandbox_verdict.tz");
            event.remove("zscaler_zia.sandbox_verdict.record_id");
            event.remove("zscaler_zia.sandbox_verdict.company.name");
            event.remove("zscaler_zia.sandbox_verdict.datacenter.name");
            event.remove("zscaler_zia.sandbox_verdict.datacenter.city");
            event.remove("zscaler_zia.sandbox_verdict.datacenter.country");
            event.remove("zscaler_zia.sandbox_verdict.file.hash.md5");
            event.remove("zscaler_zia.sandbox_verdict.file.hash.sha256");
            event.remove("zscaler_zia.sandbox_verdict.file.extension");
            event.remove("zscaler_zia.sandbox_verdict.file.ba_md5_url");
            event.remove("zscaler_zia.sandbox_verdict.threat.indicator.name");
            event.remove("zscaler_zia.sandbox_verdict.threat.tactic.id");
            event.remove("zscaler_zia.sandbox_verdict.threat.technique.id");
            event.remove("_conf");

            // Painless script
            // Source: boolean dropScalar(Object v) {\n  return v == null || v == '' || v == 'None' || v == 'Null';\n}\nvoid handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean dropScalar(Object v) {\n  return v == null || v == '' || v == 'None' || v == 'Null';\n}\nvoid handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nhandleMap(ctx);"#
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
