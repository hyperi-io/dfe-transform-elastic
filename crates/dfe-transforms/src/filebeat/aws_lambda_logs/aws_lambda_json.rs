// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `aws_lambda_json` pipeline.
pub struct AwsLambdaJson;

impl Transform for AwsLambdaJson {
    fn name(&self) -> &str {
        "aws_lambda_json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value = serde_json::from_str(&s)
                        .map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("parsed", parsed)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("parsed.timestamp") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("parsed.timestamp") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss,SSSZ"], None, None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("parsed.time") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("parsed.time") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["ISO8601"], None, None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("parsed._aws.Timestamp") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("parsed._aws.Timestamp") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["UNIX_MS"], None, None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("parsed.record") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("parsed.record").cloned() {
                event.set("aws.lambda.message", v)?;
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("parsed._aws") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("parsed._aws").cloned() {
                event.set("aws.lambda.message", v)?;
            }
                Ok(())
            })();
            }

                if event.has("parsed.service") {
                    event.rename("parsed.service", "service.name")?;
                }

                if event.has("parsed.level") {
                    event.rename("parsed.level", "log.level")?;
                }

                if event.has("parsed.requestId") {
                    event.rename("parsed.requestId", "aws.lambda.request_id")?;
                }

            let _cond = { event.get("parsed").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: // Flatten ctx.parsed.stackTrace if it's a list\nif (ctx.parsed.containsKey('stackTrace') && ctx.parsed.stackTrace instanceof List) {\n    def completionText = new StringBuilder();\n    for (int i = 0; i < ctx.parsed.stackTrace.length; i++) {\n        completionText.append(ctx.parsed.stackTrace[i]);\n        if (i != ctx.parsed.stackTrace.length - 1) {\n            completionText.append(\"\\\\n\");\n        }\n    }\n    ctx.parsed.stack_trace_flattened = completionText.toString();\n}\n// Flatten ctx.parsed.message.stackTrace if message is a map and contains a list\nif (ctx.parsed.containsKey('message') && ctx.parsed.message instanceof Map && ctx.parsed.message.containsKey('stackTrace') && ctx.parsed.message.stackTrace instanceof List) {\n    def completionText = new StringBuilder();\n    for (int i = 0; i < ctx.parsed.message.stackTrace.length; i++) {\n        completionText.append(ctx.parsed.message.stackTrace[i]);\n        if (i != ctx.parsed.message.stackTrace.length - 1) {\n            completionText.append(\"\\\\n\");\n        }\n    }\n    ctx.parsed.message.stack_trace_flattened = completionText.toString();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"// Flatten ctx.parsed.stackTrace if it's a list\nif (ctx.parsed.containsKey('stackTrace') && ctx.parsed.stackTrace instanceof List) {\n    def completionText = new StringBuilder();\n    for (int i = 0; i < ctx.parsed.stackTrace.length; i++) {\n        completionText.append(ctx.parsed.stackTrace[i]);\n        if (i != ctx.parsed.stackTrace.length - 1) {\n            completionText.append(\"\\\\n\");\n        }\n    }\n    ctx.parsed.stack_trace_flattened = completionText.toString();\n}\n// Flatten ctx.parsed.message.stackTrace if message is a map and contains a list\nif (ctx.parsed.containsKey('message') && ctx.parsed.message instanceof Map && ctx.parsed.message.containsKey('stackTrace') && ctx.parsed.message.stackTrace instanceof List) {\n    def completionText = new StringBuilder();\n    for (int i = 0; i < ctx.parsed.message.stackTrace.length; i++) {\n        completionText.append(ctx.parsed.message.stackTrace[i]);\n        if (i != ctx.parsed.message.stackTrace.length - 1) {\n            completionText.append(\"\\\\n\");\n        }\n    }\n    ctx.parsed.message.stack_trace_flattened = completionText.toString();\n}\n"#))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.functionArn") {
                    event.rename("parsed.record.functionArn", "aws.lambda.arn")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.requestId") {
                    event.rename("parsed.record.requestId", "aws.lambda.request_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.version") {
                    event.rename("parsed.record.version", "aws.lambda.version")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.status") {
                    event.rename("parsed.record.status", "aws.lambda.status")?;
                }
                Ok(())
            })();

            let _cond = { event.get("parsed.record").is_some_and(|v| v.is_object()) && event.get("parsed.record.metrics").is_some_and(|v| v.is_object()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: String underscore(String s) {\n    def regex = /_?([a-z])([A-Z]+)/;\n    s = regex.matcher(s).replaceAll('$1_$2').toLowerCase();\n    String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n    String result1 = /[\\ufeff]/.matcher(result).replaceAll('');\n    return /[()]/.matcher(result1).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.parsed.record.metrics.entrySet()) {\n    out[underscore(item.getKey())] = item.getValue();\n}\nctx.aws.lambda.metrics = out\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"String underscore(String s) {\n    def regex = /_?([a-z])([A-Z]+)/;\n    s = regex.matcher(s).replaceAll('$1_$2').toLowerCase();\n    String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n    String result1 = /[\\ufeff]/.matcher(result).replaceAll('');\n    return /[()]/.matcher(result1).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.parsed.record.metrics.entrySet()) {\n    out[underscore(item.getKey())] = item.getValue();\n}\nctx.aws.lambda.metrics = out\n"#))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.tracing.spanId") {
                    event.rename("parsed.record.tracing.spanId", "aws.lambda.tracing.span_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.tracing.type") {
                    event.rename("parsed.record.tracing.type", "aws.lambda.tracing.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.tracing.value") {
                    event.rename("parsed.record.tracing.value", "aws.lambda.tracing.value")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.errorType") {
                    event.rename("parsed.record.errorType", "aws.lambda.error.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.initializationType") {
                    event.rename("parsed.record.initializationType", "aws.lambda.initialization_type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.phase") {
                    event.rename("parsed.record.phase", "aws.lambda.phase")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.functionVersion") {
                    event.rename("parsed.record.functionVersion", "aws.lambda.version")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.functionName") {
                    event.rename("parsed.record.functionName", "aws.lambda.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.instanceId") {
                    event.rename("parsed.record.instanceId", "aws.lambda.instance_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.runtimeVersion") {
                    event.rename("parsed.record.runtimeVersion", "aws.lambda.runtime_version")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.runtimeVersionArn") {
                    event.rename("parsed.record.runtimeVersionArn", "aws.lambda.runtime_version_arn")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.name") {
                    event.rename("parsed.record.name", "aws.lambda.extension.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.state") {
                    event.rename("parsed.record.state", "aws.lambda.extension.state")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.events") {
                    event.rename("parsed.record.events", "aws.lambda.extension.events")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.record.spans") {
                    event.rename("parsed.record.spans", "aws.lambda.spans")?;
                }
                Ok(())
            })();

            // SKIPPED: condition not transpiled: ctx['@timestamp'] == null
            #[allow(unreachable_code, unused_variables)]
            if false {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.time") {
                    event.rename("parsed.time", "@timestamp")?;
                }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.errorMessage") {
                    event.rename("parsed.errorMessage", "aws.lambda.error.message")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.errorType") {
                    event.rename("parsed.errorType", "aws.lambda.error.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.logger") {
                    event.rename("parsed.logger", "log.logger")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.stack_trace_flattened") {
                    event.rename("parsed.stack_trace_flattened", "aws.lambda.error.stack_trace")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.message.stack_trace_flattened") {
                    event.rename("parsed.message.stack_trace_flattened", "aws.lambda.error.stack_trace")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.message.errorType") {
                    event.rename("parsed.message.errorType", "aws.lambda.error.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.message.errorMessage") {
                    event.rename("parsed.message.errorMessage", "aws.lambda.error.message")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.time") {
                    event.rename("parsed.time", "@timestamp")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.timestamp") {
                    event.rename("parsed.timestamp", "@timestamp")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.type") {
                    event.rename("parsed.type", "aws.lambda.event_type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.cold_start") {
                    event.rename("parsed.cold_start", "aws.lambda.cold_start")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.correlation_id") {
                    event.rename("parsed.correlation_id", "aws.lambda.correlation_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.function_arn") {
                    event.rename("parsed.function_arn", "aws.lambda.arn")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("parsed.function_memory_size") {
                if let Some(val) = event.get("parsed.function_memory_size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "parsed.function_memory_size".into(),
                            message,
                        })?;
                    event.set("aws.lambda.metrics.memory_size_mb", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.function_name") {
                    event.rename("parsed.function_name", "aws.lambda.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.function_request_id") {
                    event.rename("parsed.function_request_id", "aws.lambda.request_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.location") {
                    event.rename("parsed.location", "aws.lambda.error.location")?;
                }
                Ok(())
            })();

            let _cond = { event.get("parsed.message").is_some_and(|v| v.is_object()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.message") {
                    event.rename("parsed.message", "aws.lambda.message")?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("parsed.message") && !(event.get("parsed.message").is_some_and(|v| v.is_object())) };
            if _cond {
            if let Some(v) = event.get("parsed.message").cloned() {
                event.set("message", v)?;
            }
            }

            let _cond = { !event.has_value("message") && !event.has_value("aws.lambda.message") };
            if _cond {
            if let Some(v) = event.get("event.original").cloned() {
                event.set("message", v)?;
            }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.users") {
                    event.rename("parsed.users", "aws.lambda.users")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.tracing.xray_trace_id") {
                    event.rename("parsed.tracing.xray_trace_id", "aws.lambda.xray_trace_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.ColdStart") {
                    event.rename("parsed.ColdStart", "aws.lambda.cold_start_int")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.FunctionName") {
                    event.rename("parsed.FunctionName", "aws.lambda.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.Service") {
                    event.rename("parsed.Service", "aws.lambda.service.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.executionEnvironment") {
                    event.rename("parsed.executionEnvironment", "aws.lambda.execution_environment")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.functionVersion") {
                    event.rename("parsed.functionVersion", "aws.lambda.version")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.logStreamId") {
                    event.rename("parsed.logStreamId", "aws.lambda.log_stream_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.traceId") {
                    event.rename("parsed.traceId", "aws.lambda.trace_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed.AWSRequestId") {
                    event.rename("parsed.AWSRequestId", "aws.lambda.aws_request_id")?;
                }
                Ok(())
            })();

            let _cond = { event.get("parsed").is_some_and(|v| v.is_object()) && !(event.has("parsed.record") || event.has("parsed._aws") || event.has("parsed.time")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("parsed") {
                    event.rename("parsed", "aws.lambda.message")?;
                }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("parsed");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("parsed._aws");
                Ok(())
            })();

                // Painless script
                // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n"#))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.set("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
