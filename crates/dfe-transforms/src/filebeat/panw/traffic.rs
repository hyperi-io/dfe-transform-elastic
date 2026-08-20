// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `traffic` pipeline.
pub struct Traffic;

impl Transform for Traffic {
    fn name(&self) -> &str {
        "traffic"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
                            event.set("panw.panos.network.bytes", val)?;
                        }
                    }
                    if let Some(val) = record.get(25) {
                        if !val.is_empty() {
                            event.set("panw.panos.bytes_sent", val)?;
                        }
                    }
                    if let Some(val) = record.get(26) {
                        if !val.is_empty() {
                            event.set("panw.panos.bytes_received", val)?;
                        }
                    }
                    if let Some(val) = record.get(27) {
                        if !val.is_empty() {
                            event.set("panw.panos.network.packets", val)?;
                        }
                    }
                    if let Some(val) = record.get(28) {
                        if !val.is_empty() {
                            event.set("panw.panos.start_time", val)?;
                        }
                    }
                    if let Some(val) = record.get(29) {
                        if !val.is_empty() {
                            event.set("panw.panos.elapsed_time", val)?;
                        }
                    }
                    if let Some(val) = record.get(30) {
                        if !val.is_empty() {
                            event.set("panw.panos.url.category", val)?;
                        }
                    }
                    if let Some(val) = record.get(31) {
                        if !val.is_empty() {
                            event.set("_temp_.future_use2", val)?;
                        }
                    }
                    if let Some(val) = record.get(32) {
                        if !val.is_empty() {
                            event.set("panw.panos.sequence_number", val)?;
                        }
                    }
                    if let Some(val) = record.get(33) {
                        if !val.is_empty() {
                            event.set("panw.panos.action_flags", val)?;
                        }
                    }
                    if let Some(val) = record.get(34) {
                        if !val.is_empty() {
                            event.set("_temp_.srcloc", val)?;
                        }
                    }
                    if let Some(val) = record.get(35) {
                        if !val.is_empty() {
                            event.set("_temp_.dstloc", val)?;
                        }
                    }
                    if let Some(val) = record.get(36) {
                        if !val.is_empty() {
                            event.set("_temp_.future_use3", val)?;
                        }
                    }
                    if let Some(val) = record.get(37) {
                        if !val.is_empty() {
                            event.set("panw.panos.packets_sent", val)?;
                        }
                    }
                    if let Some(val) = record.get(38) {
                        if !val.is_empty() {
                            event.set("panw.panos.packets_received", val)?;
                        }
                    }
                    if let Some(val) = record.get(39) {
                        if !val.is_empty() {
                            event.set("panw.panos.endreason", val)?;
                        }
                    }
                    if let Some(val) = record.get(40) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy1", val)?;
                        }
                    }
                    if let Some(val) = record.get(41) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy2", val)?;
                        }
                    }
                    if let Some(val) = record.get(42) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy3", val)?;
                        }
                    }
                    if let Some(val) = record.get(43) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy4", val)?;
                        }
                    }
                    if let Some(val) = record.get(44) {
                        if !val.is_empty() {
                            event.set("panw.panos.vsys_name", val)?;
                        }
                    }
                    if let Some(val) = record.get(45) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_name", val)?;
                        }
                    }
                    if let Some(val) = record.get(46) {
                        if !val.is_empty() {
                            event.set("panw.panos.action_source", val)?;
                        }
                    }
                    if let Some(val) = record.get(47) {
                        if !val.is_empty() {
                            event.set("panw.panos.source_vm_uuid", val)?;
                        }
                    }
                    if let Some(val) = record.get(48) {
                        if !val.is_empty() {
                            event.set("panw.panos.destination_vm_uuid", val)?;
                        }
                    }
                    if let Some(val) = record.get(49) {
                        if !val.is_empty() {
                            event.set("panw.panos.imsi", val)?;
                        }
                    }
                    if let Some(val) = record.get(50) {
                        if !val.is_empty() {
                            event.set("panw.panos.imei", val)?;
                        }
                    }
                    if let Some(val) = record.get(51) {
                        if !val.is_empty() {
                            event.set("panw.panos.parent_session.id", val)?;
                        }
                    }
                    if let Some(val) = record.get(52) {
                        if !val.is_empty() {
                            event.set("panw.panos.parent_session.start_time", val)?;
                        }
                    }
                    if let Some(val) = record.get(53) {
                        if !val.is_empty() {
                            event.set("panw.panos.tunnel_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(54) {
                        if !val.is_empty() {
                            event.set("panw.panos.sctp.assoc_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(55) {
                        if !val.is_empty() {
                            event.set("panw.panos.sctp.chunks", val)?;
                        }
                    }
                    if let Some(val) = record.get(56) {
                        if !val.is_empty() {
                            event.set("panw.panos.sctp.chunks_sent", val)?;
                        }
                    }
                    if let Some(val) = record.get(57) {
                        if !val.is_empty() {
                            event.set("panw.panos.sctp.chunks_received", val)?;
                        }
                    }
                    if let Some(val) = record.get(58) {
                        if !val.is_empty() {
                            event.set("panw.panos.rule_uuid", val)?;
                        }
                    }
                    if let Some(val) = record.get(59) {
                        if !val.is_empty() {
                            event.set("panw.panos.http2_connection", val)?;
                        }
                    }
                    if let Some(val) = record.get(60) {
                        if !val.is_empty() {
                            event.set("panw.panos.link.change_count", val)?;
                        }
                    }
                    if let Some(val) = record.get(61) {
                        if !val.is_empty() {
                            event.set("panw.panos.policy.id", val)?;
                        }
                    }
                    if let Some(val) = record.get(62) {
                        if !val.is_empty() {
                            event.set("panw.panos.link.switches", val)?;
                        }
                    }
                    if let Some(val) = record.get(63) {
                        if !val.is_empty() {
                            event.set("panw.panos.sdwan.cluster.name", val)?;
                        }
                    }
                    if let Some(val) = record.get(64) {
                        if !val.is_empty() {
                            event.set("panw.panos.sdwan.device_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(65) {
                        if !val.is_empty() {
                            event.set("panw.panos.sdwan.cluster.type", val)?;
                        }
                    }
                    if let Some(val) = record.get(66) {
                        if !val.is_empty() {
                            event.set("panw.panos.sdwan.site", val)?;
                        }
                    }
                    if let Some(val) = record.get(67) {
                        if !val.is_empty() {
                            event.set("panw.panos.dynamic_user.group.name", val)?;
                        }
                    }
                    if let Some(val) = record.get(68) {
                        if !val.is_empty() {
                            event.set("panw.panos.xff.ip", val)?;
                        }
                    }
                    if let Some(val) = record.get(69) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.category", val)?;
                        }
                    }
                    if let Some(val) = record.get(70) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.profile", val)?;
                        }
                    }
                    if let Some(val) = record.get(71) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.model", val)?;
                        }
                    }
                    if let Some(val) = record.get(72) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.vendor", val)?;
                        }
                    }
                    if let Some(val) = record.get(73) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.os.family", val)?;
                        }
                    }
                    if let Some(val) = record.get(74) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.os.version", val)?;
                        }
                    }
                    if let Some(val) = record.get(75) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.host", val)?;
                        }
                    }
                    if let Some(val) = record.get(76) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.mac", val)?;
                        }
                    }
                    if let Some(val) = record.get(77) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.category", val)?;
                        }
                    }
                    if let Some(val) = record.get(78) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.profile", val)?;
                        }
                    }
                    if let Some(val) = record.get(79) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.model", val)?;
                        }
                    }
                    if let Some(val) = record.get(80) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.vendor", val)?;
                        }
                    }
                    if let Some(val) = record.get(81) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.os.family", val)?;
                        }
                    }
                    if let Some(val) = record.get(82) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.os.version", val)?;
                        }
                    }
                    if let Some(val) = record.get(83) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.host", val)?;
                        }
                    }
                    if let Some(val) = record.get(84) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.mac", val)?;
                        }
                    }
                    if let Some(val) = record.get(85) {
                        if !val.is_empty() {
                            event.set("panw.panos.container.id", val)?;
                        }
                    }
                    if let Some(val) = record.get(86) {
                        if !val.is_empty() {
                            event.set("panw.panos.pod.namespace", val)?;
                        }
                    }
                    if let Some(val) = record.get(87) {
                        if !val.is_empty() {
                            event.set("panw.panos.pod.name", val)?;
                        }
                    }
                    if let Some(val) = record.get(88) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.external_dynamic_list", val)?;
                        }
                    }
                    if let Some(val) = record.get(89) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.external_dynamic_list", val)?;
                        }
                    }
                    if let Some(val) = record.get(90) {
                        if !val.is_empty() {
                            event.set("panw.panos.host.id", val)?;
                        }
                    }
                    if let Some(val) = record.get(91) {
                        if !val.is_empty() {
                            event.set("panw.panos.serial_number", val)?;
                        }
                    }
                    if let Some(val) = record.get(92) {
                        if !val.is_empty() {
                            event.set("panw.panos.src.dynamic_address_group", val)?;
                        }
                    }
                    if let Some(val) = record.get(93) {
                        if !val.is_empty() {
                            event.set("panw.panos.dst.dynamic_address_group", val)?;
                        }
                    }
                    if let Some(val) = record.get(94) {
                        if !val.is_empty() {
                            event.set("panw.panos.session.owner", val)?;
                        }
                    }
                    if let Some(val) = record.get(95) {
                        if !val.is_empty() {
                            event.set("_temp_.high_res_timestamp", val)?;
                        }
                    }
                    if let Some(val) = record.get(96) {
                        if !val.is_empty() {
                            event.set("panw.panos.nsdsai_sst", val)?;
                        }
                    }
                    if let Some(val) = record.get(97) {
                        if !val.is_empty() {
                            event.set("panw.panos.nsdsai_sd", val)?;
                        }
                    }
                    if let Some(val) = record.get(98) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.sub_category", val)?;
                        }
                    }
                    if let Some(val) = record.get(99) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.category", val)?;
                        }
                    }
                    if let Some(val) = record.get(100) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.technology", val)?;
                        }
                    }
                    if let Some(val) = record.get(101) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.risk_level", val)?;
                        }
                    }
                    if let Some(val) = record.get(102) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.characteristics", val)?;
                        }
                    }
                    if let Some(val) = record.get(103) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.container", val)?;
                        }
                    }
                    if let Some(val) = record.get(104) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.tunneled", val)?;
                        }
                    }
                    if let Some(val) = record.get(105) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.is_saas", val)?;
                        }
                    }
                    if let Some(val) = record.get(106) {
                        if !val.is_empty() {
                            event.set("panw.panos.application.is_sanctioned", val)?;
                        }
                    }
                    if let Some(val) = record.get(107) {
                        if !val.is_empty() {
                            event.set("panw.panos.is_offloaded", val)?;
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("panw.panos.http2_connection")
                    && event.get_i64("panw.panos.http2_connection") != Some(0)
                    && event.get_str("panw.panos.http2_connection") != Some("0")
            };
            if _cond {
                event.set("http.version", json!("2"))?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            let _cond = { event.get_str("panw.panos.action") == Some("allow") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                !event.has_value("event.outcome")
                    || event.get_str("event.outcome").is_none_or(|s| s.is_empty())
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if let Some(v) = event
                .get("panw.panos.bytes_received")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.bytes", v)?;
            }

            if let Some(v) = event
                .get("_temp_.dstloc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("panw.panos.destination.location", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.destination.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.destination.nat.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.ip", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.destination.nat.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.port", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.packets_received")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.packets", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.elapsed_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.duration", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.network.application")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.application", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.network.bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.bytes", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.network.packets")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.packets", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.outbound_interface")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.egress.interface.name", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.destination.zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.egress.zone", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.device_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.inbound_interface")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.ingress.interface.name", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.source.zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.ingress.zone", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.rule_uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.uuid", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.bytes_sent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.bytes", v)?;
            }

            if let Some(v) = event
                .get("_temp_.srcloc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("panw.panos.source.location", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.source.nat.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.ip", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.packets_sent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.packets", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if let Some(v) = event
                .get("panw.panos.source.nat.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.port", v)?;
            }

            if let Some(v) = event
                .get("_conf.external_zones")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("_temp_.external_zones", v)?;
            }

            if let Some(v) = event
                .get("_conf.internal_zones")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("_temp_.internal_zones", v)?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = {
                event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("internal"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("external"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && ((!(event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })) && !(event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    }))) || (!(event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })) && !(event.get("_temp_.internal_zones").is_some_and(
                        |v| match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        },
                    ))))
            };
            if _cond {
                event.set("network.direction", json!("unknown"))?;
            }

            let _cond = {
                !event.has_value("event.timezone")
                    && event.has_value("panw.panos.parent_session.start_time")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("panw.panos.parent_session.start_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("panw.panos.parent_session.start_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_panw_panos_parent_session_start_time_to_panw_panos_parent_session_start_time_809881d3")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("event.timezone")
                    && event.has_value("panw.panos.parent_session.start_time")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("panw.panos.parent_session.start_time")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("panw.panos.parent_session.start_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_panw_panos_parent_session_start_time_to_panw_panos_parent_session_start_time_65e492d1")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
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
