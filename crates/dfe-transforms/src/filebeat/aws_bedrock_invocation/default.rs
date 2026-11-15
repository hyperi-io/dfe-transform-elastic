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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                event.rename("message", "event.original")?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            event.remove("host");
            event.remove("cloud");
            event.remove("container");

            parse_json_field(event, "event.original", "aws_bedrock.invocation")?;

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("cloud.service.name", json!("bedrock"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("aws_bedrock.invocation.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "aws_bedrock.invocation.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("aws_bedrock.invocation.timestamp");

            if event.has_value("aws_bedrock.invocation.operation") {
                event.rename("aws_bedrock.invocation.operation", "event.action")?;
            }

            if event.has_value("aws_bedrock.invocation.identity.arn") {
                event.rename("aws_bedrock.invocation.identity.arn", "user.id")?;
            }

            if event.has_value("aws_bedrock.invocation.accountId") {
                event.rename("aws_bedrock.invocation.accountId", "cloud.account.id")?;
            }

            if event.has_value("aws_bedrock.invocation.region") {
                event.rename("aws_bedrock.invocation.region", "cloud.region")?;
            }

            let _cond = {
                event
                    .get("aws_bedrock.invocation.artifacts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "e0b6f07003814e3d0f4c9d4986bf9e850425553e5d0f881274378557a4b5b9ab",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            event.set("gen_ai.system", json!("aws"))?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("gen_ai.response.timestamp", v)?;
            }

            event.set("gen_ai.prompt", json!(""))?;

            event.set("gen_ai.completion", json!(""))?;

            event.set("gen_ai.performance.request_size", json!(0))?;

            event.set("gen_ai.performance.response_size", json!(0))?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "b0012de32a76e5529c8000d241aa427b7cf151963a73d15b6fda9f20e940e513",
                    r#"{"params":null,"known":[]}"#
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "ee41e28a79b4dd2788d42f3a1de274491bc8f0c1f7ddf9faf938f85ae66ec661",
                    r#"{"params":{"RenameKeys":{"names":"Params","coerce":null,"rename_key":null,"write":{"InPlace":"aws_bedrock"}}},"known":[]}"#
                ),
                cached_params!(
                    "{\"modelId\":\"model_id\",\"inputBodyJson\":\"input_body_json\",\"inputContentType\":\"input_content_type\",\"inputTokenCount\":\"input_token_count\",\"outputBodyJson\":\"output_body_json\",\"outputContentType\":\"output_content_type\",\"outputTokenCount\":\"output_token_count\",\"requestId\":\"request_id\",\"schemaType\":\"schema_type\",\"schemaVersion\":\"schema_version\",\"errorCode\":\"error_code\",\"imageGenerationConfig\":\"image_generation_config\",\"cfgScale\":\"cfg_scale\",\"numberOfImages\":\"number_of_images\",\"imageVariationParams\":\"image_variation_params\",\"inputBodyS3Path\":\"input_body_s3_path\",\"outputBodyS3Path\":\"output_body_s3_path\",\"taskType\":\"task_type\",\"finishReason\":\"finish_reason\",\"amazon-bedrock-invocationMetrics\":\"amazon_bedrock_invocation_metrics\",\"firstByteLatency\":\"first_byte_latency\",\"invocationLatency\":\"invocation_latency\",\"amazon-bedrock-guardrailAction\":\"amazon_bedrock_guardrail_action\",\"amazon-bedrock-trace\":\"amazon_bedrock_trace\",\"contentPolicy\":\"content_policy\",\"wordPolicy\":\"word_policy\",\"topicPolicy\":\"topic_policy\",\"customWords\":\"custom_words\",\"sensitiveInformationPolicy\":\"sensitive_information_policy\",\"contextualGroundingPolicy\":\"contextual_grounding_policy\",\"invocationMetrics\":\"invocation_metrics\",\"guardrailCoverage\":\"guardrail_coverage\",\"textCharacters\":\"text_characters\",\"guardrailProcessingLatency\":\"guardrail_processing_latency\",\"contentPolicyUnits\":\"content_policy_units\",\"contextualGroundingPolicyUnits\":\"contextual_grounding_policy_units\",\"sensitiveInformationPolicyFreeUnits\":\"sensitive_information_policy_free_units\",\"sensitiveInformationPolicyUnits\":\"sensitive_information_policy_units\",\"topicPolicyUnits\":\"topic_policy_units\",\"wordPolicyUnits\":\"word_policy_units\",\"piiEntities\":\"pii_entities\",\"normalizeIndex\":\"normalize_index\",\"latencyMs\":\"latency_ms\",\"stopReason\":\"stop_reason\"}"
                ),
            )?;

            let _cond = {
                event
                    .get("aws_bedrock.invocation.system")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "97bec2516ca64fa920dbf36d0f45418a17aaff41a23b890c76baac4895f19184",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("aws_bedrock.invocation.messages") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "a75500797ef8531652247312a2135011ba515766d853adef6f303b30765e4e25",
                        r#"{"params":null,"known":["JoinOptional"]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("aws_bedrock.invocation.messages") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "3c116f6d604137b7723606a7685e33c1b58c5f9543b305942b402001a01210db",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("aws_bedrock.invocation.model_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("aws_bedrock.invocation.model_id").cloned() {
                        event.set("gen_ai.request.model.id", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("aws_bedrock.invocation.input.input_body_json")
                    && event
                        .get("gen_ai.request.model.id")
                        .is_some_and(|v| v.is_string())
                    && event
                        .get("gen_ai.request.model.id")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("."))
                            }
                            serde_json::Value::String(s) => s.contains("."),
                            _ => false,
                        })
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "1516b826d27996a34f9baeaa20650d1c5d06ae1e8a0d206c68f7017ec3aab05b",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("aws_bedrock.invocation.input.input_token_count") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("aws_bedrock.invocation.input.input_token_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "aws_bedrock.invocation.input.input_token_count".into(),
                                message,
                            }
                        })?;
                        event.set("gen_ai.usage.prompt_tokens", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("aws_bedrock.invocation.input.input_body_json.top_k") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("aws_bedrock.invocation.input.input_body_json.top_k")
                    {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "aws_bedrock.invocation.input.input_body_json.top_k".into(),
                                message,
                            }
                        })?;
                        event.set("gen_ai.request.top_k", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("aws_bedrock.invocation.input.input_body_json.top_p") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("aws_bedrock.invocation.input.input_body_json.top_p")
                    {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "aws_bedrock.invocation.input.input_body_json.top_p".into(),
                                message,
                            }
                        })?;
                        event.set("gen_ai.request.top_p", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("aws_bedrock.invocation.input.input_body_json.temperature") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("aws_bedrock.invocation.input.input_body_json.temperature")
                    {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "aws_bedrock.invocation.input.input_body_json.temperature"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("gen_ai.request.temperature", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("aws_bedrock.invocation.input.input_body_json.max_tokens") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("aws_bedrock.invocation.input.input_body_json.max_tokens")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "aws_bedrock.invocation.input.input_body_json.max_tokens"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("gen_ai.request.max_tokens", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("aws_bedrock.invocation.output.output_token_count") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("aws_bedrock.invocation.output.output_token_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "aws_bedrock.invocation.output.output_token_count".into(),
                                message,
                            }
                        })?;
                        event.set("gen_ai.usage.completion_tokens", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("aws_bedrock.invocation.request_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("aws_bedrock.invocation.request_id").cloned() {
                        event.set("gen_ai.request.id", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                !(event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array()))
                    && event.has_value("aws_bedrock.invocation.output.output_body_json.id")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("aws_bedrock.invocation.output.output_body_json.id")
                        .cloned()
                    {
                        event.set("gen_ai.response.id", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("aws_bedrock.invocation.input.input_body_json.messages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "0d7f6e83a19c5507f12defde7297abf91098cbeb2da2dcb431e86e0cfd84ba3a",
                        r#"{"params":null,"known":[{"AppendEach":{"source":"aws_bedrock.invocation.input.input_body_json.messages","target":"aws_bedrock.invocation.input.input_body_json.messages","map_keys":[],"separator":",","deduped":false}}]}"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "4aee7cadeb7406b510bca77f329675668740eb4cc1c51c530a5dcce3c50be02c",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "54a89f8995c11f15f43ece944fbefac779fcfdcd6414c5acdad9ca0c5da35f28",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = {
                !(event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array()))
                    && event.has_value(
                        "aws_bedrock.invocation.output.output_body_json.output.message.role",
                    )
            };
            if _cond {
                if let Some(v) = event
                    .get("aws_bedrock.invocation.output.output_body_json.output.message.role")
                    .cloned()
                {
                    event.set("gen_ai.request.model.role", v)?;
                }
            }

            let _cond = {
                !(event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array()))
                    && event.has_value("aws_bedrock.invocation.output.output_body_json.stop_reason")
            };
            if _cond {
                if let Some(v) = event
                    .get("aws_bedrock.invocation.output.output_body_json.stop_reason")
                    .cloned()
                {
                    event.set("gen_ai.response.finish_reasons", v)?;
                }
            }

            let _cond = {
                event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "c70fc35a7a19eda11d89a23b85431ef43ec2909384a96d144a039647c0dde4ce",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                if let Some(v) = event.get("user.id").cloned() {
                    event.set("gen_ai.user.id", v)?;
                }
            }

            let _cond = { event.has_value("aws_bedrock.invocation.error_code") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("aws_bedrock.invocation.error_code").cloned() {
                        event.set("gen_ai.response.error_code", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("aws_bedrock.invocation.output.output_body_json") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "5a2ab6d2f8a87683d36cae3c6554e3d251d015b984a461fb14f1d6e65f68aacf",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("gen_ai.response.error_code") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if !event.has("event.outcome") {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get("gen_ai.prompt").is_some_and(|v| v.is_string()) };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "7a6359986918764f5954609ab2b9508bc3f557f09a2ed9eb602ce422b82a6009",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "1d455a1b48f0edc915a1dea4faee8ac39130fd5196cf1b1f02f4b9d400fdad75",
                    r#"{"params":null,"known":[]}"#
                ),
            )?;

            let _cond = {
                event
                    .get("gen_ai.completion")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "cd5488bb3ae04bd38827b9edb013c6fbc2676d1564246fd311dc64e4d9daea4e",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            // The pipeline's script, emitted as the runtime's equivalent
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("aws_bedrock.invocation.input.input_body_json");
                event.remove("aws_bedrock.invocation.output.output_body_json");
            }

            let _cond = {
                event.get("event.original").is_some_and(|v| v.is_string())
                    && event
                        .get_as_string("event.original")
                        .is_some_and(|s| s.len() > 32766)
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "3567f9d7c3b51975767b2a0a58678701a97b1c62215c653de82afccde8aa6e6c",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
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
