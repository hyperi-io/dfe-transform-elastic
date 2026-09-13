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
            let _cond = { event.has_value("json.creationDate") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.creationDate") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.creationDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

                if event.has_value("json.remoteAddress") {
                    event.rename("json.remoteAddress", "source.address")?;
                }

                if event.has_value("json.author.accountId") {
                    event.rename("json.author.accountId", "user.id")?;
                }

                if event.has_value("json.author.displayName") {
                    event.rename("json.author.displayName", "user.full_name")?;
                }

                if event.has_value("json.author.externalCollaborator") {
                    event.rename("json.author.externalCollaborator", "confluence.audit.external_collaborator")?;
                }

                if event.has_value("json.category") {
                    event.rename("json.category", "confluence.audit.type.category")?;
                }

                if event.has_value("json.summary") {
                    event.rename("json.summary", "confluence.audit.type.action")?;
                }

            if let Some(v) = event.get("confluence.audit.type.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

                if event.has_value("json.associatedObjects") {
                    event.rename("json.associatedObjects", "confluence.audit.affected_objects")?;
                }

                if event.has_value("json.changedValues") {
                    event.rename("json.changedValues", "confluence.audit.changed_values")?;
                }

            let _cond = { event.has_value("confluence.audit") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: if(ctx.confluence.audit.affected_objects == null) {\n    ArrayList items = new ArrayList();\n    ctx.confluence.audit.put(\"affected_objects\", items);\n} if(ctx.json?.affectedObject != null && !ctx.confluence?.audit?.affected_objects.contains(ctx.json?.affectedObject)) {\n    ctx.confluence.audit.affected_objects.add(ctx.json?.affectedObject);\n}\n        \nif(ctx.confluence.audit.affected_objects != null) {\n    for (def j = 0; j < ctx.confluence?.audit?.affected_objects.length; j++) {\n        if(ctx.confluence.audit.affected_objects[j]?.objectType != null) {\n            ctx.confluence.audit.affected_objects[j].put('type', ctx.confluence.audit.affected_objects[j].objectType);\n            ctx.confluence.audit.affected_objects[j].remove('objectType');\n        }\n    }\n} if(ctx.confluence.audit.changed_values != null) {\n    for (def j = 0; j < ctx.confluence.audit.changed_values.length; j++) {\n        if(ctx.confluence.audit.changed_values[j]?.name != null) {\n            ctx.confluence.audit.changed_values[j].put('i18nKey', ctx.confluence.audit.changed_values[j].name);\n            ctx.confluence.audit.changed_values[j].put('key', ctx.confluence.audit.changed_values[j].name);\n            ctx.confluence.audit.changed_values[j].remove('name');\n        }\n        if(ctx.confluence.audit.changed_values[j]?.newValue != null) {\n            ctx.confluence.audit.changed_values[j].put('to', ctx.confluence.audit.changed_values[j].newValue);\n            ctx.confluence.audit.changed_values[j].remove('newValue');\n        }\n        if(ctx.confluence.audit.changed_values[j]?.oldValue != null) {\n            ctx.confluence.audit.changed_values[j].put('from', ctx.confluence.audit.changed_values[j].oldValue);\n            ctx.confluence.audit.changed_values[j].remove('oldValue');\n        }\n    }\n}
                list_item_renames(event, &ListItemRenames::new(Some(EnsureItem::new("confluence.audit.affected_objects", "json.affectedObject")), vec![ListWalk::new("confluence.audit.affected_objects", vec![ItemRename::new("objectType", vec!["type".into()])]), ListWalk::new("confluence.audit.changed_values", vec![ItemRename::new("name", vec!["i18nKey".into(), "key".into()]), ItemRename::new("newValue", vec!["to".into()]), ItemRename::new("oldValue", vec!["from".into()])])]));
            }

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
