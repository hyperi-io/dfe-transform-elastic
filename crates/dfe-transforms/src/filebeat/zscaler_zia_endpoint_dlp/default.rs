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

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("event.original") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "event.original".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("resp", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("input.type") == Some("http_endpoint") };
            if _cond {
                event.remove("json");
            }

            if event.has("resp.event") {
                event.rename("resp.event", "json")?;
            }

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.strict_fields") == Some(true) };
            if _cond {
                // Painless script
                // Source: if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}"#
                    ),
                    cached_params!(
                        "{\"data_stream\":\"endpoint-dlp-log\",\"expect\":{\"fields\":\"actiontaken|activitytype|additionalinfo|channel|confirmaction|confirmjustification|datacenter|datacentercity|datacentercountry|datetime|day|dd|department|deviceappversion|devicehostname|devicemodel|devicename|deviceostype|deviceosversion|deviceowner|deviceplatform|devicetype|dlpdictcount|dlpdictnames|dlpenginenames|dlpidentifier|dsttype|eventtime|expectedaction|feedtime|filedoctype|filedstpath|filemd5|filesha|filesrcpath|filetypecategory|filetypename|hh|itemdstname|itemname|itemsrcname|itemtype|logtype|mm|mon|mth|numdlpdictids|numdlpengineids|odepartment|odevicehostname|odevicename|odeviceowner|odlpdictnames|odlpenginenames|ofiledstpath|ofilesrcpath|oitemdstname|oitemname|oitemsrcname|ootherrulelabels|orulename|otherrulelabels|ouser|recordid|rulename|scannedbytes|scantime|severity|srctype|ss|timezone|user|yyyy|zdpmode\",\"version\":\"v1\"},\"pkg_version\":\"3.15.1\"}"
                    ),
                )?;
            }

            event.remove("resp");

            event.append_unique("event.category", json!("intrusion_detection"))?;

            event.set("event.kind", json!("alert"))?;

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);"#
                ),
            )?;

            if event.has("json.actiontaken") {
                event.rename("json.actiontaken", "zscaler_zia.endpoint_dlp.action_taken")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.action_taken")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let lowered = s.to_lowercase();
                    event.set("event.action", lowered)?;
                }
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let re = cached_regex!(" ");
                            let replaced = re.replace_all(&s, "-").into_owned();
                            event.set("event.action", replaced)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") != Some("allow")
                    && event.get_str("event.action") != Some("block")
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("event.action") && event.get_str("event.action") == Some("allow")
            };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.has_value("event.action") && event.get_str("event.action") == Some("block")
            };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            if event.has("json.activitytype") {
                event.rename(
                    "json.activitytype",
                    "zscaler_zia.endpoint_dlp.activity_type",
                )?;
            }

            if event.has("json.additionalinfo") {
                event.rename(
                    "json.additionalinfo",
                    "zscaler_zia.endpoint_dlp.additional_info",
                )?;
            }

            if event.has("json.channel") {
                event.rename("json.channel", "zscaler_zia.endpoint_dlp.channel")?;
            }

            if event.has("json.confirmaction") {
                event.rename(
                    "json.confirmaction",
                    "zscaler_zia.endpoint_dlp.confirm_action",
                )?;
            }

            if event.has("json.confirmjustification") {
                event.rename(
                    "json.confirmjustification",
                    "zscaler_zia.endpoint_dlp.confirm_just",
                )?;
            }

            let _cond = {
                event.has_value("json.dlpdictcount")
                    && event.get_str("json.dlpdictcount") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def dlpdictcount = ctx.json.dlpdictcount;\nString[] parts = dlpdictcount.splitOnToken('|');\nArrayList numbersList = new ArrayList();\nfor (String part: parts) {\n  try {\n    numbersList.add(Integer.parseInt(part));\n  } catch (NumberFormatException e) {}\n}\nctx.json.dlpdictcount = numbersList;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"def dlpdictcount = ctx.json.dlpdictcount;\nString[] parts = dlpdictcount.splitOnToken('|');\nArrayList numbersList = new ArrayList();\nfor (String part: parts) {\n  try {\n    numbersList.add(Integer.parseInt(part));\n  } catch (NumberFormatException e) {}\n}\nctx.json.dlpdictcount = numbersList;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "extract_values_from_dlpdictcount",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.dlpdictcount") {
                event.rename("json.dlpdictcount", "zscaler_zia.endpoint_dlp.counts")?;
            }

            if event.has("json.datacentercity") {
                event.rename(
                    "json.datacentercity",
                    "zscaler_zia.endpoint_dlp.datacenter.city",
                )?;
            }

            if event.has("json.datacentercountry") {
                event.rename(
                    "json.datacentercountry",
                    "zscaler_zia.endpoint_dlp.datacenter.country",
                )?;
            }

            if event.has("json.datacenter") {
                event.rename(
                    "json.datacenter",
                    "zscaler_zia.endpoint_dlp.datacenter.name",
                )?;
            }

            if event.has("json.day") {
                event.rename("json.day", "zscaler_zia.endpoint_dlp.day")?;
            }

            let _cond = { event.get_str("json.dd") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dd") {
                        if let Some(val) = event.get("json.dd") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dd".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.day_of_month", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dd_to_long")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.department") {
                event.rename("json.department", "zscaler_zia.endpoint_dlp.department")?;
            }

            if event.has("json.dsttype") {
                event.rename("json.dsttype", "zscaler_zia.endpoint_dlp.destination_type")?;
            }

            if event.has("json.deviceappversion") {
                event.rename(
                    "json.deviceappversion",
                    "zscaler_zia.endpoint_dlp.device.appversion",
                )?;
            }

            if event.has("json.devicehostname") {
                event.rename(
                    "json.devicehostname",
                    "zscaler_zia.endpoint_dlp.device.hostname",
                )?;
            }

            let _cond = { event.get_str("zscaler_zia.endpoint_dlp.device.hostname") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("zscaler_zia.endpoint_dlp.device.hostname") {
                        if let Some(s) =
                            event.get_string("zscaler_zia.endpoint_dlp.device.hostname")
                        {
                            let lowered = s.to_lowercase();
                            event.set("host.name", lowered)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "lowercase")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.devicemodel") {
                event.rename("json.devicemodel", "zscaler_zia.endpoint_dlp.device.model")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.device.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.identifier", v)?;
            }

            if event.has("json.devicename") {
                event.rename("json.devicename", "zscaler_zia.endpoint_dlp.device.name")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.device.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.deviceostype") {
                event.rename(
                    "json.deviceostype",
                    "zscaler_zia.endpoint_dlp.device.os.type",
                )?;
            }

            if event.has("json.deviceosversion") {
                event.rename(
                    "json.deviceosversion",
                    "zscaler_zia.endpoint_dlp.device.os.version",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.device.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has("json.deviceowner") {
                event.rename("json.deviceowner", "zscaler_zia.endpoint_dlp.device.owner")?;
            }

            let _cond = { event.has_value("zscaler_zia.endpoint_dlp.device.owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.endpoint_dlp.device.owner")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.deviceplatform") {
                event.rename(
                    "json.deviceplatform",
                    "zscaler_zia.endpoint_dlp.device.platform",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.device.platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            if event.has("json.devicetype") {
                event.rename("json.devicetype", "zscaler_zia.endpoint_dlp.device.type")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.device.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            let _cond = { event.get_str("json.numdlpdictids") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.numdlpdictids") {
                        if let Some(val) = event.get("json.numdlpdictids") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.numdlpdictids".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.dictionary.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_numdlpdictids_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("json.dlpdictnames")
                    && event.get_str("json.dlpdictnames") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def dlpdictnames = ctx.json.dlpdictnames;\nString[] parts = dlpdictnames.splitOnToken('|');\nArrayList numbersList = new ArrayList();\nfor (String part: parts) {\n  try {\n    String[] subParts = part.splitOnToken(':');\n    numbersList.add(subParts[0]);\n  } catch (NumberFormatException e) {}\n}\nctx.json.dlpdictnames = numbersList;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"def dlpdictnames = ctx.json.dlpdictnames;\nString[] parts = dlpdictnames.splitOnToken('|');\nArrayList numbersList = new ArrayList();\nfor (String part: parts) {\n  try {\n    String[] subParts = part.splitOnToken(':');\n    numbersList.add(subParts[0]);\n  } catch (NumberFormatException e) {}\n}\nctx.json.dlpdictnames = numbersList;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "extract_values_from_dlpdictnames",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.dlpdictnames") {
                event.rename(
                    "json.dlpdictnames",
                    "zscaler_zia.endpoint_dlp.dictionary_names",
                )?;
            }

            let _cond = { event.get_str("json.numdlpengineids") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.numdlpengineids") {
                        if let Some(val) = event.get("json.numdlpengineids") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.numdlpengineids".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.engine.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_numdlpengineids_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("json.dlpenginenames")
                    && event.get_str("json.dlpenginenames") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def dlpenginenames = ctx.json.dlpenginenames;\nString[] parts = dlpenginenames.splitOnToken('|');\nArrayList numbersList = new ArrayList();\nfor (String part: parts) {\n  try {\n    numbersList.add(part);\n  } catch (NumberFormatException e) {}\n}\nctx.json.dlpenginenames = numbersList;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"def dlpenginenames = ctx.json.dlpenginenames;\nString[] parts = dlpenginenames.splitOnToken('|');\nArrayList numbersList = new ArrayList();\nfor (String part: parts) {\n  try {\n    numbersList.add(part);\n  } catch (NumberFormatException e) {}\n}\nctx.json.dlpenginenames = numbersList;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "extract_values_from_dlpenginenames",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.dlpenginenames") {
                event.rename(
                    "json.dlpenginenames",
                    "zscaler_zia.endpoint_dlp.engine_names",
                )?;
            }

            if event.has("json.timezone") {
                event.rename("json.timezone", "zscaler_zia.endpoint_dlp.timezone")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.timezone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                event.set("event.timezone", json!("UTC"))?;
            }

            let _cond = {
                event.has_value("json.eventtime") && event.get_str("json.eventtime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.eventtime") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("zscaler_zia.endpoint_dlp.event_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_eventtime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.expectedaction") {
                event.rename(
                    "json.expectedaction",
                    "zscaler_zia.endpoint_dlp.expected_action",
                )?;
            }

            let _cond =
                { event.has_value("json.feedtime") && event.get_str("json.feedtime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.feedtime") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("zscaler_zia.endpoint_dlp.feed_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_feedtime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.filedstpath") {
                event.rename(
                    "json.filedstpath",
                    "zscaler_zia.endpoint_dlp.file.destination_path",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.file.destination_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if event.has("json.filedoctype") {
                event.rename("json.filedoctype", "zscaler_zia.endpoint_dlp.file.doc_type")?;
            }

            if event.has("json.filemd5") {
                event.rename("json.filemd5", "zscaler_zia.endpoint_dlp.file.md5")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.filesha") {
                event.rename("json.filesha", "zscaler_zia.endpoint_dlp.file.sha256")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.filesrcpath") {
                event.rename(
                    "json.filesrcpath",
                    "zscaler_zia.endpoint_dlp.file.source_path",
                )?;
            }

            if event.has("json.filetypename") {
                event.rename(
                    "json.filetypename",
                    "zscaler_zia.endpoint_dlp.file.type.name",
                )?;
            }

            event.set("file.type", json!("file"))?;

            if event.has("json.filetypecategory") {
                event.rename(
                    "json.filetypecategory",
                    "zscaler_zia.endpoint_dlp.file.type_category",
                )?;
            }

            let _cond = { event.get_str("json.hh") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.hh") {
                        if let Some(val) = event.get("json.hh") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.hh".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.hour", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_hh_to_long")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.dlpidentifier") {
                if let Some(val) = event.get("json.dlpidentifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.dlpidentifier".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.endpoint_dlp.identifier", converted)?;
                }
            }

            if event.has("json.itemdstname") {
                event.rename(
                    "json.itemdstname",
                    "zscaler_zia.endpoint_dlp.item.destination_name",
                )?;
            }

            if event.has("json.itemname") {
                event.rename("json.itemname", "zscaler_zia.endpoint_dlp.item.name")?;
            }

            if event.has("json.itemsrcname") {
                event.rename(
                    "json.itemsrcname",
                    "zscaler_zia.endpoint_dlp.item.source_name",
                )?;
            }

            if event.has("json.itemtype") {
                event.rename("json.itemtype", "zscaler_zia.endpoint_dlp.item.type")?;
            }

            if event.has("json.logtype") {
                event.rename("json.logtype", "zscaler_zia.endpoint_dlp.log_type")?;
            }

            let _cond = { event.get_str("json.mm") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.mm") {
                        if let Some(val) = event.get("json.mm") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.mm".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.minute", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_mm_to_long")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.mon") {
                event.rename("json.mon", "zscaler_zia.endpoint_dlp.month")?;
            }

            let _cond = { event.get_str("json.mth") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.mth") {
                        if let Some(val) = event.get("json.mth") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.mth".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.month_of_year", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_mth_to_long")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.odepartment") {
                event.rename(
                    "json.odepartment",
                    "zscaler_zia.endpoint_dlp.obfuscated.department",
                )?;
            }

            if event.has("json.odevicehostname") {
                event.rename(
                    "json.odevicehostname",
                    "zscaler_zia.endpoint_dlp.obfuscated.device.hostname",
                )?;
            }

            if event.has("json.odevicename") {
                event.rename(
                    "json.odevicename",
                    "zscaler_zia.endpoint_dlp.obfuscated.device.name",
                )?;
            }

            if event.has("json.odeviceowner") {
                event.rename(
                    "json.odeviceowner",
                    "zscaler_zia.endpoint_dlp.obfuscated.device.owner",
                )?;
            }

            if event.has("json.odlpdictnames") {
                event.rename(
                    "json.odlpdictnames",
                    "zscaler_zia.endpoint_dlp.obfuscated.dlp.dictionary_names",
                )?;
            }

            if event.has("json.odlpenginenames") {
                event.rename(
                    "json.odlpenginenames",
                    "zscaler_zia.endpoint_dlp.obfuscated.dlp.engine_names",
                )?;
            }

            if event.has("json.ofiledstpath") {
                event.rename(
                    "json.ofiledstpath",
                    "zscaler_zia.endpoint_dlp.obfuscated.file.destination_path",
                )?;
            }

            if event.has("json.ofilesrcpath") {
                event.rename(
                    "json.ofilesrcpath",
                    "zscaler_zia.endpoint_dlp.obfuscated.file.source_path",
                )?;
            }

            if event.has("json.oitemdstname") {
                event.rename(
                    "json.oitemdstname",
                    "zscaler_zia.endpoint_dlp.obfuscated.item.destination_names",
                )?;
            }

            if event.has("json.oitemname") {
                event.rename(
                    "json.oitemname",
                    "zscaler_zia.endpoint_dlp.obfuscated.item.name",
                )?;
            }

            if event.has("json.oitemsrcname") {
                event.rename(
                    "json.oitemsrcname",
                    "zscaler_zia.endpoint_dlp.obfuscated.item.source_names",
                )?;
            }

            if event.has("json.ootherrulelabels") {
                event.rename(
                    "json.ootherrulelabels",
                    "zscaler_zia.endpoint_dlp.obfuscated.other_rule_labels",
                )?;
            }

            if event.has("json.orulename") {
                event.rename(
                    "json.orulename",
                    "zscaler_zia.endpoint_dlp.obfuscated.triggered_rule_label",
                )?;
            }

            if event.has("json.ouser") {
                event.rename("json.ouser", "zscaler_zia.endpoint_dlp.obfuscated.user")?;
            }

            if event.has("json.otherrulelabels") {
                event.rename(
                    "json.otherrulelabels",
                    "zscaler_zia.endpoint_dlp.other_rule_labels",
                )?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.other_rule_labels")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.recordid") {
                if let Some(val) = event.get("json.recordid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.recordid".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.endpoint_dlp.record.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.record.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.get_str("json.scantime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.scantime") {
                        if let Some(val) = event.get("json.scantime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.scantime".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.scan_time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_scantime_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("json.scannedbytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.scannedbytes") {
                        if let Some(val) = event.get("json.scannedbytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.scannedbytes".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.scanned_bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_scannedbytes_to_long",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("json.ss") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ss") {
                        if let Some(val) = event.get("json.ss") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ss".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.second", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ss_to_long")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.severity") {
                event.rename("json.severity", "zscaler_zia.endpoint_dlp.severity")?;
            }

            if event.has("json.srctype") {
                event.rename("json.srctype", "zscaler_zia.endpoint_dlp.source_type")?;
            }

            let _cond =
                { event.has_value("json.datetime") && event.get_str("json.datetime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.datetime") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("zscaler_zia.endpoint_dlp.time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_datetime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has("json.rulename") {
                event.rename(
                    "json.rulename",
                    "zscaler_zia.endpoint_dlp.triggered_rule_label",
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.endpoint_dlp.triggered_rule_label") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("zscaler_zia.endpoint_dlp.triggered_rule_label")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.user") {
                event.rename("json.user", "zscaler_zia.endpoint_dlp.user")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.endpoint_dlp.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "create_user_name_and_user_domain",
                    )?;
                    if event.has("user.email") {
                        event.rename("user.email", "user.name")?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.yyyy") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.yyyy") {
                        if let Some(val) = event.get("json.yyyy") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.yyyy".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.endpoint_dlp.year", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_yyyy_to_long")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.zdpmode") {
                event.rename("json.zdpmode", "zscaler_zia.endpoint_dlp.zdp_mode")?;
            }

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
                event.remove("zscaler_zia.endpoint_dlp.action_taken");
                event.remove("zscaler_zia.endpoint_dlp.device.hostname");
                event.remove("zscaler_zia.endpoint_dlp.device.model");
                event.remove("zscaler_zia.endpoint_dlp.device.name");
                event.remove("zscaler_zia.endpoint_dlp.device.os.version");
                event.remove("zscaler_zia.endpoint_dlp.device.platform");
                event.remove("zscaler_zia.endpoint_dlp.device.type");
                event.remove("zscaler_zia.endpoint_dlp.file.destination_path");
                event.remove("zscaler_zia.endpoint_dlp.file.md5");
                event.remove("zscaler_zia.endpoint_dlp.file.sha256");
                event.remove("zscaler_zia.endpoint_dlp.other_rule_labels");
                event.remove("zscaler_zia.endpoint_dlp.record.id");
                event.remove("zscaler_zia.endpoint_dlp.time");
                event.remove("zscaler_zia.endpoint_dlp.triggered_rule_label");
                event.remove("zscaler_zia.endpoint_dlp.user");
            }

            event.remove("json");
            event.remove("_conf");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
