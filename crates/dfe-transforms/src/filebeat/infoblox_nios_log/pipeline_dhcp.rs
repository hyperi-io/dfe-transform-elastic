// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_dhcp` pipeline.
pub struct PipelineDhcp;

impl Transform for PipelineDhcp {
    fn name(&self) -> &str {
        "pipeline_dhcp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("network.protocol", json!("dhcp"))?;

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPDISCOVER")), serde_json::Value::String(s) => s.contains("DHCPDISCOVER"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: network %{DATA:infoblox_nios.log.dhcp.network}: %{GREEDYDATA:infoblox_nios.log.dhcp.discover.message}$
                    // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                    // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: network %{DATA:infoblox_nios.log.dhcp.network}: %{GREEDYDATA:infoblox_nios.log.dhcp.discover.message}$"),
                            cached_grok!("^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"),
                            cached_grok!("^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPOFFER")), serde_json::Value::String(s) => s.contains("DHCPOFFER"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPREQUEST")), serde_json::Value::String(s) => s.contains("DHCPREQUEST"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                    // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"),
                            cached_grok!("^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPACK")), serde_json::Value::String(s) => s.contains("DHCPACK"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} (?:\\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) )?via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\)$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\)$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$
                    // Grok pattern: ^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} (?:\\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) )?via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\)$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\)$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$"),
                            cached_grok!("^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("RELEASE")), serde_json::Value::String(s) => s.contains("RELEASE"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                    // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"),
                            cached_grok!("^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"),
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPEXPIRE")), serde_json::Value::String(s) => s.contains("DHCPEXPIRE"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{GREEDYDATA:client.mac}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{GREEDYDATA:client.mac}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPINFORM")), serde_json::Value::String(s) => s.contains("DHCPINFORM"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.inform.message}$
                    // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.inform.message}$"),
                            cached_grok!("^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPDECLINE")), serde_json::Value::String(s) => s.contains("DHCPDECLINE"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$
                    // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}): %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$"),
                            cached_grok!("^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}): %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPNAK")), serde_json::Value::String(s) => s.contains("DHCPNAK"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("DHCPLEASEQUERY")), serde_json::Value::String(s) => s.contains("DHCPLEASEQUERY"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip}: %{GREEDYDATA:infoblox_nios.log.dhcp.lease_query.message}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:event.action} from %{IP:client.ip}: %{GREEDYDATA:infoblox_nios.log.dhcp.lease_query.message}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("REFUSED")), serde_json::Value::String(s) => s.contains("REFUSED"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^(?P<event_action>(?:(?i:reverse map update))) for %{IP:client.ip} abandoned because of non-retryable failure: %{DATA:event.outcome}$
                    // Grok pattern: ^Unable to (?P<event_action>(?:(?i:add forward map))) from %{DATA:infoblox_nios.log.dhcp.forward_name} to %{IP:infoblox_nios.log.dhcp.ip} by server %{IP:server.ip}#%{NUMBER:server.port:long}: %{DATA:event.outcome}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!("^(?P<event_action>(?:(?i:reverse map update))) for %{IP:client.ip} abandoned because of non-retryable failure: %{DATA:event.outcome}$", [("event_action", "event.action")]),
                            cached_grok_mapped!("^Unable to (?P<event_action>(?:(?i:add forward map))) from %{DATA:infoblox_nios.log.dhcp.forward_name} to %{IP:infoblox_nios.log.dhcp.ip} by server %{IP:server.ip}#%{NUMBER:server.port:long}: %{DATA:event.outcome}$", [("event_action", "event.action")]),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("event.outcome").is_some_and(|s| s.eq_ignore_ascii_case("refused")) };
            if _cond {
                gsub_field(event, "event.action", "event.action", cached_regex!(" "), "_")?;
            }

            let _cond = { event.get_str("event.outcome").is_some_and(|s| s.eq_ignore_ascii_case("refused")) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Encapsulated Solicit")), serde_json::Value::String(s) => s.contains("Encapsulated Solicit"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long} from client DUID %{GREEDYDATA:infoblox_nios.log.dhcp.duid}, transaction ID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long} from client DUID %{GREEDYDATA:infoblox_nios.log.dhcp.duid}, transaction ID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Advertise NA")), serde_json::Value::String(s) => s.contains("Advertise NA"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA:event.action}: address %{IP:client.ip} to client with duid %{GREEDYDATA:infoblox_nios.log.dhcp.duid} iaid = -%{GREEDYDATA:infoblox_nios.log.dhcp.iaid} valid for %{NUMBER:infoblox_nios.log.dhcp.validation_second:long} seconds$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:event.action}: address %{IP:client.ip} to client with duid %{GREEDYDATA:infoblox_nios.log.dhcp.duid} iaid = -%{GREEDYDATA:infoblox_nios.log.dhcp.iaid} valid for %{NUMBER:infoblox_nios.log.dhcp.validation_second:long} seconds$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Relay-forward")), serde_json::Value::String(s) => s.contains("Relay-forward"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long}, link address %{IP:infoblox_nios.log.dhcp.link_address}, peer address %{IP:infoblox_nios.log.dhcp.peer_address}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long}, link address %{IP:infoblox_nios.log.dhcp.link_address}, peer address %{IP:infoblox_nios.log.dhcp.peer_address}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Encapsulating Advertise")), serde_json::Value::String(s) => s.contains("Encapsulating Advertise"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA:event.action} message to send to %{IP:client.ip} port %{NUMBER:client.port:long}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:event.action} message to send to %{IP:client.ip} port %{NUMBER:client.port:long}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Sending Relay-reply")), serde_json::Value::String(s) => s.contains("Sending Relay-reply"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA:event.action} message to %{IP:client.ip} port %{NUMBER:client.port:long}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:event.action} message to %{IP:client.ip} port %{NUMBER:client.port:long}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                    if !cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }
                Ok(())
            })();

            if event.has_value("client.mac") {
                gsub_field(event, "client.mac", "client.mac", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("client.mac") {
                map_strings(event, "client.mac", "client.mac", str::to_uppercase)?;
            }

            let _cond = { event.has_value("client.ip") && event.get_str("client.ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("client.ip") {
                if let Some(val) = event.get("client.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "client.ip".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_ip")?;
                        event.remove("client.ip");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.link_address") && event.get_str("infoblox_nios.log.dhcp.link_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("infoblox_nios.log.dhcp.link_address") {
                if let Some(val) = event.get("infoblox_nios.log.dhcp.link_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "infoblox_nios.log.dhcp.link_address".into(),
                            message,
                        })?;
                    event.set("infoblox_nios.log.dhcp.link_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dhcp_link_address")?;
                        event.remove("infoblox_nios.log.dhcp.link_address");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.link_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("infoblox_nios.log.dhcp.link_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.peer_address") && event.get_str("infoblox_nios.log.dhcp.peer_address") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("infoblox_nios.log.dhcp.peer_address") {
                if let Some(val) = event.get("infoblox_nios.log.dhcp.peer_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "infoblox_nios.log.dhcp.peer_address".into(),
                            message,
                        })?;
                    event.set("infoblox_nios.log.dhcp.peer_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dhcp_peer_address")?;
                        event.remove("infoblox_nios.log.dhcp.peer_address");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.peer_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("infoblox_nios.log.dhcp.peer_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.router.ip") && event.get_str("infoblox_nios.log.dhcp.router.ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("infoblox_nios.log.dhcp.router.ip") {
                if let Some(val) = event.get("infoblox_nios.log.dhcp.router.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "infoblox_nios.log.dhcp.router.ip".into(),
                            message,
                        })?;
                    event.set("infoblox_nios.log.dhcp.router.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dhcp_router_ip")?;
                        event.remove("infoblox_nios.log.dhcp.router.ip");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.router.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("infoblox_nios.log.dhcp.router.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.interface.ip") && event.get_str("infoblox_nios.log.dhcp.interface.ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("infoblox_nios.log.dhcp.interface.ip") {
                if let Some(val) = event.get("infoblox_nios.log.dhcp.interface.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "infoblox_nios.log.dhcp.interface.ip".into(),
                            message,
                        })?;
                    event.set("infoblox_nios.log.dhcp.interface.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dhcp_interface_ip")?;
                        event.remove("infoblox_nios.log.dhcp.interface.ip");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.interface.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("infoblox_nios.log.dhcp.interface.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.relay.interface.ip") && event.get_str("infoblox_nios.log.dhcp.relay.interface.ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("infoblox_nios.log.dhcp.relay.interface.ip") {
                if let Some(val) = event.get("infoblox_nios.log.dhcp.relay.interface.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "infoblox_nios.log.dhcp.relay.interface.ip".into(),
                            message,
                        })?;
                    event.set("infoblox_nios.log.dhcp.relay.interface.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dhcp_relay_interface_ip")?;
                        event.remove("infoblox_nios.log.dhcp.relay.interface.ip");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.relay.interface.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("infoblox_nios.log.dhcp.relay.interface.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("infoblox_nios.log.dhcp.client_hostname") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hosts", json!(event.get("infoblox_nios.log.dhcp.client_hostname").map_or_else(String::new, template_to_string)))?;
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
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
