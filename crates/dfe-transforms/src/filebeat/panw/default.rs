// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.17.0"))?;

            event.set("observer.vendor", json!("Palo Alto Networks"))?;

            event.set("observer.product", json!("PAN-OS"))?;

            event.set("observer.type", json!("firewall"))?;

            let _cond = {
                event.get("_conf.tz_offset").is_some_and(|v| v.is_string())
                    && !(event
                        .get_str("_conf.tz_offset")
                        .is_some_and(|s| s.eq_ignore_ascii_case("local")))
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    event.set("event.timezone", v)?;
                }
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if let Some(v) = event.get("message").cloned() {
                    event.set("event.original", v)?;
                }
            }

            event.rename("message", "_temp_.message")?;

            if let Some(input) = event.get_string("_temp_.message") {
                // Grok pattern: ^%{DATA},(?P<_temp__received_time>(?:(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})|%{TIMESTAMP_ISO8601})),(?P<observer_serial_number>(?:[^,]*)),(?P<panw_panos_type>(?:[^,]*)),(?:(?P<panw_panos_sub_type>(?:[^,]*)))?,(?P<_temp__config_version>(?:[^,]*)),(?P<_temp__generated_time>(?:(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})|%{TIMESTAMP_ISO8601})),%{GREEDYDATA:message}$
                // Grok pattern: ^(?:<\\d+>)?%{SYSLOGTIMESTAMP:_temp_.syslog_time} %{IPORHOST:observer.hostname} %{NOTSPACE:observer.serial_number},(?P<_temp__generated_time>(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})),(?P<panw_panos_type>(?:[^,]*)),%{GREEDYDATA:message}$
                if !extract_first_match(
                    &[
                        cached_grok_mapped!(
                            "^%{DATA},(?P<_temp__received_time>(?:(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})|%{TIMESTAMP_ISO8601})),(?P<observer_serial_number>(?:[^,]*)),(?P<panw_panos_type>(?:[^,]*)),(?:(?P<panw_panos_sub_type>(?:[^,]*)))?,(?P<_temp__config_version>(?:[^,]*)),(?P<_temp__generated_time>(?:(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})|%{TIMESTAMP_ISO8601})),%{GREEDYDATA:message}$",
                            [
                                ("_temp__received_time", "_temp_.received_time"),
                                ("observer_serial_number", "observer.serial_number"),
                                ("panw_panos_type", "panw.panos.type"),
                                ("panw_panos_sub_type", "panw.panos.sub_type"),
                                ("_temp__config_version", "_temp_.config_version"),
                                ("_temp__generated_time", "_temp_.generated_time")
                            ]
                        ),
                        cached_grok_mapped!(
                            "^(?:<\\d+>)?%{SYSLOGTIMESTAMP:_temp_.syslog_time} %{IPORHOST:observer.hostname} %{NOTSPACE:observer.serial_number},(?P<_temp__generated_time>(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})),(?P<panw_panos_type>(?:[^,]*)),%{GREEDYDATA:message}$",
                            [
                                ("_temp__generated_time", "_temp_.generated_time"),
                                ("panw_panos_type", "panw.panos.type")
                            ]
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = { event.get_str("panw.panos.type") == Some("TRAFFIC") };
            if _cond {
                // Begin nested pipeline: "traffic"
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
                    !event.has_value("event.outcome") || event.get_str("event.outcome") == Some("")
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
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })
                        && event.get("_temp_.internal_zones").is_some_and(|v| {
                            match (v, event.get("observer.egress.zone")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
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
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })
                        && event.get("_temp_.internal_zones").is_some_and(|v| {
                            match (v, event.get("observer.ingress.zone")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
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
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })
                        && event.get("_temp_.internal_zones").is_some_and(|v| {
                            match (v, event.get("observer.ingress.zone")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
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
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })
                        && event.get("_temp_.external_zones").is_some_and(|v| {
                            match (v, event.get("observer.ingress.zone")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
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
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })) && !(event.get("_temp_.internal_zones").is_some_and(|v| match (
                            v,
                            event.get("observer.egress.zone"),
                        ) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }))) || (!(event.get("_temp_.external_zones").is_some_and(
                            |v| match (v, event.get("observer.ingress.zone")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            },
                        )) && !(event.get("_temp_.internal_zones").is_some_and(
                            |v| match (v, event.get("observer.ingress.zone")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
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
                            match parse_date_out(
                                &date_str,
                                &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("panw.panos.parent_session.start_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.parent_session.start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
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
                                    .map_or_else(String::new, template_to_string)
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
                            match parse_date_out(
                                &date_str,
                                &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("panw.panos.parent_session.start_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.parent_session.start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
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
                                    .map_or_else(String::new, template_to_string)
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
                // End nested pipeline: "traffic"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("THREAT") };
            if _cond {
                // Begin nested pipeline: "threat"
                let _cond = { event.get_str("panw.panos.sub_type") == Some("url") };
                if _cond {
                    // Painless script
                    // Source: def fixHttpHeadersEscaping(String input) {\n  // Find a CSV fragment like `,Some-Header:\"eg1.com, eg2.com\";`\n  //     and correct it to be `,\"Some-Header:\"\"eg1.com, eg2.com\"\";\"`\n  Matcher matcher = /,(([A-Za-z0-9\\-_]+: *\\\"[^\\\"]*\\\"; *)+)/.matcher(input);\n  if (matcher.find()) {\n    String match = matcher.group(0);\n    String value = matcher.group(1);\n    String fixed = ',\"' + value.replace('\"', '\"\"') + '\"';\n    return input.replace(match, fixed);\n  } else {\n    return input;\n  }\n}\nctx.message = fixHttpHeadersEscaping(ctx.message);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def fixHttpHeadersEscaping(String input) {\n  // Find a CSV fragment like `,Some-Header:\"eg1.com, eg2.com\";`\n  //     and correct it to be `,\"Some-Header:\"\"eg1.com, eg2.com\"\";\"`\n  Matcher matcher = /,(([A-Za-z0-9\\-_]+: *\\\"[^\\\"]*\\\"; *)+)/.matcher(input);\n  if (matcher.find()) {\n    String match = matcher.group(0);\n    String value = matcher.group(1);\n    String fixed = ',\"' + value.replace('\"', '\"\"') + '\"';\n    return input.replace(match, fixed);\n  } else {\n    return input;\n  }\n}\nctx.message = fixHttpHeadersEscaping(ctx.message);\n"#
                        ),
                    )?;
                }
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
                                event.set("_temp_.logged_time", val)?;
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
                                event.set("panw.panos.misc", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("panw.panos.threat.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("panw.panos.url.category", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("panw.panos.severity", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("_temp_.direction", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("panw.panos.action_flags", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("_temp_.srcloc", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("_temp_.dstloc", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use2", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("panw.panos.http_content_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set("panw.panos.network.pcap_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("panw.panos.file.hash", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event.set("panw.panos.wildfire.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event.set("panw.panos.url_idx", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event.set("_temp_.user_agent", val)?;
                            }
                        }
                        if let Some(val) = record.get(40) {
                            if !val.is_empty() {
                                event.set("panw.panos.file.type", val)?;
                            }
                        }
                        if let Some(val) = record.get(41) {
                            if !val.is_empty() {
                                event.set("_temp_.forwarded_ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(42) {
                            if !val.is_empty() {
                                event.set("panw.panos.referrer", val)?;
                            }
                        }
                        if let Some(val) = record.get(43) {
                            if !val.is_empty() {
                                event.set("panw.panos.sender", val)?;
                            }
                        }
                        if let Some(val) = record.get(44) {
                            if !val.is_empty() {
                                event.set("panw.panos.subject", val)?;
                            }
                        }
                        if let Some(val) = record.get(45) {
                            if !val.is_empty() {
                                event.set("panw.panos.recipient", val)?;
                            }
                        }
                        if let Some(val) = record.get(46) {
                            if !val.is_empty() {
                                event.set("panw.panos.wildfire.report_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(47) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(48) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(49) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(50) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(51) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(52) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(53) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use3", val)?;
                            }
                        }
                        if let Some(val) = record.get(54) {
                            if !val.is_empty() {
                                event.set("panw.panos.source_vm_uuid", val)?;
                            }
                        }
                        if let Some(val) = record.get(55) {
                            if !val.is_empty() {
                                event.set("panw.panos.destination_vm_uuid", val)?;
                            }
                        }
                        if let Some(val) = record.get(56) {
                            if !val.is_empty() {
                                event.set("panw.panos.http_method", val)?;
                            }
                        }
                        if let Some(val) = record.get(57) {
                            if !val.is_empty() {
                                event.set("panw.panos.imsi", val)?;
                            }
                        }
                        if let Some(val) = record.get(58) {
                            if !val.is_empty() {
                                event.set("panw.panos.imei", val)?;
                            }
                        }
                        if let Some(val) = record.get(59) {
                            if !val.is_empty() {
                                event.set("panw.panos.parent_session.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(60) {
                            if !val.is_empty() {
                                event.set("panw.panos.parent_session.start_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(61) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(62) {
                            if !val.is_empty() {
                                event.set("panw.panos.threat_category", val)?;
                            }
                        }
                        if let Some(val) = record.get(63) {
                            if !val.is_empty() {
                                event.set("panw.panos.content_version", val)?;
                            }
                        }
                        if let Some(val) = record.get(64) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use4", val)?;
                            }
                        }
                        if let Some(val) = record.get(65) {
                            if !val.is_empty() {
                                event.set("panw.panos.sctp.assoc_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(66) {
                            if !val.is_empty() {
                                event.set("panw.panos.payload_protocol_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(67) {
                            if !val.is_empty() {
                                event.set("panw.panos.http_headers", val)?;
                            }
                        }
                        if let Some(val) = record.get(68) {
                            if !val.is_empty() {
                                event.set("panw.panos.url_category_list", val)?;
                            }
                        }
                        if let Some(val) = record.get(69) {
                            if !val.is_empty() {
                                event.set("panw.panos.rule_uuid", val)?;
                            }
                        }
                        if let Some(val) = record.get(70) {
                            if !val.is_empty() {
                                event.set("panw.panos.http2_connection", val)?;
                            }
                        }
                        if let Some(val) = record.get(71) {
                            if !val.is_empty() {
                                event.set("panw.panos.dynamic_user.group.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(72) {
                            if !val.is_empty() {
                                event.set("panw.panos.xff.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(73) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.category", val)?;
                            }
                        }
                        if let Some(val) = record.get(74) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.profile", val)?;
                            }
                        }
                        if let Some(val) = record.get(75) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.model", val)?;
                            }
                        }
                        if let Some(val) = record.get(76) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.vendor", val)?;
                            }
                        }
                        if let Some(val) = record.get(77) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.os.family", val)?;
                            }
                        }
                        if let Some(val) = record.get(78) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.os.version", val)?;
                            }
                        }
                        if let Some(val) = record.get(79) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.host", val)?;
                            }
                        }
                        if let Some(val) = record.get(80) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.mac", val)?;
                            }
                        }
                        if let Some(val) = record.get(81) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.category", val)?;
                            }
                        }
                        if let Some(val) = record.get(82) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.profile", val)?;
                            }
                        }
                        if let Some(val) = record.get(83) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.model", val)?;
                            }
                        }
                        if let Some(val) = record.get(84) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.vendor", val)?;
                            }
                        }
                        if let Some(val) = record.get(85) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.os.family", val)?;
                            }
                        }
                        if let Some(val) = record.get(86) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.os.version", val)?;
                            }
                        }
                        if let Some(val) = record.get(87) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.host", val)?;
                            }
                        }
                        if let Some(val) = record.get(88) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.mac", val)?;
                            }
                        }
                        if let Some(val) = record.get(89) {
                            if !val.is_empty() {
                                event.set("panw.panos.container.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(90) {
                            if !val.is_empty() {
                                event.set("panw.panos.pod.namespace", val)?;
                            }
                        }
                        if let Some(val) = record.get(91) {
                            if !val.is_empty() {
                                event.set("panw.panos.pod.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(92) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.external_dynamic_list", val)?;
                            }
                        }
                        if let Some(val) = record.get(93) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.external_dynamic_list", val)?;
                            }
                        }
                        if let Some(val) = record.get(94) {
                            if !val.is_empty() {
                                event.set("panw.panos.host.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(95) {
                            if !val.is_empty() {
                                event.set("panw.panos.serial_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(96) {
                            if !val.is_empty() {
                                event.set("panw.panos.domain_edl", val)?;
                            }
                        }
                        if let Some(val) = record.get(97) {
                            if !val.is_empty() {
                                event.set("panw.panos.src.dynamic_address_group", val)?;
                            }
                        }
                        if let Some(val) = record.get(98) {
                            if !val.is_empty() {
                                event.set("panw.panos.dst.dynamic_address_group", val)?;
                            }
                        }
                        if let Some(val) = record.get(99) {
                            if !val.is_empty() {
                                event.set("panw.panos.partial_hash", val)?;
                            }
                        }
                        if let Some(val) = record.get(100) {
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                        if let Some(val) = record.get(101) {
                            if !val.is_empty() {
                                event.set("panw.panos.reason", val)?;
                            }
                        }
                        if let Some(val) = record.get(102) {
                            if !val.is_empty() {
                                event.set("panw.panos.justification", val)?;
                            }
                        }
                        if let Some(val) = record.get(103) {
                            if !val.is_empty() {
                                event.set("panw.panos.nssai_sst", val)?;
                            }
                        }
                        if let Some(val) = record.get(104) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.sub_category", val)?;
                            }
                        }
                        if let Some(val) = record.get(105) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.category", val)?;
                            }
                        }
                        if let Some(val) = record.get(106) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.technology", val)?;
                            }
                        }
                        if let Some(val) = record.get(107) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.risk_level", val)?;
                            }
                        }
                        if let Some(val) = record.get(108) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.characteristics", val)?;
                            }
                        }
                        if let Some(val) = record.get(109) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.container", val)?;
                            }
                        }
                        if let Some(val) = record.get(110) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.tunneled", val)?;
                            }
                        }
                        if let Some(val) = record.get(111) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.is_saas", val)?;
                            }
                        }
                        if let Some(val) = record.get(112) {
                            if !val.is_empty() {
                                event.set("panw.panos.application.is_sanctioned", val)?;
                            }
                        }
                        if let Some(val) = record.get(113) {
                            if !val.is_empty() {
                                event.set("panw.panos.cloud_report.id", val)?;
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
                let _cond = {
                    event.get_str("_temp_.direction") == Some("0")
                        || event.get_str("_temp_.direction") == Some("client-to-server")
                };
                if _cond {
                    event.set("network.direction", json!("inbound"))?;
                }
                let _cond = {
                    event.get_str("_temp_.direction") == Some("1")
                        || event.get_str("_temp_.direction") == Some("server-to-client")
                };
                if _cond {
                    event.set("network.direction", json!("outbound"))?;
                }
                let _cond = { !event.has_value("network.direction") };
                if _cond {
                    event.set("network.direction", json!("unknown"))?;
                }
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("intrusion_detection"))?;
                event.append("event.category", json!("threat"))?;
                event.append("event.category", json!("network"))?;
                let _cond = {
                    event.has_value("panw.panos.action")
                        && ["alert", "allow", "continue"]
                            .contains(&event.get_str("panw.panos.action").unwrap_or(""))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    !event.has_value("event.outcome") || event.get_str("event.outcome") == Some("")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("_temp_.forwarded_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "_temp_.forwarded_ip".into(),
                                message,
                            }
                        })?;
                        event.set("network.forwarded_ip", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert__temp__forwarded_ip_to_network_forwarded_ip_e6722dcc",
                    )?;
                    if event.has_value("_temp_.forwarded_ip") {
                        event.rename("_temp_.forwarded_ip", "panw.panos.x_forwarded_for")?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
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
                    .get("panw.panos.destination.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.port", v)?;
                }
                if let Some(v) = event
                    .get("panw.panos.recipient")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.user.email", v)?;
                }
                if let Some(v) = event
                    .get("panw.panos.file.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.type", v)?;
                }
                if let Some(v) = event
                    .get("panw.panos.http_method")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.method", v)?;
                }
                if let Some(v) = event
                    .get("panw.panos.referrer")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.referrer", v)?;
                }
                if let Some(v) = event
                    .get("panw.panos.severity")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("log.level", v)?;
                }
                if let Some(v) = event
                    .get("panw.panos.network.application")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.application", v)?;
                }
                if let Some(v) = event
                    .get("_temp_.direction")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("panw.panos.network.direction", v)?;
                }
                if let Some(v) = event
                    .get("network.forwarded_ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("panw.panos.forwarded_ip", v)?;
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
                    .get("panw.panos.sender")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user.email", v)?;
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("url")
                        && event.get("panw.panos.misc").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event.get("panw.panos.misc").cloned() {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("virus")
                        && event.get("panw.panos.misc").is_some_and(|v| v.is_string())
                        && event.get("panw.panos.misc").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("/"))
                            }
                            serde_json::Value::String(s) => s.contains("/"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event.get("panw.panos.misc").cloned() {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("vulnerability")
                        && event.get("panw.panos.misc").is_some_and(|v| v.is_string())
                        && event.get("panw.panos.misc").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("/"))
                            }
                            serde_json::Value::String(s) => s.contains("/"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event.get("panw.panos.misc").cloned() {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("file")
                        && event
                            .get("_temp_.future_use3")
                            .is_some_and(|v| v.is_string())
                        && event.get("_temp_.future_use3").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("/"))
                            }
                            serde_json::Value::String(s) => s.contains("/"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event.get("_temp_.future_use3").cloned() {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = {
                    event.get_str("panw.panos.threat_category") == Some("domain-edl")
                        && event.get("panw.panos.misc").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event.get("panw.panos.misc").cloned() {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("spyware")
                        && event.get_str("panw.panos.protocol") == Some("tcp")
                        && ["block-url", "drop", "sinkhole"]
                            .contains(&event.get_str("panw.panos.action").unwrap_or(""))
                        && event.get("panw.panos.misc").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event.get("panw.panos.misc").cloned() {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("spyware")
                        && event.get_str("panw.panos.protocol") == Some("udp")
                        && ["sinkhole", "drop", "drop-packet"]
                            .contains(&event.get_str("panw.panos.action").unwrap_or(""))
                        && event.get("panw.panos.misc").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event.get("panw.panos.misc").cloned() {
                        event.set("url.original", v)?;
                    }
                }
                let _cond = { event.has_value("url.original") };
                if _cond {
                    // Painless script
                    // Source: Map url = new HashMap();\nString url_original = ctx.url.original;\nString domainPort = url_original;\nurl.original = url_original;\n\nif (url_original.contains(\"/\")) {\n    int idxSlash = url_original.indexOf(\"/\");\n    domainPort = url_original.substring(0, idxSlash);\n    String afterDomain = url_original.substring(idxSlash);\n    int idxQuery = afterDomain.indexOf(\"?\");\n    if (idxQuery == -1) {\n        url.path = afterDomain;\n    }\n    else {\n        url.path = afterDomain.substring(0, idxQuery);\n        url.query = afterDomain.substring(idxQuery + 1);\n    }\n    int idxExtn = url.path.lastIndexOf(\".\");\n    if (idxExtn != -1) {\n        url.extension = url.path.substring(idxExtn+1);\n    }\n}\nelse {\n    int idxQuery = url_original.indexOf(\"?\");\n    if (idxQuery != -1) {\n        domainPort = url_original.substring(0, idxQuery);\n        url.query = url_original.substring(idxQuery + 1);\n    }\n}\n\nif (domainPort.indexOf(\":\") != -1) {\n    url.domain = domainPort.splitOnToken(\":\")[0];\n    try {\n        url.port = Long.parseLong(domainPort.splitOnToken(\":\")[1]);\n    } catch ( NumberFormatException e) {\n    }\n}\nelse {\n    url.domain = domainPort;\n    ctx.destination.domain = domainPort;\n}\n\nctx.url = url;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"Map url = new HashMap();\nString url_original = ctx.url.original;\nString domainPort = url_original;\nurl.original = url_original;\n\nif (url_original.contains(\"/\")) {\n    int idxSlash = url_original.indexOf(\"/\");\n    domainPort = url_original.substring(0, idxSlash);\n    String afterDomain = url_original.substring(idxSlash);\n    int idxQuery = afterDomain.indexOf(\"?\");\n    if (idxQuery == -1) {\n        url.path = afterDomain;\n    }\n    else {\n        url.path = afterDomain.substring(0, idxQuery);\n        url.query = afterDomain.substring(idxQuery + 1);\n    }\n    int idxExtn = url.path.lastIndexOf(\".\");\n    if (idxExtn != -1) {\n        url.extension = url.path.substring(idxExtn+1);\n    }\n}\nelse {\n    int idxQuery = url_original.indexOf(\"?\");\n    if (idxQuery != -1) {\n        domainPort = url_original.substring(0, idxQuery);\n        url.query = url_original.substring(idxQuery + 1);\n    }\n}\n\nif (domainPort.indexOf(\":\") != -1) {\n    url.domain = domainPort.splitOnToken(\":\")[0];\n    try {\n        url.port = Long.parseLong(domainPort.splitOnToken(\":\")[1]);\n    } catch ( NumberFormatException e) {\n    }\n}\nelse {\n    url.domain = domainPort;\n    ctx.destination.domain = domainPort;\n}\n\nctx.url = url;"#
                        ),
                    )?;
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("file")
                        && (event.get("panw.panos.misc").is_some_and(|v| v.is_string()))
                        && (event.get("panw.panos.misc").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("/"))
                            }
                            serde_json::Value::String(s) => s.contains("/"),
                            _ => false,
                        }) || event.get("panw.panos.misc").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("panw.panos.misc")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.path", v)?;
                    }
                }
                let _cond = { event.get("file.path").is_some_and(|v| v.is_string()) };
                if _cond {
                    // Painless script
                    // Source: // For file.path to be set it must have had one of '/' or '\\' present.\nint idx = ctx.file.path.lastIndexOf('/');\nif (idx == -1) {\n  idx = ctx.file.path.lastIndexOf('\\\\');\n}\nctx.file[\"name\"] = ctx.file.path.substring(idx+1);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// For file.path to be set it must have had one of '/' or '\\' present.\nint idx = ctx.file.path.lastIndexOf('/');\nif (idx == -1) {\n  idx = ctx.file.path.lastIndexOf('\\\\');\n}\nctx.file[\"name\"] = ctx.file.path.substring(idx+1);"#
                        ),
                    )?;
                }
                let _cond = {
                    [
                        "file",
                        "virus",
                        "vulnerability",
                        "wildfire",
                        "wildfire-virus",
                    ]
                    .contains(&event.get_str("panw.panos.sub_type").unwrap_or(""))
                        && (event.get("panw.panos.misc").is_some_and(|v| v.is_string()))
                        && !(event.get("panw.panos.misc").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("/"))
                            }
                            serde_json::Value::String(s) => s.contains("/"),
                            _ => false,
                        }) || event.get("panw.panos.misc").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("panw.panos.misc")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.name", v)?;
                    }
                }
                let _cond = {
                    event.get_str("panw.panos.sub_type") == Some("vulnerability")
                        && event.has_value("file.name")
                };
                if _cond {
                    event.remove("url");
                }
                if let Some(v) = event
                    .get("_temp_.user_agent")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("panw.panos.user_agent", v)?;
                }
                let _cond = { event.has_value("panw.panos.url_category_list") };
                if _cond {
                    map_strings(
                        event,
                        "panw.panos.url_category_list",
                        "panw.panos.url_category_list",
                        |s| s.trim().to_string(),
                    )?;
                }
                let _cond = { event.has_value("panw.panos.url_category_list") };
                if _cond {
                    if let Some(s) = event.get_string("panw.panos.url_category_list") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("panw.panos.url_category_list", Value::Array(parts))?;
                    }
                }
                // End nested pipeline: "threat"
            }

            let _cond = {
                event.get_str("panw.panos.type") == Some("HIPMATCH")
                    || event.get_str("panw.panos.type") == Some("HIP-MATCH")
            };
            if _cond {
                // Begin nested pipeline: "hipmatch"
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
                                event.set("_temp_.srcuser", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("panw.panos.virtual_sys", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("panw.panos.machine.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("panw.panos.machine.os", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("panw.panos.matchname", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("panw.panos.repeat_count", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("panw.panos.matchtype", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use3", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("_temp_.future_use4", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("panw.panos.action_flags", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("_temp_.source_ipv6", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("panw.panos.host.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("panw.panos.serial_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("panw.panos.machine.mac_address", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.source.ip").cloned() {
                        event.set("source.ip", v)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("_temp_.source_ipv6")
                        && event.get_str("_temp_.source_ipv6") != Some("")
                        && event.get_str("_temp_.source_ipv6") != Some("0.0.0.0")
                };
                if _cond {
                    event.set(
                        "source.ip",
                        json!(
                            event
                                .get("_temp_.source_ipv6")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.host.id").cloned() {
                        event.set("host.id", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.machine.mac_address").cloned() {
                        event.set("host.mac", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("panw.panos.machine.name") };
                if _cond {
                    map_strings(
                        event,
                        "panw.panos.machine.name",
                        "host.name",
                        str::to_lowercase,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.machine.os").cloned() {
                        event.set("host.os.full", v)?;
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
                    if let Some(v) = event.get("_temp_.source_ipv6").cloned() {
                        event.set("panw.panos.source.ipv6", v)?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "hipmatch"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("USERID") };
            if _cond {
                // Begin nested pipeline: "userid"
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
                                event.set("panw.panos.virtual_sys", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("_temp_.srcuser", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("panw.panos.datasourcename", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("panw.panos.event.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("panw.panos.repeat_count", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("panw.panos.timeout", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("panw.panos.source.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("panw.panos.destination.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("panw.panos.datasource", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("panw.panos.datasourcetype", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("panw.panos.action_flags", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("panw.panos.factortype", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("panw.panos.factorcompletiontime", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("panw.panos.factorno", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("panw.panos.ugflags", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("panw.panos.user_by_source", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("panw.panos.tag.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                    }
                }
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                event.append("event.category", json!("iam"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.destination.port").cloned() {
                        event.set("destination.port", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.event.id").cloned() {
                        event.set("event.code", v)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.user_by_source").cloned() {
                        event.set("source.user.name", v)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("panw.panos.factorcompletiontime")
                        && !event.has_value("event.timezone")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw.panos.factorcompletiontime")
                        {
                            match parse_date_out(&date_str, &["yyyy/MM/dd HH:mm:ss"], None, None) {
                                Some(parsed) => {
                                    event.set("panw.panos.factorcompletiontime", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.factorcompletiontime".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_panw_panos_factorcompletiontime_to_panw_panos_factorcompletiontime_527c776e")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
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
                    event.has_value("panw.panos.factorcompletiontime")
                        && event.has_value("event.timezone")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw.panos.factorcompletiontime")
                        {
                            match parse_date_out(
                                &date_str,
                                &["yyyy/MM/dd HH:mm:ss"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("panw.panos.factorcompletiontime", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.factorcompletiontime".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_panw_panos_factorcompletiontime_to_panw_panos_factorcompletiontime_02fdb26c")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
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
                // End nested pipeline: "userid"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("GLOBALPROTECT") };
            if _cond {
                // Begin nested pipeline: "globalprotect"
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
                                event.set("panw.panos.virtual_sys", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("panw.panos.event.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("panw.panos.stage", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("panw.panos.auth_method", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("panw.panos.tunnel_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("_temp_.srcuser", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("_temp_.srcloc", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("panw.panos.machine.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("panw.panos.public.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("_temp_.public_ipv6", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("panw.panos.private.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("_temp_.private_ipv6", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("panw.panos.host.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("panw.panos.serial_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("panw.panos.client_ver", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("panw.panos.client.os", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("panw.panos.client.os_version", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("panw.panos.repeat_count", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("panw.panos.event.reason", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("panw.panos.error_message", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("panw.panos.description", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("panw.panos.event.status", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("panw.panos.location", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("panw.panos.login_duration", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("panw.panos.connect_method", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("panw.panos.error_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("panw.panos.portal", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("panw.panos.action_flags", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("panw.panos.selection_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("panw.panos.response_time", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("panw.panos.priority", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("panw.panos.attempted_gateways", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("panw.panos.gateway", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(40) {
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(41) {
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_id", val)?;
                            }
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.private.ip").cloned() {
                        event.set("source.ip", v)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    (!event.has_value("source.ip") || event.get_str("source.ip") == Some("0.0.0.0"))
                        && event.has_value("_temp_.private_ipv6")
                        && event.get_str("_temp_.private_ipv6") != Some("0.0.0.0")
                };
                if _cond {
                    event.set(
                        "source.ip",
                        json!(
                            event
                                .get("_temp_.private_ipv6")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "host.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.public.ip").cloned() {
                        event.set("source.nat.ip", v)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    (!event.has_value("source.nat.ip")
                        || event.get_str("source.nat.ip") == Some("0.0.0.0"))
                        && event.has_value("_temp_.public_ipv6")
                        && event.get_str("_temp_.public_ipv6") != Some("0.0.0.0")
                };
                if _cond {
                    event.set(
                        "source.nat.ip",
                        json!(
                            event
                                .get("_temp_.public_ipv6")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.event.id").cloned() {
                        event.set("event.code", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.login_duration").cloned() {
                        event.set("event.duration", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.event.status").cloned() {
                        event.set("event.outcome", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.event.reason").cloned() {
                        event.set("event.reason", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.host.id").cloned() {
                        event.set("host.id", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("panw.panos.machine.name") };
                if _cond {
                    map_strings(
                        event,
                        "panw.panos.machine.name",
                        "host.name",
                        str::to_lowercase,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.client.os").cloned() {
                        event.set("host.os.family", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.client.os_version").cloned() {
                        event.set("host.os.full", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.location").cloned() {
                        event.set("observer.geo.name", v)?;
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
                    if let Some(v) = event.get("source.geo.name").cloned() {
                        event.set("panw.panos.source.region", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("_temp_.public_ipv6").cloned() {
                        event.set("panw.panos.public.ipv6", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("_temp_.private_ipv6").cloned() {
                        event.set("panw.panos.private.ipv6", v)?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "globalprotect"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("CONFIG") };
            if _cond {
                // Begin nested pipeline: "config"
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
                                event.set("panw.panos.host.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("panw.panos.virtual_sys", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("panw.panos.cmd", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("panw.panos.admin", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("panw.panos.client_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("panw.panos.result", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("panw.panos.path", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("_temp_.check_field", val)?;
                            }
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp_.check_field") {
                        if let Some(val) = event.get("_temp_.check_field") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_temp_.check_field".into(),
                                    message,
                                }
                            })?;
                            event.set("_temp_.check_field", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    !(event
                        .get("_temp_.check_field")
                        .is_some_and(|v| v.is_number()))
                };
                if _cond {
                    if let Some(csv_str) = event.get_string("message") {
                        let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                        let mut rdr = csv::ReaderBuilder::new()
                            .delimiter(b',')
                            .quote(b'\"')
                            .has_headers(false)
                            .from_reader(csv_str.as_bytes());
                        if let Some(Ok(record)) = rdr.records().next() {
                            if let Some(val) = record.get(0) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.host.ip", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.virtual_sys", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.cmd", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.admin", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.client_type", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.result", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.path", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.before_change_detail", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.after_change_detail", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.sequence_number", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.action_flags", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy1", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy2", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy3", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy4", val)?;
                                }
                            }
                            if let Some(val) = record.get(15) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(16) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(17) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(18) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.comment", val)?;
                                }
                            }
                            if let Some(val) = record.get(19) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("_temp_.future_use1", val)?;
                                }
                            }
                            if let Some(val) = record.get(20) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("_temp_.high_res_timestamp", val)?;
                                }
                            }
                        }
                    }
                }
                let _cond = {
                    event
                        .get("_temp_.check_field")
                        .is_some_and(|v| v.is_number())
                };
                if _cond {
                    if let Some(csv_str) = event.get_string("message") {
                        let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                        let mut rdr = csv::ReaderBuilder::new()
                            .delimiter(b',')
                            .quote(b'\"')
                            .has_headers(false)
                            .from_reader(csv_str.as_bytes());
                        if let Some(Ok(record)) = rdr.records().next() {
                            if let Some(val) = record.get(0) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.host.ip", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.virtual_sys", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.cmd", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.admin", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.client_type", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.result", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.path", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.sequence_number", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.action_flags", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy1", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy2", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy3", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy4", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(15) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(16) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("panw.panos.comment", val)?;
                                }
                            }
                            if let Some(val) = record.get(17) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("_temp_.future_use1", val)?;
                                }
                            }
                            if let Some(val) = record.get(18) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("_temp_.high_res_timestamp", val)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("panw.panos.cmd") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script
                        // Source: ctx.event.action = params.get(ctx.panw.panos.cmd);
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"ctx.event.action = params.get(ctx.panw.panos.cmd);"#
                            ),
                            cached_params!(
                                "{\"add\":\"cmd-add\",\"clone\":\"cmd-clone\",\"commit\":\"cmd-commit\",\"delete\":\"cmd-delete\",\"edit\":\"cmd-edit\",\"move\":\"cmd-move\",\"rename\":\"cmd-rename\",\"set\":\"cmd-set\"}"
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("panw.panos.result") == Some("Succeeded") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("panw.panos.result") == Some("Failed") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("failure"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    !(["Succeeded", "Failed"]
                        .contains(&event.get_str("panw.panos.action").unwrap_or("")))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("unknown"))?;
                        Ok(())
                    })();
                }
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("configuration"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.host.ip").cloned() {
                        event.set("host.ip", v)?;
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
                let _cond = { event.get_str("panw.panos.result") == Some("Succeeded") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("panw.panos.result") == Some("Failed") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    !event.has_value("event.outcome") || event.get_str("event.outcome") == Some("")
                };
                if _cond {
                    event.set("event.outcome", json!("unknown"))?;
                }
                // End nested pipeline: "config"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("DECRYPTION") };
            if _cond {
                // Begin nested pipeline: "decryption"
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
                                    event.set("_temp_.logged_time", val)?;
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
                                    event.set("panw.panos.tunnel_type", val)?;
                                }
                            }
                            if let Some(val) = record.get(25) {
                                if !val.is_empty() {
                                    event.set("_temp_.future_use1", val)?;
                                }
                            }
                            if let Some(val) = record.get(26) {
                                if !val.is_empty() {
                                    event.set("_temp_.future_use2", val)?;
                                }
                            }
                            if let Some(val) = record.get(27) {
                                if !val.is_empty() {
                                    event.set("panw.panos.source_vm_uuid", val)?;
                                }
                            }
                            if let Some(val) = record.get(28) {
                                if !val.is_empty() {
                                    event.set("panw.panos.destination_vm_uuid", val)?;
                                }
                            }
                            if let Some(val) = record.get(29) {
                                if !val.is_empty() {
                                    event.set("panw.panos.rule_uuid", val)?;
                                }
                            }
                            if let Some(val) = record.get(30) {
                                if !val.is_empty() {
                                    event.set("panw.panos.hs_stage_c2f", val)?;
                                }
                            }
                            if let Some(val) = record.get(31) {
                                if !val.is_empty() {
                                    event.set("panw.panos.hs_stage_f2s", val)?;
                                }
                            }
                            if let Some(val) = record.get(32) {
                                if !val.is_empty() {
                                    event.set("_temp_.tls", val)?;
                                }
                            }
                            if let Some(val) = record.get(33) {
                                if !val.is_empty() {
                                    event.set("panw.panos.tls.key_exchange_algorithm", val)?;
                                }
                            }
                            if let Some(val) = record.get(34) {
                                if !val.is_empty() {
                                    event.set("panw.panos.tls.encryption", val)?;
                                }
                            }
                            if let Some(val) = record.get(35) {
                                if !val.is_empty() {
                                    event.set("panw.panos.tls.auth", val)?;
                                }
                            }
                            if let Some(val) = record.get(36) {
                                if !val.is_empty() {
                                    event.set("panw.panos.policy.name", val)?;
                                }
                            }
                            if let Some(val) = record.get(37) {
                                if !val.is_empty() {
                                    event.set("panw.panos.elliptic_curve", val)?;
                                }
                            }
                            if let Some(val) = record.get(38) {
                                if !val.is_empty() {
                                    event.set("panw.panos.tls.error_type", val)?;
                                }
                            }
                            if let Some(val) = record.get(39) {
                                if !val.is_empty() {
                                    event.set("panw.panos.root_certificate_status", val)?;
                                }
                            }
                            if let Some(val) = record.get(40) {
                                if !val.is_empty() {
                                    event.set("panw.panos.chain_status", val)?;
                                }
                            }
                            if let Some(val) = record.get(41) {
                                if !val.is_empty() {
                                    event.set("panw.panos.proxy_type", val)?;
                                }
                            }
                            if let Some(val) = record.get(42) {
                                if !val.is_empty() {
                                    event.set("panw.panos.certificate.serial_number", val)?;
                                }
                            }
                            if let Some(val) = record.get(43) {
                                if !val.is_empty() {
                                    event.set("_temp_.hash", val)?;
                                }
                            }
                            if let Some(val) = record.get(44) {
                                if !val.is_empty() {
                                    event.set("panw.panos.certificate.not_before", val)?;
                                }
                            }
                            if let Some(val) = record.get(45) {
                                if !val.is_empty() {
                                    event.set("panw.panos.certificate.not_after", val)?;
                                }
                            }
                            if let Some(val) = record.get(46) {
                                if !val.is_empty() {
                                    event.set("panw.panos.certificate.version", val)?;
                                }
                            }
                            if let Some(val) = record.get(47) {
                                if !val.is_empty() {
                                    event.set("panw.panos.certificate.size", val)?;
                                }
                            }
                            if let Some(val) = record.get(48) {
                                if !val.is_empty() {
                                    event.set("panw.panos.subject_common_name.length", val)?;
                                }
                            }
                            if let Some(val) = record.get(49) {
                                if !val.is_empty() {
                                    event.set("panw.panos.issuer_common_name.length", val)?;
                                }
                            }
                            if let Some(val) = record.get(50) {
                                if !val.is_empty() {
                                    event.set("panw.panos.root_common_name.length", val)?;
                                }
                            }
                            if let Some(val) = record.get(51) {
                                if !val.is_empty() {
                                    event.set("panw.panos.server_name_indication.length", val)?;
                                }
                            }
                            if let Some(val) = record.get(52) {
                                if !val.is_empty() {
                                    event.set("panw.panos.certificate.flags", val)?;
                                }
                            }
                            if let Some(val) = record.get(53) {
                                if !val.is_empty() {
                                    event.set("panw.panos.subject_common_name.value", val)?;
                                }
                            }
                            if let Some(val) = record.get(54) {
                                if !val.is_empty() {
                                    event.set("panw.panos.issuer_common_name.value", val)?;
                                }
                            }
                            if let Some(val) = record.get(55) {
                                if !val.is_empty() {
                                    event.set("panw.panos.root_common_name.value", val)?;
                                }
                            }
                            if let Some(val) = record.get(56) {
                                if !val.is_empty() {
                                    event.set("panw.panos.server_name_indication.value", val)?;
                                }
                            }
                            if let Some(val) = record.get(57) {
                                if !val.is_empty() {
                                    event.set("panw.panos.error_message", val)?;
                                }
                            }
                            if let Some(val) = record.get(58) {
                                if !val.is_empty() {
                                    event.set("panw.panos.container.id", val)?;
                                }
                            }
                            if let Some(val) = record.get(59) {
                                if !val.is_empty() {
                                    event.set("panw.panos.pod.namespace", val)?;
                                }
                            }
                            if let Some(val) = record.get(60) {
                                if !val.is_empty() {
                                    event.set("panw.panos.pod.name", val)?;
                                }
                            }
                            if let Some(val) = record.get(61) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.external_dynamic_list", val)?;
                                }
                            }
                            if let Some(val) = record.get(62) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.external_dynamic_list", val)?;
                                }
                            }
                            if let Some(val) = record.get(63) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.dynamic_address_group", val)?;
                                }
                            }
                            if let Some(val) = record.get(64) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.dynamic_address_group", val)?;
                                }
                            }
                            if let Some(val) = record.get(65) {
                                if !val.is_empty() {
                                    event.set("_temp_.high_res_timestamp", val)?;
                                }
                            }
                            if let Some(val) = record.get(66) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.category", val)?;
                                }
                            }
                            if let Some(val) = record.get(67) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.profile", val)?;
                                }
                            }
                            if let Some(val) = record.get(68) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.model", val)?;
                                }
                            }
                            if let Some(val) = record.get(69) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.vendor", val)?;
                                }
                            }
                            if let Some(val) = record.get(70) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.os.family", val)?;
                                }
                            }
                            if let Some(val) = record.get(71) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.os.version", val)?;
                                }
                            }
                            if let Some(val) = record.get(72) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.host", val)?;
                                }
                            }
                            if let Some(val) = record.get(73) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.mac", val)?;
                                }
                            }
                            if let Some(val) = record.get(74) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.category", val)?;
                                }
                            }
                            if let Some(val) = record.get(75) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.profile", val)?;
                                }
                            }
                            if let Some(val) = record.get(76) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.model", val)?;
                                }
                            }
                            if let Some(val) = record.get(77) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.vendor", val)?;
                                }
                            }
                            if let Some(val) = record.get(78) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.os.family", val)?;
                                }
                            }
                            if let Some(val) = record.get(79) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.os.version", val)?;
                                }
                            }
                            if let Some(val) = record.get(80) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.host", val)?;
                                }
                            }
                            if let Some(val) = record.get(81) {
                                if !val.is_empty() {
                                    event.set("panw.panos.dst.mac", val)?;
                                }
                            }
                            if let Some(val) = record.get(82) {
                                if !val.is_empty() {
                                    event.set("panw.panos.sequence_number", val)?;
                                }
                            }
                            if let Some(val) = record.get(83) {
                                if !val.is_empty() {
                                    event.set("panw.panos.action_flags", val)?;
                                }
                            }
                            if let Some(val) = record.get(84) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy1", val)?;
                                }
                            }
                            if let Some(val) = record.get(85) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy2", val)?;
                                }
                            }
                            if let Some(val) = record.get(86) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy3", val)?;
                                }
                            }
                            if let Some(val) = record.get(87) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy4", val)?;
                                }
                            }
                            if let Some(val) = record.get(88) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(89) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(90) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(91) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.sub_category", val)?;
                                }
                            }
                            if let Some(val) = record.get(92) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.category", val)?;
                                }
                            }
                            if let Some(val) = record.get(93) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.technology", val)?;
                                }
                            }
                            if let Some(val) = record.get(94) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.risk_level", val)?;
                                }
                            }
                            if let Some(val) = record.get(95) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.characteristics", val)?;
                                }
                            }
                            if let Some(val) = record.get(96) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.container", val)?;
                                }
                            }
                            if let Some(val) = record.get(97) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.tunneled", val)?;
                                }
                            }
                            if let Some(val) = record.get(98) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.is_saas", val)?;
                                }
                            }
                            if let Some(val) = record.get(99) {
                                if !val.is_empty() {
                                    event.set("panw.panos.application.is_sanctioned", val)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("_temp_.config_version") {
                    event.rename("_temp_.config_version", "panw.panos.config_version")?;
                }
                let _cond = {
                    !event.has_value("event.timezone")
                        && event.has_value("panw.panos.certificate.not_after")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw.panos.certificate.not_after")
                        {
                            match parse_date_out(
                                &date_str,
                                &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("panw.panos.certificate.not_after", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.certificate.not_after".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_panw_panos_certificate_not_after_to_panw_panos_certificate_not_after_02479cef")?;
                        if event.remove("panw.panos.certificate.not_after").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "panw.panos.certificate.not_after".into(),
                            });
                        }
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
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
                        && event.has_value("panw.panos.certificate.not_after")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw.panos.certificate.not_after")
                        {
                            match parse_date_out(
                                &date_str,
                                &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("panw.panos.certificate.not_after", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.certificate.not_after".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_panw_panos_certificate_not_after_to_panw_panos_certificate_not_after_cfa3d515")?;
                        if event.remove("panw.panos.certificate.not_after").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "panw.panos.certificate.not_after".into(),
                            });
                        }
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
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
                    !event.has_value("event.timezone")
                        && event.has_value("panw.panos.certificate.not_before")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw.panos.certificate.not_before")
                        {
                            match parse_date_out(
                                &date_str,
                                &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                                None,
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("panw.panos.certificate.not_before", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.certificate.not_before".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_panw_panos_certificate_not_before_to_panw_panos_certificate_not_before_fe9abb71")?;
                        if event.remove("panw.panos.certificate.not_before").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "panw.panos.certificate.not_before".into(),
                            });
                        }
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
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
                        && event.has_value("panw.panos.certificate.not_before")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw.panos.certificate.not_before")
                        {
                            match parse_date_out(
                                &date_str,
                                &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => {
                                    event.set("panw.panos.certificate.not_before", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw.panos.certificate.not_before".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_panw_panos_certificate_not_before_to_panw_panos_certificate_not_before_a788166b")?;
                        if event.remove("panw.panos.certificate.not_before").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "panw.panos.certificate.not_before".into(),
                            });
                        }
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
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
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                let _cond = {
                    !event.has_value("panw.panos.error_message")
                        || event.get_str("panw.panos.error_message") == Some("")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("panw.panos.error_message") != Some("") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("_temp_.hash").cloned() {
                        event.set("panw.panos.hash", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("_temp_.tls").cloned() {
                        event.set("panw.panos.tls.version", v)?;
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
                    if let Some(v) = event.get("panw.panos.destination.port").cloned() {
                        event.set("destination.port", v)?;
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
                    if let Some(v) = event.get("panw.panos.subject_common_name.value").cloned() {
                        event.set("tls.client.x509.subject.common_name", v)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event
                        .get("tls.client.x509.subject.common_name")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.client.x509.subject.common_name",
                        Value::Array(vec![json!(
                            event
                                .get("tls.client.x509.subject.common_name")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.issuer_common_name.value").cloned() {
                        event.set("tls.client.x509.issuer.common_name", v)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event
                        .get("tls.client.x509.issuer.common_name")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "tls.client.x509.issuer.common_name",
                        Value::Array(vec![json!(
                            event
                                .get("tls.client.x509.issuer.common_name")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
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
                    if let Some(v) = event.get("panw.panos.tls.encryption").cloned() {
                        event.set("tls.cipher", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.certificate.not_after").cloned() {
                        event.set("tls.client.not_after", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.certificate.not_before").cloned() {
                        event.set("tls.client.not_before", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("panw.panos.server_name_indication.value")
                        .cloned()
                    {
                        event.set("tls.client.server_name", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.certificate.size").cloned() {
                        event.set("tls.client.x509.public_key_size", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.certificate.serial_number").cloned() {
                        event.set("tls.client.x509.serial_number", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.certificate.version").cloned() {
                        event.set("tls.client.x509.version_number", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.elliptic_curve").cloned() {
                        event.set("tls.curve", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_temp_.hash") };
                if _cond {
                    // Painless script
                    // Source: ctx.tls.client.hash = new HashMap();\nif (ctx._temp_.hash.length() == 32) {ctx.tls.client.hash.md5 = ctx._temp_.hash}\nelse if (ctx._temp_.hash.length() == 40) {ctx.tls.client.hash.sha1 = ctx._temp_.hash}\nelse if (ctx._temp_.hash.length() == 64) {ctx.tls.client.hash.sha256 = ctx._temp_.hash}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.tls.client.hash = new HashMap();\nif (ctx._temp_.hash.length() == 32) {ctx.tls.client.hash.md5 = ctx._temp_.hash}\nelse if (ctx._temp_.hash.length() == 40) {ctx.tls.client.hash.sha1 = ctx._temp_.hash}\nelse if (ctx._temp_.hash.length() == 64) {ctx.tls.client.hash.sha256 = ctx._temp_.hash}\n"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("_temp_.tls") };
                if _cond {
                    // Painless script
                    // Source: ctx.tls.version = new HashMap();\nctx.tls.version_protocol = ctx._temp_?.tls.substring(0,3).toLowerCase();\nctx.tls.version = ctx._temp_?.tls.substring(3,6);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.tls.version = new HashMap();\nctx.tls.version_protocol = ctx._temp_?.tls.substring(0,3).toLowerCase();\nctx.tls.version = ctx._temp_?.tls.substring(3,6);\n"#
                        ),
                    )?;
                }
                // End nested pipeline: "decryption"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("SYSTEM") };
            if _cond {
                // Begin nested pipeline: "system"
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
                                    event.set("panw.panos.virtual_sys", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                if !val.is_empty() {
                                    event.set("panw.panos.event.id", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("panw.panos.object.id", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("_temp_.future_use1", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("_temp_.future_use2", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                if !val.is_empty() {
                                    event.set("panw.panos.module", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                if !val.is_empty() {
                                    event.set("panw.panos.severity", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                if !val.is_empty() {
                                    event.set("panw.panos.description", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                if !val.is_empty() {
                                    event.set("panw.panos.sequence_number", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                if !val.is_empty() {
                                    event.set("panw.panos.action_flags", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy1", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy2", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy3", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy4", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(15) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(16) {
                                if !val.is_empty() {
                                    event.set("_temp_.future_use3", val)?;
                                }
                            }
                            if let Some(val) = record.get(17) {
                                if !val.is_empty() {
                                    event.set("_temp_.future_use4", val)?;
                                }
                            }
                            if let Some(val) = record.get(18) {
                                if !val.is_empty() {
                                    event.set("_temp_.high_res_timestamp", val)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("configuration"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.event.id").cloned() {
                        event.set("event.code", v)?;
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
                    if let Some(v) = event.get("panw.panos.device_name").cloned() {
                        event.set("observer.hostname", v)?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "system"
            }

            let _cond = {
                event.get_str("panw.panos.type") == Some("AUTHENTICATION")
                    || event.get_str("panw.panos.type") == Some("AUTH")
            };
            if _cond {
                // Begin nested pipeline: "authentication"
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
                                    event.set("panw.panos.virtual_sys", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                if !val.is_empty() {
                                    event.set("panw.panos.source.ip", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("_temp_.user", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("panw.panos.normalize_user", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("panw.panos.object.id", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                if !val.is_empty() {
                                    event.set("panw.panos.authentication.policy", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                if !val.is_empty() {
                                    event.set("panw.panos.repeat_count", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                if !val.is_empty() {
                                    event.set("panw.panos.authentication.id", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vendor", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                if !val.is_empty() {
                                    event.set("panw.panos.log_profile", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                if !val.is_empty() {
                                    event.set("panw.panos.server_profile", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                if !val.is_empty() {
                                    event.set("panw.panos.description", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                if !val.is_empty() {
                                    event.set("panw.panos.client_type", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                if !val.is_empty() {
                                    event.set("panw.panos.event.result", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                if !val.is_empty() {
                                    event.set("panw.panos.factorno", val)?;
                                }
                            }
                            if let Some(val) = record.get(15) {
                                if !val.is_empty() {
                                    event.set("panw.panos.sequence_number", val)?;
                                }
                            }
                            if let Some(val) = record.get(16) {
                                if !val.is_empty() {
                                    event.set("panw.panos.action_flags", val)?;
                                }
                            }
                            if let Some(val) = record.get(17) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy1", val)?;
                                }
                            }
                            if let Some(val) = record.get(18) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy2", val)?;
                                }
                            }
                            if let Some(val) = record.get(19) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy3", val)?;
                                }
                            }
                            if let Some(val) = record.get(20) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy4", val)?;
                                }
                            }
                            if let Some(val) = record.get(21) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(22) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(23) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(24) {
                                if !val.is_empty() {
                                    event.set("panw.panos.authentication.protocol", val)?;
                                }
                            }
                            if let Some(val) = record.get(25) {
                                if !val.is_empty() {
                                    event.set("panw.panos.rule_uuid", val)?;
                                }
                            }
                            if let Some(val) = record.get(26) {
                                if !val.is_empty() {
                                    event.set("_temp_.high_res_timestamp", val)?;
                                }
                            }
                            if let Some(val) = record.get(27) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.category", val)?;
                                }
                            }
                            if let Some(val) = record.get(28) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.profile", val)?;
                                }
                            }
                            if let Some(val) = record.get(29) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.model", val)?;
                                }
                            }
                            if let Some(val) = record.get(30) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.vendor", val)?;
                                }
                            }
                            if let Some(val) = record.get(31) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.os.family", val)?;
                                }
                            }
                            if let Some(val) = record.get(32) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.os.version", val)?;
                                }
                            }
                            if let Some(val) = record.get(33) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.host", val)?;
                                }
                            }
                            if let Some(val) = record.get(34) {
                                if !val.is_empty() {
                                    event.set("panw.panos.src.mac", val)?;
                                }
                            }
                            if let Some(val) = record.get(35) {
                                if !val.is_empty() {
                                    event.set("panw.panos.region", val)?;
                                }
                            }
                            if let Some(val) = record.get(36) {
                                if !val.is_empty() {
                                    event.set("_temp_.future_use3", val)?;
                                }
                            }
                            if let Some(val) = record.get(37) {
                                if !val.is_empty() {
                                    event.set("_temp_.user_agent", val)?;
                                }
                            }
                            if let Some(val) = record.get(38) {
                                if !val.is_empty() {
                                    event.set("panw.panos.flow_id", val)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond =
                    { event.has_value("_temp_.user") && event.get_str("_temp_.user") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "source.user.name",
                            json!(
                                event
                                    .get("_temp_.user")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("panw.panos.normalize_user")
                        && event.get_str("panw.panos.normalize_user") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "source.user.name",
                            json!(
                                event
                                    .get("panw.panos.normalize_user")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("authentication"))?;
                event.set("event.outcome", json!("success"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.device_name").cloned() {
                        event.set("observer.hostname", v)?;
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
                    if let Some(v) = event.get("_temp_.user").cloned() {
                        event.set("panw.panos.user", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("_temp_.user_agent").cloned() {
                        event.set("panw.panos.user_agent", v)?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "authentication"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("CORRELATION") };
            if _cond {
                // Begin nested pipeline: "correlated_event"
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
                                    event.set("_temp_.srcuser", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("panw.panos.virtual_sys", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("panw.panos.category", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("panw.panos.severity", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy1", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy2", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy3", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy4", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                if !val.is_empty() {
                                    event.set("panw.panos.object.name", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                if !val.is_empty() {
                                    event.set("panw.panos.object.id", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                if !val.is_empty() {
                                    event.set("panw.panos.evidence", val)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                event.set("event.outcome", json!("success"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.severity").cloned() {
                        event.set("log.level", v)?;
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
                    if let Some(v) = event.get("panw.panos.source.ip").cloned() {
                        event.set("source.ip", v)?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "correlated_event"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("GTP") };
            if _cond {
                // Begin nested pipeline: "gtp"
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
                // End nested pipeline: "gtp"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("IPTAG") };
            if _cond {
                // Begin nested pipeline: "ip_tag"
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
                                    event.set("panw.panos.virtual_sys", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                if !val.is_empty() {
                                    event.set("panw.panos.source.ip", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("panw.panos.tag.name", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("panw.panos.event.id", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("panw.panos.repeat_count", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                if !val.is_empty() {
                                    event.set("panw.panos.timeout", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                if !val.is_empty() {
                                    event.set("panw.panos.datasourcename", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                if !val.is_empty() {
                                    event.set("panw.panos.datasource_type", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                if !val.is_empty() {
                                    event.set("panw.panos.datasource_subtype", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                if !val.is_empty() {
                                    event.set("panw.panos.sequence_number", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                if !val.is_empty() {
                                    event.set("panw.panos.action_flags", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy1", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy2", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy3", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_group_hierarchy4", val)?;
                                }
                            }
                            if let Some(val) = record.get(15) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(16) {
                                if !val.is_empty() {
                                    event.set("panw.panos.device_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(17) {
                                if !val.is_empty() {
                                    event.set("panw.panos.vsys_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(18) {
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
                    if let Some(v) = event.get("panw.panos.event.id").cloned() {
                        event.set("event.code", v)?;
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
                    if let Some(v) = event.get("panw.panos.source.ip").cloned() {
                        event.set("source.ip", v)?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "ip_tag"
            }

            let _cond = { event.get_str("panw.panos.type") == Some("SCTP") };
            if _cond {
                // Begin nested pipeline: "sctp"
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
                // End nested pipeline: "sctp"
            }

            let _cond = {
                event.get_str("panw.panos.type") == Some("START")
                    || event.get_str("panw.panos.type") == Some("END")
            };
            if _cond {
                // Begin nested pipeline: "tunnel_inspection"
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
                let _cond = {
                    !event.has_value("event.outcome") || event.get_str("event.outcome") == Some("")
                };
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
                // End nested pipeline: "tunnel_inspection"
            }

            let _cond = {
                event.get_str("panw.panos.type") == Some("AUDIT")
                    || event.get_str("panw.panos.type") == Some("audit")
            };
            if _cond {
                // Begin nested pipeline: "audit"
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
                                    event.set("panw.panos.config_version", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                if !val.is_empty() {
                                    event.set("panw.panos.cmd_source", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                if !val.is_empty() {
                                    event.set("user.name", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                if !val.is_empty() {
                                    event.set("panw.panos.cmd", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                if !val.is_empty() {
                                    event.set("event.outcome", val)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("panw.panos.type") {
                    map_strings(
                        event,
                        "panw.panos.type",
                        "panw.panos.type",
                        str::to_uppercase,
                    )?;
                }
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("configuration"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("panw.panos.device_name").cloned() {
                        event.set("observer.hostname", v)?;
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
                // End nested pipeline: "audit"
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("observer.serial_number").cloned() {
                    event.set("panw.panos.observer.serial_number", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.generated_time") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.generated_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.generated_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__generated_time_to_panw_panos_generated_time_7759c4c7",
                    )?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.generated_time") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.generated_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.generated_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__generated_time_to_panw_panos_generated_time_7b45f5a5",
                    )?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond =
                { !event.has_value("event.timezone") && event.has_value("_temp_.received_time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.received_time") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.received_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.received_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__received_time_to_panw_panos_received_time_b4d2c73b",
                    )?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond =
                { event.has_value("event.timezone") && event.has_value("_temp_.received_time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.received_time") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.received_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.received_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__received_time_to_panw_panos_received_time_81c983b3",
                    )?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond =
                { !event.has_value("event.timezone") && event.has_value("_temp_.logged_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.logged_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.logged_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.logged_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__logged_time_to_panw_panos_logged_time_6a2e15d6",
                    )?;
                    if event.remove("_temp_.logged_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_temp_.logged_time".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond =
                { event.has_value("event.timezone") && event.has_value("_temp_.logged_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.logged_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.logged_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.logged_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__logged_time_to_panw_panos_logged_time_31e0a874",
                    )?;
                    if event.remove("_temp_.logged_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_temp_.logged_time".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                event
                    .get("_temp_.high_res_timestamp")
                    .is_some_and(|v| v.is_string())
                    && (event
                        .get_str("_temp_.high_res_timestamp")
                        .is_some_and(|s| s.starts_with("1969"))
                        || event
                            .get_str("_temp_.high_res_timestamp")
                            .is_some_and(|s| s.starts_with("1970")))
            };
            if _cond {
                event.remove("_temp_.high_res_timestamp");
            }

            let _cond = {
                !event.has_value("event.timezone") && event.has_value("_temp_.high_res_timestamp")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.high_res_timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("panw.panos.high_resolution_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.high_res_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date__temp__high_res_timestamp_to_panw_panos_high_resolution_timestamp_1766e392")?;
                    if event.remove("_temp_.high_res_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_temp_.high_res_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                event.has_value("event.timezone") && event.has_value("_temp_.high_res_timestamp")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.high_res_timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("panw.panos.high_resolution_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.high_res_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date__temp__high_res_timestamp_to_panw_panos_high_resolution_timestamp_46b86ff8")?;
                    if event.remove("_temp_.high_res_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_temp_.high_res_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            if let Some(v) = event
                .get("panw.panos.high_resolution_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                !event.has_value("panw.panos.high_resolution_timestamp")
                    && event.has_value("panw.panos.received_time")
            };
            if _cond {
                if let Some(v) = event.get("panw.panos.received_time").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { !event.has_value("event.timezone") && event.has_value("event.start") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.start") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_event_start_to_event_start_1687c744",
                    )?;
                    if event.remove("event.start").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "event.start".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond = { event.has_value("event.timezone") && event.has_value("event.start") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.start") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_event_start_to_event_start_047d728e",
                    )?;
                    if event.remove("event.start").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "event.start".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond =
                { !event.has_value("event.timezone") && event.has_value("panw.panos.start_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("panw.panos.start_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "panw.panos.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_panw_panos_start_time_to_panw_panos_start_time_d03e47cc",
                    )?;
                    if event.remove("panw.panos.start_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "panw.panos.start_time".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond =
                { event.has_value("event.timezone") && event.has_value("panw.panos.start_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("panw.panos.start_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("panw.panos.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "panw.panos.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_panw_panos_start_time_to_panw_panos_start_time_96aa3ce6",
                    )?;
                    if event.remove("panw.panos.start_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "panw.panos.start_time".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                !event.has_value("event.timezone")
                    && event.has_value("panw.panos.parent_session.start_time")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("panw.panos.parent_session.start_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("panw.panos.parent_session.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "panw.panos.parent_session.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_panw_panos_parent_session_start_time_to_panw_panos_parent_session_start_time_7f88322d")?;
                    if event
                        .remove("panw.panos.parent_session.start_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "panw.panos.parent_session.start_time".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("panw.panos.parent_session.start_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy/MM/dd HH:mm:ss",
                                "strict_date_optional_time_nanos",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("panw.panos.parent_session.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "panw.panos.parent_session.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_panw_panos_parent_session_start_time_to_panw_panos_parent_session_start_time_b57b547f")?;
                    if event
                        .remove("panw.panos.parent_session.start_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "panw.panos.parent_session.start_time".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond = { event.has_value("panw.panos.parent_session.start_time") };
            if _cond {
                if let Some(v) = event.get("panw.panos.parent_session.start_time").cloned() {
                    event.set("session.start_time", v)?;
                }
            }

            let _cond = {
                event.get_str("source.nat.ip") == Some("0.0.0.0")
                    && event.get_str("source.nat.port") == Some("0")
            };
            if _cond {
                if event.remove("source.nat.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.nat.ip".into(),
                    });
                }
                if event.remove("source.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.nat.port".into(),
                    });
                }
            }

            let _cond = {
                event.get_str("destination.nat.ip") == Some("0.0.0.0")
                    && event.get_str("destination.nat.port") == Some("0")
            };
            if _cond {
                if event.remove("destination.nat.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.nat.ip".into(),
                    });
                }
                if event.remove("destination.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.nat.port".into(),
                    });
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.ip") {
                    if let Some(val) = event.get("source.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_ip_66e8d43f",
                )?;
                if event.remove("source.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.ip") {
                    if let Some(val) = event.get("destination.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.ip".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_ip_bc8fdf9f",
                )?;
                if event.remove("destination.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.nat.ip") {
                    if let Some(val) = event.get("source.nat.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.nat.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.nat.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_nat_ip_36181efd",
                )?;
                if event.remove("source.nat.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.nat.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.nat.ip") {
                    if let Some(val) = event.get("destination.nat.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.nat.ip".into(),
                                message,
                            }
                        })?;
                        event.set("destination.nat.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_nat_ip_a1755483",
                )?;
                if event.remove("destination.nat.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.nat.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.ip") {
                    if let Some(val) = event.get("host.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "host.ip".into(),
                                message,
                            }
                        })?;
                        event.set("host.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_host_ip_67e7c965",
                )?;
                if event.remove("host.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "host.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "host.ip",
                    Value::Array(vec![json!(
                        event
                            .get("host.ip")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("network.forwarded_ip") {
                    if let Some(val) = event.get("network.forwarded_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "network.forwarded_ip".into(),
                                message,
                            }
                        })?;
                        event.set("network.forwarded_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_network_forwarded_ip_8bdee41f",
                )?;
                if event.remove("network.forwarded_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "network.forwarded_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.xff.ip") {
                    if let Some(val) = event.get("panw.panos.xff.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.xff.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.xff.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_xff_ip_29a861db",
                )?;
                if event.remove("panw.panos.xff.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.xff.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.source.ip") {
                    if let Some(val) = event.get("panw.panos.source.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.source.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_source_ip_ef8be7e7",
                )?;
                if event.remove("panw.panos.source.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.source.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.source.ipv6") {
                    if let Some(val) = event.get("panw.panos.source.ipv6") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.source.ipv6".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.source.ipv6", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_source_ipv6_ba18a7b3",
                )?;
                if event.remove("panw.panos.source.ipv6").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.source.ipv6".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.source.nat.ip") {
                    if let Some(val) = event.get("panw.panos.source.nat.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.source.nat.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.source.nat.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_source_nat_ip_21a2745b",
                )?;
                if event.remove("panw.panos.source.nat.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.source.nat.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.destination.ip") {
                    if let Some(val) = event.get("panw.panos.destination.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.destination.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.destination.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_destination_ip_440f1cb3",
                )?;
                if event.remove("panw.panos.destination.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.destination.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.destination.nat.ip") {
                    if let Some(val) = event.get("panw.panos.destination.nat.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.destination.nat.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.destination.nat.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_destination_nat_ip_c69c8cc5",
                )?;
                if event.remove("panw.panos.destination.nat.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.destination.nat.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.forwarded_ip") {
                    if let Some(val) = event.get("panw.panos.forwarded_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.forwarded_ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.forwarded_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_forwarded_ip_fd1736f5",
                )?;
                if event.remove("panw.panos.forwarded_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.forwarded_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.host.ip") {
                    if let Some(val) = event.get("panw.panos.host.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.host.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.host.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_host_ip_079468ff",
                )?;
                if event.remove("panw.panos.host.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.host.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.public.ip") {
                    if let Some(val) = event.get("panw.panos.public.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.public.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.public.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_public_ip_e622dfb3",
                )?;
                if event.remove("panw.panos.public.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.public.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.private.ip") {
                    if let Some(val) = event.get("panw.panos.private.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.private.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.private.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_private_ip_70bae4ad",
                )?;
                if event.remove("panw.panos.private.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.private.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.public.ipv6") {
                    if let Some(val) = event.get("panw.panos.public.ipv6") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.public.ipv6".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.public.ipv6", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_public_ipv6_a4cf5eef",
                )?;
                if event.remove("panw.panos.public.ipv6").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.public.ipv6".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.private.ipv6") {
                    if let Some(val) = event.get("panw.panos.private.ipv6") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.private.ipv6".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.private.ipv6", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_private_ipv6_51ac2039",
                )?;
                if event.remove("panw.panos.private.ipv6").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.private.ipv6".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.end_ip_address") {
                    if let Some(val) = event.get("panw.panos.end_ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.end_ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.end_ip_address", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_end_ip_address_7b4803b9",
                )?;
                if event.remove("panw.panos.end_ip_address").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.end_ip_address".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.remote_user.ip") {
                    if let Some(val) = event.get("panw.panos.remote_user.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.remote_user.ip".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.remote_user.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_remote_user_ip_538f2d3f",
                )?;
                if event.remove("panw.panos.remote_user.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.remote_user.ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.bytes") {
                    if let Some(val) = event.get("source.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("source.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_bytes_895571fa",
                )?;
                if event.remove("source.bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.bytes".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.packets") {
                    if let Some(val) = event.get("source.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.packets".into(),
                                message,
                            }
                        })?;
                        event.set("source.packets", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_packets_d535b112",
                )?;
                if event.remove("source.packets").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.packets".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.port") {
                    if let Some(val) = event.get("source.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_port_5c60e782",
                )?;
                if event.remove("source.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.source.port") {
                    if let Some(val) = event.get("panw.panos.source.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.source.port".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_source_port_036f5c12",
                )?;
                if event.remove("panw.panos.source.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.source.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.source.nat.port") {
                    if let Some(val) = event.get("panw.panos.source.nat.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.source.nat.port".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.source.nat.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_source_nat_port_b2b9abd6",
                )?;
                if event.remove("panw.panos.source.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.source.nat.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.bytes") {
                    if let Some(val) = event.get("destination.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("destination.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_bytes_0a242610",
                )?;
                if event.remove("destination.bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.bytes".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.packets") {
                    if let Some(val) = event.get("destination.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.packets".into(),
                                message,
                            }
                        })?;
                        event.set("destination.packets", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_packets_7d192c10",
                )?;
                if event.remove("destination.packets").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.packets".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.port") {
                    if let Some(val) = event.get("destination.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_port_171174d6",
                )?;
                if event.remove("destination.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.destination.port") {
                    if let Some(val) = event.get("panw.panos.destination.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.destination.port".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_destination_port_aaa32012",
                )?;
                if event.remove("panw.panos.destination.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.destination.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.destination.nat.port") {
                    if let Some(val) = event.get("panw.panos.destination.nat.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.destination.nat.port".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.destination.nat.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_destination_nat_port_f49d8810",
                )?;
                if event.remove("panw.panos.destination.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.destination.nat.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("network.bytes") {
                    if let Some(val) = event.get("network.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "network.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("network.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_network_bytes_137cfc8c",
                )?;
                if event.remove("network.bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "network.bytes".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("network.packets") {
                    if let Some(val) = event.get("network.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "network.packets".into(),
                                message,
                            }
                        })?;
                        event.set("network.packets", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_network_packets_f604bedc",
                )?;
                if event.remove("network.packets").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "network.packets".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("event.duration") {
                    if let Some(val) = event.get("event.duration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "event.duration".into(),
                                message,
                            }
                        })?;
                        event.set("event.duration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_duration_d0a307da",
                )?;
                if event.remove("event.duration").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "event.duration".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("_temp_.labels") {
                    if let Some(val) = event.get("_temp_.labels") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "_temp_.labels".into(),
                                message,
                            }
                        })?;
                        event.set("_temp_.labels", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert__temp__labels_e07bf37c",
                )?;
                if event.remove("_temp_.labels").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_temp_.labels".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.nat.port") {
                    if let Some(val) = event.get("source.nat.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.nat.port".into(),
                                message,
                            }
                        })?;
                        event.set("source.nat.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_nat_port_89a5f1c4",
                )?;
                if event.remove("source.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.nat.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.nat.port") {
                    if let Some(val) = event.get("destination.nat.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.nat.port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.nat.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_nat_port_2c88a98a",
                )?;
                if event.remove("destination.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.nat.port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.repeat_count") {
                    if let Some(val) = event.get("panw.panos.repeat_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.repeat_count".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.repeat_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_repeat_count_2ee4705a",
                )?;
                if event.remove("panw.panos.repeat_count").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.repeat_count".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.sctp.chunks") {
                    if let Some(val) = event.get("panw.panos.sctp.chunks") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.sctp.chunks".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.sctp.chunks", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_sctp_chunks_fb742a1a",
                )?;
                if event.remove("panw.panos.sctp.chunks").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.sctp.chunks".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.sctp.chunks_sent") {
                    if let Some(val) = event.get("panw.panos.sctp.chunks_sent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.sctp.chunks_sent".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.sctp.chunks_sent", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_sctp_chunks_sent_9407c142",
                )?;
                if event.remove("panw.panos.sctp.chunks_sent").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.sctp.chunks_sent".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.sctp.chunks_received") {
                    if let Some(val) = event.get("panw.panos.sctp.chunks_received") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.sctp.chunks_received".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.sctp.chunks_received", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_sctp_chunks_received_1072fedc",
                )?;
                if event.remove("panw.panos.sctp.chunks_received").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.sctp.chunks_received".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.network.bytes") {
                    if let Some(val) = event.get("panw.panos.network.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.network.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.network.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_network_bytes_230bf142",
                )?;
                if event.remove("panw.panos.network.bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.network.bytes".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.network.packets") {
                    if let Some(val) = event.get("panw.panos.network.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.network.packets".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.network.packets", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_network_packets_e69ffcf6",
                )?;
                if event.remove("panw.panos.network.packets").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.network.packets".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.bytes_sent") {
                    if let Some(val) = event.get("panw.panos.bytes_sent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.bytes_sent".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.bytes_sent", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_bytes_sent_0d384bc0",
                )?;
                if event.remove("panw.panos.bytes_sent").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.bytes_sent".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.packets_sent") {
                    if let Some(val) = event.get("panw.panos.packets_sent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.packets_sent".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.packets_sent", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_packets_sent_35a39ad0",
                )?;
                if event.remove("panw.panos.packets_sent").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.packets_sent".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.bytes_received") {
                    if let Some(val) = event.get("panw.panos.bytes_received") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.bytes_received".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.bytes_received", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_bytes_received_c5631a1e",
                )?;
                if event.remove("panw.panos.bytes_received").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.bytes_received".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.packets_received") {
                    if let Some(val) = event.get("panw.panos.packets_received") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.packets_received".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.packets_received", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_packets_received_36117ab2",
                )?;
                if event.remove("panw.panos.packets_received").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.packets_received".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.elapsed_time") {
                    if let Some(val) = event.get("panw.panos.elapsed_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.elapsed_time".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.elapsed_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_elapsed_time_4184f01c",
                )?;
                if event.remove("panw.panos.elapsed_time").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.elapsed_time".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.timeout") {
                    if let Some(val) = event.get("panw.panos.timeout") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.timeout".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.timeout", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_timeout_ee1fbe26",
                )?;
                if event.remove("panw.panos.timeout").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.timeout".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.error_code") {
                    if let Some(val) = event.get("panw.panos.error_code") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.error_code".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.error_code", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_error_code_2df2f9b4",
                )?;
                if event.remove("panw.panos.error_code").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.error_code".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.factorno") {
                    if let Some(val) = event.get("panw.panos.factorno") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.factorno".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.factorno", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_factorno_2e801b20",
                )?;
                if event.remove("panw.panos.factorno").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.factorno".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.certificate.size") {
                    if let Some(val) = event.get("panw.panos.certificate.size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.certificate.size".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.certificate.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_certificate_size_62afc8ad",
                )?;
                event.rename(
                    "panw.panos.certificate.size",
                    "panw.panos.certificate.raw_size",
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("tls.client.x509.public_key_size") {
                    if let Some(val) = event.get("tls.client.x509.public_key_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "tls.client.x509.public_key_size".into(),
                                message,
                            }
                        })?;
                        event.set("tls.client.x509.public_key_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_tls_client_x509_public_key_size_99d5cab1",
                )?;
                if event.remove("tls.client.x509.public_key_size").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "tls.client.x509.public_key_size".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.issuer_common_name.length") {
                    if let Some(val) = event.get("panw.panos.issuer_common_name.length") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.issuer_common_name.length".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.issuer_common_name.length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_issuer_common_name_length_dff31c66",
                )?;
                if event
                    .remove("panw.panos.issuer_common_name.length")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.issuer_common_name.length".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.root_common_name.length") {
                    if let Some(val) = event.get("panw.panos.root_common_name.length") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.root_common_name.length".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.root_common_name.length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_root_common_name_length_22db08f2",
                )?;
                if event.remove("panw.panos.root_common_name.length").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.root_common_name.length".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.subject_common_name.length") {
                    if let Some(val) = event.get("panw.panos.subject_common_name.length") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.subject_common_name.length".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.subject_common_name.length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_subject_common_name_length_d4151838",
                )?;
                if event
                    .remove("panw.panos.subject_common_name.length")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.subject_common_name.length".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.server_name_indication.length") {
                    if let Some(val) = event.get("panw.panos.server_name_indication.length") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.server_name_indication.length".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.server_name_indication.length", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_server_name_indication_length_c2866a8e",
                )?;
                if event
                    .remove("panw.panos.server_name_indication.length")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.server_name_indication.length".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.link.change_count") {
                    if let Some(val) = event.get("panw.panos.link.change_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.link.change_count".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.link.change_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_link_change_count_af93a36e",
                )?;
                if event.remove("panw.panos.link.change_count").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.link.change_count".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.login_duration") {
                    if let Some(val) = event.get("panw.panos.login_duration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.login_duration".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.login_duration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_login_duration_210eba48",
                )?;
                if event.remove("panw.panos.login_duration").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.login_duration".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.strict_check") {
                    if let Some(val) = event.get("panw.panos.strict_check") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.strict_check".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.strict_check", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_strict_check_02f0ed04",
                )?;
                if event.remove("panw.panos.strict_check").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.strict_check".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.tunnel_fragment") {
                    if let Some(val) = event.get("panw.panos.tunnel_fragment") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.tunnel_fragment".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.tunnel_fragment", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_tunnel_fragment_fdd66972",
                )?;
                if event.remove("panw.panos.tunnel_fragment").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.tunnel_fragment".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.unknown_protocol") {
                    if let Some(val) = event.get("panw.panos.unknown_protocol") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.unknown_protocol".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.unknown_protocol", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_unknown_protocol_8ce8f526",
                )?;
                if event.remove("panw.panos.unknown_protocol").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.unknown_protocol".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.sessions.closed") {
                    if let Some(val) = event.get("panw.panos.sessions.closed") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.sessions.closed".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.sessions.closed", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_sessions_closed_eacc3612",
                )?;
                if event.remove("panw.panos.sessions.closed").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.sessions.closed".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.sessions.created") {
                    if let Some(val) = event.get("panw.panos.sessions.created") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.sessions.created".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.sessions.created", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_sessions_created_90c7e452",
                )?;
                if event.remove("panw.panos.sessions.created").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.sessions.created".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.max_encapsulation") {
                    if let Some(val) = event.get("panw.panos.max_encapsulation") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.max_encapsulation".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.max_encapsulation", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_max_encapsulation_98d354ea",
                )?;
                if event.remove("panw.panos.max_encapsulation").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.max_encapsulation".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.application.risk_level") {
                    if let Some(val) = event.get("panw.panos.application.risk_level") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.application.risk_level".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.application.risk_level", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_application_risk_level_efb5ad08",
                )?;
                if event.remove("panw.panos.application.risk_level").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.application.risk_level".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("panw.panos.response_time") {
                    if let Some(val) = event.get("panw.panos.response_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw.panos.response_time".into(),
                                message,
                            }
                        })?;
                        event.set("panw.panos.response_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_panw_panos_response_time_4d887d4a",
                )?;
                if event.remove("panw.panos.response_time").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.response_time".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("panw.panos.src.mac") {
                gsub_field(
                    event,
                    "panw.panos.src.mac",
                    "panw.panos.src.mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("panw.panos.src.mac") {
                map_strings(
                    event,
                    "panw.panos.src.mac",
                    "panw.panos.src.mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("panw.panos.dst.mac") {
                gsub_field(
                    event,
                    "panw.panos.dst.mac",
                    "panw.panos.dst.mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("panw.panos.dst.mac") {
                map_strings(
                    event,
                    "panw.panos.dst.mac",
                    "panw.panos.dst.mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[:.]"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            if event.has_value("panw.panos.machine.mac_address") {
                gsub_field(
                    event,
                    "panw.panos.machine.mac_address",
                    "panw.panos.machine.mac_address",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("panw.panos.machine.mac_address") {
                map_strings(
                    event,
                    "panw.panos.machine.mac_address",
                    "panw.panos.machine.mac_address",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("network.application") {
                map_strings(
                    event,
                    "network.application",
                    "network.application",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("panw.panos.network.pcap_id") == Some("0") };
            if _cond {
                if event.remove("panw.panos.network.pcap_id").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "panw.panos.network.pcap_id".into(),
                    });
                }
            }

            let _cond =
                { event.has_value("_temp_.labels") && event.get_i64("_temp_.labels") != Some(0) };
            if _cond {
                // Painless script
                // Source: def labels = ctx.labels; if (labels == null) {\n  labels = new HashMap();\n  ctx['labels'] = labels;\n} long value = ctx._temp_.labels; for (entry in params.entrySet()) {\n  def flag = entry.getValue();\n  if (flag instanceof String) {\n      flag = Long.decode(flag);\n  }\n  if ((value & flag) != 0) {\n      labels[entry.getKey()] = true;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def labels = ctx.labels; if (labels == null) {\n  labels = new HashMap();\n  ctx['labels'] = labels;\n} long value = ctx._temp_.labels; for (entry in params.entrySet()) {\n  def flag = entry.getValue();\n  if (flag instanceof String) {\n      flag = Long.decode(flag);\n  }\n  if ((value & flag) != 0) {\n      labels[entry.getKey()] = true;\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"pcap_included\":2147483648,\"connect_to_destination_host\":1073741824,\"file_submitted_to_WildFire\":536870912,\"enterprise_credential_submission\":268435456,\"source_flow_allow_list\":134217728,\"ipv6_session\":33554432,\"ssl_decrypted\":16777216,\"url_filter_denied\":8388608,\"nat_translated\":4194304,\"captive_portal\":2097152,\"non_standard_port_usage\":1048576,\"x_forwarded_for\":524288,\"http_proxy\":262144,\"client_server_policy_based_forwarding\":131072,\"server_client_policy_based_forwarding\":65536,\"container_page\":32768,\"temporary_match\":8192,\"symmetric_return\":2048,\"decrypted_traffic\":1024,\"payload_outer_tunnel\":256}"
                    ),
                )?;
            }

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script
                // Source: long nanos = ctx['event']['duration'] * params.NANOS_IN_A_SECOND; ctx['event']['duration'] = nanos; def start = ctx.event?.start; if (start != null) {\n  ctx.event['end'] = ZonedDateTime.parse(start).plusNanos(nanos);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"long nanos = ctx['event']['duration'] * params.NANOS_IN_A_SECOND; ctx['event']['duration'] = nanos; def start = ctx.event?.start; if (start != null) {\n  ctx.event['end'] = ZonedDateTime.parse(start).plusNanos(nanos);\n}\n"#
                    ),
                    cached_params!("{\"NANOS_IN_A_SECOND\":1000000000}"),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.srcuser")
                    && event.has_value("_temp_.labels")
                    && event
                        .get_i64("_temp_.labels")
                        .is_some_and(|n| (n & 524288) != 0)
            };
            if _cond {
                event.rename("_temp_.srcuser", "panw.panos.x_forwarded_for")?;
            }

            if event.has_value("panw.panos.x_forwarded_for") {
                gsub_field(
                    event,
                    "panw.panos.x_forwarded_for",
                    "panw.panos.x_forwarded_for",
                    cached_regex!("x-fwd-for: "),
                    "",
                )?;
            }

            let _cond = { event.has_value("_temp_.srcuser") };
            if _cond {
                if event.has_value("_temp_.srcuser") {
                    if let Some(input) = event.get_string("_temp_.srcuser") {
                        // Grok pattern: ^(?P<source_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\(?P<source_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$
                        // Grok pattern: ^(?P<source_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\\\\\(?P<source_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$
                        // Grok pattern: ^(?P<source_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))@(?P<source_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))$
                        // Grok pattern: ^%{GREEDYDATA:source.user.name}$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^(?P<source_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\(?P<source_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$",
                                    [
                                        ("source_user_domain", "source.user.domain"),
                                        ("source_user_name", "source.user.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^(?P<source_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\\\\\(?P<source_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$",
                                    [
                                        ("source_user_domain", "source.user.domain"),
                                        ("source_user_name", "source.user.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^(?P<source_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))@(?P<source_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))$",
                                    [
                                        ("source_user_name", "source.user.name"),
                                        ("source_user_domain", "source.user.domain")
                                    ]
                                ),
                                cached_grok!("^%{GREEDYDATA:source.user.name}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = { event.has_value("_temp_.dstuser") };
            if _cond {
                if event.has_value("_temp_.dstuser") {
                    if let Some(input) = event.get_string("_temp_.dstuser") {
                        // Grok pattern: ^(?P<destination_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\(?P<destination_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$
                        // Grok pattern: ^(?P<destination_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\\\\\(?P<destination_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$
                        // Grok pattern: ^(?P<destination_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))@(?P<destination_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))$
                        // Grok pattern: ^%{GREEDYDATA:destination.user.name}$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^(?P<destination_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\(?P<destination_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$",
                                    [
                                        ("destination_user_domain", "destination.user.domain"),
                                        ("destination_user_name", "destination.user.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^(?P<destination_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))\\\\\\\\(?P<destination_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))$",
                                    [
                                        ("destination_user_domain", "destination.user.domain"),
                                        ("destination_user_name", "destination.user.name")
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "^(?P<destination_user_name>(?:[ a-zA-Z0-9#.:_'-]+[$]?))@(?P<destination_user_domain>(?:(?:\\.{0,1}|\\b(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62})(?:\\.{1,2}(?:[0-9A-Za-z_][0-9A-Za-z_\\-]{0,62}))*(\\.?|\\b))))$",
                                    [
                                        ("destination_user_name", "destination.user.name"),
                                        ("destination_user_domain", "destination.user.domain")
                                    ]
                                ),
                                cached_grok!("^%{GREEDYDATA:destination.user.name}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                if let Some(v) = event.get("source.user.name").cloned() {
                    event.set("panw.panos.source.user", v)?;
                }
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                if let Some(v) = event.get("destination.user.name").cloned() {
                    event.set("panw.panos.destination.user", v)?;
                }
            }

            let _cond = { event.has_value("source.user") };
            if _cond {
                if let Some(v) = event.get("source.user").cloned() {
                    event.set("user", v)?;
                }
            }

            let _cond = {
                event.has_value("panw.panos.action")
                    && ["alert", "allow", "continue"]
                        .contains(&event.get_str("panw.panos.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.has_value("panw.panos.action")
                    && [
                        "deny",
                        "drop",
                        "reset-client",
                        "reset-server",
                        "reset-both",
                        "block-url",
                        "block-ip",
                        "random-drop",
                        "sinkhole",
                        "block",
                    ]
                    .contains(&event.get_str("panw.panos.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("start") };
            if _cond {
                event.set("event.action", json!("flow_started"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("start") };
            if _cond {
                event.append("event.type", json!("start"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("end") };
            if _cond {
                event.set("event.action", json!("flow_terminated"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("end") };
            if _cond {
                event.append("event.type", json!("end"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("drop") };
            if _cond {
                event.set("event.action", json!("flow_dropped"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("drop") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
                event.append_unique("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("deny") };
            if _cond {
                event.set("event.action", json!("flow_denied"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("deny") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
                event.append_unique("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("data") };
            if _cond {
                event.set("event.action", json!("data_match"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("file") };
            if _cond {
                event.set("event.action", json!("file_match"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("flood") };
            if _cond {
                event.set("event.action", json!("flood_detected"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("packet") };
            if _cond {
                event.set("event.action", json!("packet_attack"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("scan") };
            if _cond {
                event.set("event.action", json!("scan_detected"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("spyware") };
            if _cond {
                event.set("event.action", json!("spyware_detected"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("url") };
            if _cond {
                event.set("event.action", json!("url_filtering"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("virus") };
            if _cond {
                event.set("event.action", json!("virus_detected"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("vulnerability") };
            if _cond {
                event.set("event.action", json!("exploit_detected"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("wildfire") };
            if _cond {
                event.set("event.action", json!("wildfire_verdict"))?;
            }

            let _cond = { event.get_str("panw.panos.sub_type") == Some("wildfire-virus") };
            if _cond {
                event.set("event.action", json!("wildfire_virus_detected"))?;
            }

            if event.has_value("log.level") {
                map_strings(event, "log.level", "log.level", str::to_lowercase)?;
            }

            let _cond = { event.get_str("log.level") == Some("critical") };
            if _cond {
                event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("log.level") == Some("high") };
            if _cond {
                event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("log.level") == Some("medium") };
            if _cond {
                event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("log.level") == Some("low") };
            if _cond {
                event.set("event.severity", json!(4))?;
            }

            let _cond = { event.get_str("log.level") == Some("informational") };
            if _cond {
                event.set("event.severity", json!(5))?;
            }

            let _cond = {
                event.get_str("panw.panos.action") == Some("drop icmp")
                    || event.get_str("panw.panos.action") == Some("drop ICMP")
            };
            if _cond {
                event.set("panw.panos.action", json!("drop-icmp"))?;
            }

            let _cond = { event.get_str("panw.panos.action") == Some("reset both") };
            if _cond {
                event.set("panw.panos.action", json!("reset-both"))?;
            }

            let _cond = { event.get_str("panw.panos.action") == Some("reset client") };
            if _cond {
                event.set("panw.panos.action", json!("reset-client"))?;
            }

            let _cond = { event.get_str("panw.panos.action") == Some("reset server") };
            if _cond {
                event.set("panw.panos.action", json!("reset-server"))?;
            }

            let _cond = {
                !event.has_value("network.type")
                    && event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                !event.has_value("network.type")
                    && event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("host.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "host.ip", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("panw.panos.xff.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("panw.panos.xff.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("network.forwarded_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("network.forwarded_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("panw.panos.remote_user.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("panw.panos.remote_user.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("panw.panos.end_ip_address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("panw.panos.end_ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("source.nat.ip") && !event.has_value("source.geo") };
            if _cond {
                if let Some(ip_str) = event.get_string("source.nat.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond =
                { event.has_value("destination.nat.ip") && !event.has_value("destination.geo") };
            if _cond {
                if let Some(ip_str) = event.get_string("destination.nat.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("_temp_.user_agent") {
                if let Some(ua_str) = event.get_string("_temp_.user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("source.nat.ip") && !event.has_value("source.as") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            let _cond =
                { event.has_value("destination.nat.ip") && !event.has_value("destination.as") };
            if _cond {
                if event.has_value("destination.nat.ip") {
                    if let Some(ip_str) = event.get_string("destination.nat.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = { !event.has_value("source.geo.name") };
            if _cond {
                if event.has_value("_temp_.srcloc") {
                    event.rename("_temp_.srcloc", "source.geo.name")?;
                }
            }

            let _cond = { !event.has_value("destination.geo.name") };
            if _cond {
                if event.has_value("_temp_.dstloc") {
                    event.rename("_temp_.dstloc", "destination.geo.name")?;
                }
            }

            let _cond = { event.has_value("source.port") };
            if _cond {
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            let _cond = { event.has_value("destination.port") };
            if _cond {
                if let Some(val) = event.get("destination.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            let _cond = { event.has_value("source.nat.port") };
            if _cond {
                if let Some(val) = event.get("source.nat.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.nat.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.nat.port", converted)?;
                }
            }

            let _cond = { event.has_value("destination.nat.port") };
            if _cond {
                if let Some(val) = event.get("destination.nat.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.nat.port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.nat.port", converted)?;
                }
            }

            let _cond = {
                event.has_value("source.port")
                    && event.get_i64("source.port") != Some(0)
                    && event.has_value("destination.port")
                    && event.get_i64("destination.port") != Some(0)
            };
            if _cond {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("source.nat.port")
                    && event.get_i64("source.nat.port") != Some(0)
                    && event.has_value("destination.nat.port")
                    && event.get_i64("destination.nat.port") != Some(0)
            };
            if _cond {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.nat.ip"),
                    event.get_string("destination.nat.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.nat.port", "destination.nat.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("panw.panos.network.nat.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "panw.panos.network.nat.community_id".into(),
                                message,
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("panw.panos.network.nat.community_id")
                    && !condition_eq(
                        event.get("panw.panos.network.nat.community_id"),
                        event.get("network.community_id"),
                    )
            };
            if _cond {
                event.append(
                    "network.community_id",
                    json!(
                        event
                            .get("panw.panos.network.nat.community_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("panw.panos.threat.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("panw.panos.threat.name") {
                        // Grok pattern: ^%{GREEDYDATA:panw.panos.threat.name}\\(\\s*%{NUMBER:panw.panos.threat.id}\\s*\\)$
                        if !cached_grok!("^%{GREEDYDATA:panw.panos.threat.name}\\(\\s*%{NUMBER:panw.panos.threat.id}\\s*\\)$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("panw.panos.threat.id") == Some("9999") };
            if _cond {
                event.set("panw.panos.threat.name", json!("URL-filtering"))?;
            }

            let _cond = { !event.has_value("rule.name") };
            if _cond {
                if let Some(v) = event
                    .get("panw.panos.ruleset")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
            }

            let _cond = { event.has_value("client.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("client.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name")
                    && !(event.get("source.user.name").is_some_and(|v| v.is_array()))
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event.get("source.user.name").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "source.user.name", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("server.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("server.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("panw.panos.admin") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("panw.panos.admin")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("panw.panos.file.hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("panw.panos.file.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("tls.client.hash.md5")
                    && event.get_str("tls.client.hash.md5") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("tls.client.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("tls.client.hash.sha1")
                    && event.get_str("tls.client.hash.sha1") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("tls.client.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("tls.client.hash.sha256")
                    && event.get_str("tls.client.hash.sha256") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("tls.client.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("observer.hostname")
                    && event.get_str("observer.hostname") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("observer.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") && event.get_str("host.name") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("panw.panos.dst.host")
                    && event.get_str("panw.panos.dst.host") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("panw.panos.dst.host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("panw.panos.src.host")
                    && event.get_str("panw.panos.src.host") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("panw.panos.src.host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.remove("_temp_");
            event.remove("_conf");
            event.remove("message");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("panw.panos.bytes_received");
                    event.remove("panw.panos.bytes_sent");
                    event.remove("panw.panos.certificate.fingerprint");
                    event.remove("panw.panos.certificate.not_after");
                    event.remove("panw.panos.certificate.not_before");
                    event.remove("panw.panos.certificate.serial_number");
                    event.remove("panw.panos.certificate.size");
                    event.remove("panw.panos.certificate.version");
                    event.remove("panw.panos.client.os");
                    event.remove("panw.panos.client.os_version");
                    event.remove("panw.panos.destination.ip");
                    event.remove("panw.panos.destination.location");
                    event.remove("panw.panos.destination.nat.ip");
                    event.remove("panw.panos.destination.nat.port");
                    event.remove("panw.panos.destination.port");
                    event.remove("panw.panos.destination.user");
                    event.remove("panw.panos.destination.zone");
                    event.remove("panw.panos.device_name");
                    event.remove("panw.panos.elapsed_time");
                    event.remove("panw.panos.elliptic_curve");
                    event.remove("panw.panos.event.id");
                    event.remove("panw.panos.event.reason");
                    event.remove("panw.panos.event.status");
                    event.remove("panw.panos.file.type");
                    event.remove("panw.panos.forwarded_ip");
                    event.remove("panw.panos.host.id");
                    event.remove("panw.panos.host.ip");
                    event.remove("panw.panos.http_method");
                    event.remove("panw.panos.inbound_interface");
                    event.remove("panw.panos.location");
                    event.remove("panw.panos.login_duration");
                    event.remove("panw.panos.machine.mac_address");
                    event.remove("panw.panos.machine.name");
                    event.remove("panw.panos.machine.os");
                    event.remove("panw.panos.misc");
                    event.remove("panw.panos.network.application");
                    event.remove("panw.panos.network.bytes");
                    event.remove("panw.panos.network.direction");
                    event.remove("panw.panos.network.packets");
                    event.remove("panw.panos.normalize_user");
                    event.remove("panw.panos.observer.serial_number");
                    event.remove("panw.panos.outbound_interface");
                    event.remove("panw.panos.packets_received");
                    event.remove("panw.panos.packets_sent");
                    event.remove("panw.panos.private.ip");
                    event.remove("panw.panos.private.ipv6");
                    event.remove("panw.panos.protocol");
                    event.remove("panw.panos.public.ip");
                    event.remove("panw.panos.public.ipv6");
                    event.remove("panw.panos.recipient");
                    event.remove("panw.panos.referrer");
                    event.remove("panw.panos.rule_uuid");
                    event.remove("panw.panos.sender");
                    event.remove("panw.panos.server_name_indication.value");
                    event.remove("panw.panos.severity");
                    event.remove("panw.panos.source.ip");
                    event.remove("panw.panos.source.ipv6");
                    event.remove("panw.panos.source.location");
                    event.remove("panw.panos.source.nat.ip");
                    event.remove("panw.panos.source.nat.port");
                    event.remove("panw.panos.source.port");
                    event.remove("panw.panos.source.region");
                    event.remove("panw.panos.source.user");
                    event.remove("panw.panos.source.zone");
                    event.remove("panw.panos.start_time");
                    event.remove("panw.panos.tls.encryption");
                    event.remove("panw.panos.tls.version");
                    event.remove("panw.panos.tunnel_inspection_rule");
                    event.remove("panw.panos.user");
                    event.remove("panw.panos.user_agent");
                    event.remove("panw.panos.user_by_source");
                    event.remove("panw.panos.subject_common_name.value");
                    event.remove("panw.panos.issuer_common_name.value");
                    event.remove("panw.panos.hash");
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_temp_");
                event.remove("_conf");
                event.remove("message");
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
