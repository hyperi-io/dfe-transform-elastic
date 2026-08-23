// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `sctp` pipeline.
pub struct Sctp;

impl Transform for Sctp {
    fn name(&self) -> &str {
        "sctp"
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
                                event.set("_temp_.future_use1", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use2", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("panw.panos.ruleset", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use3", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use4", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use5", val)?;
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
                                event.set("_temp_.future_use6", val)?;
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
                                event.set("_temp_.future_use7", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use8", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use9", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use10", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("panw.panos.protocol", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("panw.panos.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use11", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.assoc_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("panw.panos.payload_protocol_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set("panw.panos.severity", val)?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.chunk_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use12", val)?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.verification.tag_1", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.verification.tag_2", val)?;
                            }
                        }
                        if let Some(val) = record.get(40) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.cause_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(41) {
                            if !val.is_empty() {
                                event.set("panw.panos.diameter_app_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(42) {
                            if !val.is_empty() {
                                event.set("panw.panos.diameter_cmd_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(43) {
                            if !val.is_empty() {
                                event.set("panw.panos.diameter_avp_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(44) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.stream_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(45) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.assoc_end_reason", val)?;
                            }
                        }
                        if let Some(val) = record.get(46) {
                            if !val.is_empty() {
                                event.set("panw.panos.op_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(47) {
                            if !val.is_empty() {
                                event.set("panw.panos.sccp.calling_ssn", val)?;
                            }
                        }
                        if let Some(val) = record.get(48) {
                            if !val.is_empty() {
                                event.set("panw.panos.sccp.calling_gt", val)?;
                            }
                        }
                        if let Some(val) = record.get(49) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.filter", val)?;
                            }
                        }
                        if let Some(val) = record.get(50) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.chunks", val)?;
                            }
                        }
                        if let Some(val) = record.get(51) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.chunks_sent", val)?;
                            }
                        }
                        if let Some(val) = record.get(52) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.chunks_received", val)?;
                            }
                        }
                        if let Some(val) = record.get(53) {
                            if !val.is_empty() {
                                event.set("panw.panos.network.packets", val)?;
                            }
                        }
                        if let Some(val) = record.get(54) {
                            if !val.is_empty() {
                                event.set("panw.panos.packets_sent", val)?;
                            }
                        }
                        if let Some(val) = record.get(55) {
                            if !val.is_empty() {
                                event.set("panw.panos.packets_received", val)?;
                            }
                        }
                        if let Some(val) = record.get(56) {
                            if !val.is_empty() {
                                event.set("panw.panos.rule_uuid", val)?;
                            }
                        }
                        if let Some(val) = record.get(57) {
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.destination.ip").cloned() {
                    event.set("destination.ip", v)?;
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
                if let Some(v) = event.get("panw.panos.severity").cloned() {
                    event.set("log.level", v)?;
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
                if let Some(v) = event.get("observer.serial_number").cloned() {
                    event.set("panw.panos.serial_number", v)?;
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
                if let Some(v) = event.get("panw.panos.source.ip").cloned() {
                    event.set("source.ip", v)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.source.port").cloned() {
                    event.set("source.port", v)?;
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
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
