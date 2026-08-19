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

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        if let Some(input) = event.get_string("event.original") {
            let mut remaining: &str = &input;
            if let Some(pos) = remaining.find(" airmarshal_events ") {
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" airmarshal_events ") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find("=") {
                event.set("type", &remaining[..pos])?;
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix("=") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find(" ") {
                event.set("type", &remaining[..pos])?;
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
        }

        event.rename("type", "cisco_meraki.event_subtype")?;

        if let Some(input) = event.get_string("event.original") {
            // Grok pattern: %{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}
            cached_grok!("%{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}")
                .extract_into(&input, event)?;
        }

        if let Some(input) = event.get_string("_temp.ssid") {
            let mut remaining: &str = &input;
            if let Some(rest) = remaining.strip_prefix("'") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find("'") {
                event.set("_temp.kv.ssid", &remaining[..pos])?;
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix("'") {
                remaining = rest;
            }
        }

        if let Some(kv_str) = event.get_string("_temp.kvline") {
            for pair in kv_str.split(" ") {
                if let Some((key, value)) = pair.split_once("=") {
                    if !key.is_empty() {
                        event.set(&format!("_temp.kv.{}", key), value)?;
                    }
                }
            }
        }

        let _cond = { event.has("_temp.kv.ssid") };
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
            let re = regex::Regex::new("[-:.]").unwrap();
            let replaced = re.replace_all(&s, "-").into_owned();
            event.set("source.mac", replaced)?;
        }

        if let Some(s) = event.get_string("_temp.kv.dst") {
            let re = regex::Regex::new("[-:.]").unwrap();
            let replaced = re.replace_all(&s, "-").into_owned();
            event.set("destination.mac", replaced)?;
        }

        let _cond = { event.get_str("cisco_meraki.event_subtype") == Some("rogue_ssid_detected") };
        if _cond {
            if let Some(s) = event.get_string("_temp.kv.wired_mac") {
                let re = regex::Regex::new("[-:.]").unwrap();
                let replaced = re.replace_all(&s, "-").into_owned();
                event.set("_temp.observer.mac", replaced)?;
            }
        }

        let _cond = { event.has("_temp.observer.mac") };
        if _cond {
            event.append(
                "observer.mac",
                event
                    .get("_temp.observer.mac")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let _cond = { event.get_str("cisco_meraki.event_subtype") == Some("rogue_ssid_detected") };
        if _cond {
            event.rename("_temp.kv.vlan_id", "network.vlan.id")?;
        }

        event.rename("_temp.kv.channel", "cisco_meraki.channel")?;

        event.rename("_temp.kv.fc_type", "cisco_meraki.fc_type")?;

        event.rename("_temp.kv.fc_subtype", "cisco_meraki.fc_subtype")?;

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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
