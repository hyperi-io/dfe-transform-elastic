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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("event.code", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("_tmp_.date", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("_tmp_.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("message", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("source.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("source.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("_tmp_.source.mac", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("user.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.transaction_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.result", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.probation_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.correlation_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.dhc_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.vendor.hex", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.vendor.string", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.user.hex", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.user.string", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.relay_agent_info", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.dns_error_code", val)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("source.address") };
            if _cond {
                map_strings(event, "source.address", "source.address", str::to_lowercase)?;
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
            if let Some(v) = event.get("source.address").cloned() {
                event.set("source.domain", v)?;
            }
            }

            event.set("_tmp_.timestamp", json!(format!("{} {}", event.get("_tmp_.date").map_or_else(String::new, template_to_string), event.get("_tmp_.time").map_or_else(String::new, template_to_string))))?;

                if let Some(date_str) = event.get_as_string("_tmp_.timestamp") {
                    match parse_date_out(&date_str, &["MM/dd/yy HH:mm:ss"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp_.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

                // Painless script
                // Source: if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);"#), cached_params!("{\"00\":{\"action\":\"log-start\",\"category\":[\"process\"],\"reason\":\"The log was started.\",\"type\":[\"start\"]},\"01\":{\"action\":\"log-end\",\"category\":[\"process\"],\"reason\":\"The log was stopped.\",\"type\":[\"end\"]},\"02\":{\"action\":\"log-pause\",\"category\":[\"process\"],\"reason\":\"The log was temporarily paused due to low disk space.\",\"type\":[\"change\"],\"outcome\":\"failure\"},\"10\":{\"action\":\"dhcp-new\",\"category\":[\"network\"],\"reason\":\"A new IP address was leased to a client.\",\"type\":[\"allowed\",\"connection\"]},\"11\":{\"action\":\"dhcp-renew\",\"category\":[\"network\"],\"reason\":\"A lease was renewed by a client.\",\"type\":[\"allowed\",\"connection\"]},\"12\":{\"action\":\"dhcp-release\",\"category\":[\"network\"],\"reason\":\"A lease was released by a client.\",\"type\":[\"allowed\",\"connection\"]},\"13\":{\"category\":[\"network\"],\"reason\":\"An IP address was found to be in use on the network.\",\"type\":[\"connection\"]},\"14\":{\"category\":[\"network\"],\"reason\":\"A lease request could not be satisfied because the scope's address pool was exhausted.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"15\":{\"action\":\"dhcp-deny\",\"category\":[\"network\"],\"reason\":\"A lease was denied.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"16\":{\"action\":\"dhcp-delete\",\"category\":[\"network\"],\"reason\":\"A lease was deleted.\",\"type\":[\"connection\"]},\"17\":{\"action\":\"dhcp-expire\",\"category\":[\"network\"],\"reason\":\"A lease was expired and DNS records for an expired leases have not been deleted.\",\"type\":[\"connection\"]},\"18\":{\"action\":\"dhcp-expire\",\"category\":[\"network\"],\"reason\":\"A lease was expired and DNS records were deleted.\",\"type\":[\"connection\"]},\"20\":{\"category\":[\"network\"],\"reason\":\"A BOOTP address was leased to a client.\",\"type\":[\"allowed\",\"connection\"]},\"21\":{\"category\":[\"network\"],\"reason\":\"A dynamic BOOTP address was leased to a client.\",\"type\":[\"allowed\",\"connection\"]},\"22\":{\"category\":[\"network\"],\"reason\":\"A BOOTP request could not be satisfied because the scope's address pool for BOOTP was exhausted.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"23\":{\"category\":[\"network\"],\"reason\":\"A BOOTP IP address was deleted after checking to see it was not in use.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"24\":{\"action\":\"ip-cleanup-start\",\"category\":[\"process\"],\"reason\":\"IP address cleanup operation has began.\",\"type\":[\"start\"]},\"25\":{\"action\":\"ip-cleanup-end\",\"category\":[\"process\"],\"reason\":\"IP address cleanup statistics.\",\"type\":[\"start\"]},\"30\":{\"action\":\"dhcp-dns-update\",\"category\":[\"network\"],\"reason\":\"DNS update request to the named DNS server.\",\"type\":[\"connection\"]},\"31\":{\"action\":\"dhcp-dns-update\",\"category\":[\"network\"],\"reason\":\"DNS update failed.\",\"type\":[\"connection\"],\"outcome\":\"failure\"},\"32\":{\"action\":\"dhcp-dns-update\",\"category\":[\"network\"],\"reason\":\"DNS update successful.\",\"type\":[\"connection\"]},\"33\":{\"category\":[\"network\"],\"reason\":\"Packet dropped due to NAP policy.\",\"type\":[\"connection\"],\"outcome\":\"failure\"},\"34\":{\"action\":\"dhcp-dns-update\",\"category\":[\"network\"],\"reason\":\"DNS update request failed.as the DNS update request queue limit exceeded.\",\"type\":[\"connection\"],\"outcome\":\"failure\"},\"35\":{\"action\":\"dhcp-dns-update\",\"category\":[\"network\"],\"reason\":\"DNS update request failed.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"36\":{\"category\":[\"network\"],\"reason\":\"Packet dropped because the server is in failover standby role or the hash of the client ID does not match.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"50\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server could not locate the applicable domain for its configured Active Directory installation.\",\"type\":[\"connection\"],\"outcome\":\"failure\"},\"51\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server was authorized to start on the network.\",\"type\":[\"allowed\",\"connection\"]},\"52\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server was recently upgraded to a Windows Server 2008 operating system, and, therefore, the unauthorized DHCP server detection feature (used to determine whether the server has been authorized in Active Directory) was disabled.\",\"type\":[\"connection\"]},\"53\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server was authorized to start using previously cached information. AD DS could not be found at the time the server was started on the network.\",\"type\":[\"allowed\",\"connection\"]},\"54\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server was not authorized to start on the network. When this event occurs, it is likely followed by the server being stopped.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"55\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server was successfully authorized to start on the network.\",\"type\":[\"allowed\",\"connection\"]},\"56\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server was not authorized to start on the network and was shut down by the operating system. You must first authorize the server in the directory before starting it again.\",\"type\":[\"connection\",\"denied\"],\"outcome\":\"failure\"},\"57\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"Another DHCP server exists and is authorized for service in the same domain.\",\"type\":[\"connection\"]},\"58\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server could not locate the specified domain.\",\"type\":[\"connection\"],\"outcome\":\"failure\"},\"59\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"A network-related failure prevented the server from determining if it is authorized.\",\"type\":[\"connection\"],\"outcome\":\"failure\"},\"60\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"No domain controller running Windows Server 2008 was located. For detecting whether the server is authorized, a domain controller that is enabled for AD DS is required.\",\"type\":[\"connection\"]},\"61\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"Another DHCP server was found on the network that belongs to the Active Directory domain.\",\"type\":[\"connection\"]},\"62\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"Another DHCP server was found on the network.\",\"type\":[\"connection\"]},\"63\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server is trying once more to determine whether it is authorized to start and provide service on the network.\",\"type\":[\"connection\"]},\"64\":{\"action\":\"rogue-server-detection\",\"category\":[\"authentication\",\"network\"],\"reason\":\"The DHCP server has its service bindings or network connections configured so that it is not enabled to provide service.\",\"type\":[\"connection\"]}}"))?;

                // Painless script
                // Source: if (ctx.microsoft?.dhcp?.result == null) {\n  return;\n}\ndef desc = params.get(ctx.microsoft.dhcp.result);\nif (desc == null) {\n  return;\n}\nctx.microsoft.dhcp['result_description'] = desc; 
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.microsoft?.dhcp?.result == null) {\n  return;\n}\ndef desc = params.get(ctx.microsoft.dhcp.result);\nif (desc == null) {\n  return;\n}\nctx.microsoft.dhcp['result_description'] = desc; "#), cached_params!("{\"0\":\"NoQuarantine\",\"1\":\"Quarantine\",\"2\":\"Drop Packet\",\"3\":\"Probation\",\"6\":\"No Quarantine Information\"}"))?;

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("event.reasson") };
            if _cond {
            event.set("event.reason", json!(event.get("event.reason").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("_tmp_.source.mac") {
                map_strings(event, "_tmp_.source.mac", "_tmp_.source.mac", str::to_uppercase)?;
            }

            if event.has_value("_tmp_.source.mac") {
                gsub_field(event, "_tmp_.source.mac", "_tmp_.source.mac", cached_regex!("(..)(?!$)"), "$1-")?;
            }

            let _cond = { event.has_value("_tmp_.source.mac") };
            if _cond {
            if let Some(v) = event.get("_tmp_.source.mac").cloned() {
                event.set("source.mac", v)?;
            }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
