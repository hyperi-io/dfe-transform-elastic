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
            event.set("ecs.version", json!("8.3.1"))?;

            event.set("service.type", json!("elastic_package_registry"))?;

            event.set("event.kind", json!("metric"))?;

            if event.has_value("prometheus") {
                event.rename("prometheus", "tmp")?;
            }

            if event.has_value("tmp.labels") {
                event.rename("tmp.labels", "package_registry.labels")?;
            }

            if event.has_value("tmp.epr_indexer_get_duration_seconds") {
                event.rename(
                    "tmp.epr_indexer_get_duration_seconds",
                    "package_registry.indexer.get_duration_seconds",
                )?;
            }

            if event.has_value("tmp.epr_storage_indexer_update_index_error_total") {
                event.rename(
                    "tmp.epr_storage_indexer_update_index_error_total",
                    "package_registry.storage_indexer.update_index_error_total",
                )?;
            }

            if event.has_value("tmp.epr_storage_indexer_update_index_success_total") {
                event.rename(
                    "tmp.epr_storage_indexer_update_index_success_total",
                    "package_registry.storage_indexer.update_index_success_total",
                )?;
            }

            if event.has_value("tmp.epr_storage_indexer_update_index_duration_seconds") {
                event.rename(
                    "tmp.epr_storage_indexer_update_index_duration_seconds",
                    "package_registry.storage_indexer.update_index_duration_seconds",
                )?;
            }

            if event.has_value("tmp.epr_in_flight_requests.value") {
                event.rename(
                    "tmp.epr_in_flight_requests.value",
                    "package_registry.in_flight_requests",
                )?;
            }

            if event.has_value("tmp.epr_number_indexed_packages.value") {
                event.rename(
                    "tmp.epr_number_indexed_packages.value",
                    "package_registry.number_indexed_packages",
                )?;
            }

            if event.has_value("tmp.epr_storage_requests_total") {
                event.rename(
                    "tmp.epr_storage_requests_total",
                    "package_registry.storage_requests_total",
                )?;
            }

            let _cond = { event.has_value("tmp.process_start_time_seconds.value") };
            if _cond {
                if event.has_value("tmp.process_start_time_seconds.value") {
                    event.rename(
                        "tmp.process_start_time_seconds.value",
                        "package_registry.start_time_seconds",
                    )?;
                }
            }

            if event.has_value("tmp.epr_http_request_duration_seconds.histogram") {
                event.rename(
                    "tmp.epr_http_request_duration_seconds.histogram",
                    "package_registry.http.request_duration_seconds.histogram",
                )?;
            }

            if event.has_value("tmp.epr_http_request_size_bytes.histogram") {
                event.rename(
                    "tmp.epr_http_request_size_bytes.histogram",
                    "package_registry.http.request_size_bytes.histogram",
                )?;
            }

            if event.has_value("tmp.epr_http_response_size_bytes.histogram") {
                event.rename(
                    "tmp.epr_http_response_size_bytes.histogram",
                    "package_registry.http.response_size_bytes.histogram",
                )?;
            }

            if event.has_value("tmp.epr_http_requests_total") {
                event.rename(
                    "tmp.epr_http_requests_total",
                    "package_registry.http_requests_total",
                )?;
            }

            event.remove("tmp");

            let _cond = { event.has_value("package_registry.start_time_seconds") };
            if _cond {
                if let Some(date_str) = event.get_as_string("package_registry.start_time_seconds") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("package_registry.start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "package_registry.start_time_seconds".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            // Painless script
            // Source: if (ctx.package_registry.containsKey(\"start_time\") && ctx.containsKey(\"@timestamp\")) {\n    ZonedDateTime created = ZonedDateTime.parse(ctx[\"@timestamp\"]);\n    ZonedDateTime start = ZonedDateTime.parse(ctx.package_registry['start_time']);\n    long uptime = ChronoUnit.MILLIS.between(start, created)/1000;\n    ctx.package_registry[\"uptime\"] = uptime;\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.package_registry.containsKey(\"start_time\") && ctx.containsKey(\"@timestamp\")) {\n    ZonedDateTime created = ZonedDateTime.parse(ctx[\"@timestamp\"]);\n    ZonedDateTime start = ZonedDateTime.parse(ctx.package_registry['start_time']);\n    long uptime = ChronoUnit.MILLIS.between(start, created)/1000;\n    ctx.package_registry[\"uptime\"] = uptime;\n}"#
                ),
            )?;

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
