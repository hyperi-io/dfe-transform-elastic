// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `aws_lambda_plaintext` pipeline.
pub struct AwsLambdaPlaintext;

impl Transform for AwsLambdaPlaintext {
    fn name(&self) -> &str {
        "aws_lambda_plaintext"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?P<aws_lambda_event_type>START)\\s+RequestId:\\s+%{DATA:aws.lambda.request_id}\\s+Version:\\s+(?P<aws_lambda_version>\\$LATEST|[^\\s]+)\\s+(?P<message>(?:(.|\n|\t)*))
                    if !cached_grok_mapped!("^(?P<aws_lambda_event_type>START)\\s+RequestId:\\s+%{DATA:aws.lambda.request_id}\\s+Version:\\s+(?P<aws_lambda_version>\\$LATEST|[^\\s]+)\\s+(?P<message>(?:(.|\n|\t)*))", [("aws_lambda_event_type", "aws.lambda.event_type"), ("aws_lambda_version", "aws.lambda.version")]).extract_into(&input, event)? {
                        // Grok pattern: ^(?P<aws_lambda_event_type>INIT_START)\\s+Runtime Version: %{DATA:aws.lambda.runtime_version}\\s+Runtime Version ARN: %{GREEDYDATA:aws.lambda.arn}
                        if !cached_grok_mapped!("^(?P<aws_lambda_event_type>INIT_START)\\s+Runtime Version: %{DATA:aws.lambda.runtime_version}\\s+Runtime Version ARN: %{GREEDYDATA:aws.lambda.arn}", [("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                            // Grok pattern: ^(?P<aws_lambda_event_type>LOGS)\\s+Name: %{DATA:aws.lambda.log_extension.name}\\s+State: %{DATA:aws.lambda.log_extension.state}\\s+Types: \\[%{DATA:aws.lambda.log_extension.types}\\]
                            if !cached_grok_mapped!("^(?P<aws_lambda_event_type>LOGS)\\s+Name: %{DATA:aws.lambda.log_extension.name}\\s+State: %{DATA:aws.lambda.log_extension.state}\\s+Types: \\[%{DATA:aws.lambda.log_extension.types}\\]", [("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                // Grok pattern: ^(?P<aws_lambda_event_type>EXTENSION)\\s+Name: %{DATA:aws.lambda.extension.name}\\s+State: %{DATA:aws.lambda.extension.state}\\s+Events: \\[%{DATA:aws.lambda.extension.events}\\]
                                if !cached_grok_mapped!("^(?P<aws_lambda_event_type>EXTENSION)\\s+Name: %{DATA:aws.lambda.extension.name}\\s+State: %{DATA:aws.lambda.extension.state}\\s+Events: \\[%{DATA:aws.lambda.extension.events}\\]", [("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                    // Grok pattern: ^(?P<aws_lambda_event_type>REPORT)\\s+RequestId:\\s+%{DATA:aws.lambda.request_id}\\s+Duration:\\s+%{NUMBER:aws.lambda.metrics.duration_ms:float} ms\\s+Billed Duration:\\s+%{NUMBER:aws.lambda.metrics.billed_duration_ms:float} ms\\s+Memory Size:\\s+%{NUMBER:aws.lambda.metrics.memory_size_mb:float} MB\\s+Max Memory Used:\\s+%{NUMBER:aws.lambda.metrics.max_memory_used_mb:float} MB(?:\\s+Init Duration:\\s+%{NUMBER:aws.lambda.metrics.init_duration_ms:float} ms)?
                                    if !cached_grok_mapped!("^(?P<aws_lambda_event_type>REPORT)\\s+RequestId:\\s+%{DATA:aws.lambda.request_id}\\s+Duration:\\s+%{NUMBER:aws.lambda.metrics.duration_ms:float} ms\\s+Billed Duration:\\s+%{NUMBER:aws.lambda.metrics.billed_duration_ms:float} ms\\s+Memory Size:\\s+%{NUMBER:aws.lambda.metrics.memory_size_mb:float} MB\\s+Max Memory Used:\\s+%{NUMBER:aws.lambda.metrics.max_memory_used_mb:float} MB(?:\\s+Init Duration:\\s+%{NUMBER:aws.lambda.metrics.init_duration_ms:float} ms)?", [("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                        // Grok pattern: ^(?P<aws_lambda_event_type>XRAY)\\s+TraceId:\\s+(?P<aws_lambda_tracing_xray_trace_id>(?:[^\\s]+))\\s+SegmentId:\\s+(?P<aws_lambda_tracing_segment_id>(?:[^\\s]+))(?:\\s+Sampled:\\s+(?P<aws_lambda_tracing_sampled>(?:[^\\s]+)))?
                                        if !cached_grok_mapped!("^(?P<aws_lambda_event_type>XRAY)\\s+TraceId:\\s+(?P<aws_lambda_tracing_xray_trace_id>(?:[^\\s]+))\\s+SegmentId:\\s+(?P<aws_lambda_tracing_segment_id>(?:[^\\s]+))(?:\\s+Sampled:\\s+(?P<aws_lambda_tracing_sampled>(?:[^\\s]+)))?", [("aws_lambda_tracing_xray_trace_id", "aws.lambda.tracing.xray_trace_id"), ("aws_lambda_tracing_segment_id", "aws.lambda.tracing.segment_id"), ("aws_lambda_tracing_sampled", "aws.lambda.tracing.sampled"), ("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                            // Grok pattern: ^(?P<aws_lambda_event_type>END)\\s+RequestId:\\s+%{GREEDYDATA:aws.lambda.request_id}
                                            if !cached_grok_mapped!("^(?P<aws_lambda_event_type>END)\\s+RequestId:\\s+%{GREEDYDATA:aws.lambda.request_id}", [("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                                // Grok pattern: ^\\[%{WORD:log.level}\\]\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))\\s+(?P<message>(?:(.|\n|\t)*))
                                                if !cached_grok_mapped!("^\\[%{WORD:log.level}\\]\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))\\s+(?P<message>(?:(.|\n|\t)*))", [("aws_lambda_request_id", "aws.lambda.request_id")]).extract_into(&input, event)? {
                                                    // Grok pattern: ^%{TIMESTAMP_ISO8601:timestamp}\\s+%{DATA:aws.lambda.request_id}\\s+%{LOGLEVEL:log.level}\\s+(?P<message>(?:(.|\n|\t)*))
                                                    if !cached_grok!("^%{TIMESTAMP_ISO8601:timestamp}\\s+%{DATA:aws.lambda.request_id}\\s+%{LOGLEVEL:log.level}\\s+(?P<message>(?:(.|\n|\t)*))").extract_into(&input, event)? {
                                                        // Grok pattern: ^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>INIT_START)\\s+Runtime Version:\\s+(?P<aws_lambda_runtime_version>(?:[^\\s]+))\\s+Runtime Version ARN:\\s+(?P<aws_lambda_runtime_version_arn>(?:[^\\s]+))
                                                        if !cached_grok_mapped!("^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>INIT_START)\\s+Runtime Version:\\s+(?P<aws_lambda_runtime_version>(?:[^\\s]+))\\s+Runtime Version ARN:\\s+(?P<aws_lambda_runtime_version_arn>(?:[^\\s]+))", [("aws_lambda_log_stream_id", "aws.lambda.log_stream_id"), ("aws_lambda_runtime_version", "aws.lambda.runtime_version"), ("aws_lambda_runtime_version_arn", "aws.lambda.runtime_version_arn"), ("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                                            // Grok pattern: ^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>START)\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))\\s+Version:\\s+(?P<aws_lambda_version>(?:[^\\s]+))\\s+(?P<message>(?:(.|\n|\t)*))
                                                            if !cached_grok_mapped!("^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>START)\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))\\s+Version:\\s+(?P<aws_lambda_version>(?:[^\\s]+))\\s+(?P<message>(?:(.|\n|\t)*))", [("aws_lambda_log_stream_id", "aws.lambda.log_stream_id"), ("aws_lambda_request_id", "aws.lambda.request_id"), ("aws_lambda_version", "aws.lambda.version"), ("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                                                // Grok pattern: ^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>REPORT)\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))\\s+Duration:\\s+%{NUMBER:aws.lambda.metrics.duration_ms:float} ms\\s+Billed Duration:\\s+%{NUMBER:aws.lambda.metrics.billed_duration_ms:float} ms\\s+Memory Size:\\s+%{NUMBER:aws.lambda.metrics.memory_size_mb:float} MB\\s+Max Memory Used:\\s+%{NUMBER:aws.lambda.metrics.max_memory_used_mb:float} MB(?:\\s+Init Duration:\\s+%{NUMBER:aws.lambda.metrics.init_duration_ms:float} ms)?
                                                                if !cached_grok_mapped!("^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>REPORT)\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))\\s+Duration:\\s+%{NUMBER:aws.lambda.metrics.duration_ms:float} ms\\s+Billed Duration:\\s+%{NUMBER:aws.lambda.metrics.billed_duration_ms:float} ms\\s+Memory Size:\\s+%{NUMBER:aws.lambda.metrics.memory_size_mb:float} MB\\s+Max Memory Used:\\s+%{NUMBER:aws.lambda.metrics.max_memory_used_mb:float} MB(?:\\s+Init Duration:\\s+%{NUMBER:aws.lambda.metrics.init_duration_ms:float} ms)?", [("aws_lambda_log_stream_id", "aws.lambda.log_stream_id"), ("aws_lambda_request_id", "aws.lambda.request_id"), ("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                                                    // Grok pattern: ^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>END)\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))
                                                                    if !cached_grok_mapped!("^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<aws_lambda_event_type>END)\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))", [("aws_lambda_log_stream_id", "aws.lambda.log_stream_id"), ("aws_lambda_request_id", "aws.lambda.request_id"), ("aws_lambda_event_type", "aws.lambda.event_type")]).extract_into(&input, event)? {
                                                                        // Grok pattern: ^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+%{WORD:aws.lambda.event_type}\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))
                                                                        if !cached_grok_mapped!("^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+%{WORD:aws.lambda.event_type}\\s+RequestId:\\s+(?P<aws_lambda_request_id>(?:[^\\s]+))", [("aws_lambda_log_stream_id", "aws.lambda.log_stream_id"), ("aws_lambda_request_id", "aws.lambda.request_id")]).extract_into(&input, event)? {
                                                                            // Grok pattern: ^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<message>(?:(.|\n|\t)*))
                                                                            if !cached_grok_mapped!("^(?P<aws_lambda_log_stream_id>(?:[^\\s]+))\\s+%{TIMESTAMP_ISO8601:timestamp}\\s+(?P<message>(?:(.|\n|\t)*))", [("aws_lambda_log_stream_id", "aws.lambda.log_stream_id")]).extract_into(&input, event)? {
                                                                                // Grok pattern: ^(?i)(?:%{LOGLEVEL:log.level}:?\\s*)?(?P<message>(?:(.|\n|\t)*))
                                                                                if !cached_grok!("^(?i)(?:%{LOGLEVEL:log.level}:?\\s*)?(?P<message>(?:(.|\n|\t)*))").extract_into(&input, event)? {
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("timestamp") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("timestamp") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSZ", "yyyy-MM-dd HH:mm:ss", "yyyy/MM/dd HH:mm:ss", "ISO8601"], None, None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();
            }

            // SKIPPED: condition not transpiled: ctx['@timestamp'] == null
            #[allow(unreachable_code, unused_variables)]
            if false {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("@timestamp", json!(event.get("_ingest.timestamp").map_or_else(String::new, painless_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("event.original").cloned() {
                event.set("message", v)?;
            }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("timestamp");
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
                event.set("error.message", json!(format!("Processor '{}'\n  {}with tag '{}'\n  {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
