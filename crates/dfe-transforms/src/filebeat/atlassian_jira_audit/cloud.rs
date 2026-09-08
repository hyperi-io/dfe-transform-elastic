// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cloud` pipeline.
pub struct Cloud;

impl Transform for Cloud {
    fn name(&self) -> &str {
        "cloud"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("json.created") };
            if _cond {
            if let Some(v) = event.get("json.created").cloned() {
                event.set("_tmp.timestamp", v)?;
            }
            }

            if event.has_value("json.id") {
                if let Some(val) = event.get("json.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.id".into(),
                            message,
                        })?;
                    event.set("event.id", converted)?;
                }
            }

                if event.has_value("json.remoteAddress") {
                    event.rename("json.remoteAddress", "source.address")?;
                }

                if event.has_value("json.authorAccountId") {
                    event.rename("json.authorAccountId", "user.id")?;
                }

                if event.has_value("json.category") {
                    event.rename("json.category", "jira.audit.type.category")?;
                }

                if event.has_value("json.summary") {
                    event.rename("json.summary", "jira.audit.type.action")?;
                }

            if let Some(v) = event.get("jira.audit.type.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

                if event.has_value("json.associatedItems") {
                    event.rename("json.associatedItems", "jira.audit.affected_objects")?;
                }

                if event.has_value("json.changedValues") {
                    event.rename("json.changedValues", "jira.audit.changed_values")?;
                }

                // Painless script, resolved to its runners at generation time
                // Source: if(ctx.jira?.audit?.affected_objects == null) {\n    ArrayList items = new ArrayList();\n    ctx.jira?.audit.put(\"affected_objects\", items);\n} if(ctx.json?.objectItem != null && !ctx.jira?.audit?.affected_objects.contains(ctx.json?.objectItem)) {\n    ctx.jira?.audit?.affected_objects.add(ctx.json?.objectItem);\n}\n        \nif(ctx.jira?.audit?.affected_objects != null) {\n    for (def j = 0; j < ctx.jira?.audit?.affected_objects.length; j++) {\n        if(ctx.jira.audit.affected_objects[j]?.typeName != null) {\n            ctx.jira.audit.affected_objects[j].put('type', ctx.jira.audit.affected_objects[j].typeName);\n            ctx.jira.audit.affected_objects[j].remove('typeName');\n        }\n    }\n} if(ctx.jira?.audit?.changed_values != null) {\n    for (def j = 0; j < ctx.jira?.audit?.changed_values.length; j++) {\n        if(ctx.jira.audit.changed_values[j]?.fieldName != null) {\n            ctx.jira.audit.changed_values[j].put('i18nKey', ctx.jira.audit.changed_values[j].fieldName);\n            ctx.jira.audit.changed_values[j].put('key', ctx.jira.audit.changed_values[j].fieldName);\n            ctx.jira.audit.changed_values[j].remove('fieldName');\n        }\n        if(ctx.jira.audit.changed_values[j]?.changedTo != null) {\n            ctx.jira.audit.changed_values[j].put('to', ctx.jira.audit.changed_values[j].changedTo);\n            ctx.jira.audit.changed_values[j].remove('changedTo');\n        }\n        if(ctx.jira.audit.changed_values[j]?.changedFrom != null) {\n            ctx.jira.audit.changed_values[j].put('from', ctx.jira.audit.changed_values[j].changedFrom);\n            ctx.jira.audit.changed_values[j].remove('changedFrom');\n        }\n    }\n}
                list_item_renames(event, &ListItemRenames::new(Some(EnsureItem::new("jira.audit.affected_objects", "json.objectItem")), vec![ListWalk::new("jira.audit.affected_objects", vec![ItemRename::new("typeName", vec!["type".into()])]), ListWalk::new("jira.audit.changed_values", vec![ItemRename::new("fieldName", vec!["i18nKey".into(), "key".into()]), ItemRename::new("changedTo", vec!["to".into()]), ItemRename::new("changedFrom", vec!["from".into()])])]));

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
