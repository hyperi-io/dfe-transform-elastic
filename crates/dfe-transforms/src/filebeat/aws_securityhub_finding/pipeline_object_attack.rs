// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_attack` pipeline.
pub struct PipelineObjectAttack;

impl Transform for PipelineObjectAttack {
    fn name(&self) -> &str {
        "pipeline_object_attack"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("_ingest._value.tactic.uid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("_ingest._value.tactic.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.tactic.reference",
                        json!(
                            event
                                .get("_ingest._value.tactic.src_url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.technique.id",
                        json!(
                            event
                                .get("_ingest._value.technique.uid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.technique.name",
                        json!(
                            event
                                .get("_ingest._value.technique.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.technique.reference",
                        json!(
                            event
                                .get("_ingest._value.technique.src_url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.technique.subtechnique.id",
                        json!(
                            event
                                .get("_ingest._value.sub_technique.uid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.technique.subtechnique.name",
                        json!(
                            event
                                .get("_ingest._value.sub_technique.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.attacks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.attacks", |event| {
                    event.append_unique(
                        "threat.technique.subtechnique.reference",
                        json!(
                            event
                                .get("_ingest._value.sub_technique.src_url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
