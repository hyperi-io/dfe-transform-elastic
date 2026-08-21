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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "", "*")?;
                Ok(())
            })();

            if let Some(v) = event
                .get("cloud.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("cloud.account.name") {
                    event.set("cloud.account.name", v)?;
                }
            }

            if event.has("aws.s3.metrics.AllRequests.sum") {
                event.rename(
                    "aws.s3.metrics.AllRequests.sum",
                    "aws.s3_request.requests.total",
                )?;
            }

            if event.has("aws.s3.metrics.GetRequests.sum") {
                event.rename(
                    "aws.s3.metrics.GetRequests.sum",
                    "aws.s3_request.requests.get",
                )?;
            }

            if event.has("aws.s3.metrics.PutRequests.sum") {
                event.rename(
                    "aws.s3.metrics.PutRequests.sum",
                    "aws.s3_request.requests.put",
                )?;
            }

            if event.has("aws.s3.metrics.DeleteRequests.sum") {
                event.rename(
                    "aws.s3.metrics.DeleteRequests.sum",
                    "aws.s3_request.requests.delete",
                )?;
            }

            if event.has("aws.s3.metrics.HeadRequests.sum") {
                event.rename(
                    "aws.s3.metrics.HeadRequests.sum",
                    "aws.s3_request.requests.head",
                )?;
            }

            if event.has("aws.s3.metrics.PostRequests.sum") {
                event.rename(
                    "aws.s3.metrics.PostRequests.sum",
                    "aws.s3_request.requests.post",
                )?;
            }

            if event.has("aws.s3.metrics.SelectRequests.sum") {
                event.rename(
                    "aws.s3.metrics.SelectRequests.sum",
                    "aws.s3_request.requests.select",
                )?;
            }

            if event.has("aws.s3.metrics.SelectScannedBytes.avg") {
                event.rename(
                    "aws.s3.metrics.SelectScannedBytes.avg",
                    "aws.s3_request.requests.select_scanned.bytes",
                )?;
            }

            if event.has("aws.s3.metrics.SelectReturnedBytes.avg") {
                event.rename(
                    "aws.s3.metrics.SelectReturnedBytes.avg",
                    "aws.s3_request.requests.select_returned.bytes",
                )?;
            }

            if event.has("aws.s3.metrics.ListRequests.sum") {
                event.rename(
                    "aws.s3.metrics.ListRequests.sum",
                    "aws.s3_request.requests.list",
                )?;
            }

            if event.has("aws.s3.metrics.BytesDownloaded.avg") {
                event.rename(
                    "aws.s3.metrics.BytesDownloaded.avg",
                    "aws.s3_request.downloaded.bytes",
                )?;
            }

            if event.has("aws.s3.metrics.BytesUploaded.avg") {
                event.rename(
                    "aws.s3.metrics.BytesUploaded.avg",
                    "aws.s3_request.uploaded.bytes",
                )?;
            }

            if event.has("aws.s3.metrics.BytesDownloaded.sum") {
                event.rename(
                    "aws.s3.metrics.BytesDownloaded.sum",
                    "aws.s3_request.downloaded.bytes_per_period",
                )?;
            }

            if event.has("aws.s3.metrics.BytesUploaded.sum") {
                event.rename(
                    "aws.s3.metrics.BytesUploaded.sum",
                    "aws.s3_request.uploaded.bytes_per_period",
                )?;
            }

            if event.has("aws.s3.metrics.4xxErrors.avg") {
                event.rename("aws.s3.metrics.4xxErrors.avg", "aws.s3_request.errors.4xx")?;
            }

            if event.has("aws.s3.metrics.5xxErrors.avg") {
                event.rename("aws.s3.metrics.5xxErrors.avg", "aws.s3_request.errors.5xx")?;
            }

            if event.has("aws.s3.metrics.FirstByteLatency.avg") {
                event.rename(
                    "aws.s3.metrics.FirstByteLatency.avg",
                    "aws.s3_request.latency.first_byte.ms",
                )?;
            }

            if event.has("aws.s3.metrics.TotalRequestLatency.avg") {
                event.rename(
                    "aws.s3.metrics.TotalRequestLatency.avg",
                    "aws.s3_request.latency.total_request.ms",
                )?;
            }

            if event.has("aws.dimensions.BucketName") {
                event.rename("aws.dimensions.BucketName", "aws.s3.bucket.name")?;
            }

            let _cond = { event.get_str("agent.type") != Some("firehose") };
            if _cond {
                event.remove("aws.s3.metrics");
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
