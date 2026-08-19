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
            let _cond = { !event.has("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { !event.has("event.original") };
            if _cond {
                event.set(
                    "event.original",
                    event.get("o365audit").cloned().unwrap_or(Value::Null),
                )?;
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.category", json!("web"))?;

            {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                if let Some(v) = event.get("o365audit.Id") {
                    hasher.update(v.to_string().as_bytes());
                }
                let hash = format!("{:x}", hasher.finalize());
                event.set("_id", json!(hash))?;
            }

            let _cond = { event.has("o365audit.CreationTime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.CreationTime") {
                    // Try ISO8601 format
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        })
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                        })
                    {
                        event.set(
                            "@timestamp",
                            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        )?;
                    }
                }
            }

            if event.has("o365audit.Id") {
                event.rename("o365audit.Id", "event.id")?;
            }

            let _cond = { event.has("o365audit.ListBaseType") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("o365audit.ListBaseType") {
                        let converted = match val {
                            Value::String(_) => val.clone(),
                            Value::Number(n) => json!(n.to_string()),
                            Value::Bool(b) => json!(b.to_string()),
                            Value::Null => json!("null"),
                            _ => json!(val.to_string()),
                        };
                        event.set("o365audit.ListBaseType", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert-listbasetype-to-string",
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
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                }
            }

            if event.has("o365audit.ClientIPAddress") {
                event.rename("o365audit.ClientIPAddress", "client._temp")?;
            }

            let _cond = { !event.has("client._temp") };
            if _cond {
                if event.has("o365audit.ClientIP") {
                    event.rename("o365audit.ClientIP", "client._temp")?;
                }
            }

            let _cond = { !event.has("client._temp") };
            if _cond {
                if event.has("o365audit.ActorIpAddress") {
                    event.rename("o365audit.ActorIpAddress", "client._temp")?;
                }
            }

            if event.has("o365audit.UserId") {
                if let Some(val) = event.get("o365audit.UserId") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("user.id", converted)?;
                }
            }

            if event.has("o365audit.Workload") {
                event.rename("o365audit.Workload", "event.provider")?;
            }

            if event.has("o365audit.Operation") {
                event.rename("o365audit.Operation", "event.action")?;
            }

            if event.has("o365audit.OrganizationId") {
                event.rename("o365audit.OrganizationId", "organization.id")?;
            }

            let _cond = {
                event
                    .get("o365audit.OperationProperties")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("o365audit.OperationProperties") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "o365audit.OperationProperties".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("o365audit.OperationProperties", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json-extract-stringly-OperationProperties",
                    )?;
                    if event.remove("o365audit.OperationProperties").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "o365audit.OperationProperties".into(),
                        });
                    }
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
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                }
            }

            if event.has("o365audit.UserAgent") {
                event.rename("o365audit.UserAgent", "user_agent.original")?;
            }

            let _cond = { event.has("o365audit.RecordType") };
            if _cond {
                // Painless script
                // Source: def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n"#,
                )?;
            }

            let _cond = {
                event.has("o365audit.ResultStatus")
                    && event.get_str("o365audit.ResultStatus").is_some_and(|s| {
                        ["succeeded", "success", "partiallysucceeded", "true"]
                            .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has("o365audit.ResultStatus")
                    && event
                        .get_str("o365audit.ResultStatus")
                        .is_some_and(|s| ["failed", "false"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has("o365audit.Parameters")
                    && event
                        .get("o365audit.Parameters")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def newparams = new HashMap();  def oldparams = ctx.o365audit.Parameters; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.Parameters = newparams;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.Parameters; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.Parameters = newparams;\n"#,
                )?;
            }

            let _cond = {
                event.has("o365audit.Parameters")
                    && event
                        .get("o365audit.Parameters")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("o365audit.Parameters", "o365audit.Parameters._raw")?;
            }

            let _cond = { event.has("o365audit.Platform") };
            if _cond {
                // Painless script
                // Source: def value = ctx.o365audit.Platform.toString(); def name = params[value]; if (name != null) {\n  ctx.o365audit.Platform = name;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def value = ctx.o365audit.Platform.toString(); def name = params[value]; if (name != null) {\n  ctx.o365audit.Platform = name;\n}\n"#,
                )?;
            }

            let _cond = {
                event.has("o365audit.ExtendedProperties")
                    && event
                        .get("o365audit.ExtendedProperties")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def newparams = new HashMap();  def oldparams = ctx.o365audit.ExtendedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.ExtendedProperties = newparams;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.ExtendedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.ExtendedProperties = newparams;\n"#,
                )?;
            }

            let _cond = {
                event.has("o365audit.ExtendedProperties")
                    && event
                        .get("o365audit.ExtendedProperties")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "o365audit.ExtendedProperties",
                    "o365audit.ExtendedProperties._raw",
                )?;
            }

            let _cond = {
                event.has("o365audit.ModifiedProperties")
                    && event
                        .get("o365audit.ModifiedProperties")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def newparams = new HashMap();  def oldparams = ctx.o365audit.ModifiedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i] instanceof Map && oldparams[i][\"OldValue\"] != null && oldparams[i][\"NewValue\"] != null) {\n    def validname = oldparams[i][\"Name\"].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n    newparams[validname][\"NewValue\"] = oldparams[i][\"NewValue\"];\n    newparams[validname][\"OldValue\"] = oldparams[i][\"OldValue\"];\n  }\n  if (oldparams[i] instanceof String) {\n    def validname = oldparams[i].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n  }\n} if (newparams.isEmpty()) {\n  ctx.o365audit.remove(\"ModifiedProperties\");\n  return;\n} ctx.o365audit.ModifiedProperties = newparams;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.ModifiedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i] instanceof Map && oldparams[i][\"OldValue\"] != null && oldparams[i][\"NewValue\"] != null) {\n    def validname = oldparams[i][\"Name\"].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n    newparams[validname][\"NewValue\"] = oldparams[i][\"NewValue\"];\n    newparams[validname][\"OldValue\"] = oldparams[i][\"OldValue\"];\n  }\n  if (oldparams[i] instanceof String) {\n    def validname = oldparams[i].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n  }\n} if (newparams.isEmpty()) {\n  ctx.o365audit.remove(\"ModifiedProperties\");\n  return;\n} ctx.o365audit.ModifiedProperties = newparams;\n"#,
                )?;
            }

            let _cond = {
                event.has("o365audit.ModifiedProperties")
                    && event
                        .get("o365audit.ModifiedProperties")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "o365audit.ModifiedProperties",
                    "o365audit.ModifiedProperties._raw",
                )?;
            }

            let _cond = {
                event.has("o365audit.AlertLinks")
                    && event
                        .get("o365audit.AlertLinks")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def list = ctx.o365audit.AlertLinks; def links = new ArrayList(); for (int i = 0; i < list.length; ++i) {\n  if (list[i] instanceof Map && list[i].containsKey(\"AlertLinkHref\") && list[i][\"AlertLinkHref\"] != null && list[i][\"AlertLinkHref\"] instanceof String) {\n    links.add(list[i][\"AlertLinkHref\"]);\n  }\n} if (links.length == 0) {\n  ctx.o365audit.remove(\"AlertLinks\");\n  return;\n} ctx.o365audit.AlertLinks = links;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def list = ctx.o365audit.AlertLinks; def links = new ArrayList(); for (int i = 0; i < list.length; ++i) {\n  if (list[i] instanceof Map && list[i].containsKey(\"AlertLinkHref\") && list[i][\"AlertLinkHref\"] != null && list[i][\"AlertLinkHref\"] instanceof String) {\n    links.add(list[i][\"AlertLinkHref\"]);\n  }\n} if (links.length == 0) {\n  ctx.o365audit.remove(\"AlertLinks\");\n  return;\n} ctx.o365audit.AlertLinks = links;\n"#,
                )?;
            }

            let _cond = { event.get_str("o365audit.Severity") == Some("informational") };
            if _cond {
                event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("o365audit.Severity") == Some("low") };
            if _cond {
                event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("o365audit.Severity") == Some("medium") };
            if _cond {
                event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("o365audit.Severity") == Some("high") };
            if _cond {
                event.set("event.severity", json!(4))?;
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeAdmin") };
            if _cond {
                if event.has("o365audit.OrganizationName") {
                    event.rename("o365audit.OrganizationName", "organization.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeAdmin") };
            if _cond {
                if event.has("o365audit.OriginatingServer") {
                    event.rename("o365audit.OriginatingServer", "server._temp")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has("o365audit.MailboxOwnerUPN") {
                    event.rename("o365audit.MailboxOwnerUPN", "user.email")?;
                }
            }

            let _cond = {
                !event.has("user.id")
                    && event.has("o365audit.LogonUserSid")
                    && event.get_str("event.code") == Some("ExchangeItem")
            };
            if _cond {
                if event.has("o365audit.LogonUserSid") {
                    if let Some(val) = event.get("o365audit.LogonUserSid") {
                        let converted = match val {
                            Value::String(_) => val.clone(),
                            Value::Number(n) => json!(n.to_string()),
                            Value::Bool(b) => json!(b.to_string()),
                            Value::Null => json!("null"),
                            _ => json!(val.to_string()),
                        };
                        event.set("user.id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has("o365audit.LogonUserDisplayName") {
                    event.rename("o365audit.LogonUserDisplayName", "user.full_name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has("o365audit.OrganizationName") {
                    event.rename("o365audit.OrganizationName", "organization.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has("o365audit.OriginatingServer") {
                    event.rename("o365audit.OriginatingServer", "server._temp")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has("o365audit.ClientIPAddress") {
                    event.rename("o365audit.ClientIPAddress", "client._temp")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has("o365audit.ClientProcessName") {
                    event.rename("o365audit.ClientProcessName", "process.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("AzureActiveDirectory") };
            if _cond {
                event.set(
                    "user.target.id",
                    event
                        .get("o365audit.ObjectId")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("Add user.")
            };
            if _cond {
                event.set("event.action", json!("added-user-account"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("added-user-account")
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("added-user-account")
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("added-user-account")
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("Update user.")
            };
            if _cond {
                event.set("event.action", json!("modified-user-account"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("modified-user-account")
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("modified-user-account")
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("modified-user-account")
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("Delete user.")
            };
            if _cond {
                event.set("event.action", json!("deleted-user-account"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("deleted-user-account")
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("deleted-user-account")
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AzureActiveDirectory")
                    && event.get_str("event.action") == Some("deleted-user-account")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.get_str("event.code") == Some("AzureActiveDirectoryStsLogon") };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("event.code") == Some("AzureActiveDirectoryStsLogon") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("event.code") == Some("AzureActiveDirectoryStsLogon") };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                event.has("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.ObjectId") {
                    event.rename("o365audit.ObjectId", "url.original")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SourceRelativeUrl") {
                    event.rename("o365audit.SourceRelativeUrl", "file.directory")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SourceFileName") {
                    event.rename("o365audit.SourceFileName", "file.name")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SourceFileExtension") {
                    event.rename("o365audit.SourceFileExtension", "file.extension")?;
                }
            }

            let _cond = {
                event.has("event.action")
                    && [
                        "FileAccessed",
                        "FileDeleted",
                        "FileDownloaded",
                        "FileModified",
                        "FileMoved",
                        "FileRenamed",
                        "FileRestored",
                        "FileUploaded",
                        "FolderCopied",
                        "FolderCreated",
                        "FolderDeleted",
                        "FolderModified",
                        "FolderMoved",
                        "FolderRenamed",
                        "FolderRestored",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = { event.get_str("event.action") == Some("ComplianceSettingChanged") };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has("event.action")
                    && ["FileAccessed", "FileDownloaded"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                event.has("event.action")
                    && [
                        "ComplianceSettingChanged",
                        "FileModified",
                        "FileMoved",
                        "FileRenamed",
                        "FileRestored",
                        "FolderModified",
                        "FolderMoved",
                        "FolderRenamed",
                        "FolderRestored",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.has("event.action")
                    && ["FileDeleted", "FolderDeleted"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has("event.action")
                    && ["FileUploaded", "FolderCopied", "FolderCreated"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has("o365audit.Comments") {
                    event.rename("o365audit.Comments", "message")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has("o365audit.Name") {
                    event.rename("o365audit.Name", "rule.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has("o365audit.PolicyId") {
                    event.rename("o365audit.PolicyId", "rule.id")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has("o365audit.Category") {
                    event.rename("o365audit.Category", "rule.category")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has("o365audit.EntityType") {
                    event.rename("o365audit.EntityType", "rule.ruleset")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has("o365audit.AlertEntityId") {
                    event.rename("o365audit.AlertEntityId", "rule.description")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has("o365audit.AlertLinks") {
                    event.rename("o365audit.AlertLinks", "rule.reference")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.get_str("o365audit.Category") == Some("AccessGovernance")
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.has("o365audit.Category")
                    && ["DataGovernance", "DataLossPrevention"]
                        .contains(&event.get_str("o365audit.Category").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.get_str("o365audit.Category") == Some("ThreatManagement")
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.has("o365audit.Category")
                    && !([
                        "DataGovernance",
                        "DataLossPrevention",
                        "ThreatManagement",
                        "AccessGovernance",
                    ]
                    .contains(&event.get_str("o365audit.Category").unwrap_or("")))
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                event.append("event.category", json!("web"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                !event.has("user.id")
                    && event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.get_str("rule.ruleset") == Some("User")
            };
            if _cond {
                if event.has("o365audit.AlertEntityId") {
                    if let Some(val) = event.get("o365audit.AlertEntityId") {
                        let converted = match val {
                            Value::String(_) => val.clone(),
                            Value::Number(n) => json!(n.to_string()),
                            Value::Bool(b) => json!(b.to_string()),
                            Value::Null => json!("null"),
                            _ => json!(val.to_string()),
                        };
                        event.set("user.id", converted)?;
                    }
                }
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.has("rule.ruleset")
                    && ["Recipients", "Sender"]
                        .contains(&event.get_str("rule.ruleset").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.AlertEntityId") {
                    event.rename("o365audit.AlertEntityId", "user.email")?;
                }
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.get_str("rule.ruleset") == Some("MalwareFamily")
            };
            if _cond {
                if event.has("o365audit.AlertEntityId") {
                    event.rename("o365audit.AlertEntityId", "threat.technique.id")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                !event.has("user.id")
                    && event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.From") {
                    event.rename("o365audit.SharePointMetaData.From", "user.id")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.FileName") {
                    event.rename("o365audit.SharePointMetaData.FileName", "file.name")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.FilePathUrl") {
                    event.rename("o365audit.SharePointMetaData.FilePathUrl", "url.original")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.UniqueId") {
                    event.rename("o365audit.SharePointMetaData.UniqueId", "file.inode")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.UniqueID") {
                    event.rename("o365audit.SharePointMetaData.UniqueID", "file.inode")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.FileOwner") {
                    event.rename("o365audit.SharePointMetaData.FileOwner", "file.owner")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.ExchangeMetaData.From") {
                    event.rename("o365audit.ExchangeMetaData.From", "source.user.email")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.ExchangeMetaData.Subject") {
                    event.rename("o365audit.ExchangeMetaData.Subject", "message")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.PolicyId") {
                    event.rename("o365audit.PolicyId", "rule.id")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.PolicyName") {
                    event.rename("o365audit.PolicyName", "rule.name")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                    && event.has("o365audit.SharePointMetaData.LastModifiedTime")
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("o365audit.SharePointMetaData.LastModifiedTime")
                {
                    // Try ISO8601 format
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        })
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                        })
                    {
                        event.set(
                            "file.mtime",
                            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        )?;
                    }
                }
            }

            let _cond = {
                event.has("event.code")
                    && event.has("o365audit.ExchangeMetaData")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                // Painless script
                // Source: def fields = new def[] {\"To\", \"CC\", \"BCC\"}; if (ctx.destination == null) {\n  ctx.destination = new HashMap();\n} if (ctx.destination.user == null) {\n  ctx.destination.user = new HashMap();\n} ctx.destination.user.email = new ArrayList(); for (int i = 0; i < fields.length; ++i) {\n  if (ctx.o365audit.ExchangeMetaData instanceof Map && ctx.o365audit.ExchangeMetaData.containsKey(fields[i])) {\n    def emails = ctx.o365audit.ExchangeMetaData[fields[i]];\n    if (emails instanceof List){\n      for (int e = 0; e < emails.length; ++e) {\n        ctx.destination.user.email.add(emails[e]);\n      }\n    }\n    if (emails instanceof String){\n      ctx.destination.user.email.add(emails);\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def fields = new def[] {\"To\", \"CC\", \"BCC\"}; if (ctx.destination == null) {\n  ctx.destination = new HashMap();\n} if (ctx.destination.user == null) {\n  ctx.destination.user = new HashMap();\n} ctx.destination.user.email = new ArrayList(); for (int i = 0; i < fields.length; ++i) {\n  if (ctx.o365audit.ExchangeMetaData instanceof Map && ctx.o365audit.ExchangeMetaData.containsKey(fields[i])) {\n    def emails = ctx.o365audit.ExchangeMetaData[fields[i]];\n    if (emails instanceof List){\n      for (int e = 0; e < emails.length; ++e) {\n        ctx.destination.user.email.add(emails[e]);\n      }\n    }\n    if (emails instanceof String){\n      ctx.destination.user.email.add(emails);\n    }\n  }\n}\n"#,
                )?;
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                    && event.has("o365audit.ExceptionInfo")
                    && event
                        .get("o365audit.ExceptionInfo")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has("o365audit.ExceptionInfo") {
                    event.rename("o365audit.ExceptionInfo", "o365audit.ExceptionInfo.Reason")?;
                }
            }

            let _cond = {
                event.has("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                    && event.has("o365audit.PolicyDetails")
            };
            if _cond {
                // Painless script
                // Source: int severityToCode(def x) { \n  if (x.toLowerCase() == \"informational\") {\n    return 1;\n  }\n  if (x.toLowerCase() == \"low\") {\n    return 2;\n  }\n  if (x.toLowerCase() == \"medium\") {\n    return 3;\n  }\n  if (x.toLowerCase() == \"high\") {\n    return 4;\n  }\n  return 0;\n} def policies = ctx.o365audit.PolicyDetails; if (policies == null) {\n  return;\n} if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n} if (ctx.rule.id == null) {\n  ctx.rule.id = new ArrayList();\n} if (ctx.rule.name == null) {\n  ctx.rule.name = new ArrayList();\n} def maxSeverity = 0; def allowed = true; for (int i = 0; i < policies.length && policies instanceof List; ++i) {\n  def rules = policies[i].Rules;\n  if (rules == null) {\n    continue;\n  }\n  for (int j = 0; j < rules.length; ++j) {\n    def rule = rules[j];\n    def id = rule.RuleId;\n    def name = rule.RuleName;\n    def sev = severityToCode(rule.Severity);\n    if (id != null && name != null) {\n      ctx.rule.id.add(id);\n      ctx.rule.name.add(name);\n    }\n    if (sev > maxSeverity) {\n      maxSeverity = sev;\n    }\n    if (allowed) {\n      if (rule.Actions != null && rule.Actions.contains(\"BlockAccess\")) {\n        allowed = false;\n      }\n    }\n  }\n} if (maxSeverity > -1) {\n  ctx.event.severity = maxSeverity;\n} if (allowed) {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpRuleUndo\") {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpInfo\") {\n  ctx.event.outcome = \"failure\";\n  return;\n} if (ctx.o365audit?.ExceptionInfo != null && !ctx.o365audit?.ExceptionInfo.isEmpty()) {\n  ctx.event.outcome = \"success\";\n  return;\n} ctx.event.outcome = \"failure\";\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"int severityToCode(def x) { \n  if (x.toLowerCase() == \"informational\") {\n    return 1;\n  }\n  if (x.toLowerCase() == \"low\") {\n    return 2;\n  }\n  if (x.toLowerCase() == \"medium\") {\n    return 3;\n  }\n  if (x.toLowerCase() == \"high\") {\n    return 4;\n  }\n  return 0;\n} def policies = ctx.o365audit.PolicyDetails; if (policies == null) {\n  return;\n} if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n} if (ctx.rule.id == null) {\n  ctx.rule.id = new ArrayList();\n} if (ctx.rule.name == null) {\n  ctx.rule.name = new ArrayList();\n} def maxSeverity = 0; def allowed = true; for (int i = 0; i < policies.length && policies instanceof List; ++i) {\n  def rules = policies[i].Rules;\n  if (rules == null) {\n    continue;\n  }\n  for (int j = 0; j < rules.length; ++j) {\n    def rule = rules[j];\n    def id = rule.RuleId;\n    def name = rule.RuleName;\n    def sev = severityToCode(rule.Severity);\n    if (id != null && name != null) {\n      ctx.rule.id.add(id);\n      ctx.rule.name.add(name);\n    }\n    if (sev > maxSeverity) {\n      maxSeverity = sev;\n    }\n    if (allowed) {\n      if (rule.Actions != null && rule.Actions.contains(\"BlockAccess\")) {\n        allowed = false;\n      }\n    }\n  }\n} if (maxSeverity > -1) {\n  ctx.event.severity = maxSeverity;\n} if (allowed) {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpRuleUndo\") {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpInfo\") {\n  ctx.event.outcome = \"failure\";\n  return;\n} if (ctx.o365audit?.ExceptionInfo != null && !ctx.o365audit?.ExceptionInfo.isEmpty()) {\n  ctx.event.outcome = \"success\";\n  return;\n} ctx.event.outcome = \"failure\";\n"#,
                )?;
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.ActorUserId") {
                    event.rename("o365audit.ActorUserId", "user.email")?;
                }
            }

            let _cond = { !event.has("user.id") && event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.ActorYammerUserId") {
                    if let Some(val) = event.get("o365audit.ActorYammerUserId") {
                        let converted = match val {
                            Value::String(_) => val.clone(),
                            Value::Number(n) => json!(n.to_string()),
                            Value::Bool(b) => json!(b.to_string()),
                            Value::Null => json!("null"),
                            _ => json!(val.to_string()),
                        };
                        event.set("user.id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.FileId") {
                    event.rename("o365audit.FileId", "file.inode")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.FileName") {
                    event.rename("o365audit.FileName", "file.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.GroupName") {
                    event.rename("o365audit.GroupName", "group.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.TargetUserId") {
                    event.rename("o365audit.TargetUserId", "destination.user.email")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.TargetYammerUserId") {
                    event.rename("o365audit.TargetYammerUserId", "destination.user.id")?;
                }
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.has("event.action")
                    && [
                        "NetworkConfigurationUpdated",
                        "NetworkSecurityConfigurationUpdated",
                        "SoftDeleteSettingsUpdated",
                        "ProcessProfileFields",
                        "SupervisorAdminToggled",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.has("event.action")
                    && [
                        "NetworkSecurityConfigurationUpdated",
                        "GroupCreation",
                        "GroupDeletion",
                        "NetworkUserSuspended",
                        "UserSuspension",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.has("event.action")
                    && [
                        "FileCreated",
                        "FileDownloaded",
                        "FileShared",
                        "FileUpdateDescription",
                        "FileUpdateName",
                        "FileVisited",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.has("event.action")
                    && [
                        "NetworkConfigurationUpdated",
                        "NetworkSecurityConfigurationUpdated",
                        "SoftDeleteSettingsUpdated",
                        "ProcessProfileFields",
                        "SupervisorAdminToggled",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.get_str("event.action") == Some("NetworkSecurityConfigurationUpdated")
            };
            if _cond {
                event.append("event.type", json!("admin"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.has("event.action")
                    && ["FileCreated", "GroupCreation", "FileUpdateName"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.get_str("event.action") == Some("GroupDeletion")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.has("event.action")
                    && [
                        "FileDownloaded",
                        "FileShared",
                        "FileUpdateDescription",
                        "FileVisited",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("Yammer")
                    && event.has("event.action")
                    && ["GroupCreation", "GroupDeletion"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("TeamCreated")
            };
            if _cond {
                event.set("event.action", json!("added-group-account-to"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("added-group-account-to")
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("added-group-account-to")
            };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("added-group-account-to")
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.code") == Some("MicrosoftTeams") };
            if _cond {
                if event.has("o365audit.TeamName") {
                    event.rename("o365audit.TeamName", "group.name")?;
                }
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("MemberAdded")
            };
            if _cond {
                event.set("event.action", json!("added-users-to-group"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("added-users-to-group")
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("added-users-to-group")
            };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("added-users-to-group")
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("Delete user.")
            };
            if _cond {
                event.set("event.action", json!("deleted-user-account"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("deleted-user-account")
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("deleted-user-account")
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("deleted-user-account")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.get_str("event.action") == Some("deleted-user-account")
            };
            if _cond {
                if event.has("o365audit.ObjectId") {
                    event.rename("o365audit.ObjectId", "user.target.id")?;
                }
            }

            let _cond = {
                event.get_str("event.code") == Some("MicrosoftTeams")
                    && event.has("o365audit.Members")
                    && event.get("o365audit.Members").is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def members = ctx.o365audit?.Members; if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} for (int i = 0; i < members.length; ++i) {\n  if (members[i] instanceof Map && members[i].containsKey(\"UPN\") && !members[i][\"UPN\"].isEmpty()) {\n    ctx.related.user.add(members[i][\"UPN\"]);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def members = ctx.o365audit?.Members; if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} for (int i = 0; i < members.length; ++i) {\n  if (members[i] instanceof Map && members[i].containsKey(\"UPN\") && !members[i][\"UPN\"].isEmpty()) {\n    ctx.related.user.add(members[i][\"UPN\"]);\n  }\n}\n"#,
                )?;
            }

            if event.has("client._temp") {
                if let Some(s) = event.get_string("client._temp") {
                    let re = cached_regex!("::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)");
                    let replaced = re.replace_all(&s, "$1").into_owned();
                    event.set("client._temp", replaced)?;
                }
            }

            let _cond = {
                event.has("client._temp")
                    && !(event.get_str("client._temp").is_none_or(|s| s.is_empty()))
            };
            if _cond {
                if let Some(input) = event.get_string("client._temp") {
                    // Grok pattern: (?:^\\[%{IP:client.address}\\]:%{POSINT:client._port})
                    if !cached_grok!("(?:^\\[%{IP:client.address}\\]:%{POSINT:client._port})")
                        .extract_into(&input, event)?
                    {
                        // Grok pattern: ^%{IP:client.address}$
                        if !cached_grok!("^%{IP:client.address}$").extract_into(&input, event)? {
                            // Grok pattern: ^\\[%{IP:client.address}\\]$
                            if !cached_grok!("^\\[%{IP:client.address}\\]$")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: (?:^%{IP:client.address}:%{POSINT:client._port})
                                if !cached_grok!("(?:^%{IP:client.address}:%{POSINT:client._port})")
                                    .extract_into(&input, event)?
                                {
                                    // Grok pattern: ^%{NOTSPACE:client.domain}$
                                    if !cached_grok!("^%{NOTSPACE:client.domain}$")
                                        .extract_into(&input, event)?
                                    {
                                        // Grok pattern: (?:^\\[%{NOTSPACE:client.domain}\\]:%{POSINT:client._port})
                                        if !cached_grok!("(?:^\\[%{NOTSPACE:client.domain}\\]:%{POSINT:client._port})").extract_into(&input, event)? {
                                            // Grok pattern: (?:^%{NOTSPACE:client.domain}:%{POSINT:client._port})
                                            if !cached_grok!("(?:^%{NOTSPACE:client.domain}:%{POSINT:client._port})").extract_into(&input, event)? {
                                                // Grok pattern: ^\\[(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))\\]$
                                                if !cached_grok_mapped!("^\\[(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))\\]$", [("client_address", "client.address")]).extract_into(&input, event)? {
                                                    // Grok pattern: ^(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))$
                                                    if !cached_grok_mapped!("^(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))$", [("client_address", "client.address")]).extract_into(&input, event)? {
                                                        // Grok pattern: %{GREEDYDATA:client.address}
                                                        if !cached_grok!("%{GREEDYDATA:client.address}").extract_into(&input, event)? {
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if event.has("server._temp") {
                if let Some(s) = event.get_string("server._temp") {
                    let re = cached_regex!("[\n\r]");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("server._temp", replaced)?;
                }
            }

            let _cond = {
                event.has("server._temp")
                    && !(event.get_str("server._temp").is_none_or(|s| s.is_empty()))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("server._temp") {
                        // Grok pattern: ^\\[(?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))\\]$
                        if !cached_grok_mapped!("^\\[(?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))\\]$", [("server_address", "server.address")]).extract_into(&input, event)? {
                        // Grok pattern: (?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))
                        if !cached_grok_mapped!("(?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))", [("server_address", "server.address")]).extract_into(&input, event)? {
                            // Grok pattern: %{GREEDYDATA:server.address}
                            if !cached_grok!("%{GREEDYDATA:server.address}").extract_into(&input, event)? {
                            }
                        }
                    }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("client.address") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "client.address".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("client.ip", s)?;
                }
                Ok(())
            })();

            if event.has("client._port") {
                if let Some(val) = event.get("client._port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "client._port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "client._port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "client._port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("client.port", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("server.address") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "server.address".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("server.ip", s)?;
                }
                Ok(())
            })();

            event.remove("client._port");
            event.remove("client._temp");
            event.remove("server._temp");

            let _cond = { event.has("client.ip") };
            if _cond {
                event.set(
                    "source.ip",
                    event.get("client.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("client.port") };
            if _cond {
                event.set(
                    "source.port",
                    event.get("client.port").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("server.ip") };
            if _cond {
                event.set(
                    "destination.ip",
                    event.get("server.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has("user.id")
                    && event.get("user.id").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"String[] splitmail = ctx.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];\n"#,
                )?;
            }

            let _cond = {
                event.has("user.target.id")
                    && event.get("user.target.id").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.user.target.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.target.email = ctx.user.target.id; ctx.user.target.domain = splitmail[1]; ctx.user.target.name = splitmail[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"String[] splitmail = ctx.user.target.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.target.email = ctx.user.target.id; ctx.user.target.domain = splitmail[1]; ctx.user.target.name = splitmail[0];\n"#,
                )?;
            }

            let _cond = {
                event.has("source.user.id")
                    && event.get("source.user.id").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.source.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.source.user.email = ctx.source.user.id; ctx.source.user.domain = splitmail[1]; ctx.source.user.name = splitmail[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"String[] splitmail = ctx.source.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.source.user.email = ctx.source.user.id; ctx.source.user.domain = splitmail[1]; ctx.source.user.name = splitmail[0];\n"#,
                )?;
            }

            let _cond = {
                event.has("destination.user.id")
                    && event.get("destination.user.id").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.destination.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.destination.user.email = ctx.destination.user.id; ctx.destination.user.domain = splitmail[1]; ctx.destination.user.name = splitmail[0];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"String[] splitmail = ctx.destination.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.destination.user.email = ctx.destination.user.id; ctx.destination.user.domain = splitmail[1]; ctx.destination.user.name = splitmail[0];\n"#,
                )?;
            }

            let _cond = {
                event.has("client.ip")
                    && event.get("client.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { !event.has("network.type") && event.has("client.ip") };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = { event.has("client.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("client.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("server.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("server.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("user.name").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("user.target.name") };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("user.target.name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("file.owner") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("file.owner").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("o365audit.Parameters.User") };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("o365audit.Parameters.User")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("o365audit.ExtendedProperties.UserAgent") };
            if _cond {
                if event.has("o365audit.ExtendedProperties.UserAgent") {
                    event.rename(
                        "o365audit.ExtendedProperties.UserAgent",
                        "user_agent.original",
                    )?;
                }
            }

            if event.has("organization.id") {
                if let Some(s) = event.get_string("organization.id") {
                    let lowered = s.to_lowercase();
                    event.set("organization.id", lowered)?;
                }
            }

            let _cond = { event.has("organization.id") };
            if _cond {
                event.set(
                    "host.id",
                    event.get("organization.id").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("organization.id") && event.has("_conf.tenants") };
            if _cond {
                // Painless script
                // Source: def conftenants = ctx._conf.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n  ctx.host.name = conftenants[orgid];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def conftenants = ctx._conf.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n  ctx.host.name = conftenants[orgid];\n}\n"#,
                )?;
            }

            let _cond = { event.has("organization.name") && !event.has("host.name") };
            if _cond {
                event.set(
                    "host.name",
                    event
                        .get("organization.name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has("user.domain") && !event.has("host.name") };
            if _cond {
                event.set(
                    "host.name",
                    event.get("user.domain").cloned().unwrap_or(Value::Null),
                )?;
            }

            if event.has("o365audit.AzureActiveDirectoryEventType") {
                if let Some(val) = event.get("o365audit.AzureActiveDirectoryEventType") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.AzureActiveDirectoryEventType", converted)?;
                }
            }

            if event.has("o365audit.RecordType") {
                if let Some(val) = event.get("o365audit.RecordType") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.RecordType", converted)?;
                }
            }

            if event.has("o365audit.UserType") {
                if let Some(val) = event.get("o365audit.UserType") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.UserType", converted)?;
                }
            }

            let _cond = { event.get("o365audit.Actor").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("o365audit.Actor").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has("_ingest._value.Type") {
                            if let Some(val) = event.get("_ingest._value.Type") {
                                let converted = match val {
                                    Value::String(_) => val.clone(),
                                    Value::Number(n) => json!(n.to_string()),
                                    Value::Bool(b) => json!(b.to_string()),
                                    Value::Null => json!("null"),
                                    _ => json!(val.to_string()),
                                };
                                event.set("_ingest._value.Type", converted)?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("o365audit.Actor", Value::Array(out))?;
                }
            }

            let _cond = { event.get("o365audit.Target").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("o365audit.Target").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has("_ingest._value.Type") {
                            if let Some(val) = event.get("_ingest._value.Type") {
                                let converted = match val {
                                    Value::String(_) => val.clone(),
                                    Value::Number(n) => json!(n.to_string()),
                                    Value::Bool(b) => json!(b.to_string()),
                                    Value::Null => json!("null"),
                                    _ => json!(val.to_string()),
                                };
                                event.set("_ingest._value.Type", converted)?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("o365audit.Target", Value::Array(out))?;
                }
            }

            if event.has("o365audit.Version") {
                if let Some(val) = event.get("o365audit.Version") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.Version", converted)?;
                }
            }

            if event.has("o365audit.InternalLogonType") {
                if let Some(val) = event.get("o365audit.InternalLogonType") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.InternalLogonType", converted)?;
                }
            }

            if event.has("o365audit.LogonType") {
                if let Some(val) = event.get("o365audit.LogonType") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.LogonType", converted)?;
                }
            }

            if event.has("o365audit.ActorYammerUserId") {
                if let Some(val) = event.get("o365audit.ActorYammerUserId") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.ActorYammerUserId", converted)?;
                }
            }

            if event.has("o365audit.YammerNetworkId") {
                if let Some(val) = event.get("o365audit.YammerNetworkId") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("o365audit.YammerNetworkId", converted)?;
                }
            }

            // SKIPPED: condition not transpiled: ctx.o365audit?.containsKey('Data') == true
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(s) = event.get_string("o365audit.Data") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "o365audit.Data".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("o365audit.Data", parsed)?;
                }
            }

            if event.has("o365audit.Data") {
                event.rename("o365audit.Data", "o365audit.Data.flattened")?;
            }

            let _cond = {
                event
                    .get("o365audit.Data.flattened")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: def knownKeys = ['ad', 'af', 'aii', 'ail', 'alk', 'als', 'an', 'at',\n  'cid', 'cpid', 'dm', 'dpn', 'eid', 'etps', 'etype', 'f3u', 'fvs',\n  'imsgid', 'lon', 'mat', 'md', 'ms', 'od', 'op', 'ot', 'plk', 'pud',\n  'reid', 'rid', 'sev', 'sict', 'sid', 'sip', 'sitmi', 'srt', 'ssic',\n  'suid', 'tdc', 'te', 'thn', 'tht', 'tid', 'tpid', 'tpt', 'trc', 'ts',\n  'tsd', 'ttdt', 'ttr', 'upfc', 'upfv', 'ut', 'von', 'wl', 'zfh', 'zfn',\n  'zmfh', 'zmfn', 'zu'];\nfor (def key : knownKeys) {\n  if (ctx.o365audit.Data.flattened.containsKey(key)) {\n    ctx.o365audit.Data[key] = ctx.o365audit.Data.flattened[key];\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    r#"def knownKeys = ['ad', 'af', 'aii', 'ail', 'alk', 'als', 'an', 'at',\n  'cid', 'cpid', 'dm', 'dpn', 'eid', 'etps', 'etype', 'f3u', 'fvs',\n  'imsgid', 'lon', 'mat', 'md', 'ms', 'od', 'op', 'ot', 'plk', 'pud',\n  'reid', 'rid', 'sev', 'sict', 'sid', 'sip', 'sitmi', 'srt', 'ssic',\n  'suid', 'tdc', 'te', 'thn', 'tht', 'tid', 'tpid', 'tpt', 'trc', 'ts',\n  'tsd', 'ttdt', 'ttr', 'upfc', 'upfv', 'ut', 'von', 'wl', 'zfh', 'zfn',\n  'zmfh', 'zmfn', 'zu'];\nfor (def key : knownKeys) {\n  if (ctx.o365audit.Data.flattened.containsKey(key)) {\n    ctx.o365audit.Data[key] = ctx.o365audit.Data.flattened[key];\n  }\n}\n"#,
                )?;
            }

            if event.has("o365audit.Data.sip") {
                if let Some(s) = event.get_string("o365audit.Data.sip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "o365audit.Data.sip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("o365audit.Data.sip", s)?;
                }
            }

            let _cond = { event.has("o365audit.Data.at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.at") {
                    // Try ISO8601 format
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        })
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                        })
                    {
                        event.set(
                            "o365audit.Data.at",
                            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        )?;
                    }
                }
            }

            let _cond = { event.has("o365audit.Data.md") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.md") {
                    // Try ISO8601 format
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        })
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                        })
                    {
                        event.set(
                            "o365audit.Data.md",
                            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        )?;
                    }
                }
            }

            let _cond = { event.has("o365audit.Data.te") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.te") {
                    // Try ISO8601 format
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        })
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                        })
                    {
                        event.set(
                            "o365audit.Data.te",
                            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        )?;
                    }
                }
            }

            let _cond = { event.has("o365audit.Data.ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.ts") {
                    // Try ISO8601 format
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        })
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                        })
                    {
                        event.set(
                            "o365audit.Data.ts",
                            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        )?;
                    }
                }
            }

            let _cond = { event.has("o365audit.Data.ttdt") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.ttdt") {
                    // Try ISO8601 format
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        })
                        .or_else(|_| {
                            chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                        })
                    {
                        event.set(
                            "o365audit.Data.ttdt",
                            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.f3u")
                    .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("o365audit.Data.f3u")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.suid")
                    .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("o365audit.Data.suid")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.tsd")
                    .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("o365audit.Data.tsd")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.trc")
                    .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("o365audit.Data.trc")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            if event.has("o365audit") {
                event.rename("o365audit", "o365.audit")?;
            }

            if event.has("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            if event.has("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            event.remove("_conf");

            let _cond = {
                !event.has("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("event.original");
                    Ok(())
                })();
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
                    event
                        .get("_ingest.on_failure_message")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
