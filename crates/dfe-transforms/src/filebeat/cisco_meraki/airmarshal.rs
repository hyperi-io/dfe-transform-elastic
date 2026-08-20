// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `airmarshal` pipeline.
pub struct Airmarshal;

impl Transform for Airmarshal {
    fn name(&self) -> &str {
        "airmarshal"
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
                        let Some(pos) = remaining.find(" airmarshal_events ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" airmarshal_events ") else {
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
                    let Some(pos) = remaining.find(" airmarshal_events ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" airmarshal_events ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find("=") else {
                        break 'dissect false;
                    };
                    captured.push(("type", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("=") else {
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
                // Grok pattern: %{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}
                if !cached_grok!(
                    "%{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}"
                )
                .extract_into(&input, event)?
                {}
            }

            if let Some(input) = event.get_string("_temp.ssid") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(rest) = remaining.strip_prefix("'") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find("'") else {
                        break 'dissect false;
                    };
                    captured.push(("_temp.kv.ssid", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("'") else {
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
                        path: "_temp.ssid".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }

            if let Some(kv_str) = event.get_string("_temp.kvline") {
                for pair in kv_str.split(" ") {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "_temp.kvline".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        let value = match (value.chars().next(), value.chars().last()) {
                            (Some('('), Some(')'))
                            | (Some('['), Some(']'))
                            | (Some('<'), Some('>'))
                            | (Some('"'), Some('"'))
                            | (Some('\''), Some('\''))
                                if value.chars().count() > 1 =>
                            {
                                &value[1..value.len() - 1]
                            }
                            _ => value,
                        };
                        if !key.is_empty() {
                            event.set(&format!("_temp.kv.{}", key), value)?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("_temp.kv.ssid") };
            if _cond {
                event.rename("_temp.kv.ssid", "network.name")?;
            }

            event.rename("_temp.kv.bssid", "cisco_meraki.bssid")?;

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("ssid_spoofing_detected") };
            if _cond {
                event.rename("_temp.kv.vap", "cisco_meraki.vap")?;
            }

            if let Some(s) = event.get_string("_temp.kv.src") {
                let re = cached_regex!("[-:.]");
                let replaced = re.replace_all(&s, "-").into_owned();
                event.set("source.mac", replaced)?;
            }

            if let Some(s) = event.get_string("_temp.kv.dst") {
                let re = cached_regex!("[-:.]");
                let replaced = re.replace_all(&s, "-").into_owned();
                event.set("destination.mac", replaced)?;
            }

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("rogue_ssid_detected") };
            if _cond {
                if let Some(s) = event.get_string("_temp.kv.wired_mac") {
                    let re = cached_regex!("[-:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("_temp.observer.mac", replaced)?;
                }
            }

            let _cond = { event.has_value("_temp.observer.mac") };
            if _cond {
                event.append(
                    "observer.mac",
                    json!(
                        event
                            .get("_temp.observer.mac")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("rogue_ssid_detected") };
            if _cond {
                event.rename("_temp.kv.vlan_id", "network.vlan.id")?;
            }

            event.rename("_temp.kv.channel", "cisco_meraki.channel")?;

            event.rename("_temp.kv.fc_type", "cisco_meraki.fc_type")?;

            event.rename("_temp.kv.fc_subtype", "cisco_meraki.fc_subtype")?;

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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
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
