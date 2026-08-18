// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `ip_tag` pipeline.
pub struct IpTag;

impl Transform for IpTag {
    fn name(&self) -> &str {
        "ip_tag"
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
            event.set(
                "event.code",
                event
                    .get("panw.panos.event.id")
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
                "source.ip",
                event
                    .get("panw.panos.source.ip")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        Ok(TransformResult::Continue)
    }
}
