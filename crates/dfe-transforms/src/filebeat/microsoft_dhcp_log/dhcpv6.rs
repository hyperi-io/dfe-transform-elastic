// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `dhcpv6` pipeline.
pub struct Dhcpv6;

impl Transform for Dhcpv6 {
    fn name(&self) -> &str {
        "dhcpv6"
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
                                event.set("microsoft.dhcp.error_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.duid.length", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.duid.hex", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.user.string", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.dhc_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("microsoft.dhcp.subnet_prefix", val)?;
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
                // Source: if (ctx?.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx?.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);"#), cached_params!("{\"11000\":{\"action\":\"dhcpv6-solicit\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\"]},\"11001\":{\"action\":\"dhcpv6-advertise\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\"]},\"11002\":{\"action\":\"dhcpv6-request\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\"]},\"11003\":{\"action\":\"dhcpv6-confirm\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\"]},\"11004\":{\"action\":\"dhcpv6-renew\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\"]},\"11005\":{\"action\":\"dhcpv6-rebind\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\"]},\"11006\":{\"action\":\"dhcpv6-decline\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\"],\"outcome\":\"failure\"},\"11007\":{\"action\":\"dhcpv6-release\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11008\":{\"action\":\"dhcpv6-info-request\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11009\":{\"action\":\"dhcpv6-scope-full\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11010\":{\"action\":\"log-start\",\"category\":[\"process\"],\"type\":[\"start\"]},\"11011\":{\"action\":\"log-stop\",\"category\":[\"process\"],\"type\":[\"end\"]},\"11012\":{\"action\":\"log-pause\",\"category\":[\"process\"],\"type\":[\"change\"]},\"11013\":{\"action\":\"log-file\",\"category\":[\"process\"],\"type\":[\"info\"]},\"11014\":{\"action\":\"dhcpv6-bad-address\",\"category\":[\"network\"],\"type\":[\"connection\"],\"outcome\":\"failure\"},\"11015\":{\"action\":\"dhcpv6-address-in-use\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11016\":{\"action\":\"dhcpv6-client-deleted\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11017\":{\"action\":\"ipv6-dns-record-not-deleted\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11018\":{\"action\":\"dhcpv6-expired\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11019\":{\"action\":\"dhcpv6-lease-expired-deleted\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"11020\":{\"action\":\"dhcpv6-cleanup-start\",\"category\":[\"process\"],\"type\":[\"start\"]},\"11021\":{\"action\":\"dhcpv6-cleanup-end\",\"category\":[\"process\"],\"type\":[\"end\"]},\"11022\":{\"action\":\"ipv6-dns-update-request\",\"category\":[\"network\"],\"type\":[\"connection\",\"start\"]},\"11023\":{\"action\":\"ipv6-dns-update-failed\",\"category\":[\"network\"],\"type\":[\"connection\",\"end\"],\"outcome\":\"failure\"},\"11024\":{\"action\":\"ipv6-dns-update-successful\",\"category\":[\"network\"],\"type\":[\"connection\",\"end\"]},\"11028\":{\"action\":\"ipv6-dns-update-request-queue-exceeded\",\"category\":[\"network\"],\"type\":[\"connection\",\"end\"],\"outcome\":\"failure\"},\"11029\":{\"action\":\"ipv6-dns-update-request-failed\",\"category\":[\"network\"],\"type\":[\"connection\",\"end\"],\"outcome\":\"failure\"},\"11030\":{\"action\":\"dhcpv6-stateless-clients-pruged\",\"category\":[\"process\"],\"type\":[\"change\"]},\"11031\":{\"action\":\"dhcpv6-stateless-clients-expired\",\"category\":[\"process\"],\"type\":[\"change\"]},\"11032\":{\"action\":\"dhcpv6-stateless-client-info-request\",\"category\":[\"network\"],\"type\":[\"info\"]}}"))?;

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
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
