// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `sctp` pipeline.
pub struct Sctp;

impl Transform for Sctp {
    fn name(&self) -> &str {
        "sctp"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(csv_str) = event.get_str("message").map(String::from) {
                let csv_str = csv_str.as_str();
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
            event.set(
                "destination.ip",
                event
                    .get("panw.panos.destination.ip")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "destination.packets",
                event
                    .get("panw.panos.packets_received")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "destination.port",
                event
                    .get("panw.panos.destination.port")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "log.level",
                event
                    .get("panw.panos.severity")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "network.packets",
                event
                    .get("panw.panos.network.packets")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "network.transport",
                event
                    .get("panw.panos.protocol")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "observer.egress.interface.name",
                event
                    .get("panw.panos.outbound_interface")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "observer.egress.zone",
                event
                    .get("panw.panos.destination.zone")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "observer.hostname",
                event
                    .get("panw.panos.device_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "observer.ingress.interface.name",
                event
                    .get("panw.panos.inbound_interface")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "observer.ingress.zone",
                event
                    .get("panw.panos.source.zone")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "panw.panos.serial_number",
                event
                    .get("observer.serial_number")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "rule.uuid",
                event
                    .get("panw.panos.rule_uuid")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "source.ip",
                event
                    .get("panw.panos.source.ip")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "source.packets",
                event
                    .get("panw.panos.packets_sent")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "source.port",
                event
                    .get("panw.panos.source.port")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        Ok(TransformResult::Continue)
    }
}
