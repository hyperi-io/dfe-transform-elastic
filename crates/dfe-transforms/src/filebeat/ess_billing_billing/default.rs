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
            let _cond = { !event.has_value("ess.billing") && !event.has_value("error.message") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.remove("ess.billing.quantities");

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            if let Some(date_str) = event.get_as_string("ess.billing.from") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ess.billing.from".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("ess.billing.deployment_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.from") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.sku") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.to") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.total_ecu") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if let Some(v) = event
                .get("ess.billing.organization_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if let Some(v) = event
                .get("ess.billing.deployment_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if let Some(v) = event.get("ess.billing.deployment_id").cloned() {
                event.set("cloud.instance.name", v)?;
            }

            if let Some(v) = event
                .get("ess.billing.deployment_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.name", v)?;
            }

            let _cond = {
                event.get_str("ess.billing.type") == Some("capacity")
                    && event.get_str("ess.billing.deployment_type") == Some("deployment")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("ess.billing.sku") {
                        // Grok pattern: (?:Cloud-Enterprise_)?(?P<cloud_provider>(?:aws|gcp|azure|global))\\.%{NOTSPACE:cloud.machine.type}_%{NOTSPACE:cloud.region}_%{NUMBER:ess.billing.ram_per_zone:int}_%{NUMBER:ess.billing.zone_count:int}
                        if !cached_grok_mapped!("(?:Cloud-Enterprise_)?(?P<cloud_provider>(?:aws|gcp|azure|global))\\.%{NOTSPACE:cloud.machine.type}_%{NOTSPACE:cloud.region}_%{NUMBER:ess.billing.ram_per_zone:int}_%{NUMBER:ess.billing.zone_count:int}", [("cloud_provider", "cloud.provider")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("ess.billing.type") != Some("capacity")
                    && event.get_str("ess.billing.deployment_type") == Some("deployment")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("ess.billing.sku") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(".") else {
                                break 'dissect false;
                            };
                            captured.push(("cloud.provider", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(".") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("ess.billing.deployment_type") != Some("deployment") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("ess.billing.sku") {
                        // Grok pattern: %{DATA}\\.(?P<ess_billing_cloud_service_type>[^_]+)(?:_(?P<cloud_region>.+))?
                        if !cached_grok_mapped!("%{DATA}\\.(?P<ess_billing_cloud_service_type>[^_]+)(?:_(?P<cloud_region>.+))?", [("ess_billing_cloud_service_type", "ess.billing.cloud.service.type"), ("cloud_region", "cloud.region")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("ess.billing.deployment_type") != Some("deployment") };
            if _cond {
                event.set("cloud.provider", json!("serverless"))?;
            }

            if event.has_value("ess.billing.ram_per_zone") {
                if let Some(val) = event.get("ess.billing.ram_per_zone") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "ess.billing.ram_per_zone".into(),
                            message,
                        }
                    })?;
                    event.set("ess.billing.ram_per_zone", converted)?;
                }
            }

            if event.has_value("ess.billing.zone_count") {
                if let Some(val) = event.get("ess.billing.zone_count") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "ess.billing.zone_count".into(),
                            message,
                        }
                    })?;
                    event.set("ess.billing.zone_count", converted)?;
                }
            }

            event.set("cloud.geo", cached_params!("{}").clone())?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "fd78808f86da6362137b5fe4698b42eab1a6c9638b0cec3d335575818bb4d0b7",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { !event.has_value("cloud.geo.name") };
            if _cond {
                event.set(
                    "cloud.geo",
                    cached_params!(
                        "{\"name\":\"unknown\",\"location\":{\"lon\":135.0,\"lat\":90.0}}"
                    )
                    .clone(),
                )?;
            }

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
                        "Processor '{}' {}failed with message '{}'",
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
