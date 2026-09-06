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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "json")?;

            if let Some(date_str) = event.get_as_string("json.isoTimestamp") {
                match parse_date_out(&date_str, &["date_optional_time"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.isoTimestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("json.auditType") {
                if let Some(val) = event.get("json.auditType") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.auditType".into(),
                            message,
                        }
                    })?;
                    event.set("json.auditType", converted)?;
                }
            }

            if event.has_value("json.auditType") {
                event.rename("json.auditType", "event.code")?;
            }

            if event.has_value("json.performedBy.id") {
                event.rename("json.performedBy.id", "user.id")?;
            }

            if event.has_value("json.performedBy.name") {
                event.rename("json.performedBy.name", "user.full_name")?;
            }

            let _cond = { event.has_value("json.enforcementSource") };
            if _cond {
                event.rename(
                    "json.enforcementSource",
                    "zeronetworks.audit.enforcementSource",
                )?;
            }

            let _cond = { event.has_value("json.userRole") };
            if _cond {
                event.rename("json.userRole", "zeronetworks.audit.userRole")?;
            }

            let _cond = { event.has_value("json.reportedObjectId") };
            if _cond {
                event.rename(
                    "json.reportedObjectId",
                    "zeronetworks.audit.reportedObjectId",
                )?;
            }

            let _cond = { event.has_value("json.reportedObjectGeneration") };
            if _cond {
                event.rename(
                    "json.reportedObjectGeneration",
                    "zeronetworks.audit.reportedObjectGeneration",
                )?;
            }

            let _cond = { event.has_value("json.parentObjectId") };
            if _cond {
                if event.has_value("json.parentObjectId") {
                    event.rename("json.parentObjectId", "zeronetworks.audit.parentObjectId")?;
                }
            }

            if event.has_value("json.destinationEntitiesList") {
                foreach_array(event, "json.destinationEntitiesList", |event| {
                    event.set(
                        "zeronetworks.audit.destinationEntitiesList.name",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.destinationEntitiesList") {
                foreach_array(event, "json.destinationEntitiesList", |event| {
                    event.set(
                        "zeronetworks.audit.destinationEntitiesList.id",
                        json!(
                            event
                                .get("_ingest._value.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("json.details") };
            if _cond {
                parse_json_field(event, "json.details", "zeronetworks.audit.details")?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.code") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "event.code".into(),
                    });
                }
                if let Some(v) = event.get("json.timestamp") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.timestamp".into(),
                    });
                }
                if let Some(v) = event.get("user.full_name") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "user.full_name".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("event.id", json!(fingerprint_default(&values)))?;
                }
            }

            if let Some(v) = event.get("event.id").cloned() {
                event.set("_id", v)?;
            }

            // Painless script
            // Source: ctx.event.kind = 'event'; if (ctx?.event?.code == null) {\n    return;\n} if (params.get(ctx.event.code) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.code)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; if (ctx?.event?.code == null) {\n    return;\n} if (params.get(ctx.event.code) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.code)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"1\":{\"action\":\"Asset is being added to protection\",\"outcome\":\"success\",\"type\":[\"start\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"2\":{\"action\":\"Asset added to protection\",\"outcome\":\"success\",\"type\":[\"end\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"4\":{\"action\":\"Asset is being removed from protection\",\"outcome\":\"success\",\"type\":[\"start\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"5\":{\"action\":\"Removed asset from protection\",\"outcome\":\"success\",\"type\":[\"end\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"7\":{\"action\":\"Asset added to learning\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"8\":{\"action\":\"Asset removed from learning\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"9\":{\"action\":\"Inbound allow rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"10\":{\"action\":\"Inbound allow rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"11\":{\"action\":\"Inbound allow rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"12\":{\"action\":\"Inbound allow rule edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"17\":{\"action\":\"Inbound MFA policy created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"18\":{\"action\":\"Inbound MFA policy deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"19\":{\"action\":\"Inbound MFA policy edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"20\":{\"action\":\"Inbound JIT rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"21\":{\"action\":\"Inbound JIT rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"22\":{\"action\":\"Inbound JIT rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"23\":{\"action\":\"Inbound JIT rule revived\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"24\":{\"action\":\"Inbound JIT rule edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"25\":{\"action\":\"API Token created\",\"outcome\":\"success\",\"type\":[\"info\",\"access\",\"creation\"],\"category\":[\"configuration\",\"api\"]},\"26\":{\"action\":\"API Token deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"access\",\"deletion\"],\"category\":[\"configuration\",\"api\"]},\"27\":{\"action\":\"API Token regenerated\",\"outcome\":\"success\",\"type\":[\"info\",\"access\",\"change\"],\"category\":[\"api\",\"configuration\"]},\"28\":{\"action\":\"Asset protection date postponed\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"29\":{\"action\":\"Outbound block rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"30\":{\"action\":\"Outbound block rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"31\":{\"action\":\"Outbound block rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"32\":{\"action\":\"Outbound block rule edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"33\":{\"action\":\"Inbound block rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"34\":{\"action\":\"Inbound block rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"35\":{\"action\":\"Inbound block rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"36\":{\"action\":\"Inbound block rule edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"39\":{\"action\":\"Removed asset from protection (overriding policy)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"40\":{\"action\":\"Asset is being removed from protection (overriding policy)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"41\":{\"action\":\"Asset removed from learning (overriding policy)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"42\":{\"action\":\"Asset is being added to protection (policy)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"43\":{\"action\":\"Asset added to protection (policy)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"44\":{\"action\":\"Asset added to learning (policy)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"45\":{\"action\":\"Protection policy created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"46\":{\"action\":\"Protection policy deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"47\":{\"action\":\"Protection policy edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"48\":{\"action\":\"Inbound JIT access rejected\",\"outcome\":\"failure\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"49\":{\"action\":\"Inbound JIT fallback rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"50\":{\"action\":\"Inbound JIT fallback rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"51\":{\"action\":\"Inbound JIT fallback rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"53\":{\"action\":\"Outbound allow rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"54\":{\"action\":\"Outbound allow rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"55\":{\"action\":\"Outbound allow rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"56\":{\"action\":\"Outbound allow rule edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"58\":{\"action\":\"Admin portal role changed to admin\",\"outcome\":\"success\",\"type\":[\"info\",\"access\",\"change\"],\"category\":[\"configuration\"]},\"59\":{\"action\":\"Admin portal role changed to viewer\",\"outcome\":\"success\",\"type\":[\"info\",\"access\",\"change\"],\"category\":[\"configuration\"]},\"60\":{\"action\":\"Admin portal role revoked\",\"outcome\":\"success\",\"type\":[\"info\",\"access\",\"change\"],\"category\":[\"configuration\"]},\"61\":{\"action\":\"Outbound JIT rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"62\":{\"action\":\"Outbound JIT rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"63\":{\"action\":\"Outbound JIT rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"64\":{\"action\":\"Outbound MFA policy created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"65\":{\"action\":\"Outbound MFA policy deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"66\":{\"action\":\"Outbound MFA policy edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"67\":{\"action\":\"Outbound JIT access rejected\",\"outcome\":\"failure\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"68\":{\"action\":\"Asset learning is done\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\",\"configuration\"]},\"69\":{\"action\":\"Asset learning (policy) is done\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\",\"configuration\"]},\"70\":{\"action\":\"Manual Linux asset created\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\",\"configuration\"]},\"71\":{\"action\":\"Manual OT/IoT asset created\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\",\"configuration\"]},\"72\":{\"action\":\"Asset learning extended\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"73\":{\"action\":\"Admin portal logon\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"authentication\"]},\"74\":{\"action\":\"Asset manager added\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"75\":{\"action\":\"Asset manager removed\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"76\":{\"action\":\"Asset is directly monitored\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"77\":{\"action\":\"Asset is no longer directly monitored\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"78\":{\"action\":\"Asset is remotely monitored\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"79\":{\"action\":\"Asset is back to learning\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"80\":{\"action\":\"Manual OT/IoT asset edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"81\":{\"action\":\"Admin portal role changed to operator\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"82\":{\"action\":\"Segment server deployed\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"83\":{\"action\":\"AI inbound allow rule rejected\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"84\":{\"action\":\"AI inbound block rule rejected\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"85\":{\"action\":\"AI outbound allow rule rejected\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"86\":{\"action\":\"AI outbound block rule rejected\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"87\":{\"action\":\"AI inbound allow rule approved\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"88\":{\"action\":\"AI inbound block rule approved\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"89\":{\"action\":\"AI outbound allow rule approved\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"90\":{\"action\":\"AI outbound block rule approved\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"91\":{\"action\":\"AI inbound allow rule approved with changes\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"92\":{\"action\":\"AI inbound block rule approved with changes\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"93\":{\"action\":\"AI outbound allow rule approved with changes\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"94\":{\"action\":\"AI outbound block rule approved with changes\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"95\":{\"action\":\"Region created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"96\":{\"action\":\"Connect session created\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\"]},\"97\":{\"action\":\"Connect session expired\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\"]},\"98\":{\"action\":\"Connect session revoked\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\"]},\"99\":{\"action\":\"Connect session logout\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\"]},\"100\":{\"action\":\"User access configuration created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"101\":{\"action\":\"User access configuration edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"102\":{\"action\":\"User access configuration deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"103\":{\"action\":\"Connect server deployed\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"104\":{\"action\":\"Connect asset created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"105\":{\"action\":\"Asset segmentation postponed (network) (pending review rules)\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\"]},\"106\":{\"action\":\"Region edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"107\":{\"action\":\"Connect server edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"108\":{\"action\":\"Asset is being segmented (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"109\":{\"action\":\"Asset segmented (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"110\":{\"action\":\"Asset is being unsegmented (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"111\":{\"action\":\"Asset unsegmented (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"112\":{\"action\":\"Identity rule created\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"113\":{\"action\":\"Identity rule deleted\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"114\":{\"action\":\"Identity rule expired\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"115\":{\"action\":\"Identity rule edited\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"116\":{\"action\":\"User segmented (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"user\",\"configuration\"]},\"117\":{\"action\":\"User unsegmented (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"user\",\"configuration\"]},\"118\":{\"action\":\"User added to learning (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"user\",\"configuration\"]},\"119\":{\"action\":\"User removed from learning (identity)\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"user\",\"configuration\"]},\"120\":{\"action\":\"Asset added to RPC monitoring\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"121\":{\"action\":\"Asset removed from RPC monitoring\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"122\":{\"action\":\"User classification changed\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"user\",\"configuration\"]},\"123\":{\"action\":\"Connect session extended\",\"outcome\":\"success\",\"type\":[\"info\"],\"category\":[\"host\"]},\"124\":{\"action\":\"Asset marked as inactive by repo delete\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"125\":{\"action\":\"Asset marked as active by repo revive\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"126\":{\"action\":\"Asset marked as inactive by user\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"127\":{\"action\":\"Asset marked as active by user\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"128\":{\"action\":\"Break glass activated\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"129\":{\"action\":\"Break glass deactivated\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"configuration\"]},\"130\":{\"action\":\"Asset marked as inactive by repo disable\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]},\"131\":{\"action\":\"Asset marked as active by repo enable\",\"outcome\":\"success\",\"type\":[\"info\",\"change\"],\"category\":[\"host\",\"configuration\"]}}"
                ),
            )?;

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
