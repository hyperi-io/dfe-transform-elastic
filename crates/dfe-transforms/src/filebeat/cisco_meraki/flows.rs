// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `flows` pipeline.
pub struct Flows;

impl Transform for Flows {
    fn name(&self) -> &str {
        "flows"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall) %{GREEDYDATA:message}
                    let _ = cached_grok!("(?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall) %{GREEDYDATA:message}").extract_into(&input, event)?;
                }
                Ok(())
            })();

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall)( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?
                let _ = cached_grok!("(?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall)( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?").extract_into(&input, event)?;
            }

            let _cond = {
                event.has_value("cisco_meraki.firewall.pattern")
                    && (event
                        .get_str("cisco_meraki.firewall.pattern")
                        .is_some_and(|s| s.starts_with("allow"))
                        || event
                            .get_str("cisco_meraki.firewall.pattern")
                            .is_some_and(|s| s.starts_with("deny")))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cisco_meraki.firewall.pattern") {
                        // Grok pattern: %{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}
                        let _ = cached_grok!("%{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("cisco_meraki.firewall.rule") };
            if _cond {
                if event.remove("cisco_meraki.firewall.pattern").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "cisco_meraki.firewall.pattern".into(),
                    });
                }
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            let _cond = { !event.has_value("cisco_meraki.flows.op") };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("ip_session_initiated"))?;
            }

            let _cond = { event.get_str("cisco_meraki.flows.op") == Some("allow") };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("flow_allowed"))?;
            }

            let _cond = { event.get_str("cisco_meraki.flows.op") == Some("deny") };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("flow_denied"))?;
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
