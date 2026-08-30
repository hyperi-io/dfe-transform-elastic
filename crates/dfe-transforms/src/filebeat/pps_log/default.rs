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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?:\\d+ )?<%{NUMBER:log.syslog.priority:long}>%{SYSLOGTIMESTAMP:event.created}\\s+%{NOTSPACE:host.hostname}\\s%{DATA}:%{IP:client.ip}\\s-\\s%{USERNAME:user.name}@%{DATA:user.domain}\\s%{DATA}(?P<event_outcome>(Success)|(Error))\\s-\\s%{DATA:event.reason}\\s-\\s%{GREEDYDATA:message}
                // Grok pattern: ^(?:\\d+ )?<%{NUMBER:log.syslog.priority:long}>%{SYSLOGTIMESTAMP:event.created}\\s+%{NOTSPACE:host.hostname}\\s%{DATA}:%{IP:client.ip}\\s-\\s%{USERNAME:user.name}\\s%{DATA}(?P<event_outcome>(Success)|(Error))\\s-\\s%{DATA:event.reason}\\s-\\s%{GREEDYDATA:message}
                // Grok pattern: ^(?:\\d+ )?<%{NUMBER:log.syslog.priority:long}>%{SYSLOGTIMESTAMP:event.created}\\s+%{NOTSPACE:host.domain}\\s%{DATA}:%{IP:client.ip}%{DATA}(?P<event_outcome>(Success)|(Error))\\s-\\s%{DATA:event.reason}\\s-\\s%{GREEDYDATA:message}
                // Grok pattern: ^%{GREEDYDATA:message}$
                let _ = extract_first_match(
                    &[
                        cached_grok_mapped!(
                            "^(?:\\d+ )?<%{NUMBER:log.syslog.priority:long}>%{SYSLOGTIMESTAMP:event.created}\\s+%{NOTSPACE:host.hostname}\\s%{DATA}:%{IP:client.ip}\\s-\\s%{USERNAME:user.name}@%{DATA:user.domain}\\s%{DATA}(?P<event_outcome>(Success)|(Error))\\s-\\s%{DATA:event.reason}\\s-\\s%{GREEDYDATA:message}",
                            [("event_outcome", "event.outcome")]
                        ),
                        cached_grok_mapped!(
                            "^(?:\\d+ )?<%{NUMBER:log.syslog.priority:long}>%{SYSLOGTIMESTAMP:event.created}\\s+%{NOTSPACE:host.hostname}\\s%{DATA}:%{IP:client.ip}\\s-\\s%{USERNAME:user.name}\\s%{DATA}(?P<event_outcome>(Success)|(Error))\\s-\\s%{DATA:event.reason}\\s-\\s%{GREEDYDATA:message}",
                            [("event_outcome", "event.outcome")]
                        ),
                        cached_grok_mapped!(
                            "^(?:\\d+ )?<%{NUMBER:log.syslog.priority:long}>%{SYSLOGTIMESTAMP:event.created}\\s+%{NOTSPACE:host.domain}\\s%{DATA}:%{IP:client.ip}%{DATA}(?P<event_outcome>(Success)|(Error))\\s-\\s%{DATA:event.reason}\\s-\\s%{GREEDYDATA:message}",
                            [("event_outcome", "event.outcome")]
                        ),
                        cached_grok!("^%{GREEDYDATA:message}$"),
                    ],
                    &input,
                    event,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: backing\\sup\\sdatabase\\sto\\s<%{DATA:pps.entry.path}>
                        // Grok pattern: (?:created|updated)\\sentry\\s<%{DATA:pps.entry.path}>
                        // Grok pattern: fetched\\sthe\\spassword\\sfor\\s<%{DATA:pps.entry.path}>$
                        // Grok pattern: fetched\\sthe\\spassword\\sfor\\s<%{DATA:pps.entry.path}>\\s-\\s%{DATA:pps.entry.reason}$
                        // Grok pattern: on\\sentry\\s<%{DATA:pps.entry.path}>\\sfor\\suser
                        // Grok pattern: moved\\sentry\\s<%{DATA:pps.entry.path}>\\sto\\s<%{DATA:pps.entry.target.path}>
                        // Grok pattern: created\\sfolder\\s<%{DATA:pps.entry.path}>$
                        // Grok pattern: comment\\srequirement\\s<.*>\\s(?:from|to)\\s<%{DATA:pps.entry.path}>$
                        // Grok pattern: notification\\s.*>\\s(?:from|to)\\s<%{DATA:pps.entry.path}>$
                        // Grok pattern: updated\\sentry\\s<%{DATA:pps.entry.path}>\\schanging\\sthe\\sname\\sfrom\\s<%{DATA:pps.entry.name}>\\sto\\s<%{DATA:pps.entry.target.name}>
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "backing\\sup\\sdatabase\\sto\\s<%{DATA:pps.entry.path}>"
                                ),
                                cached_grok!(
                                    "(?:created|updated)\\sentry\\s<%{DATA:pps.entry.path}>"
                                ),
                                cached_grok!(
                                    "fetched\\sthe\\spassword\\sfor\\s<%{DATA:pps.entry.path}>$"
                                ),
                                cached_grok!(
                                    "fetched\\sthe\\spassword\\sfor\\s<%{DATA:pps.entry.path}>\\s-\\s%{DATA:pps.entry.reason}$"
                                ),
                                cached_grok!("on\\sentry\\s<%{DATA:pps.entry.path}>\\sfor\\suser"),
                                cached_grok!(
                                    "moved\\sentry\\s<%{DATA:pps.entry.path}>\\sto\\s<%{DATA:pps.entry.target.path}>"
                                ),
                                cached_grok!("created\\sfolder\\s<%{DATA:pps.entry.path}>$"),
                                cached_grok!(
                                    "comment\\srequirement\\s<.*>\\s(?:from|to)\\s<%{DATA:pps.entry.path}>$"
                                ),
                                cached_grok!(
                                    "notification\\s.*>\\s(?:from|to)\\s<%{DATA:pps.entry.path}>$"
                                ),
                                cached_grok!(
                                    "updated\\sentry\\s<%{DATA:pps.entry.path}>\\schanging\\sthe\\sname\\sfrom\\s<%{DATA:pps.entry.name}>\\sto\\s<%{DATA:pps.entry.target.name}>"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: \\schanging\\sthe\\susername\\sfrom\\s<%{DATA:pps.entry.username}>\\sto\\s<%{DATA:pps.entry.target.username}>
                        let _ = cached_grok!("\\schanging\\sthe\\susername\\sfrom\\s<%{DATA:pps.entry.username}>\\sto\\s<%{DATA:pps.entry.target.username}>").extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("event.outcome") };
            if _cond {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.outcome") == Some("error") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.outcome") != Some("success")
                    && event.get_str("event.outcome") != Some("failure")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_conf.tz_offset") {
                        event.rename("_conf.tz_offset", "event.timezone")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.timezone") && event.has_value("event.created") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.created") {
                        match parse_date_out(
                            &date_str,
                            &["MMM [dd][ d] HH:mm:ss"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("event.created");
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

            let _cond = { !event.has_value("event.timezone") && event.has_value("event.created") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.created") {
                        match parse_date_out(&date_str, &["MMM [dd][ d] HH:mm:ss"], None, None) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.remove("event.created");
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

            let _cond = { event.has_value("event.created") };
            if _cond {
                event.set(
                    "@timestamp",
                    json!(
                        event
                            .get("event.created")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") && event.has_value("user.domain") };
            if _cond {
                event.set(
                    "user.email",
                    json!(format!(
                        "{}@{}",
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("user.domain")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_conf");
                event.remove("_tmp");
                Ok(())
            })();

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' ||(v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' ||(v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '<NA>' || v == '-1' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
