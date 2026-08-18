// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `flows` pipeline.
pub struct Flows;

impl Transform for Flows {
    fn name(&self) -> &str {
        "flows"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        // Pattern definitions for grok
        // TYPE = flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall
        if let Some(input) = event.get_str("event.original").map(String::from) {
            let input = input.as_str();
            // Grok pattern: %{TYPE}( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?
            // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
            cached_grok!("%{TYPE}( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?").extract_into(input, event)?;
        }

        // TODO: conditional: ctx.cisco_meraki?.firewall?.pattern != null && (ctx.cisco_meraki.firewall.pattern.startsWith('allow') || ctx.cisco_meraki.firewall.pattern.startsWith('deny'))
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event
                    .get_str("cisco_meraki.firewall.pattern")
                    .map(String::from)
                {
                    let input = input.as_str();
                    // Grok pattern: %{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("%{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}").extract_into(input, event)?;
                }
                Ok(())
            })();
        }

        // TODO: conditional: ctx.cisco_meraki?.firewall?.rule != null
        {
            event.remove("cisco_meraki.firewall.pattern");
        }

        if event.has("source.mac") {
            if let Some(s) = event.get_str("source.mac").map(String::from) {
                let s = s.as_str();
                let re = cached_regex!("[:.]");
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("source.mac", replaced)?;
            }
        }

        // TODO: conditional: ctx.cisco_meraki?.flows?.op == null
        {
            event.set("cisco_meraki.event_subtype", json!("ip_session_initiated"))?;
        }

        // TODO: conditional: ctx.cisco_meraki?.flows?.op == 'allow'
        {
            event.set("cisco_meraki.event_subtype", json!("flow_allowed"))?;
        }

        // TODO: conditional: ctx.cisco_meraki?.flows?.op == 'deny'
        {
            event.set("cisco_meraki.event_subtype", json!("flow_denied"))?;
        }

        Ok(TransformResult::Continue)
    }
}
