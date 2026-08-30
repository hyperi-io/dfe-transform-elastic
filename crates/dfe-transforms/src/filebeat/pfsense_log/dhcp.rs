// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dhcp` pipeline.
pub struct Dhcp;

impl Transform for Dhcp {
    fn name(&self) -> &str {
        "dhcp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{DATA:_tmp.action}\\(%{DATA:observer.ingress.interface.name}\\)(?: %{IP:client.ip})? %{MAC:client.mac}(?: %{HOSTNAME:pfsense.dhcp.hostname})?
                    // Grok pattern: %{DATA:_tmp.action}/(?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))/(?P<server_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})))/%{NOTSPACE:pfsense.dhcp.subnet}
                    // Grok pattern: %{DATA:_tmp.action} %{IPV6:client.address}(/%{NUMBER})? on (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))
                    // Grok pattern: %{DATA:_tmp.action} (from|to) %{IPV6:client.address} port %{POSINT:client.port:long}(, transaction ID %{NOTSPACE:pfsense.dhcp.transaction_id})?
                    // Grok pattern: %{DATA:_tmp.action} for: %{IPV6:client.address}(, age %{POSINT:pfsense.dhcp.age:long} secs)?%{GREEDYDATA}
                    // Grok pattern: %{DATA:_tmp.action}: address %{IPV6:client.address} to client with duid (?P<pfsense_dhcp_duid>(?:(?i)[0-9a-f]{2}(:[0-9a-f]{2})+)) iaid = -%{NOTSPACE:pfsense.dhcp.iaid} valid for %{POSINT:pfsense.dhcp.lease_time:long} seconds
                    // Grok pattern: %{WORD:event.action} (?:(?:(?:from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))))|(?:on %{IP:client.address} to (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\))|(?:for %{IP:client.address} \\(%{IP:server.address}\\)? from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\)))) via (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))
                    // Grok pattern: %{DATA:_tmp.action} %{IPV6:client.address}
                    // Grok pattern: %{GREEDYDATA}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("%{DATA:_tmp.action}\\(%{DATA:observer.ingress.interface.name}\\)(?: %{IP:client.ip})? %{MAC:client.mac}(?: %{HOSTNAME:pfsense.dhcp.hostname})?"),
                            cached_grok_mapped!("%{DATA:_tmp.action}/(?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))/(?P<server_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})))/%{NOTSPACE:pfsense.dhcp.subnet}", [("observer_ingress_interface_name", "observer.ingress.interface.name"), ("server_mac", "server.mac")]),
                            cached_grok_mapped!("%{DATA:_tmp.action} %{IPV6:client.address}(/%{NUMBER})? on (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))", [("observer_ingress_interface_name", "observer.ingress.interface.name")]),
                            cached_grok!("%{DATA:_tmp.action} (from|to) %{IPV6:client.address} port %{POSINT:client.port:long}(, transaction ID %{NOTSPACE:pfsense.dhcp.transaction_id})?"),
                            cached_grok!("%{DATA:_tmp.action} for: %{IPV6:client.address}(, age %{POSINT:pfsense.dhcp.age:long} secs)?%{GREEDYDATA}"),
                            cached_grok_mapped!("%{DATA:_tmp.action}: address %{IPV6:client.address} to client with duid (?P<pfsense_dhcp_duid>(?:(?i)[0-9a-f]{2}(:[0-9a-f]{2})+)) iaid = -%{NOTSPACE:pfsense.dhcp.iaid} valid for %{POSINT:pfsense.dhcp.lease_time:long} seconds", [("pfsense_dhcp_duid", "pfsense.dhcp.duid")]),
                            cached_grok_mapped!("%{WORD:event.action} (?:(?:(?:from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))))|(?:on %{IP:client.address} to (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\))|(?:for %{IP:client.address} \\(%{IP:server.address}\\)? from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\)))) via (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))", [("observer_ingress_interface_name", "observer.ingress.interface.name"), ("client_mac", "client.mac"), ("client_mac", "client.mac"), ("client_mac", "client.mac")]),
                            cached_grok!("%{DATA:_tmp.action} %{IPV6:client.address}"),
                            cached_grok!("%{GREEDYDATA}"),
                        ],
                        &input,
                        event,
                    )?;
                }

                event.append_unique("event.type", json!("connection"))?;
                event.append_unique("event.type", json!("protocol"))?;
                event.append_unique("event.type", json!("info"))?;

            event.set("network.protocol", json!("dhcp"))?;

            let _cond = { event.get_str("event.provider") == Some("dhcp6c") || (event.has_value("server.address") && event.get("server.address").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")), serde_json::Value::String(s) => s.contains(":"), _ => false })) || (event.has_value("client.address") && event.get("client.address").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")), serde_json::Value::String(s) => s.contains(":"), _ => false })) };
            if _cond {
            event.set("network.protocol", json!("dhcpv6"))?;
            }

            event.set("network.transport", json!("udp"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("client.address") {
                if let Some(val) = event.get("client.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "client.address".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("server.address") {
                if let Some(val) = event.get("server.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "server.address".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
                }
            }
                Ok(())
            })();

            if event.has_value("client.mac") {
                map_strings(event, "client.mac", "client.mac", str::to_uppercase)?;
            }

            if event.has_value("client.mac") {
                gsub_field(event, "client.mac", "client.mac", cached_regex!("[:]"), "-")?;
            }

            if event.has_value("server.mac") {
                map_strings(event, "server.mac", "server.mac", str::to_uppercase)?;
            }

            if event.has_value("server.mac") {
                gsub_field(event, "server.mac", "server.mac", cached_regex!("[:]"), "-")?;
            }

            if event.has_value("_tmp.action") {
                map_strings(event, "_tmp.action", "_tmp.action", str::to_lowercase)?;
            }

            if event.has_value("_tmp.action") {
                gsub_field(event, "_tmp.action", "event.action", cached_regex!(" "), "-")?;
            }

            if let Some(v) = event.get("client").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source", v)?;
            }

            if let Some(v) = event.get("server").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination", v)?;
            }

                gsub_field(event, "event.provider", "event.provider", cached_regex!("dnsmasq-dhcp"), "dhcpd")?;

            let _cond = { event.has_value("pfsense.log.dhcp.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("pfsense.dhcp.hostname").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
