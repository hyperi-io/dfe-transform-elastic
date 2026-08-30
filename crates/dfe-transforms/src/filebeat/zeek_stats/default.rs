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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.stats")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            if event.has_value("zeek.stats.mem") {
                event.rename("zeek.stats.mem", "zeek.stats.memory")?;
            }

            if event.has_value("zeek.stats.pkts_proc") {
                event.rename("zeek.stats.pkts_proc", "zeek.stats.packets.processed")?;
            }

            if event.has_value("zeek.stats.pkts_dropped") {
                event.rename("zeek.stats.pkts_dropped", "zeek.stats.packets.dropped")?;
            }

            if event.has_value("zeek.stats.pkts_link") {
                event.rename("zeek.stats.pkts_link", "zeek.stats.packets.received")?;
            }

            if event.has_value("zeek.stats.bytes_recv") {
                event.rename("zeek.stats.bytes_recv", "zeek.stats.bytes.received")?;
            }

            if event.has_value("zeek.stats.tcp_conns") {
                event.rename("zeek.stats.tcp_conns", "zeek.stats.connections.tcp.count")?;
            }

            if event.has_value("zeek.stats.active_tcp_conns") {
                event.rename(
                    "zeek.stats.active_tcp_conns",
                    "zeek.stats.connections.tcp.active",
                )?;
            }

            if event.has_value("zeek.stats.udp_conns") {
                event.rename("zeek.stats.udp_conns", "zeek.stats.connections.udp.count")?;
            }

            if event.has_value("zeek.stats.active_udp_conns") {
                event.rename(
                    "zeek.stats.active_udp_conns",
                    "zeek.stats.connections.udp.active",
                )?;
            }

            if event.has_value("zeek.stats.icmp_conns") {
                event.rename("zeek.stats.icmp_conns", "zeek.stats.connections.icmp.count")?;
            }

            if event.has_value("zeek.stats.active_icmp_conns") {
                event.rename(
                    "zeek.stats.active_icmp_conns",
                    "zeek.stats.connections.icmp.active",
                )?;
            }

            if event.has_value("zeek.stats.events_proc") {
                event.rename("zeek.stats.events_proc", "zeek.stats.events.processed")?;
            }

            if event.has_value("zeek.stats.events_queued") {
                event.rename("zeek.stats.events_queued", "zeek.stats.events.queued")?;
            }

            if event.has_value("zeek.stats.timers") {
                event.rename("zeek.stats.timers", "zeek.stats.timers.count")?;
            }

            if event.has_value("zeek.stats.active_timers") {
                event.rename("zeek.stats.active_timers", "zeek.stats.timers.active")?;
            }

            if event.has_value("zeek.stats.files") {
                event.rename("zeek.stats.files", "zeek.stats.files.count")?;
            }

            if event.has_value("zeek.stats.active_files") {
                event.rename("zeek.stats.active_files", "zeek.stats.files.active")?;
            }

            if event.has_value("zeek.stats.dns_requests") {
                event.rename("zeek.stats.dns_requests", "zeek.stats.dns_requests.count")?;
            }

            if event.has_value("zeek.stats.active_dns_requests") {
                event.rename(
                    "zeek.stats.active_dns_requests",
                    "zeek.stats.dns_requests.active",
                )?;
            }

            if event.has_value("zeek.stats.reassem_tcp_size") {
                event.rename(
                    "zeek.stats.reassem_tcp_size",
                    "zeek.stats.reassembly_size.tcp",
                )?;
            }

            if event.has_value("zeek.stats.reassem_file_size") {
                event.rename(
                    "zeek.stats.reassem_file_size",
                    "zeek.stats.reassembly_size.file",
                )?;
            }

            if event.has_value("zeek.stats.reassem_frag_size") {
                event.rename(
                    "zeek.stats.reassem_frag_size",
                    "zeek.stats.reassembly_size.frag",
                )?;
            }

            if event.has_value("zeek.stats.reassem_unknown_size") {
                event.rename(
                    "zeek.stats.reassem_unknown_size",
                    "zeek.stats.reassembly_size.unknown",
                )?;
            }

            if event.has_value("zeek.stats.pkt_lag") {
                event.rename("zeek.stats.pkt_lag", "zeek.stats.timestamp_lag")?;
            }

            if let Some(date_str) = event.get_as_string("zeek.stats.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.stats.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.stats.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.stats.ts".into(),
                });
            }

            event.set("event.kind", json!("metric"))?;

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
                event.set(
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
