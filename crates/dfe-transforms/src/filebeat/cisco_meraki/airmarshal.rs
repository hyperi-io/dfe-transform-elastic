// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `airmarshal` pipeline.
pub struct Airmarshal;

impl Transform for Airmarshal {
    fn name(&self) -> &str {
        "airmarshal"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        if let Some(input) = event.get_str("event.original").map(String::from) {
            let input = input.as_str();
            let mut remaining = input;
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

        if let Some(input) = event.get_str("event.original").map(String::from) {
            let input = input.as_str();
            // Grok pattern: %{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}
            // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
            let grok_re = regex::Regex::new(&grok_to_regex(
                "%{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}",
            ))
            .unwrap();
            if let Some(caps) = grok_re.captures(input) {
                for name in grok_re.capture_names().flatten() {
                    if let Some(m) = caps.name(name) {
                        event.set(name, m.as_str())?;
                    }
                }
            }
        }

        if let Some(input) = event.get_str("_temp.ssid").map(String::from) {
            let input = input.as_str();
            let mut remaining = input;
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

        if let Some(kv_str) = event.get_str("_temp.kvline").map(String::from) {
            let kv_str = kv_str.as_str();
            for pair in kv_str.split(" ") {
                if let Some((key, value)) = pair.split_once("=") {
                    if !key.is_empty() {
                        event.set(&format!("_temp.kv.{}", key), value)?;
                    }
                }
            }
        }

        // TODO: conditional: ctx?._temp?.kv?.ssid != null
        {
            event.rename("_temp.kv.ssid", "network.name")?;
        }

        event.rename("_temp.kv.bssid", "cisco_meraki.bssid")?;

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'ssid_spoofing_detected'
        {
            event.rename("_temp.kv.vap", "cisco_meraki.vap")?;
        }

        if let Some(s) = event.get_str("_temp.kv.src").map(String::from) {
            let s = s.as_str();
            let re = regex::Regex::new("[-:.]").unwrap();
            let replaced = re.replace_all(s, "-").into_owned();
            event.set("source.mac", replaced)?;
        }

        if let Some(s) = event.get_str("_temp.kv.dst").map(String::from) {
            let s = s.as_str();
            let re = regex::Regex::new("[-:.]").unwrap();
            let replaced = re.replace_all(s, "-").into_owned();
            event.set("destination.mac", replaced)?;
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'rogue_ssid_detected'
        {
            if let Some(s) = event.get_str("_temp.kv.wired_mac").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[-:.]").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("_temp.observer.mac", replaced)?;
            }
        }

        // TODO: conditional: ctx?._temp?.observer?.mac != null
        {
            event.append(
                "observer.mac",
                event
                    .get("_temp.observer.mac")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'rogue_ssid_detected'
        {
            event.rename("_temp.kv.vlan_id", "network.vlan.id")?;
        }

        event.rename("_temp.kv.channel", "cisco_meraki.channel")?;

        event.rename("_temp.kv.fc_type", "cisco_meraki.fc_type")?;

        event.rename("_temp.kv.fc_subtype", "cisco_meraki.fc_subtype")?;

        Ok(TransformResult::Continue)
    }
}
