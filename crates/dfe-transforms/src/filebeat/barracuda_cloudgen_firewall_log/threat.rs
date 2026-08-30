// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `threat` pipeline.
pub struct Threat;

impl Transform for Threat {
    fn name(&self) -> &str {
        "threat"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("") && event.get_str("json.timestamp") != Some("-") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
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

            let _cond = { !event.has_value("_tmp.timestamp") };
            if _cond {
            event.set("_tmp.syslog_timestamp", json!(format!("{} {}", event.get("json.date").map_or_else(String::new, template_to_string), event.get("json.time").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { !event.has_value("_tmp.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.syslog_timestamp") {
                    match parse_date_out(&date_str, &["yyyy MM dd HH:mm:ss"], event.get_str("json.timezone") , None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.syslog_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.src_ip") != Some("-") };
            if _cond {
                if event.has_value("json.src_ip") {
                    event.rename("json.src_ip", "source.address")?;
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

            let _cond = { event.get_str("json.dst_ip") != Some("-") };
            if _cond {
                if event.has_value("json.dst_ip") {
                    event.rename("json.dst_ip", "destination.address")?;
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

            let _cond = { event.get_str("json.port") != Some("-") };
            if _cond {
            if event.has_value("json.port") {
                if let Some(val) = event.get("json.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
            }

            let _cond = { event.get_str("json.severity") != Some("-") };
            if _cond {
                if event.has_value("json.severity") {
                    event.rename("json.severity", "log.level")?;
                }
            }

                if event.has_value("json.fw_rule") {
                    event.rename("json.fw_rule", "rule.name")?;
                }

            let _cond = { event.get_str("json.trans_proto") != Some("-") };
            if _cond {
                if event.has_value("json.trans_proto") {
                    event.rename("json.trans_proto", "network.transport")?;
                }
            }

            if event.has_value("network.transport") {
                map_strings(event, "network.transport", "network.transport", str::to_lowercase)?;
            }

            let _cond = { event.get_str("network.transport") == Some("tcp") };
            if _cond {
            event.set("network.iana_number", json!("6"))?;
            }

            let _cond = { event.get_str("network.transport") == Some("udp") };
            if _cond {
            event.set("network.iana_number", json!("17"))?;
            }

                if event.has_value("json.description") {
                    event.rename("json.description", "rule.description")?;
                }

            let _cond = { event.get_str("json.threat_severity") != Some("-") };
            if _cond {
            if event.has_value("json.threat_severity") {
                if let Some(val) = event.get("json.threat_severity") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.threat_severity".into(),
                            message,
                        })?;
                    event.set("event.severity", converted)?;
                }
            }
            }

                if event.has_value("json.ips_category") {
                    event.rename("json.ips_category", "rule.category")?;
                }

            let _cond = { event.get_str("json.type") != Some("-") };
            if _cond {
                if event.has_value("json.type") {
                    event.rename("json.type", "rule.ruleset")?;
                }
            }

                if event.has_value("json.app_proto") {
                    event.rename("json.app_proto", "barracuda_cloudgen_firewall.log.app_proto")?;
                }

                if event.has_value("json.user") {
                    event.rename("json.user", "user.name")?;
                }

            let _cond = { event.get_str("json.operation") != Some("-") };
            if _cond {
            if event.has_value("json.operation") {
                map_strings(event, "json.operation", "event.action", str::to_lowercase)?;
            }
            }

            event.set("event.kind", json!("event"))?;

                event.append_unique("event.category", json!("network"))?;

            let _cond = { event.get_str("event.action") == Some("block") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("event.action") == Some("allow") };
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
