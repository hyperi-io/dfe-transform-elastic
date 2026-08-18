// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        // TODO: conditional: ctx.event?.original == null
        {
            if event.has("message") {
                event.rename("message", "event.original")?;
            }
        }

        // TODO: conditional: ctx.event?.original == null
        {
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

        // TODO: conditional: ctx.o365audit?.CreationTime != null
        {
            if let Some(date_str) = event.get_str("o365audit.CreationTime").map(String::from) {
                let date_str = date_str.as_str();
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%:z"))
                {
                    event.set("@timestamp", dt.to_rfc3339())?;
                }
            }
        }

        if event.has("o365audit.Id") {
            event.rename("o365audit.Id", "event.id")?;
        }

        // TODO: conditional: ctx.o365audit?.ListBaseType != null
        {
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
        }

        if event.has("o365audit.ClientIPAddress") {
            event.rename("o365audit.ClientIPAddress", "client._temp")?;
        }

        // TODO: conditional: ctx.client?._temp == null
        {
            if event.has("o365audit.ClientIP") {
                event.rename("o365audit.ClientIP", "client._temp")?;
            }
        }

        // TODO: conditional: ctx.client?._temp == null
        {
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

        // TODO: conditional: ctx.o365audit?.OperationProperties instanceof String
        {
            if let Some(s) = event
                .get_str("o365audit.OperationProperties")
                .map(String::from)
            {
                let s = s.as_str();
                let parsed: Value =
                    serde_json::from_str(s).map_err(|e| TransformError::ParseError {
                        path: "o365audit.OperationProperties".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("o365audit.OperationProperties", parsed)?;
            }
        }

        if event.has("o365audit.UserAgent") {
            event.rename("o365audit.UserAgent", "user_agent.original")?;
        }

        // TODO: conditional: ctx.o365audit?.RecordType != null
        {
            // Painless script
            // Source: def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n"#,
            )?;
        }

        // TODO: conditional: ctx.o365audit?.ResultStatus != null && ["succeeded", "success", "partiallysucceeded", "true"].contains(ctx.o365audit?.ResultStatus.toLowerCase())
        {
            event.set("event.outcome", json!("success"))?;
        }

        // TODO: conditional: ctx.o365audit?.ResultStatus != null && ["failed", "false"].contains(ctx.o365audit?.ResultStatus.toLowerCase())
        {
            event.set("event.outcome", json!("failure"))?;
        }

        // TODO: conditional: ctx.event?.outcome == null
        {
            event.set("event.outcome", json!("success"))?;
        }

        // TODO: conditional: ctx.o365audit?.Parameters != null && ctx.o365audit?.Parameters instanceof List
        {
            // Painless script
            // Source: def newparams = new HashMap();  def oldparams = ctx.o365audit.Parameters; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.Parameters = newparams;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.Parameters; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.Parameters = newparams;\n"#,
            )?;
        }

        // TODO: conditional: ctx.o365audit?.Parameters != null && ctx.o365audit?.Parameters instanceof String
        {
            event.rename("o365audit.Parameters", "o365audit.Parameters._raw")?;
        }

        // TODO: conditional: ctx.o365audit?.Platform != null
        {
            // Painless script
            // Source: def value = ctx.o365audit.Platform.toString(); def name = params[value]; if (name != null) {\n  ctx.o365audit.Platform = name;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def value = ctx.o365audit.Platform.toString(); def name = params[value]; if (name != null) {\n  ctx.o365audit.Platform = name;\n}\n"#,
            )?;
        }

        // TODO: conditional: ctx.o365audit?.ExtendedProperties != null && ctx.o365audit?.ExtendedProperties instanceof List
        {
            // Painless script
            // Source: def newparams = new HashMap();  def oldparams = ctx.o365audit.ExtendedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.ExtendedProperties = newparams;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.ExtendedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.ExtendedProperties = newparams;\n"#,
            )?;
        }

        // TODO: conditional: ctx.o365audit?.ExtendedProperties != null && ctx.o365audit?.ExtendedProperties instanceof String
        {
            event.rename(
                "o365audit.ExtendedProperties",
                "o365audit.ExtendedProperties._raw",
            )?;
        }

        // TODO: conditional: ctx.o365audit?.ModifiedProperties != null && ctx.o365audit?.ModifiedProperties instanceof List
        {
            // Painless script
            // Source: def newparams = new HashMap();  def oldparams = ctx.o365audit.ModifiedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i] instanceof Map && oldparams[i][\"OldValue\"] != null && oldparams[i][\"NewValue\"] != null) {\n    def validname = oldparams[i][\"Name\"].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n    newparams[validname][\"NewValue\"] = oldparams[i][\"NewValue\"];\n    newparams[validname][\"OldValue\"] = oldparams[i][\"OldValue\"];\n  }\n  if (oldparams[i] instanceof String) {\n    def validname = oldparams[i].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n  }\n} if (newparams.isEmpty()) {\n  ctx.o365audit.remove(\"ModifiedProperties\");\n  return;\n} ctx.o365audit.ModifiedProperties = newparams;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.ModifiedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i] instanceof Map && oldparams[i][\"OldValue\"] != null && oldparams[i][\"NewValue\"] != null) {\n    def validname = oldparams[i][\"Name\"].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n    newparams[validname][\"NewValue\"] = oldparams[i][\"NewValue\"];\n    newparams[validname][\"OldValue\"] = oldparams[i][\"OldValue\"];\n  }\n  if (oldparams[i] instanceof String) {\n    def validname = oldparams[i].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n  }\n} if (newparams.isEmpty()) {\n  ctx.o365audit.remove(\"ModifiedProperties\");\n  return;\n} ctx.o365audit.ModifiedProperties = newparams;\n"#,
            )?;
        }

        // TODO: conditional: ctx.o365audit?.ModifiedProperties != null && ctx.o365audit?.ModifiedProperties instanceof String
        {
            event.rename(
                "o365audit.ModifiedProperties",
                "o365audit.ModifiedProperties._raw",
            )?;
        }

        // TODO: conditional: ctx.o365audit?.AlertLinks != null && ctx.o365audit?.AlertLinks instanceof List
        {
            // Painless script
            // Source: def list = ctx.o365audit.AlertLinks; def links = new ArrayList(); for (int i = 0; i < list.length; ++i) {\n  if (list[i] instanceof Map && list[i].containsKey(\"AlertLinkHref\") && list[i][\"AlertLinkHref\"] != null && list[i][\"AlertLinkHref\"] instanceof String) {\n    links.add(list[i][\"AlertLinkHref\"]);\n  }\n} if (links.length == 0) {\n  ctx.o365audit.remove(\"AlertLinks\");\n  return;\n} ctx.o365audit.AlertLinks = links;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def list = ctx.o365audit.AlertLinks; def links = new ArrayList(); for (int i = 0; i < list.length; ++i) {\n  if (list[i] instanceof Map && list[i].containsKey(\"AlertLinkHref\") && list[i][\"AlertLinkHref\"] != null && list[i][\"AlertLinkHref\"] instanceof String) {\n    links.add(list[i][\"AlertLinkHref\"]);\n  }\n} if (links.length == 0) {\n  ctx.o365audit.remove(\"AlertLinks\");\n  return;\n} ctx.o365audit.AlertLinks = links;\n"#,
            )?;
        }

        // TODO: conditional: ctx.o365audit?.Severity == "informational"
        {
            event.set("event.severity", json!(1))?;
        }

        // TODO: conditional: ctx.o365audit?.Severity == "low"
        {
            event.set("event.severity", json!(2))?;
        }

        // TODO: conditional: ctx.o365audit?.Severity == "medium"
        {
            event.set("event.severity", json!(3))?;
        }

        // TODO: conditional: ctx.o365audit?.Severity == "high"
        {
            event.set("event.severity", json!(4))?;
        }

        // TODO: conditional: ctx.event?.code == "ExchangeAdmin"
        {
            if event.has("o365audit.OrganizationName") {
                event.rename("o365audit.OrganizationName", "organization.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "ExchangeAdmin"
        {
            if event.has("o365audit.OriginatingServer") {
                event.rename("o365audit.OriginatingServer", "server._temp")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "ExchangeItem"
        {
            if event.has("o365audit.MailboxOwnerUPN") {
                event.rename("o365audit.MailboxOwnerUPN", "user.email")?;
            }
        }

        // TODO: conditional: ctx.user?.id == null && ctx.o365audit?.LogonUserSid != null && ctx.event?.code == "ExchangeItem"
        {
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

        // TODO: conditional: ctx.event?.code == "ExchangeItem"
        {
            if event.has("o365audit.LogonUserDisplayName") {
                event.rename("o365audit.LogonUserDisplayName", "user.full_name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "ExchangeItem"
        {
            if event.has("o365audit.OrganizationName") {
                event.rename("o365audit.OrganizationName", "organization.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "ExchangeItem"
        {
            if event.has("o365audit.OriginatingServer") {
                event.rename("o365audit.OriginatingServer", "server._temp")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "ExchangeItem"
        {
            if event.has("o365audit.ClientIPAddress") {
                event.rename("o365audit.ClientIPAddress", "client._temp")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "ExchangeItem"
        {
            if event.has("o365audit.ClientProcessName") {
                event.rename("o365audit.ClientProcessName", "process.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory"
        {
            event.set(
                "user.target.id",
                event
                    .get("o365audit.ObjectId")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "Add user."
        {
            event.set("event.action", json!("added-user-account"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "added-user-account"
        {
            event.append("event.category", json!("iam"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "added-user-account"
        {
            event.append("event.type", json!("user"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "added-user-account"
        {
            event.append("event.type", json!("creation"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "Update user."
        {
            event.set("event.action", json!("modified-user-account"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "modified-user-account"
        {
            event.append("event.category", json!("iam"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "modified-user-account"
        {
            event.append("event.type", json!("user"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "modified-user-account"
        {
            event.append("event.type", json!("change"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "Delete user."
        {
            event.set("event.action", json!("deleted-user-account"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "deleted-user-account"
        {
            event.append("event.category", json!("iam"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "deleted-user-account"
        {
            event.append("event.type", json!("user"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectory" && ctx.event?.action == "deleted-user-account"
        {
            event.append("event.type", json!("deletion"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectoryStsLogon"
        {
            event.append("event.category", json!("authentication"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectoryStsLogon"
        {
            event.append("event.type", json!("start"))?;
        }

        // TODO: conditional: ctx.event?.code == "AzureActiveDirectoryStsLogon"
        {
            event.append("event.type", json!("access"))?;
        }

        // TODO: conditional: ctx.event?.code != null && ["SharePointFileOperation", "SharePointSharingOperation"].contains(ctx.event.code)
        {
            if event.has("o365audit.ObjectId") {
                event.rename("o365audit.ObjectId", "url.original")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["SharePointFileOperation", "SharePointSharingOperation"].contains(ctx.event.code)
        {
            if event.has("o365audit.SourceRelativeUrl") {
                event.rename("o365audit.SourceRelativeUrl", "file.directory")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["SharePointFileOperation", "SharePointSharingOperation"].contains(ctx.event.code)
        {
            if event.has("o365audit.SourceFileName") {
                event.rename("o365audit.SourceFileName", "file.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["SharePointFileOperation", "SharePointSharingOperation"].contains(ctx.event.code)
        {
            if event.has("o365audit.SourceFileExtension") {
                event.rename("o365audit.SourceFileExtension", "file.extension")?;
            }
        }

        // TODO: conditional: ctx.event?.action != null && ["FileAccessed", "FileDeleted", "FileDownloaded", "FileModified", "FileMoved", "FileRenamed", "FileRestored", "FileUploaded", "FolderCopied", "FolderCreated", "FolderDeleted", "FolderModified", "FolderMoved", "FolderRenamed", "FolderRestored"].contains(ctx.event?.action)
        {
            event.append("event.category", json!("file"))?;
        }

        // TODO: conditional: ctx.event?.action == "ComplianceSettingChanged"
        {
            event.append("event.category", json!("configuration"))?;
        }

        // TODO: conditional: ctx.event?.action != null && ["FileAccessed", "FileDownloaded"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("access"))?;
        }

        // TODO: conditional: ctx.event?.action != null && ["ComplianceSettingChanged", "FileModified", "FileMoved", "FileRenamed", "FileRestored", "FolderModified", "FolderMoved", "FolderRenamed", "FolderRestored"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("change"))?;
        }

        // TODO: conditional: ctx.event?.action != null && ["FileDeleted", "FolderDeleted"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("deletion"))?;
        }

        // TODO: conditional: ctx.event?.action != null && ["FileUploaded", "FolderCopied", "FolderCreated"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("creation"))?;
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            if event.has("o365audit.Comments") {
                event.rename("o365audit.Comments", "message")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            if event.has("o365audit.Name") {
                event.rename("o365audit.Name", "rule.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            if event.has("o365audit.PolicyId") {
                event.rename("o365audit.PolicyId", "rule.id")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            if event.has("o365audit.Category") {
                event.rename("o365audit.Category", "rule.category")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            if event.has("o365audit.EntityType") {
                event.rename("o365audit.EntityType", "rule.ruleset")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            if event.has("o365audit.AlertEntityId") {
                event.rename("o365audit.AlertEntityId", "rule.description")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            if event.has("o365audit.AlertLinks") {
                event.rename("o365audit.AlertLinks", "rule.reference")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            event.set("event.kind", json!("alert"))?;
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts" && ctx.o365audit?.Category == "AccessGovernance"
        {
            event.append("event.category", json!("authentication"))?;
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts" && ctx.o365audit?.Category != null && ["DataGovernance", "DataLossPrevention"].contains(ctx.o365audit?.Category)
        {
            event.append("event.category", json!("file"))?;
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts" && ctx.o365audit?.Category == "ThreatManagement"
        {
            event.append("event.category", json!("malware"))?;
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts" && ctx.o365audit?.Category != null && !["DataGovernance", "DataLossPrevention", "ThreatManagement", "AccessGovernance"].contains(ctx.o365audit?.Category)
        {
            event.append("event.category", json!("authentication"))?;
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            event.append("event.category", json!("web"))?;
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts"
        {
            event.append("event.type", json!("info"))?;
        }

        // TODO: conditional: ctx.user?.id == null && ctx.event?.code == "SecurityComplianceAlerts" && ctx.rule?.ruleset == "User"
        {
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

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts" && ctx.rule?.ruleset != null && ["Recipients", "Sender"].contains(ctx.rule?.ruleset)
        {
            if event.has("o365audit.AlertEntityId") {
                event.rename("o365audit.AlertEntityId", "user.email")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "SecurityComplianceAlerts" && ctx.rule?.ruleset == "MalwareFamily"
        {
            if event.has("o365audit.AlertEntityId") {
                event.rename("o365audit.AlertEntityId", "threat.technique.id")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            event.set("event.kind", json!("alert"))?;
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            event.append("event.category", json!("file"))?;
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            event.append("event.type", json!("access"))?;
        }

        // TODO: conditional: ctx.user?.id == null && ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.SharePointMetaData.From") {
                event.rename("o365audit.SharePointMetaData.From", "user.id")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.SharePointMetaData.FileName") {
                event.rename("o365audit.SharePointMetaData.FileName", "file.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.SharePointMetaData.FilePathUrl") {
                event.rename("o365audit.SharePointMetaData.FilePathUrl", "url.original")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.SharePointMetaData.UniqueId") {
                event.rename("o365audit.SharePointMetaData.UniqueId", "file.inode")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.SharePointMetaData.UniqueID") {
                event.rename("o365audit.SharePointMetaData.UniqueID", "file.inode")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.SharePointMetaData.FileOwner") {
                event.rename("o365audit.SharePointMetaData.FileOwner", "file.owner")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.ExchangeMetaData.From") {
                event.rename("o365audit.ExchangeMetaData.From", "source.user.email")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.ExchangeMetaData.Subject") {
                event.rename("o365audit.ExchangeMetaData.Subject", "message")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.PolicyId") {
                event.rename("o365audit.PolicyId", "rule.id")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            if event.has("o365audit.PolicyName") {
                event.rename("o365audit.PolicyName", "rule.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code) && ctx.o365audit?.SharePointMetaData?.LastModifiedTime != null
        {
            if let Some(date_str) = event
                .get_str("o365audit.SharePointMetaData.LastModifiedTime")
                .map(String::from)
            {
                let date_str = date_str.as_str();
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%:z"))
                {
                    event.set("file.mtime", dt.to_rfc3339())?;
                }
            }
        }

        // TODO: conditional: ctx.event?.code != null && ctx.o365audit?.ExchangeMetaData!= null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code)
        {
            // Painless script
            // Source: def fields = new def[] {\"To\", \"CC\", \"BCC\"}; if (ctx.destination == null) {\n  ctx.destination = new HashMap();\n} if (ctx.destination.user == null) {\n  ctx.destination.user = new HashMap();\n} ctx.destination.user.email = new ArrayList(); for (int i = 0; i < fields.length; ++i) {\n  if (ctx.o365audit.ExchangeMetaData instanceof Map && ctx.o365audit.ExchangeMetaData.containsKey(fields[i])) {\n    def emails = ctx.o365audit.ExchangeMetaData[fields[i]];\n    if (emails instanceof List){\n      for (int e = 0; e < emails.length; ++e) {\n        ctx.destination.user.email.add(emails[e]);\n      }\n    }\n    if (emails instanceof String){\n      ctx.destination.user.email.add(emails);\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def fields = new def[] {\"To\", \"CC\", \"BCC\"}; if (ctx.destination == null) {\n  ctx.destination = new HashMap();\n} if (ctx.destination.user == null) {\n  ctx.destination.user = new HashMap();\n} ctx.destination.user.email = new ArrayList(); for (int i = 0; i < fields.length; ++i) {\n  if (ctx.o365audit.ExchangeMetaData instanceof Map && ctx.o365audit.ExchangeMetaData.containsKey(fields[i])) {\n    def emails = ctx.o365audit.ExchangeMetaData[fields[i]];\n    if (emails instanceof List){\n      for (int e = 0; e < emails.length; ++e) {\n        ctx.destination.user.email.add(emails[e]);\n      }\n    }\n    if (emails instanceof String){\n      ctx.destination.user.email.add(emails);\n    }\n  }\n}\n"#,
            )?;
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code) && ctx.o365audit?.ExceptionInfo != null && ctx.o365audit?.ExceptionInfo instanceof String
        {
            if event.has("o365audit.ExceptionInfo") {
                event.rename("o365audit.ExceptionInfo", "o365audit.ExceptionInfo.Reason")?;
            }
        }

        // TODO: conditional: ctx.event?.code != null && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"].contains(ctx.event?.code) && ctx.o365audit?.PolicyDetails != null
        {
            // Painless script
            // Source: int severityToCode(def x) { \n  if (x.toLowerCase() == \"informational\") {\n    return 1;\n  }\n  if (x.toLowerCase() == \"low\") {\n    return 2;\n  }\n  if (x.toLowerCase() == \"medium\") {\n    return 3;\n  }\n  if (x.toLowerCase() == \"high\") {\n    return 4;\n  }\n  return 0;\n} def policies = ctx.o365audit.PolicyDetails; if (policies == null) {\n  return;\n} if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n} if (ctx.rule.id == null) {\n  ctx.rule.id = new ArrayList();\n} if (ctx.rule.name == null) {\n  ctx.rule.name = new ArrayList();\n} def maxSeverity = 0; def allowed = true; for (int i = 0; i < policies.length && policies instanceof List; ++i) {\n  def rules = policies[i].Rules;\n  if (rules == null) {\n    continue;\n  }\n  for (int j = 0; j < rules.length; ++j) {\n    def rule = rules[j];\n    def id = rule.RuleId;\n    def name = rule.RuleName;\n    def sev = severityToCode(rule.Severity);\n    if (id != null && name != null) {\n      ctx.rule.id.add(id);\n      ctx.rule.name.add(name);\n    }\n    if (sev > maxSeverity) {\n      maxSeverity = sev;\n    }\n    if (allowed) {\n      if (rule.Actions != null && rule.Actions.contains(\"BlockAccess\")) {\n        allowed = false;\n      }\n    }\n  }\n} if (maxSeverity > -1) {\n  ctx.event.severity = maxSeverity;\n} if (allowed) {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpRuleUndo\") {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpInfo\") {\n  ctx.event.outcome = \"failure\";\n  return;\n} if (ctx.o365audit?.ExceptionInfo != null && !ctx.o365audit?.ExceptionInfo.isEmpty()) {\n  ctx.event.outcome = \"success\";\n  return;\n} ctx.event.outcome = \"failure\";\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"int severityToCode(def x) { \n  if (x.toLowerCase() == \"informational\") {\n    return 1;\n  }\n  if (x.toLowerCase() == \"low\") {\n    return 2;\n  }\n  if (x.toLowerCase() == \"medium\") {\n    return 3;\n  }\n  if (x.toLowerCase() == \"high\") {\n    return 4;\n  }\n  return 0;\n} def policies = ctx.o365audit.PolicyDetails; if (policies == null) {\n  return;\n} if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n} if (ctx.rule.id == null) {\n  ctx.rule.id = new ArrayList();\n} if (ctx.rule.name == null) {\n  ctx.rule.name = new ArrayList();\n} def maxSeverity = 0; def allowed = true; for (int i = 0; i < policies.length && policies instanceof List; ++i) {\n  def rules = policies[i].Rules;\n  if (rules == null) {\n    continue;\n  }\n  for (int j = 0; j < rules.length; ++j) {\n    def rule = rules[j];\n    def id = rule.RuleId;\n    def name = rule.RuleName;\n    def sev = severityToCode(rule.Severity);\n    if (id != null && name != null) {\n      ctx.rule.id.add(id);\n      ctx.rule.name.add(name);\n    }\n    if (sev > maxSeverity) {\n      maxSeverity = sev;\n    }\n    if (allowed) {\n      if (rule.Actions != null && rule.Actions.contains(\"BlockAccess\")) {\n        allowed = false;\n      }\n    }\n  }\n} if (maxSeverity > -1) {\n  ctx.event.severity = maxSeverity;\n} if (allowed) {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpRuleUndo\") {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpInfo\") {\n  ctx.event.outcome = \"failure\";\n  return;\n} if (ctx.o365audit?.ExceptionInfo != null && !ctx.o365audit?.ExceptionInfo.isEmpty()) {\n  ctx.event.outcome = \"success\";\n  return;\n} ctx.event.outcome = \"failure\";\n"#,
            )?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer"
        {
            if event.has("o365audit.ActorUserId") {
                event.rename("o365audit.ActorUserId", "user.email")?;
            }
        }

        // TODO: conditional: ctx.user?.id == null && ctx.event?.code == "Yammer"
        {
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

        // TODO: conditional: ctx.event?.code == "Yammer"
        {
            if event.has("o365audit.FileId") {
                event.rename("o365audit.FileId", "file.inode")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "Yammer"
        {
            if event.has("o365audit.FileName") {
                event.rename("o365audit.FileName", "file.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "Yammer"
        {
            if event.has("o365audit.GroupName") {
                event.rename("o365audit.GroupName", "group.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "Yammer"
        {
            if event.has("o365audit.TargetUserId") {
                event.rename("o365audit.TargetUserId", "destination.user.email")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "Yammer"
        {
            if event.has("o365audit.TargetYammerUserId") {
                event.rename("o365audit.TargetYammerUserId", "destination.user.id")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action != null && ["NetworkConfigurationUpdated", "NetworkSecurityConfigurationUpdated", "SoftDeleteSettingsUpdated", "ProcessProfileFields", "SupervisorAdminToggled"].contains(ctx.event?.action)
        {
            event.append("event.category", json!("configuration"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action != null && ["NetworkSecurityConfigurationUpdated", "GroupCreation", "GroupDeletion", "NetworkUserSuspended", "UserSuspension"].contains(ctx.event?.action)
        {
            event.append("event.category", json!("iam"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action != null && ["FileCreated", "FileDownloaded", "FileShared", "FileUpdateDescription", "FileUpdateName", "FileVisited"].contains(ctx.event?.action)
        {
            event.append("event.category", json!("file"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action != null && ["NetworkConfigurationUpdated", "NetworkSecurityConfigurationUpdated", "SoftDeleteSettingsUpdated", "ProcessProfileFields", "SupervisorAdminToggled"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("change"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action == "NetworkSecurityConfigurationUpdated"
        {
            event.append("event.type", json!("admin"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action != null && ["FileCreated", "GroupCreation", "FileUpdateName"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("creation"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action == "GroupDeletion"
        {
            event.append("event.type", json!("deletion"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action != null && ["FileDownloaded", "FileShared", "FileUpdateDescription", "FileVisited"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("access"))?;
        }

        // TODO: conditional: ctx.event?.code == "Yammer" && ctx.event?.action != null && ["GroupCreation", "GroupDeletion"].contains(ctx.event?.action)
        {
            event.append("event.type", json!("group"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "TeamCreated"
        {
            event.set("event.action", json!("added-group-account-to"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "added-group-account-to"
        {
            event.append("event.category", json!("iam"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "added-group-account-to"
        {
            event.append("event.type", json!("group"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "added-group-account-to"
        {
            event.append("event.type", json!("creation"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams"
        {
            if event.has("o365audit.TeamName") {
                event.rename("o365audit.TeamName", "group.name")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "MemberAdded"
        {
            event.set("event.action", json!("added-users-to-group"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "added-users-to-group"
        {
            event.append("event.category", json!("iam"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "added-users-to-group"
        {
            event.append("event.type", json!("group"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "added-users-to-group"
        {
            event.append("event.type", json!("change"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "Delete user."
        {
            event.set("event.action", json!("deleted-user-account"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "deleted-user-account"
        {
            event.append("event.category", json!("iam"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "deleted-user-account"
        {
            event.append("event.type", json!("user"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "deleted-user-account"
        {
            event.append("event.type", json!("deletion"))?;
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.event?.action == "deleted-user-account"
        {
            if event.has("o365audit.ObjectId") {
                event.rename("o365audit.ObjectId", "user.target.id")?;
            }
        }

        // TODO: conditional: ctx.event?.code == "MicrosoftTeams" && ctx.o365audit?.Members != null && ctx.o365audit.Members instanceof List
        {
            // Painless script
            // Source: def members = ctx.o365audit?.Members; if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} for (int i = 0; i < members.length; ++i) {\n  if (members[i] instanceof Map && members[i].containsKey(\"UPN\") && !members[i][\"UPN\"].isEmpty()) {\n    ctx.related.user.add(members[i][\"UPN\"]);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def members = ctx.o365audit?.Members; if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} for (int i = 0; i < members.length; ++i) {\n  if (members[i] instanceof Map && members[i].containsKey(\"UPN\") && !members[i][\"UPN\"].isEmpty()) {\n    ctx.related.user.add(members[i][\"UPN\"]);\n  }\n}\n"#,
            )?;
        }

        if event.has("client._temp") {
            if let Some(s) = event.get_str("client._temp").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)").unwrap();
                let replaced = re.replace_all(s, "$1").into_owned();
                event.set("client._temp", replaced)?;
            }
        }

        // TODO: conditional: ctx.client?._temp != null && !ctx.client?._temp.isEmpty()
        {
            // Pattern definitions for grok
            // IPANDPORT = ^%{IP:client.address}:%{POSINT:client._port}
            // IPANDPORTBRACKETS = ^\[%{IP:client.address}\]:%{POSINT:client._port}
            // HOSTNAMEANDPORT = ^%{NOTSPACE:client.domain}:%{POSINT:client._port}
            // NOTCLOSINGPARENS = [^)]*
            // HOSTNAMEANDIP = %{NOTSPACE:client.domain} \(%{NOTCLOSINGPARENS:client.address}\)
            // HOSTNAMEANDPORTBRACKETS = ^\[%{NOTSPACE:client.domain}\]:%{POSINT:client._port}
            if let Some(input) = event.get_str("client._temp").map(String::from) {
                let input = input.as_str();
                // Grok pattern: %{IPANDPORTBRACKETS}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("%{IPANDPORTBRACKETS}")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
                // Additional grok pattern 1: ^%{IP:client.address}$
                // Additional grok pattern 2: ^\\[%{IP:client.address}\\]$
                // Additional grok pattern 3: %{IPANDPORT}
                // Additional grok pattern 4: ^%{NOTSPACE:client.domain}$
                // Additional grok pattern 5: %{HOSTNAMEANDPORTBRACKETS}
                // Additional grok pattern 6: %{HOSTNAMEANDPORT}
                // Additional grok pattern 7: ^\\[%{HOSTNAMEANDIP}\\]$
                // Additional grok pattern 8: ^%{HOSTNAMEANDIP}$
                // Additional grok pattern 9: %{GREEDYDATA:client.address}
            }
        }

        if event.has("server._temp") {
            if let Some(s) = event.get_str("server._temp").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[\n\r]").unwrap();
                let replaced = re.replace_all(s, "").into_owned();
                event.set("server._temp", replaced)?;
            }
        }

        // TODO: conditional: ctx.server?._temp != null && !ctx.server?._temp.isEmpty()
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // HOSTNAMEANDIP = %{NOTSPACE:server.domain} \(%{NOTCLOSINGPARENS:server.address}\)
                // NOTCLOSINGPARENS = [^)]*
                if let Some(input) = event.get_str("server._temp").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^\\[%{HOSTNAMEANDIP}\\]$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re =
                        regex::Regex::new(&grok_to_regex("^\\[%{HOSTNAMEANDIP}\\]$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: %{HOSTNAMEANDIP}
                    // Additional grok pattern 2: %{GREEDYDATA:server.address}
                }
                Ok(())
            })();
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(s) = event.get_str("client.address").map(String::from) {
                let s = s.as_str();
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
            if let Some(s) = event.get_str("server.address").map(String::from) {
                let s = s.as_str();
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

        // TODO: conditional: ctx.client?.ip != null
        {
            event.set(
                "source.ip",
                event.get("client.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.client?.port != null
        {
            event.set(
                "source.port",
                event.get("client.port").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.server?.ip != null
        {
            event.set(
                "destination.ip",
                event.get("server.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.user?.id != null && ctx.user?.id.contains("@")
        {
            // Painless script
            // Source: String[] splitmail = ctx.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"String[] splitmail = ctx.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];\n"#,
            )?;
        }

        // TODO: conditional: ctx.user?.target?.id != null && ctx.user?.target?.id.contains("@")
        {
            // Painless script
            // Source: String[] splitmail = ctx.user.target.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.target.email = ctx.user.target.id; ctx.user.target.domain = splitmail[1]; ctx.user.target.name = splitmail[0];\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"String[] splitmail = ctx.user.target.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.target.email = ctx.user.target.id; ctx.user.target.domain = splitmail[1]; ctx.user.target.name = splitmail[0];\n"#,
            )?;
        }

        // TODO: conditional: ctx.source?.user?.id != null && ctx.source?.user?.id.contains("@")
        {
            // Painless script
            // Source: String[] splitmail = ctx.source.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.source.user.email = ctx.source.user.id; ctx.source.user.domain = splitmail[1]; ctx.source.user.name = splitmail[0];\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"String[] splitmail = ctx.source.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.source.user.email = ctx.source.user.id; ctx.source.user.domain = splitmail[1]; ctx.source.user.name = splitmail[0];\n"#,
            )?;
        }

        // TODO: conditional: ctx.destination?.user?.id != null && ctx.destination?.user?.id.contains("@")
        {
            // Painless script
            // Source: String[] splitmail = ctx.destination.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.destination.user.email = ctx.destination.user.id; ctx.destination.user.domain = splitmail[1]; ctx.destination.user.name = splitmail[0];\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"String[] splitmail = ctx.destination.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.destination.user.email = ctx.destination.user.id; ctx.destination.user.domain = splitmail[1]; ctx.destination.user.name = splitmail[0];\n"#,
            )?;
        }

        // TODO: conditional: ctx.client?.ip != null && ctx.client?.ip.contains(":")
        {
            event.set("network.type", json!("ipv6"))?;
        }

        // TODO: conditional: ctx.network?.type == null && ctx.client?.ip != null
        {
            event.set("network.type", json!("ipv4"))?;
        }

        // TODO: conditional: ctx.client?.ip != null
        {
            event.append(
                "related.ip",
                event.get("client.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.server?.ip != null
        {
            event.append(
                "related.ip",
                event.get("server.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.user?.name != null
        {
            event.append(
                "related.user",
                event.get("user.name").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.user?.target?.name != null
        {
            event.append(
                "related.user",
                event
                    .get("user.target.name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.file?.owner != null
        {
            event.append(
                "related.user",
                event.get("file.owner").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.o365audit?.Parameters?.User != null
        {
            event.append(
                "related.user",
                event
                    .get("o365audit.Parameters.User")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.o365audit?.ExtendedProperties?.UserAgent != null
        {
            if event.has("o365audit.ExtendedProperties.UserAgent") {
                event.rename(
                    "o365audit.ExtendedProperties.UserAgent",
                    "user_agent.original",
                )?;
            }
        }

        if event.has("organization.id") {
            if let Some(s) = event.get_str("organization.id").map(String::from) {
                let s = s.as_str();
                let lowered = s.to_lowercase();
                event.set("organization.id", lowered)?;
            }
        }

        // TODO: conditional: ctx.organization?.id != null
        {
            event.set(
                "host.id",
                event.get("organization.id").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.organization?.id != null && ctx._conf?.tenants != null
        {
            // Painless script
            // Source: def conftenants = ctx._conf.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n  ctx.host.name = conftenants[orgid];\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def conftenants = ctx._conf.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n  ctx.host.name = conftenants[orgid];\n}\n"#,
            )?;
        }

        // TODO: conditional: ctx.organization?.name != null && ctx.host?.name == null
        {
            event.set(
                "host.name",
                event
                    .get("organization.name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.user?.domain != null && ctx.host?.name == null
        {
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

        // TODO: conditional: ctx.o365audit?.Actor instanceof List
        {
            if let Some(Value::Array(items)) = event.get("o365audit.Actor").cloned() {
                {
                    for (idx, _item) in items.iter().enumerate() {
                        // Set _ingest._value for inner processor access
                        let item_path = format!("o365audit.Actor[{}]", idx);
                        // Inner processor operates on the element:
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
                    }
                }
            }
        }

        // TODO: conditional: ctx.o365audit?.Target instanceof List
        {
            if let Some(Value::Array(items)) = event.get("o365audit.Target").cloned() {
                {
                    for (idx, _item) in items.iter().enumerate() {
                        // Set _ingest._value for inner processor access
                        let item_path = format!("o365audit.Target[{}]", idx);
                        // Inner processor operates on the element:
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
                    }
                }
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

        // TODO: conditional: ctx.o365audit?.containsKey('Data') == true
        {
            if let Some(s) = event.get_str("o365audit.Data").map(String::from) {
                let s = s.as_str();
                let parsed: Value =
                    serde_json::from_str(s).map_err(|e| TransformError::ParseError {
                        path: "o365audit.Data".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("o365audit.Data", parsed)?;
            }
        }

        if event.has("o365audit.Data") {
            event.rename("o365audit.Data", "o365audit.Data.flattened")?;
        }

        // TODO: conditional: ctx.o365audit?.Data?.flattened instanceof Map
        {
            // Painless script
            // Source: def knownKeys = ['ad', 'af', 'aii', 'ail', 'alk', 'als', 'an', 'at',\n  'cid', 'cpid', 'dm', 'dpn', 'eid', 'etps', 'etype', 'f3u', 'fvs',\n  'imsgid', 'lon', 'mat', 'md', 'ms', 'od', 'op', 'ot', 'plk', 'pud',\n  'reid', 'rid', 'sev', 'sict', 'sid', 'sip', 'sitmi', 'srt', 'ssic',\n  'suid', 'tdc', 'te', 'thn', 'tht', 'tid', 'tpid', 'tpt', 'trc', 'ts',\n  'tsd', 'ttdt', 'ttr', 'upfc', 'upfv', 'ut', 'von', 'wl', 'zfh', 'zfn',\n  'zmfh', 'zmfn', 'zu'];\nfor (def key : knownKeys) {\n  if (ctx.o365audit.Data.flattened.containsKey(key)) {\n    ctx.o365audit.Data[key] = ctx.o365audit.Data.flattened[key];\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def knownKeys = ['ad', 'af', 'aii', 'ail', 'alk', 'als', 'an', 'at',\n  'cid', 'cpid', 'dm', 'dpn', 'eid', 'etps', 'etype', 'f3u', 'fvs',\n  'imsgid', 'lon', 'mat', 'md', 'ms', 'od', 'op', 'ot', 'plk', 'pud',\n  'reid', 'rid', 'sev', 'sict', 'sid', 'sip', 'sitmi', 'srt', 'ssic',\n  'suid', 'tdc', 'te', 'thn', 'tht', 'tid', 'tpid', 'tpt', 'trc', 'ts',\n  'tsd', 'ttdt', 'ttr', 'upfc', 'upfv', 'ut', 'von', 'wl', 'zfh', 'zfn',\n  'zmfh', 'zmfn', 'zu'];\nfor (def key : knownKeys) {\n  if (ctx.o365audit.Data.flattened.containsKey(key)) {\n    ctx.o365audit.Data[key] = ctx.o365audit.Data.flattened[key];\n  }\n}\n"#,
            )?;
        }

        if event.has("o365audit.Data.sip") {
            if let Some(s) = event.get_str("o365audit.Data.sip").map(String::from) {
                let s = s.as_str();
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

        // TODO: conditional: ctx.o365audit?.Data?.at != null
        {
            if let Some(date_str) = event.get_str("o365audit.Data.at").map(String::from) {
                let date_str = date_str.as_str();
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%:z"))
                {
                    event.set("o365audit.Data.at", dt.to_rfc3339())?;
                }
            }
        }

        // TODO: conditional: ctx.o365audit?.Data?.md != null
        {
            if let Some(date_str) = event.get_str("o365audit.Data.md").map(String::from) {
                let date_str = date_str.as_str();
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%:z"))
                {
                    event.set("o365audit.Data.md", dt.to_rfc3339())?;
                }
            }
        }

        // TODO: conditional: ctx.o365audit?.Data?.te != null
        {
            if let Some(date_str) = event.get_str("o365audit.Data.te").map(String::from) {
                let date_str = date_str.as_str();
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%:z"))
                {
                    event.set("o365audit.Data.te", dt.to_rfc3339())?;
                }
            }
        }

        // TODO: conditional: ctx.o365audit?.Data?.ts != null
        {
            if let Some(date_str) = event.get_str("o365audit.Data.ts").map(String::from) {
                let date_str = date_str.as_str();
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%:z"))
                {
                    event.set("o365audit.Data.ts", dt.to_rfc3339())?;
                }
            }
        }

        // TODO: conditional: ctx.o365audit?.Data?.ttdt != null
        {
            if let Some(date_str) = event.get_str("o365audit.Data.ttdt").map(String::from) {
                let date_str = date_str.as_str();
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%:z"))
                {
                    event.set("o365audit.Data.ttdt", dt.to_rfc3339())?;
                }
            }
        }

        // TODO: conditional: ctx.o365audit?.Data?.f3u?.splitOnToken('@')?.length == 2 && ctx.o365audit.Data.f3u.length() >= 3;
        {
            event.append(
                "related.user",
                event
                    .get("o365audit.Data.f3u")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.o365audit?.Data?.suid?.splitOnToken('@')?.length == 2 && ctx.o365audit.Data.suid.length() >= 3;
        {
            event.append(
                "related.user",
                event
                    .get("o365audit.Data.suid")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.o365audit?.Data?.tsd?.splitOnToken('@')?.length == 2 && ctx.o365audit.Data.tsd.length() >= 3;
        {
            event.append(
                "related.user",
                event
                    .get("o365audit.Data.tsd")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.o365audit?.Data?.trc?.splitOnToken('@')?.length == 2 && ctx.o365audit.Data.trc.length() >= 3;
        {
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
            if let Some(ua_str) = event.get_str("user_agent.original").map(String::from) {
                let ua_str = ua_str.as_str();
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
            if let Some(ip_str) = event.get_str("source.ip").map(String::from) {
                let ip_str = ip_str.as_str();
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
            if let Some(ip_str) = event.get_str("source.ip").map(String::from) {
                let ip_str = ip_str.as_str();
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

        // TODO: conditional: ctx?.tags == null || !(ctx.tags.contains('preserve_original_event'))
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.original");
                Ok(())
            })();
        }

        Ok(TransformResult::Continue)
    }
}
