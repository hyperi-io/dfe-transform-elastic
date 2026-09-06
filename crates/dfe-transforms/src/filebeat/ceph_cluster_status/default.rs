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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("ceph"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
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

            if event.has_value("json.outb.health.status") {
                event.rename("json.outb.health.status", "ceph.cluster_status.health")?;
            }

            if event.has_value("json.outb.monmap.min_mon_release_name") {
                event.rename(
                    "json.outb.monmap.min_mon_release_name",
                    "ceph.cluster_status.cluster_version",
                )?;
            }

            if event.has_value("json.outb.monmap.num_mons") {
                event.rename(
                    "json.outb.monmap.num_mons",
                    "ceph.cluster_status.monitor.count",
                )?;
            }

            if event.has_value("json.outb.osdmap.epoch") {
                event.rename("json.outb.osdmap.epoch", "ceph.cluster_status.osd.epoch")?;
            }

            if event.has_value("json.outb.osdmap.num_osds") {
                event.rename("json.outb.osdmap.num_osds", "ceph.cluster_status.osd.count")?;
            }

            if event.has_value("json.outb.osdmap.num_up_osds") {
                event.rename(
                    "json.outb.osdmap.num_up_osds",
                    "ceph.cluster_status.osd.up.count",
                )?;
            }

            if event.has_value("json.outb.osdmap.num_in_osds") {
                event.rename(
                    "json.outb.osdmap.num_in_osds",
                    "ceph.cluster_status.osd.in.count",
                )?;
            }

            if event.has_value("json.outb.osdmap.num_remapped_pgs") {
                event.rename(
                    "json.outb.osdmap.num_remapped_pgs",
                    "ceph.cluster_status.pg.remapped.count",
                )?;
            }

            if event.has_value("json.outb.pgmap.num_objects") {
                event.rename(
                    "json.outb.pgmap.num_objects",
                    "ceph.cluster_status.object.count",
                )?;
            }

            if event.has_value("json.outb.pgmap.pgs_by_state") {
                event.rename(
                    "json.outb.pgmap.pgs_by_state",
                    "ceph.cluster_status.pg.state",
                )?;
            }

            if event.has_value("json.outb.pgmap.data_bytes") {
                event.rename(
                    "json.outb.pgmap.data_bytes",
                    "ceph.cluster_status.pg.data.bytes",
                )?;
            }

            if event.has_value("json.outb.pgmap.bytes_used") {
                event.rename(
                    "json.outb.pgmap.bytes_used",
                    "ceph.cluster_status.pg.used.bytes",
                )?;
            }

            if event.has_value("json.outb.pgmap.bytes_avail") {
                event.rename(
                    "json.outb.pgmap.bytes_avail",
                    "ceph.cluster_status.pg.available.bytes",
                )?;
            }

            if event.has_value("json.outb.pgmap.bytes_total") {
                event.rename(
                    "json.outb.pgmap.bytes_total",
                    "ceph.cluster_status.pg.total.bytes",
                )?;
            }

            if event.has_value("json.outb.pgmap.degraded_objects") {
                event.rename(
                    "json.outb.pgmap.degraded_objects",
                    "ceph.cluster_status.pg.degraded.object.count",
                )?;
            }

            if event.has_value("json.outb.pgmap.degraded_total") {
                event.rename(
                    "json.outb.pgmap.degraded_total",
                    "ceph.cluster_status.pg.degraded.total.count",
                )?;
            }

            if event.has_value("json.outb.pgmap.degraded_ratio") {
                event.rename(
                    "json.outb.pgmap.degraded_ratio",
                    "ceph.cluster_status.pg.degraded.ratio",
                )?;
            }

            if event.has_value("json.outb.pgmap.num_pgs") {
                event.rename("json.outb.pgmap.num_pgs", "ceph.cluster_status.pg.count")?;
            }

            if event.has_value("json.outb.pgmap.num_pools") {
                event.rename(
                    "json.outb.pgmap.num_pools",
                    "ceph.cluster_status.pool.count",
                )?;
            }

            if event.has_value("json.outb.pgmap.read_bytes_sec") {
                event.rename(
                    "json.outb.pgmap.read_bytes_sec",
                    "ceph.cluster_status.traffic.read.bytes",
                )?;
            }

            if event.has_value("json.outb.pgmap.write_bytes_sec") {
                event.rename(
                    "json.outb.pgmap.write_bytes_sec",
                    "ceph.cluster_status.traffic.write.bytes",
                )?;
            }

            if event.has_value("json.outb.pgmap.read_op_per_sec") {
                event.rename(
                    "json.outb.pgmap.read_op_per_sec",
                    "ceph.cluster_status.traffic.read.operation.count",
                )?;
            }

            if event.has_value("json.outb.pgmap.write_op_per_sec") {
                event.rename(
                    "json.outb.pgmap.write_op_per_sec",
                    "ceph.cluster_status.traffic.write.operation.count",
                )?;
            }

            if event.has_value("json.outb.osdmap.osdmap.epoch") {
                event.rename(
                    "json.outb.osdmap.osdmap.epoch",
                    "ceph.cluster_status.osd.epoch",
                )?;
            }

            if event.has_value("json.outb.osdmap.osdmap.num_osds") {
                event.rename(
                    "json.outb.osdmap.osdmap.num_osds",
                    "ceph.cluster_status.osd.count",
                )?;
            }

            if event.has_value("json.outb.osdmap.osdmap.num_up_osds") {
                event.rename(
                    "json.outb.osdmap.osdmap.num_up_osds",
                    "ceph.cluster_status.osd.up.count",
                )?;
            }

            if event.has_value("json.outb.osdmap.osdmap.num_in_osds") {
                event.rename(
                    "json.outb.osdmap.osdmap.num_in_osds",
                    "ceph.cluster_status.osd.in.count",
                )?;
            }

            if event.has_value("json.outb.osdmap.osdmap.num_remapped_pgs") {
                event.rename(
                    "json.outb.osdmap.osdmap.num_remapped_pgs",
                    "ceph.cluster_status.pg.remapped.count",
                )?;
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
