// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `threat` pipeline.
pub struct Threat;

impl Transform for Threat {
    fn name(&self) -> &str {
        "threat"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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

            let _cond =
                { !event.has_value("event.outcome") || event.get_str("event.outcome") == Some("") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("_temp_.forwarded_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "_temp_.forwarded_ip".into(),
                            message,
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
                if event.has("_temp_.forwarded_ip") {
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
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/")),
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
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/")),
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
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/")),
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
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/")),
                        serde_json::Value::String(s) => s.contains("/"),
                        _ => false,
                    }) || event.get("panw.panos.misc").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
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
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/")),
                        serde_json::Value::String(s) => s.contains("/"),
                        _ => false,
                    }) || event.get("panw.panos.misc").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
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
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("panw.panos.url_category_list", Value::Array(parts))?;
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
