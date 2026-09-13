// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `web` pipeline.
pub struct Web;

impl Transform for Web {
    fn name(&self) -> &str {
        "web"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("json.timestamp") != Some("-") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.source_ip") != Some("-") };
            if _cond {
                if event.has_value("json.source_ip") {
                    event.rename("json.source_ip", "source.address")?;
                }
            }

            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            let _cond = { event.get_str("json.source_port") != Some("-") };
            if _cond {
            if event.has_value("json.source_port") {
                if let Some(val) = event.get("json.source_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.source_port".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }
            }

            let _cond = { event.get_str("json.destination_ip") != Some("-") };
            if _cond {
                if event.has_value("json.destination_ip") {
                    event.rename("json.destination_ip", "destination.address")?;
                }
            }

            if event.has_value("destination.address") {
                if let Some(val) = event.get("destination.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "destination.address".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            let _cond = { event.get_str("json.destination_port") != Some("-") };
            if _cond {
            if event.has_value("json.destination_port") {
                if let Some(val) = event.get("json.destination_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.destination_port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
            }

                if event.has_value("json.method") {
                    event.rename("json.method", "http.request.method")?;
                }

            let _cond = { event.get_str("json.status_code") != Some("0") };
            if _cond {
            if event.has_value("json.status_code") {
                if let Some(val) = event.get("json.status_code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.status_code".into(),
                            message,
                        })?;
                    event.set("http.response.status_code", converted)?;
                }
            }
            }

            if event.has_value("json.user_agent") {
                if let Some(ua_str) = event.get_string("json.user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }

                if event.has_value("json.content_type") {
                    event.rename("json.content_type", "http.request.mime_type")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "json.name", "url", true, false)?;
                Ok(())
            })();

            let _cond = { event.has_value("json.domain") && event.get_str("json.domain") != Some("") && event.get_str("json.domain").is_some_and(|s| cached_regex!(r"^(?:^https?:\/\/.*$)$").is_match(s)) };
            if _cond {
                if event.has_value("json.domain") {
                    event.rename("json.domain", "http.request.referrer")?;
                }
            }

            let _cond = { !event.has_value("url.domain") && event.has_value("destination.domain") };
            if _cond {
            event.set("url.domain", json!(event.get("destination.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.size") != Some("0") };
            if _cond {
            if event.has_value("json.size") {
                if let Some(val) = event.get("json.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.size".into(),
                            message,
                        })?;
                    event.set("http.response.body.bytes", converted)?;
                }
            }
            }

            let _cond = { event.has_value("json.user_type") && event.get_str("json.user_type") != Some("-") };
            if _cond {
            if event.has_value("json.user_type") {
                if let Some(val) = event.get("json.user_type") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.user_type".into(),
                            message,
                        })?;
                    event.set("barracuda_cloudgen_firewall.log.user_type", converted)?;
                }
            }
            }

            let _cond = { event.has_value("json.user") && event.get_str("json.user") != Some("-") && event.get_i64("barracuda_cloudgen_firewall.log.user_type") == Some(1) };
            if _cond {
                if event.has_value("json.user") {
                    event.rename("json.user", "user.name")?;
                }
            }

            let _cond = { event.has_value("json.traffic_type") && event.get_str("json.traffic_type") != Some("-") };
            if _cond {
                if event.has_value("json.traffic_type") {
                    event.rename("json.traffic_type", "barracuda_cloudgen_firewall.log.traffic_type")?;
                }
            }

                if event.has_value("json.fw_rule") {
                    event.rename("json.fw_rule", "rule.name")?;
                }

                if event.has_value("json.app_rule") {
                    event.rename("json.app_rule", "barracuda_cloudgen_firewall.log.app_rule")?;
                }

            if event.has_value("json.action") {
                if let Some(val) = event.get("json.action") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.action".into(),
                            message,
                        })?;
                    event.set("event.action", converted)?;
                }
            }

            event.set("event.kind", json!("event"))?;

                event.append_unique("event.category", json!("network"))?;

            let _cond = { event.get_str("event.action") == Some("1") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("event.action") == Some("0") };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
