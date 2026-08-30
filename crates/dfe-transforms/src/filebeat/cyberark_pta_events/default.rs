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
                event.has_value("cef.extensions.deviceCustomString5")
                    && event.get_str("cef.extensions.deviceCustomString5") != Some("")
            };
            if _cond {
                event.set(
                    "event.action",
                    json!(
                        event
                            .get("cef.extensions.deviceCustomString5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("cef.extensions.deviceCustomDate1")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceCustomString2")
                    && event.get_str("cef.extensions.deviceCustomString2") != Some("")
            };
            if _cond {
                event.set(
                    "event.id",
                    json!(
                        event
                            .get("cef.extensions.deviceCustomString2")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceCustomString3")
                    && event.get_str("cef.extensions.deviceCustomString3") != Some("")
            };
            if _cond {
                event.set(
                    "event.reference",
                    json!(
                        event
                            .get("cef.extensions.deviceCustomString3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.deviceCustomString4")
                    && event.get_str("cef.extensions.deviceCustomString4") != Some("")
            };
            if _cond {
                event.set(
                    "event.url",
                    json!(
                        event
                            .get("cef.extensions.deviceCustomString4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.device.event_class_id")
                    && event.get_str("cef.device.event_class_id") != Some("")
            };
            if _cond {
                event.set(
                    "cyberark_pta.log.event_type",
                    json!(
                        event
                            .get("cef.device.event_class_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("event.original") {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: ^%{SYSLOGTIMESTAMP} (?:%{IP:observer.ip}|%{HOSTNAME:observer.hostname}) CEF
                        let _ = cached_grok!("^%{SYSLOGTIMESTAMP} (?:%{IP:observer.ip}|%{HOSTNAME:observer.hostname}) CEF").extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.get("observer.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                // Painless script
                // Source: ctx.observer.ip = Collections.singletonList(ctx.observer.ip);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.observer.ip = Collections.singletonList(ctx.observer.ip);"#
                    ),
                )?;
            }

            let _cond = { !event.has_value("event.reason") };
            if _cond {
                event.rename("message", "event.reason")?;
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

            let _cond = {
                !event.has_value("cef.extensions.sourceAddress")
                    || !event.has_value("cef.extensions.destinationAddress")
            };
            if _cond {
                if event.remove("error").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "error".into(),
                    });
                }
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event
                        .get_str("source.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("source.user.name", "source.user.email")?;
            }

            let _cond = { !event.has_value("source.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.user.email") {
                        if let Some(input) = event.get_string("source.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("source.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("source.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("source.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("source.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("destination.user.name")
                    && event
                        .get_str("destination.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("destination.user.name", "destination.user.email")?;
            }

            let _cond = { !event.has_value("destination.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("destination.user.email") {
                        if let Some(input) = event.get_string("destination.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("destination.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("destination.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("destination.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("destination.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
