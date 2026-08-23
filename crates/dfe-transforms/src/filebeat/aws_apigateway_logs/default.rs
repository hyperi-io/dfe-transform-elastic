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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "aws.apigateway")?;
                Ok(())
            })();

            if event.has("aws.apigateway.requestId") {
                event.rename("aws.apigateway.requestId", "aws.apigateway.request_id")?;
            }

            if event.has("aws.apigateway.responseLength") {
                event.rename(
                    "aws.apigateway.responseLength",
                    "aws.apigateway.response_length",
                )?;
            }

            if event.has("aws.apigateway.requestTime") {
                event.rename("aws.apigateway.requestTime", "aws.apigateway.request_time")?;
            }

            if event.has("aws.apigateway.httpMethod") {
                event.rename("aws.apigateway.httpMethod", "aws.apigateway.http_method")?;
            }

            if event.has("aws.apigateway.routeKey") {
                event.rename("aws.apigateway.routeKey", "aws.apigateway.route_key")?;
            }

            if event.has("aws.apigateway.ip") {
                event.rename("aws.apigateway.ip", "aws.apigateway.ip_address")?;
            }

            if event.has("aws.apigateway.resourcePath") {
                event.rename(
                    "aws.apigateway.resourcePath",
                    "aws.apigateway.resource_path",
                )?;
            }

            if event.has("aws.apigateway.connectionId") {
                event.rename(
                    "aws.apigateway.connectionId",
                    "aws.apigateway.connection_id",
                )?;
            }

            if event.has("aws.apigateway.eventType") {
                event.rename("aws.apigateway.eventType", "aws.apigateway.event_type")?;
            }

            if event.has("aws.apigateway.apiId") {
                event.rename("aws.apigateway.apiId", "aws.apigateway.api_id")?;
            }

            if event.has("aws.apigateway.domainName") {
                event.rename("aws.apigateway.domainName", "aws.apigateway.domain_name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("aws.apigateway.ip_address") {
                    // Grok pattern: %{IPORHOST:aws.apigateway.ip_address}
                    if !cached_grok!("%{IPORHOST:aws.apigateway.ip_address}")
                        .extract_into(&input, event)?
                    {}
                }
                Ok(())
            })();

            if event.has_value("aws.apigateway.ip_address") {
                if let Some(val) = event.get("aws.apigateway.ip_address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "aws.apigateway.ip_address".into(),
                            message,
                        })?;
                    event.set("aws.apigateway.ip_address", converted)?;
                }
            }

            if event.has_value("aws.apigateway.response_length") {
                if let Some(val) = event.get("aws.apigateway.response_length") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "aws.apigateway.response_length".into(),
                            message,
                        }
                    })?;
                    event.set("aws.apigateway.response_length", converted)?;
                }
            }

            if event.has_value("aws.apigateway.status") {
                if let Some(val) = event.get("aws.apigateway.status") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "aws.apigateway.status".into(),
                            message,
                        }
                    })?;
                    event.set("aws.apigateway.status", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("aws.apigateway.request_time") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["dd/MMM/yyyy:H:m:s Z"], None, None)
                    {
                        event.set("aws.apigateway.request_time", parsed)?;
                    }
                }
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
