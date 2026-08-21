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
            {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                if let Some(v) = event.get("gcp.labels") {
                    hasher.update(v.to_string().as_bytes());
                }
                let hash = format!("{:x}", hasher.finalize());
                event.set("gcp.labels_fingerprint", json!(hash))?;
            }

            if event.has("gcp.metrics.firewall.dropped.bytes") {
                event.rename(
                    "gcp.metrics.firewall.dropped.bytes",
                    "gcp.compute.firewall.dropped.bytes",
                )?;
            }

            if event.has("gcp.metrics.firewall.dropped_packets_count.value") {
                event.rename(
                    "gcp.metrics.firewall.dropped_packets_count.value",
                    "gcp.compute.firewall.dropped_packets_count.value",
                )?;
            }

            if event.has("gcp.metrics.instance.cpu.reserved_cores.value") {
                event.rename(
                    "gcp.metrics.instance.cpu.reserved_cores.value",
                    "gcp.compute.instance.cpu.reserved_cores.value",
                )?;
            }

            if event.has("gcp.metrics.instance.cpu.usage_time.sec") {
                event.rename(
                    "gcp.metrics.instance.cpu.usage_time.sec",
                    "gcp.compute.instance.cpu.usage_time.sec",
                )?;
            }

            if event.has("gcp.metrics.instance.cpu.usage.pct") {
                event.rename(
                    "gcp.metrics.instance.cpu.usage.pct",
                    "gcp.compute.instance.cpu.usage.pct",
                )?;
            }

            if event.has("gcp.metrics.instance.disk.read.bytes") {
                event.rename(
                    "gcp.metrics.instance.disk.read.bytes",
                    "gcp.compute.instance.disk.read.bytes",
                )?;
            }

            if event.has("gcp.metrics.instance.disk.read_ops_count.value") {
                event.rename(
                    "gcp.metrics.instance.disk.read_ops_count.value",
                    "gcp.compute.instance.disk.read_ops_count.value",
                )?;
            }

            if event.has("gcp.metrics.instance.disk.write.bytes") {
                event.rename(
                    "gcp.metrics.instance.disk.write.bytes",
                    "gcp.compute.instance.disk.write.bytes",
                )?;
            }

            if event.has("gcp.metrics.instance.disk.write_ops_count.value") {
                event.rename(
                    "gcp.metrics.instance.disk.write_ops_count.value",
                    "gcp.compute.instance.disk.write_ops_count.value",
                )?;
            }

            if event.has("gcp.metrics.instance.memory.balloon.ram_size.value") {
                event.rename(
                    "gcp.metrics.instance.memory.balloon.ram_size.value",
                    "gcp.compute.instance.memory.balloon.ram_size.value",
                )?;
            }

            if event.has("gcp.metrics.instance.memory.balloon.ram_used.value") {
                event.rename(
                    "gcp.metrics.instance.memory.balloon.ram_used.value",
                    "gcp.compute.instance.memory.balloon.ram_used.value",
                )?;
            }

            if event.has("gcp.metrics.instance.memory.balloon.swap_in.bytes") {
                event.rename(
                    "gcp.metrics.instance.memory.balloon.swap_in.bytes",
                    "gcp.compute.instance.memory.balloon.swap_in.bytes",
                )?;
            }

            if event.has("gcp.metrics.instance.memory.balloon.swap_out.bytes") {
                event.rename(
                    "gcp.metrics.instance.memory.balloon.swap_out.bytes",
                    "gcp.compute.instance.memory.balloon.swap_out.bytes",
                )?;
            }

            if event.has("gcp.metrics.instance.network.ingress.bytes") {
                event.rename(
                    "gcp.metrics.instance.network.ingress.bytes",
                    "gcp.compute.instance.network.ingress.bytes",
                )?;
            }

            if event.has("gcp.metrics.instance.network.ingress.packets.count") {
                event.rename(
                    "gcp.metrics.instance.network.ingress.packets.count",
                    "gcp.compute.instance.network.ingress.packets.count",
                )?;
            }

            if event.has("gcp.metrics.instance.network.egress.bytes") {
                event.rename(
                    "gcp.metrics.instance.network.egress.bytes",
                    "gcp.compute.instance.network.egress.bytes",
                )?;
            }

            if event.has("gcp.metrics.instance.network.egress.packets.count") {
                event.rename(
                    "gcp.metrics.instance.network.egress.packets.count",
                    "gcp.compute.instance.network.egress.packets.count",
                )?;
            }

            if event.has("gcp.metrics.instance.uptime.sec") {
                event.rename(
                    "gcp.metrics.instance.uptime.sec",
                    "gcp.compute.instance.uptime.sec",
                )?;
            }

            if event.has("gcp.metrics.instance.uptime_total.sec") {
                event.rename(
                    "gcp.metrics.instance.uptime_total.sec",
                    "gcp.compute.instance.uptime_total.sec",
                )?;
            }

            event.remove("gcp.metrics");

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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
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
