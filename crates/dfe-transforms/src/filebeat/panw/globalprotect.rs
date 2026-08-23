// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `globalprotect` pipeline.
pub struct Globalprotect;

impl Transform for Globalprotect {
    fn name(&self) -> &str {
        "globalprotect"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
