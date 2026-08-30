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
            event.remove("metricset.name");
            event.remove("service.address");
            event.remove("service.type");

            event.set("ecs.version", json!("8.6.0"))?;

            event.set("event.module", json!("istio"))?;

            event.set("event.kind", json!("metric"))?;

            if event.has_value("prometheus.labels") {
                event.rename("prometheus.labels", "istio.istiod.labels")?;
            }

            event.set("istio.istiod.labels.job", json!("istio"))?;

            if event.has_value("prometheus") {
                event.rename("prometheus", "istio.istiod.metrics")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("istio.istiod.labels") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "istio.istiod.labels_id",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
