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

            if event.has("gcp.metrics.api.request.count") {
                event.rename(
                    "gcp.metrics.api.request.count",
                    "gcp.storage.api.request.count",
                )?;
            }

            if event.has("gcp.metrics.authz.acl_based_object_access.count") {
                event.rename(
                    "gcp.metrics.authz.acl_based_object_access.count",
                    "gcp.storage.authz.acl_based_object_access.count",
                )?;
            }

            if event.has("gcp.metrics.authz.acl_operations.count") {
                event.rename(
                    "gcp.metrics.authz.acl_operations.count",
                    "gcp.storage.authz.acl_operations.count",
                )?;
            }

            if event.has("gcp.metrics.authz.object_specific_acl_mutation.count") {
                event.rename(
                    "gcp.metrics.authz.object_specific_acl_mutation.count",
                    "gcp.storage.authz.object_specific_acl_mutation.count",
                )?;
            }

            if event.has("gcp.metrics.network.received.bytes") {
                event.rename(
                    "gcp.metrics.network.received.bytes",
                    "gcp.storage.network.received.bytes",
                )?;
            }

            if event.has("gcp.metrics.network.sent.bytes") {
                event.rename(
                    "gcp.metrics.network.sent.bytes",
                    "gcp.storage.network.sent.bytes",
                )?;
            }

            if event.has("gcp.metrics.storage.object.count") {
                event.rename(
                    "gcp.metrics.storage.object.count",
                    "gcp.storage.storage.object.count",
                )?;
            }

            if event.has("gcp.metrics.storage.total_byte_seconds.bytes") {
                event.rename(
                    "gcp.metrics.storage.total_byte_seconds.bytes",
                    "gcp.storage.storage.total_byte_seconds.bytes",
                )?;
            }

            if event.has("gcp.metrics.storage.total.bytes") {
                event.rename(
                    "gcp.metrics.storage.total.bytes",
                    "gcp.storage.storage.total.bytes",
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
