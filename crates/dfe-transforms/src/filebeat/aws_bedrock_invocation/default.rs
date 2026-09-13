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
                // Painless script
                // Source: for (int i = 0; i < ctx.aws_bedrock.invocation.artifacts.length; i++) {\n  if (ctx.aws_bedrock.invocation.artifacts[i].base64 instanceof String && ctx.aws_bedrock.invocation.artifacts[i].base64.length() > 32766) {\n    ctx.aws_bedrock.invocation.artifacts[i].base64_massive_hash = ctx.aws_bedrock.invocation.artifacts[i].base64.sha1();\n    ctx.aws_bedrock.invocation.artifacts[i].base64_massive_length = ctx.aws_bedrock.invocation.artifacts[i].base64.length();\n    ctx.aws_bedrock.invocation.artifacts[i].remove(\"base64\");\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (int i = 0; i < ctx.aws_bedrock.invocation.artifacts.length; i++) {\n  if (ctx.aws_bedrock.invocation.artifacts[i].base64 instanceof String && ctx.aws_bedrock.invocation.artifacts[i].base64.length() > 32766) {\n    ctx.aws_bedrock.invocation.artifacts[i].base64_massive_hash = ctx.aws_bedrock.invocation.artifacts[i].base64.sha1();\n    ctx.aws_bedrock.invocation.artifacts[i].base64_massive_length = ctx.aws_bedrock.invocation.artifacts[i].base64.length();\n    ctx.aws_bedrock.invocation.artifacts[i].remove(\"base64\");\n  }\n}\n"#
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

            // Painless script
            // Source: String json(def val) {\n  if (val == null) {\n    return null;\n  }\n  if (val instanceof Map) {\n    String out = '{';\n    List keyList = new ArrayList(val.keySet());\n    Collections.sort(keyList);\n    for (int i=0; i < keyList.length; i++){\n      if (i != 0) {\n        out += ',';\n      }\n      String key = keyList[i];\n      out += '\"' +  key + '\"';\n      if (val[key] instanceof String){\n        out += ':\"' + val[key] + '\"';\n      } else {\n        out += ':' + json(val[key]);\n      }\n    }\n    return out + '}';\n  } else if (val instanceof ArrayList) {\n    String out = '[';\n    for (int i = 0; i < val.length; i++){\n      if (i != 0) {\n          out += ',';\n      }\n      out = out + json(val[i]);\n    }\n    return out + ']';\n  } else if (val instanceof String) {\n    return '\"' + val + '\"'; // This may not be valid JSON depending on code points in val. TODO: Add escape helper.\n  }\n  return val.toString();\n}\nctx.gen_ai.prompt = json(ctx.aws_bedrock?.invocation?.input?.inputBodyJson);\nctx.gen_ai.completion = json(ctx.aws_bedrock?.invocation?.output?.outputBodyJson);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String json(def val) {\n  if (val == null) {\n    return null;\n  }\n  if (val instanceof Map) {\n    String out = '{';\n    List keyList = new ArrayList(val.keySet());\n    Collections.sort(keyList);\n    for (int i=0; i < keyList.length; i++){\n      if (i != 0) {\n        out += ',';\n      }\n      String key = keyList[i];\n      out += '\"' +  key + '\"';\n      if (val[key] instanceof String){\n        out += ':\"' + val[key] + '\"';\n      } else {\n        out += ':' + json(val[key]);\n      }\n    }\n    return out + '}';\n  } else if (val instanceof ArrayList) {\n    String out = '[';\n    for (int i = 0; i < val.length; i++){\n      if (i != 0) {\n          out += ',';\n      }\n      out = out + json(val[i]);\n    }\n    return out + ']';\n  } else if (val instanceof String) {\n    return '\"' + val + '\"'; // This may not be valid JSON depending on code points in val. TODO: Add escape helper.\n  }\n  return val.toString();\n}\nctx.gen_ai.prompt = json(ctx.aws_bedrock?.invocation?.input?.inputBodyJson);\nctx.gen_ai.completion = json(ctx.aws_bedrock?.invocation?.output?.outputBodyJson);\n"#
                ),
            )?;

            // Painless script
            // Source: def renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\n\nctx.aws_bedrock = renameKeys(ctx.aws_bedrock, params)\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\n\nctx.aws_bedrock = renameKeys(ctx.aws_bedrock, params)\n"#
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
                // Painless script
                // Source: Map sys = [:];\nsys.type = 'text';\nsys.text = ctx.aws_bedrock.invocation.system;\nctx.aws_bedrock.invocation.system = sys;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map sys = [:];\nsys.type = 'text';\nsys.text = ctx.aws_bedrock.invocation.system;\nctx.aws_bedrock.invocation.system = sys;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("aws_bedrock.invocation.messages") };
            if _cond {
                // Painless script
                // Source: // Handle messages as either an array or single object\ndef messagesList = ctx.aws_bedrock.invocation.messages instanceof List\n  ? ctx.aws_bedrock.invocation.messages\n  : [ctx.aws_bedrock.invocation.messages];\n\nfor (def message : messagesList) {\n  if (!(message instanceof Map)) {\n    continue;\n  }\n\n  // If content is a string, promote it to an object with type and text\n  if (message.content instanceof String) {\n    Map contentObj = [:];\n    contentObj.type = 'text';\n    contentObj.text = message.content;\n    message.content = contentObj;\n  }\n  // If content is already an array, ensure each item has type and text fields\n  else if (message.content instanceof List) {\n    for (def contentItem : message.content) {\n      if (!(contentItem instanceof Map)) {\n        continue;\n      }\n\n      // Ensure type field exists\n      if (!contentItem.containsKey('type')) {\n        contentItem.type = 'text';\n      }\n\n      // Check if this item already has a text field with actual content\n      boolean hasExistingText = contentItem.containsKey('text') &&\n                               contentItem.text instanceof String &&\n                               contentItem.text != '';\n\n      // If no text field exists, we need to create one\n      if (!hasExistingText) {\n        // Handle nested content field (e.g., in tool_result)\n        if (contentItem.containsKey('content') && !(contentItem.content instanceof String)) {\n          contentItem.text = Json.dump(contentItem.content);\n        } else if (contentItem.containsKey('content') && contentItem.content instanceof String) {\n          contentItem.text = contentItem.content;\n        } else {\n          // Serialize all fields except type\n          Map itemCopy = [:];\n          for (def entry : contentItem.entrySet()) {\n            if (entry.getKey() != 'type') {\n              itemCopy[entry.getKey()] = entry.getValue();\n            }\n          }\n          contentItem.text = Json.dump(itemCopy);\n        }\n\n        // Remove all fields except type and text to avoid duplication\n        def keysToRemove = new ArrayList(contentItem.keySet());\n        for (def key : keysToRemove) {\n          if (key != 'type' && key != 'text') {\n            contentItem.remove(key);\n          }\n        }\n      }\n    }\n  }\n  // If content is an object, ensure it has type and text fields\n  else if (message.content instanceof Map) {\n    if (!message.content.containsKey('type')) {\n      message.content.type = 'text';\n    }\n\n    boolean hasExistingText = message.content.containsKey('text') &&\n                             message.content.text instanceof String &&\n                             message.content.text != '';\n\n    if (!hasExistingText) {\n      if (message.content.containsKey('content')) {\n        if (message.content.content instanceof String) {\n          message.content.text = message.content.content;\n        } else {\n          message.content.text = Json.dump(message.content.content);\n        }\n      } else {\n        Map contentCopy = [:];\n        for (def entry : message.content.entrySet()) {\n          if (entry.getKey() != 'type') {\n            contentCopy[entry.getKey()] = entry.getValue();\n          }\n        }\n        message.content.text = Json.dump(contentCopy);\n      }\n\n      def keysToRemove = new ArrayList(message.content.keySet());\n      for (def key : keysToRemove) {\n        if (key != 'type' && key != 'text') {\n          message.content.remove(key);\n        }\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Handle messages as either an array or single object\ndef messagesList = ctx.aws_bedrock.invocation.messages instanceof List\n  ? ctx.aws_bedrock.invocation.messages\n  : [ctx.aws_bedrock.invocation.messages];\n\nfor (def message : messagesList) {\n  if (!(message instanceof Map)) {\n    continue;\n  }\n\n  // If content is a string, promote it to an object with type and text\n  if (message.content instanceof String) {\n    Map contentObj = [:];\n    contentObj.type = 'text';\n    contentObj.text = message.content;\n    message.content = contentObj;\n  }\n  // If content is already an array, ensure each item has type and text fields\n  else if (message.content instanceof List) {\n    for (def contentItem : message.content) {\n      if (!(contentItem instanceof Map)) {\n        continue;\n      }\n\n      // Ensure type field exists\n      if (!contentItem.containsKey('type')) {\n        contentItem.type = 'text';\n      }\n\n      // Check if this item already has a text field with actual content\n      boolean hasExistingText = contentItem.containsKey('text') &&\n                               contentItem.text instanceof String &&\n                               contentItem.text != '';\n\n      // If no text field exists, we need to create one\n      if (!hasExistingText) {\n        // Handle nested content field (e.g., in tool_result)\n        if (contentItem.containsKey('content') && !(contentItem.content instanceof String)) {\n          contentItem.text = Json.dump(contentItem.content);\n        } else if (contentItem.containsKey('content') && contentItem.content instanceof String) {\n          contentItem.text = contentItem.content;\n        } else {\n          // Serialize all fields except type\n          Map itemCopy = [:];\n          for (def entry : contentItem.entrySet()) {\n            if (entry.getKey() != 'type') {\n              itemCopy[entry.getKey()] = entry.getValue();\n            }\n          }\n          contentItem.text = Json.dump(itemCopy);\n        }\n\n        // Remove all fields except type and text to avoid duplication\n        def keysToRemove = new ArrayList(contentItem.keySet());\n        for (def key : keysToRemove) {\n          if (key != 'type' && key != 'text') {\n            contentItem.remove(key);\n          }\n        }\n      }\n    }\n  }\n  // If content is an object, ensure it has type and text fields\n  else if (message.content instanceof Map) {\n    if (!message.content.containsKey('type')) {\n      message.content.type = 'text';\n    }\n\n    boolean hasExistingText = message.content.containsKey('text') &&\n                             message.content.text instanceof String &&\n                             message.content.text != '';\n\n    if (!hasExistingText) {\n      if (message.content.containsKey('content')) {\n        if (message.content.content instanceof String) {\n          message.content.text = message.content.content;\n        } else {\n          message.content.text = Json.dump(message.content.content);\n        }\n      } else {\n        Map contentCopy = [:];\n        for (def entry : message.content.entrySet()) {\n          if (entry.getKey() != 'type') {\n            contentCopy[entry.getKey()] = entry.getValue();\n          }\n        }\n        message.content.text = Json.dump(contentCopy);\n      }\n\n      def keysToRemove = new ArrayList(message.content.keySet());\n      for (def key : keysToRemove) {\n        if (key != 'type' && key != 'text') {\n          message.content.remove(key);\n        }\n      }\n    }\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("aws_bedrock.invocation.messages") };
            if _cond {
                // Painless script
                // Source: def messagesList = ctx.aws_bedrock.invocation.messages instanceof List\n  ? ctx.aws_bedrock.invocation.messages\n  : [ctx.aws_bedrock.invocation.messages];\n\nfor (def message : messagesList) {\n  if (!(message instanceof Map) || !message.containsKey('content')) {\n    continue;\n  }\n\n  // Process content if it's an array\n  if (message.content instanceof List) {\n    for (def contentItem : message.content) {\n      if (!(contentItem instanceof Map)) {\n        continue;\n      }\n\n      // Ensure we have a text field - if not, create it from available data\n      boolean needsTextCreation = !contentItem.containsKey('text') ||\n                                 contentItem.text == null ||\n                                 contentItem.text == '';\n\n      if (needsTextCreation) {\n        if (contentItem.containsKey('content')) {\n          if (contentItem.content instanceof String) {\n            contentItem.text = contentItem.content;\n          } else {\n            contentItem.text = Json.dump(contentItem.content);\n          }\n        } else {\n          // Create text from all fields except type\n          Map itemCopy = [:];\n          for (def entry : contentItem.entrySet()) {\n            if (entry.getKey() != 'type') {\n              itemCopy[entry.getKey()] = entry.getValue();\n            }\n          }\n          if (itemCopy.size() > 0) {\n            contentItem.text = Json.dump(itemCopy);\n          } else {\n            contentItem.text = '';\n          }\n        }\n      }\n\n      // Remove all fields except type and text\n      def keysToRemove = new ArrayList(contentItem.keySet());\n      for (def key : keysToRemove) {\n        if (key != 'type' && key != 'text') {\n          contentItem.remove(key);\n        }\n      }\n    }\n  }\n  // Process content if it's a single object\n  else if (message.content instanceof Map) {\n    boolean needsTextCreation = !message.content.containsKey('text') ||\n                               message.content.text == null ||\n                               message.content.text == '';\n\n    if (needsTextCreation) {\n      if (message.content.containsKey('content')) {\n        if (message.content.content instanceof String) {\n          message.content.text = message.content.content;\n        } else {\n          message.content.text = Json.dump(message.content.content);\n        }\n      } else {\n        Map contentCopy = [:];\n        for (def entry : message.content.entrySet()) {\n          if (entry.getKey() != 'type') {\n            contentCopy[entry.getKey()] = entry.getValue();\n          }\n        }\n        if (contentCopy.size() > 0) {\n          message.content.text = Json.dump(contentCopy);\n        } else {\n          message.content.text = '';\n        }\n      }\n    }\n\n    def keysToRemove = new ArrayList(message.content.keySet());\n    for (def key : keysToRemove) {\n      if (key != 'type' && key != 'text') {\n        message.content.remove(key);\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def messagesList = ctx.aws_bedrock.invocation.messages instanceof List\n  ? ctx.aws_bedrock.invocation.messages\n  : [ctx.aws_bedrock.invocation.messages];\n\nfor (def message : messagesList) {\n  if (!(message instanceof Map) || !message.containsKey('content')) {\n    continue;\n  }\n\n  // Process content if it's an array\n  if (message.content instanceof List) {\n    for (def contentItem : message.content) {\n      if (!(contentItem instanceof Map)) {\n        continue;\n      }\n\n      // Ensure we have a text field - if not, create it from available data\n      boolean needsTextCreation = !contentItem.containsKey('text') ||\n                                 contentItem.text == null ||\n                                 contentItem.text == '';\n\n      if (needsTextCreation) {\n        if (contentItem.containsKey('content')) {\n          if (contentItem.content instanceof String) {\n            contentItem.text = contentItem.content;\n          } else {\n            contentItem.text = Json.dump(contentItem.content);\n          }\n        } else {\n          // Create text from all fields except type\n          Map itemCopy = [:];\n          for (def entry : contentItem.entrySet()) {\n            if (entry.getKey() != 'type') {\n              itemCopy[entry.getKey()] = entry.getValue();\n            }\n          }\n          if (itemCopy.size() > 0) {\n            contentItem.text = Json.dump(itemCopy);\n          } else {\n            contentItem.text = '';\n          }\n        }\n      }\n\n      // Remove all fields except type and text\n      def keysToRemove = new ArrayList(contentItem.keySet());\n      for (def key : keysToRemove) {\n        if (key != 'type' && key != 'text') {\n          contentItem.remove(key);\n        }\n      }\n    }\n  }\n  // Process content if it's a single object\n  else if (message.content instanceof Map) {\n    boolean needsTextCreation = !message.content.containsKey('text') ||\n                               message.content.text == null ||\n                               message.content.text == '';\n\n    if (needsTextCreation) {\n      if (message.content.containsKey('content')) {\n        if (message.content.content instanceof String) {\n          message.content.text = message.content.content;\n        } else {\n          message.content.text = Json.dump(message.content.content);\n        }\n      } else {\n        Map contentCopy = [:];\n        for (def entry : message.content.entrySet()) {\n          if (entry.getKey() != 'type') {\n            contentCopy[entry.getKey()] = entry.getValue();\n          }\n        }\n        if (contentCopy.size() > 0) {\n          message.content.text = Json.dump(contentCopy);\n        } else {\n          message.content.text = '';\n        }\n      }\n    }\n\n    def keysToRemove = new ArrayList(message.content.keySet());\n    for (def key : keysToRemove) {\n      if (key != 'type' && key != 'text') {\n        message.content.remove(key);\n      }\n    }\n  }\n}"#
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
                // Painless script
                // Source: def typ = ctx.gen_ai.request.model.id.substring(0, ctx.gen_ai.request.model.id.indexOf('.'));\nctx.gen_ai.request.model.type = typ;\nctx.gen_ai.request.model.version = ctx.aws_bedrock.invocation.input.input_body_json[typ+'_version'];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def typ = ctx.gen_ai.request.model.id.substring(0, ctx.gen_ai.request.model.id.indexOf('.'));\nctx.gen_ai.request.model.type = typ;\nctx.gen_ai.request.model.version = ctx.aws_bedrock.invocation.input.input_body_json[typ+'_version'];\n"#
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
                // Painless script
                // Source: // using a Set to avoid duplicates\ndef content_kinds_set = new HashSet(ctx.aws_bedrock.invocation.input?.messages_content_kinds ?: []);\nfor (def message : ctx.aws_bedrock.invocation.input.input_body_json.messages) {\n    if (message?.content instanceof List) {\n        for (def content_item : message['content']) {\n            if (content_item instanceof Map) {\n                def content_type = '';\n                if (content_item.containsKey('text')) {\n                    content_type = 'text';\n                } else if (content_item.containsKey('document')) {\n                    content_type = 'document';\n                    if (content_item['document'] instanceof Map && content_item['document'].containsKey('format')) {\n                        content_type += '/' + content_item['document']['format'];\n                    }\n                }\n                if (content_type != '') {\n                    content_kinds_set.add(content_type);\n                }\n            }\n        }\n    }\n}\n// convert the Set back to a List\nctx.aws_bedrock.invocation.input.messages_content_kinds = new ArrayList(content_kinds_set);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// using a Set to avoid duplicates\ndef content_kinds_set = new HashSet(ctx.aws_bedrock.invocation.input?.messages_content_kinds ?: []);\nfor (def message : ctx.aws_bedrock.invocation.input.input_body_json.messages) {\n    if (message?.content instanceof List) {\n        for (def content_item : message['content']) {\n            if (content_item instanceof Map) {\n                def content_type = '';\n                if (content_item.containsKey('text')) {\n                    content_type = 'text';\n                } else if (content_item.containsKey('document')) {\n                    content_type = 'document';\n                    if (content_item['document'] instanceof Map && content_item['document'].containsKey('format')) {\n                        content_type += '/' + content_item['document']['format'];\n                    }\n                }\n                if (content_type != '') {\n                    content_kinds_set.add(content_type);\n                }\n            }\n        }\n    }\n}\n// convert the Set back to a List\nctx.aws_bedrock.invocation.input.messages_content_kinds = new ArrayList(content_kinds_set);\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: for (int i = 0; i < ctx.aws_bedrock.invocation.output.output_body_json.length; i++) {\n  def e = ctx.aws_bedrock.invocation.output.output_body_json[i];\n  if (e instanceof Map) {\n    continue;\n  }\n  ctx.aws_bedrock.invocation.output.output_body_json[i] = [:];\n  ctx.aws_bedrock.invocation.output.output_body_json[i].value = e;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (int i = 0; i < ctx.aws_bedrock.invocation.output.output_body_json.length; i++) {\n  def e = ctx.aws_bedrock.invocation.output.output_body_json[i];\n  if (e instanceof Map) {\n    continue;\n  }\n  ctx.aws_bedrock.invocation.output.output_body_json[i] = [:];\n  ctx.aws_bedrock.invocation.output.output_body_json[i].value = e;\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("aws_bedrock.invocation.output.output_body_json")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: for (def e: ctx.aws_bedrock.invocation.output.output_body_json) {\n  if (e.message?.id != null) {\n    if (ctx.gen_ai == null) {\n      ctx.gen_ai = new HashMap();\n    }\n    if (ctx.gen_ai.response == null) {\n      ctx.gen_ai.response = new HashMap();\n    }\n    ctx.gen_ai.request.model.role = e.message?.role; // Surprisingly this is in the invocation.\n    ctx.gen_ai.response.id = e.message?.id;\n    break;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (def e: ctx.aws_bedrock.invocation.output.output_body_json) {\n  if (e.message?.id != null) {\n    if (ctx.gen_ai == null) {\n      ctx.gen_ai = new HashMap();\n    }\n    if (ctx.gen_ai.response == null) {\n      ctx.gen_ai.response = new HashMap();\n    }\n    ctx.gen_ai.request.model.role = e.message?.role; // Surprisingly this is in the invocation.\n    ctx.gen_ai.response.id = e.message?.id;\n    break;\n  }\n}\n"#
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
                // Painless script
                // Source: for (int i = ctx.aws_bedrock.invocation.output.output_body_json.length-1; i >= 0; i--) {\n  def e = ctx.aws_bedrock.invocation.output.output_body_json[i];\n  if (e.amazon_bedrock_invocation_metrics?.invocation_latency != null) {\n    if (ctx.gen_ai == null) {\n      ctx.gen_ai = new HashMap();\n    }\n    if (ctx.gen_ai.performance == null) {\n      ctx.gen_ai.performance = new HashMap();\n    }\n    ctx.gen_ai.performance.start_response_time = e.amazon_bedrock_invocation_metrics.first_byte_latency;\n    ctx.gen_ai.performance.response_time = e.amazon_bedrock_invocation_metrics.invocation_latency;\n    break;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (int i = ctx.aws_bedrock.invocation.output.output_body_json.length-1; i >= 0; i--) {\n  def e = ctx.aws_bedrock.invocation.output.output_body_json[i];\n  if (e.amazon_bedrock_invocation_metrics?.invocation_latency != null) {\n    if (ctx.gen_ai == null) {\n      ctx.gen_ai = new HashMap();\n    }\n    if (ctx.gen_ai.performance == null) {\n      ctx.gen_ai.performance = new HashMap();\n    }\n    ctx.gen_ai.performance.start_response_time = e.amazon_bedrock_invocation_metrics.first_byte_latency;\n    ctx.gen_ai.performance.response_time = e.amazon_bedrock_invocation_metrics.invocation_latency;\n    break;\n  }\n}\n"#
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
                // Painless script
                // Source: def allBodies = ctx.aws_bedrock.invocation.output.output_body_json;\nif (allBodies instanceof Map) { // it can be an object or an array\n  allBodies = [allBodies];\n}\n\ndef nameAndDetailsOfPolicies = allBodies.stream()\n  .flatMap(body -> [\n    [body.trace?.guardrail?.inputAssessment],\n    body.trace?.guardrail?.outputAssessments,\n    [body.amazon_bedrock_trace?.guardrail?.input],\n    body.amazon_bedrock_trace?.guardrail?.outputs\n  ].stream())\n  .filter(Objects::nonNull)\n  .flatMap(List::stream)\n  .filter(Objects::nonNull)\n  .flatMap(assessment -> assessment.entrySet().stream())\n  .flatMap(assessmentEntry -> assessmentEntry.getValue().entrySet().stream())\n  .filter(policyEntry -> policyEntry.getKey().endsWith('_policy'))\n  .flatMap(policyEntry ->\n    policyEntry.getValue().entrySet().stream()\n      .flatMap(policyValueEntry -> policyValueEntry.getValue().stream())\n      .map(details -> [ 'name': policyEntry.getKey(), 'details': details ])\n      .collect(Collectors.toList())\n      .stream()\n  )\n  .collect(Collectors.toList());\n\n\nif (ctx.gen_ai == null) ctx.gen_ai = [:];\nif (ctx.gen_ai.compliance == null) ctx.gen_ai.compliance = [:];\nif (ctx.gen_ai.policy == null) ctx.gen_ai.policy = [:];\n\n\nctx.gen_ai.guardrail_id = allBodies.stream()\n  .flatMap(body -> [\n    body.trace?.guardrail?.inputAssessment,\n    body.amazon_bedrock_trace?.guardrail?.input\n  ].stream())\n  .filter(input -> input instanceof HashMap)\n  .flatMap(input -> input.keySet().stream())\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.name = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.name)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.action = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.details.action)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.match_detail = nameAndDetailsOfPolicies.stream()\n  .filter(policy -> policy.details.match != null || policy.name == 'contextual_grounding_policy')\n  .map(policy -> policy.details)\n  .distinct()  // no sort because HashMap isn't Comparable\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.confidence = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.details.confidence)\n  .filter(Objects::nonNull)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.compliance.violation_code = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.details.type)\n  .filter(Objects::nonNull)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nif (ctx.gen_ai.compliance.violation_code.size() > 0 && allBodies.stream().anyMatch(body ->\n  body.amazon_bedrock_guardrail_action == 'INTERVENED' ||\n  body.stop_reason == 'guardrail_intervened'\n)) {\n  ctx.gen_ai.compliance.violation_detected = true;\n  ctx.event.outcome = 'failure';\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def allBodies = ctx.aws_bedrock.invocation.output.output_body_json;\nif (allBodies instanceof Map) { // it can be an object or an array\n  allBodies = [allBodies];\n}\n\ndef nameAndDetailsOfPolicies = allBodies.stream()\n  .flatMap(body -> [\n    [body.trace?.guardrail?.inputAssessment],\n    body.trace?.guardrail?.outputAssessments,\n    [body.amazon_bedrock_trace?.guardrail?.input],\n    body.amazon_bedrock_trace?.guardrail?.outputs\n  ].stream())\n  .filter(Objects::nonNull)\n  .flatMap(List::stream)\n  .filter(Objects::nonNull)\n  .flatMap(assessment -> assessment.entrySet().stream())\n  .flatMap(assessmentEntry -> assessmentEntry.getValue().entrySet().stream())\n  .filter(policyEntry -> policyEntry.getKey().endsWith('_policy'))\n  .flatMap(policyEntry ->\n    policyEntry.getValue().entrySet().stream()\n      .flatMap(policyValueEntry -> policyValueEntry.getValue().stream())\n      .map(details -> [ 'name': policyEntry.getKey(), 'details': details ])\n      .collect(Collectors.toList())\n      .stream()\n  )\n  .collect(Collectors.toList());\n\n\nif (ctx.gen_ai == null) ctx.gen_ai = [:];\nif (ctx.gen_ai.compliance == null) ctx.gen_ai.compliance = [:];\nif (ctx.gen_ai.policy == null) ctx.gen_ai.policy = [:];\n\n\nctx.gen_ai.guardrail_id = allBodies.stream()\n  .flatMap(body -> [\n    body.trace?.guardrail?.inputAssessment,\n    body.amazon_bedrock_trace?.guardrail?.input\n  ].stream())\n  .filter(input -> input instanceof HashMap)\n  .flatMap(input -> input.keySet().stream())\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.name = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.name)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.action = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.details.action)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.match_detail = nameAndDetailsOfPolicies.stream()\n  .filter(policy -> policy.details.match != null || policy.name == 'contextual_grounding_policy')\n  .map(policy -> policy.details)\n  .distinct()  // no sort because HashMap isn't Comparable\n  .collect(Collectors.toList());\n\nctx.gen_ai.policy.confidence = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.details.confidence)\n  .filter(Objects::nonNull)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nctx.gen_ai.compliance.violation_code = nameAndDetailsOfPolicies.stream()\n  .map(policy -> policy.details.type)\n  .filter(Objects::nonNull)\n  .distinct()\n  .sorted()\n  .collect(Collectors.toList());\n\nif (ctx.gen_ai.compliance.violation_code.size() > 0 && allBodies.stream().anyMatch(body ->\n  body.amazon_bedrock_guardrail_action == 'INTERVENED' ||\n  body.stop_reason == 'guardrail_intervened'\n)) {\n  ctx.gen_ai.compliance.violation_detected = true;\n  ctx.event.outcome = 'failure';\n}\n"#
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
                // Painless script
                // Source: ctx.gen_ai.performance.request_size = ctx.gen_ai.prompt.length();\nif (ctx.gen_ai.prompt.length() > 32766) {\n  ctx.aws_bedrock.invocation.input.input_body_json_massive_hash = ctx.gen_ai.prompt.sha1();\n  ctx.aws_bedrock.invocation.input.remove(\"input_body_json\");\n  ctx.gen_ai.remove(\"prompt\");\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.gen_ai.performance.request_size = ctx.gen_ai.prompt.length();\nif (ctx.gen_ai.prompt.length() > 32766) {\n  ctx.aws_bedrock.invocation.input.input_body_json_massive_hash = ctx.gen_ai.prompt.sha1();\n  ctx.aws_bedrock.invocation.input.remove(\"input_body_json\");\n  ctx.gen_ai.remove(\"prompt\");\n}\n"#
                    ),
                )?;
            }

            // Painless script
            // Source: try {\n  def completionText = new StringBuilder();\n  if (ctx.aws_bedrock?.invocation?.output?.output_body_json != null) {\n    if (ctx.aws_bedrock.invocation.output.output_body_json instanceof List) {\n      for (def block : ctx.aws_bedrock.invocation.output.output_body_json) {\n        if (!(block instanceof Map)) continue;\n        \n        if (block.containsKey('delta') && block.delta instanceof Map && block.delta.containsKey('text')) {\n          // Titan model response\n          completionText.append(block.delta.text);\n        } else if (block.containsKey('outputs') && block.outputs instanceof List) {\n          // Mistral model response\n          for (def cont : block.outputs) {\n            if (cont instanceof Map && cont.containsKey('text')) {\n                completionText.append(cont.text);\n            }\n          }\n        } else if (block.containsKey('generation')) {\n          // Llama3 model response\n          completionText.append(block.generation);\n        } else if (block.containsKey('outputText')) {\n            // Titan Text G1 - Express response\n            completionText.append(block.outputText);\n        }\n      }\n    }else if (ctx.aws_bedrock.invocation.output.output_body_json instanceof Map) {\n      def block = ctx.aws_bedrock.invocation.output.output_body_json;\n      if (block instanceof Map) {\n        Map blockMap = (Map) block;\n        if (blockMap.containsKey('output') && blockMap.output instanceof Map) {\n          Map outputMap = (Map) blockMap.output;\n          if (outputMap.containsKey('message') && outputMap.message instanceof Map) {\n            Map messageMap = (Map) outputMap.message;\n            if (messageMap.containsKey('content') && messageMap.content instanceof List) {\n              List contentList = (List) messageMap.content;\n              for (def cont : contentList) {\n                if (cont.containsKey('text')) {\n                  completionText.append(cont.text);\n                }\n              }\n            }\n          }\n        } else if(blockMap.containsKey('content') && blockMap.content instanceof List) {\n          // To record Guardrail response content, function calling and agentic workflow response\n          List contentList = (List) blockMap.content;\n            for (def cont : contentList) {\n              if (cont.containsKey('text')) {\n                completionText.append(cont.text);\n              }\n            }\n        }\n      }\n    }else if (ctx.aws_bedrock.invocation.output.output_body_json instanceof String){\n      completionText.append(ctx.aws_bedrock.invocation.output.output_body_json)\n    }\n  }\n\n  // Ensure ctx.aws_bedrock.invocation.output is initialized\n  if (ctx.aws_bedrock?.invocation?.output == null) {\n      ctx.aws_bedrock.invocation.output = new HashMap();\n  }\n  // Trim completionText if it exceeds 32766 characters\n  if (completionText.length() > 32766) {\n    completionText.setLength(32766);\n  }\n  ctx.aws_bedrock.invocation.output.completion_text = completionText.toString();\n}\ncatch (Exception e) {\n  if (ctx.aws_bedrock?.invocation?.output == null) {\n      ctx.aws_bedrock.invocation.output = new HashMap();\n  }\n  ctx.aws_bedrock.invocation.output.completion_text =  '';\n  ctx.error.message = e.getMessage()\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"try {\n  def completionText = new StringBuilder();\n  if (ctx.aws_bedrock?.invocation?.output?.output_body_json != null) {\n    if (ctx.aws_bedrock.invocation.output.output_body_json instanceof List) {\n      for (def block : ctx.aws_bedrock.invocation.output.output_body_json) {\n        if (!(block instanceof Map)) continue;\n        \n        if (block.containsKey('delta') && block.delta instanceof Map && block.delta.containsKey('text')) {\n          // Titan model response\n          completionText.append(block.delta.text);\n        } else if (block.containsKey('outputs') && block.outputs instanceof List) {\n          // Mistral model response\n          for (def cont : block.outputs) {\n            if (cont instanceof Map && cont.containsKey('text')) {\n                completionText.append(cont.text);\n            }\n          }\n        } else if (block.containsKey('generation')) {\n          // Llama3 model response\n          completionText.append(block.generation);\n        } else if (block.containsKey('outputText')) {\n            // Titan Text G1 - Express response\n            completionText.append(block.outputText);\n        }\n      }\n    }else if (ctx.aws_bedrock.invocation.output.output_body_json instanceof Map) {\n      def block = ctx.aws_bedrock.invocation.output.output_body_json;\n      if (block instanceof Map) {\n        Map blockMap = (Map) block;\n        if (blockMap.containsKey('output') && blockMap.output instanceof Map) {\n          Map outputMap = (Map) blockMap.output;\n          if (outputMap.containsKey('message') && outputMap.message instanceof Map) {\n            Map messageMap = (Map) outputMap.message;\n            if (messageMap.containsKey('content') && messageMap.content instanceof List) {\n              List contentList = (List) messageMap.content;\n              for (def cont : contentList) {\n                if (cont.containsKey('text')) {\n                  completionText.append(cont.text);\n                }\n              }\n            }\n          }\n        } else if(blockMap.containsKey('content') && blockMap.content instanceof List) {\n          // To record Guardrail response content, function calling and agentic workflow response\n          List contentList = (List) blockMap.content;\n            for (def cont : contentList) {\n              if (cont.containsKey('text')) {\n                completionText.append(cont.text);\n              }\n            }\n        }\n      }\n    }else if (ctx.aws_bedrock.invocation.output.output_body_json instanceof String){\n      completionText.append(ctx.aws_bedrock.invocation.output.output_body_json)\n    }\n  }\n\n  // Ensure ctx.aws_bedrock.invocation.output is initialized\n  if (ctx.aws_bedrock?.invocation?.output == null) {\n      ctx.aws_bedrock.invocation.output = new HashMap();\n  }\n  // Trim completionText if it exceeds 32766 characters\n  if (completionText.length() > 32766) {\n    completionText.setLength(32766);\n  }\n  ctx.aws_bedrock.invocation.output.completion_text = completionText.toString();\n}\ncatch (Exception e) {\n  if (ctx.aws_bedrock?.invocation?.output == null) {\n      ctx.aws_bedrock.invocation.output = new HashMap();\n  }\n  ctx.aws_bedrock.invocation.output.completion_text =  '';\n  ctx.error.message = e.getMessage()\n}\n"#
                ),
            )?;

            let _cond = {
                event
                    .get("gen_ai.completion")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // Painless script
                // Source: ctx.gen_ai.performance.response_size = ctx.gen_ai.completion.length();\nif (ctx.gen_ai.completion.length() > 32766) {\n  ctx.aws_bedrock.invocation.output.output_body_json_massive_hash = ctx.gen_ai.completion.sha1();\n  ctx.aws_bedrock.invocation.output.remove(\"output_body_json\");\n  ctx.gen_ai.remove(\"completion\");\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.gen_ai.performance.response_size = ctx.gen_ai.completion.length();\nif (ctx.gen_ai.completion.length() > 32766) {\n  ctx.aws_bedrock.invocation.output.output_body_json_massive_hash = ctx.gen_ai.completion.sha1();\n  ctx.aws_bedrock.invocation.output.remove(\"output_body_json\");\n  ctx.gen_ai.remove(\"completion\");\n}\n"#
                    ),
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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
                // Painless script
                // Source: ctx.event.original = 'sha1-'+ctx.event.original.sha1()+':'+ctx.event.original.length().toString();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event.original = 'sha1-'+ctx.event.original.sha1()+':'+ctx.event.original.length().toString();\n"#
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
