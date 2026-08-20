// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `tunnel_inspection` pipeline.
pub struct TunnelInspection;

impl Transform for TunnelInspection {
    fn name(&self) -> &str {
        "tunnel_inspection"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(csv_str) = event.get_string("message") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("panw.panos.destination.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.nat.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("panw.panos.destination.nat.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("panw.panos.ruleset", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("_temp_.srcuser", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("_temp_.dstuser", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("panw.panos.network.application", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("panw.panos.virtual_sys", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.zone", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("panw.panos.destination.zone", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("panw.panos.inbound_interface", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("panw.panos.outbound_interface", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("panw.panos.log_profile", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use1", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("panw.panos.flow_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("panw.panos.repeat_count", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("panw.panos.destination.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.nat.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("panw.panos.destination.nat.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("_temp_.labels", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("panw.panos.protocol", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("panw.panos.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("panw.panos.severity", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("panw.panos.action_flags", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("_temp_.srcloc", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("_temp_.dstloc", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set("panw.panos.imsi", val)?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("panw.panos.imei", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event.set("panw.panos.parent_session.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event.set("panw.panos.parent_session.start_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(40) {
                            if !val.is_empty() {
                                event.set("panw.panos.network.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(41) {
                            if !val.is_empty() {
                                event.set("panw.panos.bytes_sent", val)?;
                            }
                        }
                        if let Some(val) = record.get(42) {
                            if !val.is_empty() {
                                event.set("panw.panos.bytes_received", val)?;
                            }
                        }
                        if let Some(val) = record.get(43) {
                            if !val.is_empty() {
                                event.set("panw.panos.network.packets", val)?;
                            }
                        }
                        if let Some(val) = record.get(44) {
                            if !val.is_empty() {
                                event.set("panw.panos.packets_sent", val)?;
                            }
                        }
                        if let Some(val) = record.get(45) {
                            if !val.is_empty() {
                                event.set("panw.panos.packets_received", val)?;
                            }
                        }
                        if let Some(val) = record.get(46) {
                            if !val.is_empty() {
                                event.set("panw.panos.max_encapsulation", val)?;
                            }
                        }
                        if let Some(val) = record.get(47) {
                            if !val.is_empty() {
                                event.set("panw.panos.unknown_protocol", val)?;
                            }
                        }
                        if let Some(val) = record.get(48) {
                            if !val.is_empty() {
                                event.set("panw.panos.strict_check", val)?;
                            }
                        }
                        if let Some(val) = record.get(49) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_fragment", val)?;
                            }
                        }
                        if let Some(val) = record.get(50) {
                            if !val.is_empty() {
                                event.set("panw.panos.sessions.created", val)?;
                            }
                        }
                        if let Some(val) = record.get(51) {
                            if !val.is_empty() {
                                event.set("panw.panos.sessions.closed", val)?;
                            }
                        }
                        if let Some(val) = record.get(52) {
                            if !val.is_empty() {
                                event.set("panw.panos.endreason", val)?;
                            }
                        }
                        if let Some(val) = record.get(53) {
                            if !val.is_empty() {
                                event.set("panw.panos.action_source", val)?;
                            }
                        }
                        if let Some(val) = record.get(54) {
                            if !val.is_empty() {
                                event.set("panw.panos.start_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(55) {
                            if !val.is_empty() {
                                event.set("panw.panos.elapsed_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(56) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_inspection_rule", val)?;
                            }
                        }
                        if let Some(val) = record.get(57) {
                            if !val.is_empty() {
                                event.set("panw.panos.remote_user.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(58) {
                            if !val.is_empty() {
                                event.set("panw.panos.remote_user.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(59) {
                            if !val.is_empty() {
                                event.set("panw.panos.rule_uuid", val)?;
                            }
                        }
                        if let Some(val) = record.get(60) {
                            if !val.is_empty() {
                                event.set("panw.panos.pcap_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(61) {
                            if !val.is_empty() {
                                event.set("panw.panos.dynamic_user.group.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(62) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.external_dynamic_list", val)?;
                            }
                        }
                        if let Some(val) = record.get(63) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.external_dynamic_list", val)?;
                            }
                        }
                        if let Some(val) = record.get(64) {
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                        if let Some(val) = record.get(65) {
                            if !val.is_empty() {
                                event.set("panw.panos.nssai_sd", val)?;
                            }
                        }
                        if let Some(val) = record.get(66) {
                            if !val.is_empty() {
                                event.set("panw.panos.nssai_sst", val)?;
                            }
                        }
                        if let Some(val) = record.get(67) {
                            if !val.is_empty() {
                                event.set("panw.panos.pdu_session.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(68) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.sub_category", val)?;
                            }
                        }
                        if let Some(val) = record.get(69) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.category", val)?;
                            }
                        }
                        if let Some(val) = record.get(70) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.technology", val)?;
                            }
                        }
                        if let Some(val) = record.get(71) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.risk_level", val)?;
                            }
                        }
                        if let Some(val) = record.get(72) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.characteristics", val)?;
                            }
                        }
                        if let Some(val) = record.get(73) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.container", val)?;
                            }
                        }
                        if let Some(val) = record.get(74) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.is_saas", val)?;
                            }
                        }
                        if let Some(val) = record.get(75) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.is_sanctioned", val)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            let _cond = { event.get_str("panw.panos.action") == Some("allow") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond =
                { !event.has_value("event.outcome") || event.get_str("event.outcome") == Some("") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.bytes_received").cloned() {
                    event.set("destination.bytes", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("_temp_.dstloc").cloned() {
                    event.set("panw.panos.destination.location", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.destination.ip").cloned() {
                    event.set("destination.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.destination.nat.ip").cloned() {
                    event.set("destination.nat.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.destination.nat.port").cloned() {
                    event.set("destination.nat.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.packets_received").cloned() {
                    event.set("destination.packets", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.destination.port").cloned() {
                    event.set("destination.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.elapsed_time").cloned() {
                    event.set("event.duration", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.start_time").cloned() {
                    event.set("event.start", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.network.application").cloned() {
                    event.set("network.application", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.network.bytes").cloned() {
                    event.set("network.bytes", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.network.packets").cloned() {
                    event.set("network.packets", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.protocol").cloned() {
                    event.set("network.transport", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.severity").cloned() {
                    event.set("log.level", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.outbound_interface").cloned() {
                    event.set("observer.egress.interface.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.destination.zone").cloned() {
                    event.set("observer.egress.zone", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.device_name").cloned() {
                    event.set("observer.hostname", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.inbound_interface").cloned() {
                    event.set("observer.ingress.interface.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.source.zone").cloned() {
                    event.set("observer.ingress.zone", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.tunnel_inspection_rule").cloned() {
                    event.set("rule.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.rule_uuid").cloned() {
                    event.set("rule.uuid", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.bytes_sent").cloned() {
                    event.set("source.bytes", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("_temp_.srcloc").cloned() {
                    event.set("panw.panos.source.location", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.source.ip").cloned() {
                    event.set("source.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.source.nat.ip").cloned() {
                    event.set("source.nat.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.source.port").cloned() {
                    event.set("source.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.source.nat.port").cloned() {
                    event.set("source.nat.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.packets_sent").cloned() {
                    event.set("source.packets", v)?;
                }
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
