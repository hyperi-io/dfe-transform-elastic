// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `gtp` pipeline.
pub struct Gtp;

impl Transform for Gtp {
    fn name(&self) -> &str {
        "gtp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(csv_str) = event.get_string("message") {
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
                                event.set("_temp_.future_use5", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("panw.panos.flow_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use6", val)?;
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
                                event.set("panw.panos.event_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("panw.panos.msisdn", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("panw.panos.access_point.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("panw.panos.radio_access_technology_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("panw.panos.message_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("panw.panos.end_ip_address", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_endpoint.identifier1", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_endpoint.identifier2", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("panw.panos.interface", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("panw.panos.cause_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("panw.panos.severity", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set("panw.panos.mcc", val)?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("panw.panos.mnc", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event.set("panw.panos.area_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event.set("panw.panos.cell.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event.set("panw.panos.event_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(40) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use10", val)?;
                            }
                        }
                        if let Some(val) = record.get(41) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use11", val)?;
                            }
                        }
                        if let Some(val) = record.get(42) {
                            if !val.is_empty() {
                                event.set("_temp_.srcloc", val)?;
                            }
                        }
                        if let Some(val) = record.get(43) {
                            if !val.is_empty() {
                                event.set("_temp_.dstloc", val)?;
                            }
                        }
                        if let Some(val) = record.get(44) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use12", val)?;
                            }
                        }
                        if let Some(val) = record.get(45) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use13", val)?;
                            }
                        }
                        if let Some(val) = record.get(46) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use14", val)?;
                            }
                        }
                        if let Some(val) = record.get(47) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use15", val)?;
                            }
                        }
                        if let Some(val) = record.get(48) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use16", val)?;
                            }
                        }
                        if let Some(val) = record.get(49) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use17", val)?;
                            }
                        }
                        if let Some(val) = record.get(50) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use18", val)?;
                            }
                        }
                        if let Some(val) = record.get(51) {
                            if !val.is_empty() {
                                event.set("panw.panos.imsi", val)?;
                            }
                        }
                        if let Some(val) = record.get(52) {
                            if !val.is_empty() {
                                event.set("panw.panos.imei", val)?;
                            }
                        }
                        if let Some(val) = record.get(53) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use19", val)?;
                            }
                        }
                        if let Some(val) = record.get(54) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use20", val)?;
                            }
                        }
                        if let Some(val) = record.get(55) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use21", val)?;
                            }
                        }
                        if let Some(val) = record.get(56) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use22", val)?;
                            }
                        }
                        if let Some(val) = record.get(57) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use23", val)?;
                            }
                        }
                        if let Some(val) = record.get(58) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use24", val)?;
                            }
                        }
                        if let Some(val) = record.get(59) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use25", val)?;
                            }
                        }
                        if let Some(val) = record.get(60) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use26", val)?;
                            }
                        }
                        if let Some(val) = record.get(61) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use27", val)?;
                            }
                        }
                        if let Some(val) = record.get(62) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use28", val)?;
                            }
                        }
                        if let Some(val) = record.get(63) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use29", val)?;
                            }
                        }
                        if let Some(val) = record.get(64) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use30", val)?;
                            }
                        }
                        if let Some(val) = record.get(65) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use31", val)?;
                            }
                        }
                        if let Some(val) = record.get(66) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use32", val)?;
                            }
                        }
                        if let Some(val) = record.get(67) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use33", val)?;
                            }
                        }
                        if let Some(val) = record.get(68) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use34", val)?;
                            }
                        }
                        if let Some(val) = record.get(69) {
                            if !val.is_empty() {
                                event.set("panw.panos.start_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(70) {
                            if !val.is_empty() {
                                event.set("panw.panos.elapsed_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(71) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_inspection_rule", val)?;
                            }
                        }
                        if let Some(val) = record.get(72) {
                            if !val.is_empty() {
                                event.set("panw.panos.remote_user.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(73) {
                            if !val.is_empty() {
                                event.set("panw.panos.remote_user.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(74) {
                            if !val.is_empty() {
                                event.set("panw.panos.rule_uuid", val)?;
                            }
                        }
                        if let Some(val) = record.get(75) {
                            if !val.is_empty() {
                                event.set("panw.panos.pcap_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(76) {
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                        if let Some(val) = record.get(77) {
                            if !val.is_empty() {
                                event.set("panw.panos.nsdsai_sst", val)?;
                            }
                        }
                        if let Some(val) = record.get(78) {
                            if !val.is_empty() {
                                event.set("panw.panos.nsdsai_sd", val)?;
                            }
                        }
                        if let Some(val) = record.get(79) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.sub_category", val)?;
                            }
                        }
                        if let Some(val) = record.get(80) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.category", val)?;
                            }
                        }
                        if let Some(val) = record.get(81) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.technology", val)?;
                            }
                        }
                        if let Some(val) = record.get(82) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.risk_level", val)?;
                            }
                        }
                        if let Some(val) = record.get(83) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.characteristics", val)?;
                            }
                        }
                        if let Some(val) = record.get(84) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.container", val)?;
                            }
                        }
                        if let Some(val) = record.get(85) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.is_saas", val)?;
                            }
                        }
                        if let Some(val) = record.get(86) {
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
            event.append("event.category", json!("malware"))?;

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
                if let Some(v) = event.get("panw.panos.severity").cloned() {
                    event.set("log.level", v)?;
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
                event.append(
                    "error.message",
                    json!(format!(
                        "error in GTP pipeline: error in [{}] processor{} with tag [{}]{} {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
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
