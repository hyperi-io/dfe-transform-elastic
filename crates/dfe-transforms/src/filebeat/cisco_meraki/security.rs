// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `security` pipeline.
pub struct Security;

impl Transform for Security {
    fn name(&self) -> &str {
        "security"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" security_event ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" security_event ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("message", remaining));
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

            if let Some(input) = event.get_string("event.original") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" security_event ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" security_event ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    captured.push(("type", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "event.original".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }

            event.rename("type", "cisco_meraki.event_subtype")?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^%{DATA} (security_event|ids-alerts) (%{WORD}\\s)?%{DATA:_temp.kvs}(\\smessage:\\s?%{DATA:message})?$
                let _ = cached_grok!("^%{DATA} (security_event|ids-alerts) (%{WORD}\\s)?%{DATA:_temp.kvs}(\\smessage:\\s?%{DATA:message})?$").extract_into(&input, event)?;
            }

            if let Some(kv_str) = event.get_string("_temp.kvs") {
                for pair in kv_str.split(" ") {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "_temp.kvs".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        let value = value.trim_matches(|c| " '\"".contains(c));
                        if !key.is_empty() {
                            kv_put(event, key, value)?;
                        }
                    }
                }
            }

            if event.has_value("priority") {
                event.rename("priority", "cisco_meraki.security.priority")?;
            }

            if event.has_value("signature") {
                event.rename("signature", "cisco_meraki.security.signature")?;
            }

            if event.has_value("dhost") {
                gsub_field(
                    event,
                    "dhost",
                    "cisco_meraki.security.dhost",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("direction") {
                event.rename("direction", "network.direction")?;
            }

            if event.has_value("protocol") {
                map_strings(event, "protocol", "network.protocol", str::to_lowercase)?;
            }

            if event.has_value("decision") {
                event.rename("decision", "cisco_meraki.security.decision")?;
            }

            let _cond = { event.has_value("url") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "url", "url", true, false)?;
                    Ok(())
                })();
            }

            if event.has_value("mac") {
                gsub_field(
                    event,
                    "mac",
                    "cisco_meraki.security.mac",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("name") {
                event.rename("name", "file.name")?;
            }

            if event.has_value("sha256") {
                event.rename("sha256", "file.hash.sha256")?;
            }

            if event.has_value("disposition") {
                event.rename("disposition", "cisco_meraki.disposition")?;
            }

            if event.has_value("action") {
                event.rename("action", "cisco_meraki.security.action")?;
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    != Some("security_filtering_disposition_change")
                    && event.has_value("src")
            };
            if _cond {
                if let Some(input) = event.get_string("src") {
                    // Grok pattern: ^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$
                    // Grok pattern: ^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$
                    // Grok pattern: ^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$
                    // Grok pattern: ^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$"),
                            cached_grok!("^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$"),
                            cached_grok_mapped!(
                                "^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$",
                                [("_temp_src_ip", "_temp.src_ip")]
                            ),
                            cached_grok!(
                                "^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$"
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            if event.has_value("_temp.src_ip") {
                if let Some(val) = event.get("_temp.src_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "_temp.src_ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            let _cond = { event.get_str("sport") != Some("0") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("sport") {
                        if let Some(val) = event.get("sport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "sport".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    != Some("security_filtering_disposition_change")
                    && event.has_value("dst")
            };
            if _cond {
                if let Some(input) = event.get_string("dst") {
                    // Grok pattern: ^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$
                    // Grok pattern: ^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$
                    // Grok pattern: ^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$
                    // Grok pattern: ^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$"),
                            cached_grok!("^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$"),
                            cached_grok_mapped!(
                                "^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$",
                                [("_temp_dst_ip", "_temp.dst_ip")]
                            ),
                            cached_grok!(
                                "^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$"
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_temp.dst_ip") {
                    if let Some(val) = event.get("_temp.dst_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "_temp.dst_ip".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.get_str("dport") != Some("0")
                    && event.get_str("cisco_meraki.event_subtype")
                        != Some("security_filtering_disposition_change")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("dport") {
                        if let Some(val) = event.get("dport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "dport".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
