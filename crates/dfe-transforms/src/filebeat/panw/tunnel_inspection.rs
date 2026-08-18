// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `tunnel_inspection` pipeline.
pub struct TunnelInspection;

impl Transform for TunnelInspection {
    fn name(&self) -> &str {
        "tunnel_inspection"
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

        // TODO: conditional: ctx.panw?.panos?.action == "allow"
        {
            event.set("event.outcome", json!("success"))?;
        }

        // TODO: conditional: ctx.event?.outcome == null || ctx.event.outcome == ""
        {
            event.set("event.outcome", json!("failure"))?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "destination.bytes",
                event
                    .get("panw.panos.bytes_received")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "panw.panos.destination.location",
                event.get("_temp_.dstloc").cloned().unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

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
                "destination.nat.ip",
                event
                    .get("panw.panos.destination.nat.ip")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "destination.nat.port",
                event
                    .get("panw.panos.destination.nat.port")
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
                "event.duration",
                event
                    .get("panw.panos.elapsed_time")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "event.start",
                event
                    .get("panw.panos.start_time")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "network.application",
                event
                    .get("panw.panos.network.application")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "network.bytes",
                event
                    .get("panw.panos.network.bytes")
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
                "rule.name",
                event
                    .get("panw.panos.tunnel_inspection_rule")
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
                "source.bytes",
                event
                    .get("panw.panos.bytes_sent")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "panw.panos.source.location",
                event.get("_temp_.srcloc").cloned().unwrap_or(Value::Null),
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
                "source.nat.ip",
                event
                    .get("panw.panos.source.nat.ip")
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

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "source.nat.port",
                event
                    .get("panw.panos.source.nat.port")
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

        Ok(TransformResult::Continue)
    }
}
