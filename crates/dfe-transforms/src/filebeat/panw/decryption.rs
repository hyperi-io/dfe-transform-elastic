// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `decryption` pipeline.
pub struct Decryption;

impl Transform for Decryption {
    fn name(&self) -> &str {
        "decryption"
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

            if event.has("_temp_.config_version") {
                event.rename("_temp_.config_version", "panw.panos.config_version")?;
            }

            let _cond = {
                !event.has_value("event.timezone")
                    && event.has_value("panw.panos.certificate.not_after")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("panw.panos.certificate.not_after")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("panw.panos.certificate.not_after", parsed)?;
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
                        event
                            .get("_ingest.on_failure_message")
                            .cloned()
                            .unwrap_or(Value::Null),
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
                    if let Some(date_str) = event.get_as_string("panw.panos.certificate.not_after")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("panw.panos.certificate.not_after", parsed)?;
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
                        event
                            .get("_ingest.on_failure_message")
                            .cloned()
                            .unwrap_or(Value::Null),
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
                    if let Some(date_str) = event.get_as_string("panw.panos.certificate.not_before")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("panw.panos.certificate.not_before", parsed)?;
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
                        event
                            .get("_ingest.on_failure_message")
                            .cloned()
                            .unwrap_or(Value::Null),
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
                    if let Some(date_str) = event.get_as_string("panw.panos.certificate.not_before")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("panw.panos.certificate.not_before", parsed)?;
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
                        event
                            .get("_ingest.on_failure_message")
                            .cloned()
                            .unwrap_or(Value::Null),
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
                    || event
                        .get_str("panw.panos.error_message")
                        .is_none_or(|s| s.is_empty())
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event
                    .get_str("panw.panos.error_message")
                    .is_some_and(|s| !s.is_empty())
            };
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
                    json!(["{{{tls.client.x509.subject.common_name}}}"]),
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
                    json!(["{{{tls.client.x509.issuer.common_name}}}"]),
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
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx.tls.client.hash = new HashMap();\nif (ctx._temp_.hash.length() == 32) {ctx.tls.client.hash.md5 = ctx._temp_.hash}\nelse if (ctx._temp_.hash.length() == 40) {ctx.tls.client.hash.sha1 = ctx._temp_.hash}\nelse if (ctx._temp_.hash.length() == 64) {ctx.tls.client.hash.sha256 = ctx._temp_.hash}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.tls") };
            if _cond {
                // Painless script
                // Source: ctx.tls.version = new HashMap();\nctx.tls.version_protocol = ctx._temp_?.tls.substring(0,3).toLowerCase();\nctx.tls.version = ctx._temp_?.tls.substring(3,6);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx.tls.version = new HashMap();\nctx.tls.version_protocol = ctx._temp_?.tls.substring(0,3).toLowerCase();\nctx.tls.version = ctx._temp_?.tls.substring(3,6);\n"#
                    ),
                )?;
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
                event.append("tags", json!("preserve_original_event"))?;
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
