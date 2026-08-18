// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `hipmatch` pipeline.
pub struct Hipmatch;

impl Transform for Hipmatch {
    fn name(&self) -> &str {
        "hipmatch"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
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
            event.set(
                "source.ip",
                event
                    .get("panw.panos.source.ip")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // TODO: conditional: ctx._temp_?.source_ipv6 != null && ctx._temp_.source_ipv6 != '' && ctx._temp_.source_ipv6 != '0.0.0.0'
        {
            event.set(
                "source.ip",
                event
                    .get("_temp_.source_ipv6")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        event.set("event.kind", json!("event"))?;

        event.append("event.category", json!("network"))?;

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "host.id",
                event
                    .get("panw.panos.host.id")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "host.mac",
                event
                    .get("panw.panos.machine.mac_address")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // TODO: conditional: ctx.panw?.panos?.machine?.name != null
        {
            if let Some(s) = event.get_str("panw.panos.machine.name").map(String::from) {
                let s = s.as_str();
                let lowered = s.to_lowercase();
                event.set("host.name", lowered)?;
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "host.os.full",
                event
                    .get("panw.panos.machine.os")
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
                "panw.panos.source.ipv6",
                event
                    .get("_temp_.source_ipv6")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        Ok(TransformResult::Continue)
    }
}
