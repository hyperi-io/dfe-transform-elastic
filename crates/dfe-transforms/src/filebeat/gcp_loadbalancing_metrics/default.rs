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
                let mut values = Vec::new();
                if let Some(v) = event.get("gcp.labels") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "gcp.labels_fingerprint",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            if event.has("gcp.metrics.https.backend_request.bytes") {
                event.rename(
                    "gcp.metrics.https.backend_request.bytes",
                    "gcp.loadbalancing_metrics.https.backend_request.bytes",
                )?;
            }

            if event.has("gcp.metrics.https.backend_request.count") {
                event.rename(
                    "gcp.metrics.https.backend_request.count",
                    "gcp.loadbalancing_metrics.https.backend_request.count",
                )?;
            }

            if event.has("gcp.metrics.https.backend_response.bytes") {
                event.rename(
                    "gcp.metrics.https.backend_response.bytes",
                    "gcp.loadbalancing_metrics.https.backend_response.bytes",
                )?;
            }

            if event.has("gcp.metrics.https.request.bytes") {
                event.rename(
                    "gcp.metrics.https.request.bytes",
                    "gcp.loadbalancing_metrics.https.request.bytes",
                )?;
            }

            if event.has("gcp.metrics.https.request.count") {
                event.rename(
                    "gcp.metrics.https.request.count",
                    "gcp.loadbalancing_metrics.https.request.count",
                )?;
            }

            if event.has("gcp.metrics.https.response.bytes") {
                event.rename(
                    "gcp.metrics.https.response.bytes",
                    "gcp.loadbalancing_metrics.https.response.bytes",
                )?;
            }

            if event.has("gcp.metrics.l3.external.egress.bytes") {
                event.rename(
                    "gcp.metrics.l3.external.egress.bytes",
                    "gcp.loadbalancing_metrics.l3.external.egress.bytes",
                )?;
            }

            if event.has("gcp.metrics.l3.external.egress_packets.count") {
                event.rename(
                    "gcp.metrics.l3.external.egress_packets.count",
                    "gcp.loadbalancing_metrics.l3.external.egress_packets.count",
                )?;
            }

            if event.has("gcp.metrics.l3.external.ingress.bytes") {
                event.rename(
                    "gcp.metrics.l3.external.ingress.bytes",
                    "gcp.loadbalancing_metrics.l3.external.ingress.bytes",
                )?;
            }

            if event.has("gcp.metrics.l3.external.ingress_packets.count") {
                event.rename(
                    "gcp.metrics.l3.external.ingress_packets.count",
                    "gcp.loadbalancing_metrics.l3.external.ingress_packets.count",
                )?;
            }

            if event.has("gcp.metrics.l3.internal.egress.bytes") {
                event.rename(
                    "gcp.metrics.l3.internal.egress.bytes",
                    "gcp.loadbalancing_metrics.l3.internal.egress.bytes",
                )?;
            }

            if event.has("gcp.metrics.l3.internal.egress_packets.count") {
                event.rename(
                    "gcp.metrics.l3.internal.egress_packets.count",
                    "gcp.loadbalancing_metrics.l3.internal.egress_packets.count",
                )?;
            }

            if event.has("gcp.metrics.l3.internal.ingress.bytes") {
                event.rename(
                    "gcp.metrics.l3.internal.ingress.bytes",
                    "gcp.loadbalancing_metrics.l3.internal.ingress.bytes",
                )?;
            }

            if event.has("gcp.metrics.l3.internal.ingress_packets.count") {
                event.rename(
                    "gcp.metrics.l3.internal.ingress_packets.count",
                    "gcp.loadbalancing_metrics.l3.internal.ingress_packets.count",
                )?;
            }

            if event.has("gcp.metrics.tcp_ssl_proxy.closed_connections.value") {
                event.rename(
                    "gcp.metrics.tcp_ssl_proxy.closed_connections.value",
                    "gcp.loadbalancing_metrics.tcp_ssl_proxy.closed_connections.value",
                )?;
            }

            if event.has("gcp.metrics.tcp_ssl_proxy.egress.bytes") {
                event.rename(
                    "gcp.metrics.tcp_ssl_proxy.egress.bytes",
                    "gcp.loadbalancing_metrics.tcp_ssl_proxy.egress.bytes",
                )?;
            }

            if event.has("gcp.metrics.tcp_ssl_proxy.ingress.bytes") {
                event.rename(
                    "gcp.metrics.tcp_ssl_proxy.ingress.bytes",
                    "gcp.loadbalancing_metrics.tcp_ssl_proxy.ingress.bytes",
                )?;
            }

            if event.has("gcp.metrics.tcp_ssl_proxy.new_connections.value") {
                event.rename(
                    "gcp.metrics.tcp_ssl_proxy.new_connections.value",
                    "gcp.loadbalancing_metrics.tcp_ssl_proxy.new_connections.value",
                )?;
            }

            if event.has("gcp.metrics.tcp_ssl_proxy.open_connections.value") {
                event.rename(
                    "gcp.metrics.tcp_ssl_proxy.open_connections.value",
                    "gcp.loadbalancing_metrics.tcp_ssl_proxy.open_connections.value",
                )?;
            }

            if event.has("gcp.metrics.https.backend_latencies.value") {
                event.rename(
                    "gcp.metrics.https.backend_latencies.value",
                    "gcp.loadbalancing_metrics.https.backend_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.https.external.regional.backend_latencies.value") {
                event.rename(
                    "gcp.metrics.https.external.regional.backend_latencies.value",
                    "gcp.loadbalancing_metrics.https.external.regional.backend_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.https.external.regional.total_latencies.value") {
                event.rename(
                    "gcp.metrics.https.external.regional.total_latencies.value",
                    "gcp.loadbalancing_metrics.https.external.regional.total_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.https.frontend_tcp_rtt.value") {
                event.rename(
                    "gcp.metrics.https.frontend_tcp_rtt.value",
                    "gcp.loadbalancing_metrics.https.frontend_tcp_rtt.value",
                )?;
            }

            if event.has("gcp.metrics.https.internal.backend_latencies.value") {
                event.rename(
                    "gcp.metrics.https.internal.backend_latencies.value",
                    "gcp.loadbalancing_metrics.https.internal.backend_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.https.internal.total_latencies.value") {
                event.rename(
                    "gcp.metrics.https.internal.total_latencies.value",
                    "gcp.loadbalancing_metrics.https.internal.total_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.https.total_latencies.value") {
                event.rename(
                    "gcp.metrics.https.total_latencies.value",
                    "gcp.loadbalancing_metrics.https.total_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.l3.external.rtt_latencies.value") {
                event.rename(
                    "gcp.metrics.l3.external.rtt_latencies.value",
                    "gcp.loadbalancing_metrics.l3.external.rtt_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.l3.internal.rtt_latencies.value") {
                event.rename(
                    "gcp.metrics.l3.internal.rtt_latencies.value",
                    "gcp.loadbalancing_metrics.l3.internal.rtt_latencies.value",
                )?;
            }

            if event.has("gcp.metrics.tcp_ssl_proxy.frontend_tcp_rtt.value") {
                event.rename(
                    "gcp.metrics.tcp_ssl_proxy.frontend_tcp_rtt.value",
                    "gcp.loadbalancing_metrics.tcp_ssl_proxy.frontend_tcp_rtt.value",
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
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
