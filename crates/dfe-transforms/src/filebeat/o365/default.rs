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

            let _cond = {
                !event.has_value("event.original")
                    && (event.has_value("tags")
                        && event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_original_event")),
                            serde_json::Value::String(s) => s.contains("preserve_original_event"),
                            _ => false,
                        }))
            };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nctx.event.original = Json.dump(ctx.o365audit)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx.event = ctx.event ?: [:];\nctx.event.original = Json.dump(ctx.o365audit)"#
                    ),
                )?;
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.category", json!("web"))?;

            let _cond = { event.has_value("o365audit") };
            if _cond {
                // Painless script
                // Source: for (def field : params.fields) {\n  def value = ctx.o365audit[field];\n  if (value instanceof Number) {\n    ctx.o365audit[field] = ((Number)value).longValue().toString();\n  } else if (value instanceof String && (value.indexOf('e') >= 0 || value.indexOf('E') >= 0)) {\n    ctx.o365audit[field] = new BigDecimal(value).toBigIntegerExact().toString();\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"for (def field : params.fields) {\n  def value = ctx.o365audit[field];\n  if (value instanceof Number) {\n    ctx.o365audit[field] = ((Number)value).longValue().toString();\n  } else if (value instanceof String && (value.indexOf('e') >= 0 || value.indexOf('E') >= 0)) {\n    ctx.o365audit[field] = new BigDecimal(value).toBigIntegerExact().toString();\n  }\n}"#
                    ),
                    cached_params!(
                        "{\"fields\":[\"RecordType\",\"ActorYammerUserId\",\"TargetYammerUserId\",\"YammerNetworkId\",\"Version\",\"InternalLogonType\",\"LogonType\",\"RunningTime\",\"FileSize\"]}"
                    ),
                )?;
            }

            let _cond = { event.get("o365audit.Sender").is_some_and(|v| v.is_object()) };
            if _cond {
                if event.has("o365audit.Sender") {
                    event.rename("o365audit.Sender", "o365audit.SenderEntity")?;
                }
            }

            let _cond = {
                event
                    .get("o365audit.Message")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                if event.has("o365audit.Message") {
                    event.rename("o365audit.Message", "o365audit.MessageObject")?;
                }
            }

            {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                if let Some(v) = event.get("o365audit.Id") {
                    hasher.update(v.to_string().as_bytes());
                }
                let hash = format!("{:x}", hasher.finalize());
                event.set("_id", json!(hash))?;
            }

            if let Some(v) = event
                .get("o365audit.FileExtension")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.extension", v)?;
            }

            if let Some(v) = event
                .get("o365audit.Sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("o365audit.Sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            if let Some(v) = event
                .get("o365audit.FilePath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("o365audit.TargetFilePath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("file.path") {
                    event.set("file.path", v)?;
                }
            }

            if let Some(v) = event
                .get("o365audit.FileSizeBytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if let Some(v) = event
                .get("o365audit.FileSize")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("file.size") {
                    event.set("file.size", v)?;
                }
            }

            if let Some(v) = event
                .get("o365audit.Application")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if let Some(v) = event
                .get("o365audit.OriginatingDomain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.domain", v)?;
            }

            let _cond = { event.has_value("o365audit.Actions") };
            if _cond {
                // Painless script
                // Source: ctx._tmp = [:];\ndef actions = [];\nctx._tmp.action_strings = [];\nif (!(ctx.o365audit.Actions instanceof List)) {\n  ctx.o365audit.Actions = [ctx.o365audit.Actions];\n}\n\n// Actions contains both a human readable `QueryTime` using AM/PM and an ISO8601 format `QueryTime`\n// We remove the AM/PM containing `QueryTime` to avoid duplicate field errors on flattening.\ndef queryTimePattern = /,\"QueryTime\":\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\"|\"QueryTime\":\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\",/;\nfor (def e: ctx.o365audit.Actions) {\n  if (e instanceof Map) {\n    actions.add(e);\n  } else if (e instanceof String) {\n    ctx._tmp.action_strings.add(queryTimePattern.matcher(e).replaceAll(''));\n  }\n}\nif (actions.length == ctx.o365audit.Actions.length) {\n  ctx._tmp.remove(\"action_strings\");\n  return\n}\nctx.o365audit.Actions = actions;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx._tmp = [:];\ndef actions = [];\nctx._tmp.action_strings = [];\nif (!(ctx.o365audit.Actions instanceof List)) {\n  ctx.o365audit.Actions = [ctx.o365audit.Actions];\n}\n\n// Actions contains both a human readable `QueryTime` using AM/PM and an ISO8601 format `QueryTime`\n// We remove the AM/PM containing `QueryTime` to avoid duplicate field errors on flattening.\ndef queryTimePattern = /,\"QueryTime\":\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\"|\"QueryTime\":\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\",/;\nfor (def e: ctx.o365audit.Actions) {\n  if (e instanceof Map) {\n    actions.add(e);\n  } else if (e instanceof String) {\n    ctx._tmp.action_strings.add(queryTimePattern.matcher(e).replaceAll(''));\n  }\n}\nif (actions.length == ctx.o365audit.Actions.length) {\n  ctx._tmp.remove(\"action_strings\");\n  return\n}\nctx.o365audit.Actions = actions;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_tmp.action_strings") };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.action_strings").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(s) = event.get_string("_ingest._value") {
                                let parsed: Value = serde_json::from_str(&s).map_err(|e| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message: format!("failed to parse JSON: {}", e),
                                    }
                                })?;
                                event.set("_ingest._value", parsed)?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "json")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.action_strings", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("_tmp.action_strings") };
            if _cond {
                // Painless script
                // Source: // To reach here, ctx._tmp.action_strings must be non-null\n// and for this to be true, script_select_string_actions\n// must have run, requiring that ctx.o365audit.Actions is\n// non-null, so we do not need to check again.\nctx.o365audit.Actions.addAll(ctx._tmp.action_strings);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"// To reach here, ctx._tmp.action_strings must be non-null\n// and for this to be true, script_select_string_actions\n// must have run, requiring that ctx.o365audit.Actions is\n// non-null, so we do not need to check again.\nctx.o365audit.Actions.addAll(ctx._tmp.action_strings);"#
                    ),
                )?;
            }

            let _cond = { event.has_value("o365audit.CreationTime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.CreationTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            if event.has("o365audit.Id") {
                event.rename("o365audit.Id", "event.id")?;
            }

            if event.has("o365audit.ClientIPAddress") {
                event.rename("o365audit.ClientIPAddress", "client._temp")?;
            }

            let _cond = { !event.has_value("client._temp") };
            if _cond {
                if event.has("o365audit.ClientIP") {
                    event.rename("o365audit.ClientIP", "client._temp")?;
                }
            }

            let _cond = { !event.has_value("client._temp") };
            if _cond {
                if event.has("o365audit.ActorIpAddress") {
                    event.rename("o365audit.ActorIpAddress", "client._temp")?;
                }
            }

            if let Some(v) = event
                .get("o365audit.UserId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has("o365audit.Workload") {
                event.rename("o365audit.Workload", "event.provider")?;
            }

            if event.has("o365audit.Operation") {
                event.rename("o365audit.Operation", "event.action")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("o365audit.OrganizationId") {
                    event.rename("o365audit.OrganizationId", "organization.id")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "rename")?;
                event.set("_ingest.on_failure_processor_tag", "rename_organization_id")?;
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

            let _cond = {
                event
                    .get("o365audit.AdditionalInfo")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("o365audit.AdditionalInfo") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "o365audit.AdditionalInfo".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("o365audit.AdditionalInfo", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json-extract-stringly-AdditionalInfo",
                    )?;
                    if event.remove("o365audit.AdditionalInfo").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "o365audit.AdditionalInfo".into(),
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
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
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
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("o365audit.UserAgent") {
                event.rename("o365audit.UserAgent", "user_agent.original")?;
            }

            let _cond = { event.has_value("o365audit.RecordType") };
            if _cond {
                // Painless script
                // Source: def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n"#
                    ),
                    cached_params!(
                        "{\"1\":\"ExchangeAdmin\",\"10\":\"DataCenterSecurityCmdlet\",\"100\":\"OnPremisesSharePointScannerDlp\",\"101\":\"ExchangeSearch\",\"102\":\"SharePointSearch\",\"103\":\"PrivacyInsights\",\"105\":\"MyAnalyticsSettings\",\"106\":\"SecurityComplianceUserChange\",\"107\":\"ComplianceDLPExchangeClassification\",\"109\":\"MipExactDataMatch\",\"11\":\"ComplianceDLPSharePoint\",\"113\":\"MS365DCustomDetection\",\"12\":\"Sway\",\"13\":\"ComplianceDLPExchange\",\"14\":\"SharePointSharingOperation\",\"147\":\"CoreReportingSettings\",\"148\":\"ComplianceConnector\",\"15\":\"AzureActiveDirectoryStsLogon\",\"16\":\"SkypeForBusinessPSTNUsage\",\"17\":\"SkypeForBusinessUsersBlocked\",\"174\":\"DataShareOperation\",\"18\":\"SecurityComplianceCenterEOPCmdlet\",\"181\":\"EduDataLakeDownloadOperation\",\"19\":\"ExchangeAggregatedOperation\",\"2\":\"ExchangeItem\",\"20\":\"PowerBIAudit\",\"21\":\"CRM\",\"22\":\"Yammer\",\"23\":\"SkypeForBusinessCmdlets\",\"24\":\"Discovery\",\"25\":\"MicrosoftTeams\",\"28\":\"ThreatIntelligence\",\"29\":\"MailSubmission\",\"3\":\"ExchangeItemGroup\",\"30\":\"MicrosoftFlow\",\"31\":\"AeD\",\"32\":\"MicrosoftStream\",\"33\":\"ComplianceDLPSharePointClassification\",\"34\":\"ThreatFinder\",\"35\":\"Project\",\"36\":\"SharePointListOperation\",\"37\":\"SharePointCommentOperation\",\"38\":\"DataGovernance\",\"39\":\"Kaizala\",\"4\":\"SharePoint\",\"40\":\"SecurityComplianceAlerts\",\"41\":\"ThreatIntelligenceUrl\",\"42\":\"SecurityComplianceInsights\",\"43\":\"MIPLabel\",\"44\":\"WorkplaceAnalytics\",\"45\":\"PowerAppsApp\",\"46\":\"PowerAppsPlan\",\"47\":\"ThreatIntelligenceAtpContent\",\"48\":\"LabelContentExplorer\",\"49\":\"TeamsHealthcare\",\"50\":\"ExchangeItemAggregated\",\"51\":\"HygieneEvent\",\"52\":\"DataInsightsRestApiAudit\",\"53\":\"InformationBarrierPolicyApplication\",\"54\":\"SharePointListItemOperation\",\"55\":\"SharePointContentTypeOperation\",\"56\":\"SharePointFieldOperation\",\"57\":\"MicrosoftTeamsAdmin\",\"58\":\"HRSignal\",\"59\":\"MicrosoftTeamsDevice\",\"6\":\"SharePointFileOperation\",\"60\":\"MicrosoftTeamsAnalytics\",\"61\":\"InformationWorkerProtection\",\"62\":\"Campaign\",\"63\":\"DLPEndpoint\",\"64\":\"AirInvestigation\",\"65\":\"Quarantine\",\"66\":\"MicrosoftForms\",\"67\":\"ApplicationAudit\",\"68\":\"ComplianceSupervisionExchange\",\"69\":\"CustomerKeyServiceEncryption\",\"7\":\"OneDrive\",\"70\":\"OfficeNative\",\"71\":\"MipAutoLabelSharePointItem\",\"72\":\"MipAutoLabelSharePointPolicyLocation\",\"73\":\"MicrosoftTeamsShifts\",\"75\":\"MipAutoLabelExchangeItem\",\"76\":\"CortanaBriefing\",\"78\":\"WDATPAlerts\",\"8\":\"AzureActiveDirectory\",\"82\":\"SensitivityLabelPolicyMatch\",\"83\":\"SensitivityLabelAction\",\"84\":\"SensitivityLabeledFileAction\",\"85\":\"AttackSim\",\"86\":\"AirManualInvestigation\",\"87\":\"SecurityComplianceRBAC\",\"88\":\"UserTraining\",\"89\":\"AirAdminActionInvestigation\",\"9\":\"AzureActiveDirectoryAccountLogon\",\"90\":\"MSTIC\",\"91\":\"PhysicalBadgingSignal\",\"93\":\"AipDiscover\",\"94\":\"AipSensitivityLabelAction\",\"95\":\"AipProtectionAction\",\"96\":\"AipFileDeleted\",\"97\":\"AipHeartBeat\",\"98\":\"MCASAlerts\",\"99\":\"OnPremisesFileShareScannerDlp\"}"
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.ResultStatus")
                    && event.get_str("o365audit.ResultStatus").is_some_and(|s| {
                        ["succeeded", "success", "partiallysucceeded", "true"]
                            .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("o365audit.ResultStatus")
                    && event
                        .get_str("o365audit.ResultStatus")
                        .is_some_and(|s| ["failed", "false"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("o365audit.Parameters")
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
                    cached_script!(
                        r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.Parameters; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.Parameters = newparams;\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Parameters")
                    && event
                        .get("o365audit.Parameters")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("o365audit.Parameters", "o365audit.Parameters._raw")?;
            }

            let _cond = {
                !event.has_value("o365audit.NetworkMessageId")
                    || event
                        .get_str("o365audit.NetworkMessageId")
                        .is_none_or(|s| s.is_empty())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("o365audit.Parameters._raw") {
                        if let Some(input) = event.get_string("o365audit.Parameters._raw") {
                            // Grok pattern: ^-?Identity\\s\"?%{DATA:o365audit.NetworkMessageId}\"?$
                            if !cached_grok!(
                                "^-?Identity\\s\"?%{DATA:o365audit.NetworkMessageId}\"?$"
                            )
                            .extract_into(&input, event)?
                            {}
                        }
                    }
                    Ok(())
                })();
            }

            // Painless script
            // Source: void splitTrimAdd(Set acc, String str) {\n    if (str != null && str != '') {\n        String[] parts = str.splitOnToken(';');\n        for (int i = 0; i < parts.length; i++) {\n            acc.add(parts[i].trim());\n        }\n    }\n}\ndef addressSet = new HashSet(ctx.email?.to?.address ?: []);\nsplitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardAsAttachmentTo); splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardTo); splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.RedirectTo);\nif (!addressSet.isEmpty()) {\n  ctx.email = ctx.email ?: [:];\n  ctx.email.to = ctx.email.to ?: [:];\n  ctx.email.to.address = addressSet.asList();\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"void splitTrimAdd(Set acc, String str) {\n    if (str != null && str != '') {\n        String[] parts = str.splitOnToken(';');\n        for (int i = 0; i < parts.length; i++) {\n            acc.add(parts[i].trim());\n        }\n    }\n}\ndef addressSet = new HashSet(ctx.email?.to?.address ?: []);\nsplitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardAsAttachmentTo); splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardTo); splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.RedirectTo);\nif (!addressSet.isEmpty()) {\n  ctx.email = ctx.email ?: [:];\n  ctx.email.to = ctx.email.to ?: [:];\n  ctx.email.to.address = addressSet.asList();\n}\n"#
                ),
            )?;

            if let Some(v) = event
                .get("o365audit.Parameters.From")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.from.address", v)?;
            }

            let _cond = { event.has_value("o365audit.Platform") };
            if _cond {
                // Painless script
                // Source: def value = ctx.o365audit.Platform.toString(); def name = params[value]; if (name != null) {\n  ctx.o365audit.Platform = name;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"def value = ctx.o365audit.Platform.toString(); def name = params[value]; if (name != null) {\n  ctx.o365audit.Platform = name;\n}\n"#
                    ),
                    cached_params!(
                        "{\"0\":\"Unknown\",\"1\":\"Windows\",\"2\":\"MacOS\",\"3\":\"iOS\",\"4\":\"Android\",\"5\":\"Web Browser\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("o365audit.Platform") };
            if _cond {
                // Painless script
                // Source: ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString lcPlatform = ctx.o365audit.Platform.toLowerCase();\nif (lcPlatform.contains('windows')) {\n    ctx.host.os.type = 'windows';\n} else if (lcPlatform.contains('linux')) {\n    ctx.host.os.type = 'linux';\n} else if (lcPlatform.contains('mac')) {\n    ctx.host.os.type = 'macos';\n} else if (lcPlatform.contains('unix')) {\n    ctx.host.os.type = 'unix';\n} else if (lcPlatform.contains('ios')) {\n    ctx.host.os.type = 'ios';\n} else if (lcPlatform.contains('android')) {\n    ctx.host.os.type = 'android';\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString lcPlatform = ctx.o365audit.Platform.toLowerCase();\nif (lcPlatform.contains('windows')) {\n    ctx.host.os.type = 'windows';\n} else if (lcPlatform.contains('linux')) {\n    ctx.host.os.type = 'linux';\n} else if (lcPlatform.contains('mac')) {\n    ctx.host.os.type = 'macos';\n} else if (lcPlatform.contains('unix')) {\n    ctx.host.os.type = 'unix';\n} else if (lcPlatform.contains('ios')) {\n    ctx.host.os.type = 'ios';\n} else if (lcPlatform.contains('android')) {\n    ctx.host.os.type = 'android';\n}\n"#
                    ),
                )?;
            }

            if event.has("host.os.type") {
                if let Some(s) = event.get_string("host.os.type") {
                    let lowered = s.to_lowercase();
                    event.set("host.os.type", lowered)?;
                }
            }

            if let Some(v) = event
                .get("o365audit.Platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            let _cond = {
                event.has_value("o365audit.ExtendedProperties")
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
                    cached_script!(
                        r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.ExtendedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i][\"Value\"] != null) {\n    newparams[oldparams[i][\"Name\"]] = oldparams[i][\"Value\"];\n  }\n} ctx.o365audit.ExtendedProperties = newparams;\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.ExtendedProperties")
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
                event.has_value("o365audit.ModifiedProperties")
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
                    cached_script!(
                        r#"def newparams = new HashMap();  def oldparams = ctx.o365audit.ModifiedProperties; for (int i = 0; i < oldparams.length; ++i) {\n  if (oldparams[i] instanceof Map && oldparams[i][\"OldValue\"] != null && oldparams[i][\"NewValue\"] != null) {\n    def validname = oldparams[i][\"Name\"].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n    newparams[validname][\"NewValue\"] = oldparams[i][\"NewValue\"];\n    newparams[validname][\"OldValue\"] = oldparams[i][\"OldValue\"];\n  }\n  if (oldparams[i] instanceof String) {\n    def validname = oldparams[i].replace(\" \",\"_\").replace(\".\",\"_\");\n    newparams[validname] = new HashMap();\n  }\n} if (newparams.isEmpty()) {\n  ctx.o365audit.remove(\"ModifiedProperties\");\n  return;\n} ctx.o365audit.ModifiedProperties = newparams;\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.ModifiedProperties")
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
                event.has_value("o365audit.AlertLinks")
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
                    cached_script!(
                        r#"def list = ctx.o365audit.AlertLinks; def links = new ArrayList(); for (int i = 0; i < list.length; ++i) {\n  if (list[i] instanceof Map && list[i].containsKey(\"AlertLinkHref\") && list[i][\"AlertLinkHref\"] != null && list[i][\"AlertLinkHref\"] instanceof String) {\n    links.add(list[i][\"AlertLinkHref\"]);\n  }\n} if (links.length == 0) {\n  ctx.o365audit.remove(\"AlertLinks\");\n  return;\n} ctx.o365audit.AlertLinks = links;\n"#
                    ),
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has("o365audit.OrganizationName") {
                        event.rename("o365audit.OrganizationName", "organization.name")?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "rename")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "rename_organization_name",
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

            let _cond = { event.get_str("event.code") == Some("ExchangeAdmin") };
            if _cond {
                if event.has("o365audit.OriginatingServer") {
                    event.rename("o365audit.OriginatingServer", "server._temp")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                event.append("event.category", json!("email"))?;
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has("o365audit.MailboxOwnerUPN") {
                    event.rename("o365audit.MailboxOwnerUPN", "user.email")?;
                }
            }

            let _cond = {
                !event.has_value("user.id")
                    && event.has_value("o365audit.LogonUserSid")
                    && event.get_str("event.code") == Some("ExchangeItem")
            };
            if _cond {
                if let Some(v) = event
                    .get("o365audit.LogonUserSid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has("o365audit.OrganizationName") {
                        event.rename("o365audit.OrganizationName", "organization.name")?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "rename")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "rename_organization_name_exchange_item",
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
                if let Some(v) = event.get("o365audit.ObjectId").cloned() {
                    event.set("user.target.id", v)?;
                }
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
                event.has_value("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.ObjectId") {
                    event.rename("o365audit.ObjectId", "url.original")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SourceRelativeUrl") {
                    event.rename("o365audit.SourceRelativeUrl", "file.directory")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SourceFileName") {
                    event.rename("o365audit.SourceFileName", "file.name")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SourceFileExtension") {
                    event.rename("o365audit.SourceFileExtension", "file.extension")?;
                }
            }

            let _cond = {
                event.has_value("event.action")
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
                event.has_value("event.action")
                    && ["FileAccessed", "FileDownloaded"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                event.has_value("event.action")
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
                event.has_value("event.action")
                    && ["FileDeleted", "FolderDeleted"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && ["FileUploaded", "FolderCopied", "FolderCreated"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if let Some(v) = event.get("o365audit.Name").cloned() {
                    event.set("message", v)?;
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
                    && event.has_value("o365audit.Category")
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
                    && event.has_value("o365audit.Category")
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
                !event.has_value("user.id")
                    && event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.get_str("rule.ruleset") == Some("User")
            };
            if _cond {
                if let Some(v) = event
                    .get("o365audit.AlertEntityId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.has_value("rule.ruleset")
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

            let _cond = { event.get_str("event.code") == Some("ComplianceDLPExchange") };
            if _cond {
                // Painless script
                // Source: def operation = ctx.event?.action ?: ''; def user = ctx.user?.id ?: ''; def subject = ctx.o365audit?.ExchangeMetaData?.Subject ?: ctx.email?.subject ?: '';\nif (operation.isEmpty() && user.isEmpty() && subject.isEmpty()) {\n  ctx.message = \"Office365 Alert\";\n} else {\n  ctx.message = \"Office365 Alert: \" + operation + \" detected in email sent by \" + user + \" with subject '\" + subject + \"'\";\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def operation = ctx.event?.action ?: ''; def user = ctx.user?.id ?: ''; def subject = ctx.o365audit?.ExchangeMetaData?.Subject ?: ctx.email?.subject ?: '';\nif (operation.isEmpty() && user.isEmpty() && subject.isEmpty()) {\n  ctx.message = \"Office365 Alert\";\n} else {\n  ctx.message = \"Office365 Alert: \" + operation + \" detected in email sent by \" + user + \" with subject '\" + subject + \"'\";\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                !event.has_value("user.id")
                    && event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.From") {
                    event.rename("o365audit.SharePointMetaData.From", "user.id")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.FileName") {
                    event.rename("o365audit.SharePointMetaData.FileName", "file.name")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.FilePathUrl") {
                    event.rename("o365audit.SharePointMetaData.FilePathUrl", "url.original")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.UniqueId") {
                    event.rename("o365audit.SharePointMetaData.UniqueId", "file.inode")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.UniqueID") {
                    event.rename("o365audit.SharePointMetaData.UniqueID", "file.inode")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.SharePointMetaData.FileOwner") {
                    event.rename("o365audit.SharePointMetaData.FileOwner", "file.owner")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.ExchangeMetaData.From") {
                    event.rename("o365audit.ExchangeMetaData.From", "source.user.email")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.PolicyId") {
                    event.rename("o365audit.PolicyId", "rule.id")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has("o365audit.PolicyName") {
                    event.rename("o365audit.PolicyName", "rule.name")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                    && event.has_value("o365audit.SharePointMetaData.LastModifiedTime")
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("o365audit.SharePointMetaData.LastModifiedTime")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("file.mtime", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && event.has_value("o365audit.ExchangeMetaData")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                // Painless script
                // Source: def fields = new def[] {\"To\", \"CC\", \"BCC\"}; if (ctx.destination == null) {\n  ctx.destination = new HashMap();\n} if (ctx.destination.user == null) {\n  ctx.destination.user = new HashMap();\n} ctx.destination.user.email = new ArrayList(); for (int i = 0; i < fields.length; ++i) {\n  if (ctx.o365audit.ExchangeMetaData instanceof Map && ctx.o365audit.ExchangeMetaData.containsKey(fields[i])) {\n    def emails = ctx.o365audit.ExchangeMetaData[fields[i]];\n    if (emails instanceof List){\n      for (int e = 0; e < emails.length; ++e) {\n        ctx.destination.user.email.add(emails[e]);\n      }\n    }\n    if (emails instanceof String){\n      ctx.destination.user.email.add(emails);\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def fields = new def[] {\"To\", \"CC\", \"BCC\"}; if (ctx.destination == null) {\n  ctx.destination = new HashMap();\n} if (ctx.destination.user == null) {\n  ctx.destination.user = new HashMap();\n} ctx.destination.user.email = new ArrayList(); for (int i = 0; i < fields.length; ++i) {\n  if (ctx.o365audit.ExchangeMetaData instanceof Map && ctx.o365audit.ExchangeMetaData.containsKey(fields[i])) {\n    def emails = ctx.o365audit.ExchangeMetaData[fields[i]];\n    if (emails instanceof List){\n      for (int e = 0; e < emails.length; ++e) {\n        ctx.destination.user.email.add(emails[e]);\n      }\n    }\n    if (emails instanceof String){\n      ctx.destination.user.email.add(emails);\n    }\n  }\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                    && event.has_value("o365audit.ExceptionInfo")
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
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                    && event.has_value("o365audit.PolicyDetails")
            };
            if _cond {
                // Painless script
                // Source: int severityToCode(def x) { \n  if (x.toLowerCase() == \"informational\") {\n    return 1;\n  }\n  if (x.toLowerCase() == \"low\") {\n    return 2;\n  }\n  if (x.toLowerCase() == \"medium\") {\n    return 3;\n  }\n  if (x.toLowerCase() == \"high\") {\n    return 4;\n  }\n  return 0;\n} def policies = ctx.o365audit.PolicyDetails; if (policies == null) {\n  return;\n} if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n} if (ctx.rule.id == null) {\n  ctx.rule.id = new ArrayList();\n} if (ctx.rule.name == null) {\n  ctx.rule.name = new ArrayList();\n} def maxSeverity = 0; def allowed = true; for (int i = 0; i < policies.length && policies instanceof List; ++i) {\n  def rules = policies[i].Rules;\n  if (rules == null) {\n    continue;\n  }\n  for (int j = 0; j < rules.length; ++j) {\n    def rule = rules[j];\n    def id = rule.RuleId;\n    def name = rule.RuleName;\n    def sev = severityToCode(rule.Severity);\n    if (id != null && name != null) {\n      ctx.rule.id.add(id);\n      ctx.rule.name.add(name);\n    }\n    if (sev > maxSeverity) {\n      maxSeverity = sev;\n    }\n    if (allowed) {\n      if (rule.Actions != null && rule.Actions.contains(\"BlockAccess\")) {\n        allowed = false;\n      }\n    }\n  }\n} if (maxSeverity > -1) {\n  ctx.event.severity = maxSeverity;\n} if (allowed) {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpRuleUndo\") {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpInfo\") {\n  ctx.event.outcome = \"failure\";\n  return;\n} if (ctx.o365audit?.ExceptionInfo != null && !ctx.o365audit?.ExceptionInfo.isEmpty()) {\n  ctx.event.outcome = \"success\";\n  return;\n} ctx.event.outcome = \"failure\";\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"int severityToCode(def x) { \n  if (x.toLowerCase() == \"informational\") {\n    return 1;\n  }\n  if (x.toLowerCase() == \"low\") {\n    return 2;\n  }\n  if (x.toLowerCase() == \"medium\") {\n    return 3;\n  }\n  if (x.toLowerCase() == \"high\") {\n    return 4;\n  }\n  return 0;\n} def policies = ctx.o365audit.PolicyDetails; if (policies == null) {\n  return;\n} if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n} if (ctx.rule.id == null) {\n  ctx.rule.id = new ArrayList();\n} if (ctx.rule.name == null) {\n  ctx.rule.name = new ArrayList();\n} def maxSeverity = 0; def allowed = true; for (int i = 0; i < policies.length && policies instanceof List; ++i) {\n  def rules = policies[i].Rules;\n  if (rules == null) {\n    continue;\n  }\n  for (int j = 0; j < rules.length; ++j) {\n    def rule = rules[j];\n    def id = rule.RuleId;\n    def name = rule.RuleName;\n    def sev = severityToCode(rule.Severity);\n    if (id != null && name != null) {\n      ctx.rule.id.add(id);\n      ctx.rule.name.add(name);\n    }\n    if (sev > maxSeverity) {\n      maxSeverity = sev;\n    }\n    if (allowed) {\n      if (rule.Actions != null && rule.Actions.contains(\"BlockAccess\")) {\n        allowed = false;\n      }\n    }\n  }\n} if (maxSeverity > -1) {\n  ctx.event.severity = maxSeverity;\n} if (allowed) {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpRuleUndo\") {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpInfo\") {\n  ctx.event.outcome = \"failure\";\n  return;\n} if (ctx.o365audit?.ExceptionInfo != null && !ctx.o365audit?.ExceptionInfo.isEmpty()) {\n  ctx.event.outcome = \"success\";\n  return;\n} ctx.event.outcome = \"failure\";\n"#
                    ),
                )?;
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has("o365audit.ActorUserId") {
                    event.rename("o365audit.ActorUserId", "user.email")?;
                }
            }

            let _cond =
                { !event.has_value("user.id") && event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if let Some(v) = event
                    .get("o365audit.ActorYammerUserId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
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
                    && event.has_value("event.action")
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
                    && event.has_value("event.action")
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
                    && event.has_value("event.action")
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
                    && event.has_value("event.action")
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
                    && event.has_value("event.action")
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
                    && event.has_value("event.action")
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
                    && event.has_value("event.action")
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
                    && event.has_value("o365audit.Members")
                    && event.get("o365audit.Members").is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def members = ctx.o365audit?.Members; if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} for (int i = 0; i < members.length; ++i) {\n  if (members[i] instanceof Map && members[i].containsKey(\"UPN\") && !members[i][\"UPN\"].isEmpty()) {\n    ctx.related.user.add(members[i][\"UPN\"]);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def members = ctx.o365audit?.Members; if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} for (int i = 0; i < members.length; ++i) {\n  if (members[i] instanceof Map && members[i].containsKey(\"UPN\") && !members[i][\"UPN\"].isEmpty()) {\n    ctx.related.user.add(members[i][\"UPN\"]);\n  }\n}\n"#
                    ),
                )?;
            }

            if event.has("client._temp") {
                if let Some(s) = event.get_string("client._temp") {
                    let re = cached_regex!(
                        "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](:[0-9]+)?)?$"
                    );
                    let replaced = re.replace_all(&s, "$1$2").into_owned();
                    event.set("client._temp", replaced)?;
                }
            }

            let _cond = {
                event.has_value("client._temp")
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
                event.has_value("server._temp")
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

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if let Some(v) = event.get("client.ip").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("client.port") };
            if _cond {
                if let Some(v) = event.get("client.port").cloned() {
                    event.set("source.port", v)?;
                }
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                if let Some(v) = event.get("server.ip").cloned() {
                    event.set("destination.ip", v)?;
                }
            }

            let _cond = {
                event.get("user.id").is_some_and(|v| v.is_string())
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
                    cached_script!(
                        r#"String[] splitmail = ctx.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get("user.target.id").is_some_and(|v| v.is_string())
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
                    cached_script!(
                        r#"String[] splitmail = ctx.user.target.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.user.target.email = ctx.user.target.id; ctx.user.target.domain = splitmail[1]; ctx.user.target.name = splitmail[0];\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get("source.user.id").is_some_and(|v| v.is_string())
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
                    cached_script!(
                        r#"String[] splitmail = ctx.source.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.source.user.email = ctx.source.user.id; ctx.source.user.domain = splitmail[1]; ctx.source.user.name = splitmail[0];\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("destination.user.id")
                    .is_some_and(|v| v.is_string())
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
                    cached_script!(
                        r#"String[] splitmail = ctx.destination.user.id.splitOnToken(\"@\"); if (splitmail.length != 2) {\n  return;\n} ctx.destination.user.email = ctx.destination.user.id; ctx.destination.user.domain = splitmail[1]; ctx.destination.user.name = splitmail[0];\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get("client.ip").is_some_and(|v| v.is_string())
                    && event.get("client.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { !event.has_value("network.type") && event.has_value("client.ip") };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("client.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("server.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("user.name").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("user.target.name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("file.owner") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("file.owner").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("o365audit.Parameters.User") };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("o365audit.Parameters.User")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("o365audit.ExtendedProperties.UserAgent") };
            if _cond {
                if event.has("o365audit.ExtendedProperties.UserAgent") {
                    event.rename(
                        "o365audit.ExtendedProperties.UserAgent",
                        "user_agent.original",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("organization.id") {
                    if let Some(s) = event.get_string("organization.id") {
                        let lowered = s.to_lowercase();
                        event.set("organization.id", lowered)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_object())
                    && event.has_value("organization.id")
            };
            if _cond {
                if let Some(v) = event.get("organization.id").cloned() {
                    event.set("host.id", v)?;
                }
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_object())
                    && event.has_value("organization.id")
                    && event.has_value("_conf.tenants")
            };
            if _cond {
                // Painless script
                // Source: def conftenants = ctx._conf.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def conftenants = ctx._conf.tenants; def orgid = ctx.organization.id; if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n  ctx.organization.name = conftenants[orgid];\n}\n"#
                    ),
                )?;
            }

            let _cond =
                { event.has_value("o365audit.DeviceName") && !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("o365audit.DeviceName").cloned() {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_object())
                    && event.has_value("organization.name")
                    && !event.has_value("host.name")
            };
            if _cond {
                if let Some(v) = event.get("organization.name").cloned() {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("user.domain") && !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("user.domain").cloned() {
                    event.set("host.name", v)?;
                }
            }

            let _cond = {
                !event.has_value("event.provider")
                    && event.has_value("o365audit.UserType")
                    && event
                        .get_str("o365audit.UserType")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                let v = event
                    .get("o365audit.UserType")
                    .cloned()
                    .unwrap_or(Value::Null);
                if !painless_is_empty_value(&v) {
                    event.set("event.provider", v)?;
                }
            }

            let _cond = {
                event.has_value("o365audit.InternetMessageId")
                    && event
                        .get_str("o365audit.InternetMessageId")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.message_id",
                    event
                        .get("o365audit.InternetMessageId")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Item.InternetMessageId")
                    && event
                        .get_str("o365audit.Item.InternetMessageId")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.message_id",
                    event
                        .get("o365audit.Item.InternetMessageId")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.NetworkMessageId")
                    && event
                        .get_str("o365audit.NetworkMessageId")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.local_id",
                    event
                        .get("o365audit.NetworkMessageId")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.P1Sender")
                    && event
                        .get_str("o365audit.P1Sender")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.sender.address",
                    event
                        .get("o365audit.P1Sender")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event
                        .get_str("source.user.email")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.sender.address",
                    event
                        .get("source.user.email")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event
                    .get("o365audit.Recipients")
                    .is_some_and(|v| v.is_array())
                    && event
                        .get_i64("o365audit.Recipients.length")
                        .is_some_and(|n| n > 0)
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("o365audit.Recipients").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.to.address",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("o365audit.Recipients", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("destination.user.email")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("destination.user.email").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.to.address",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("destination.user.email", Value::Array(out))?;
                }
            }

            let _cond = {
                event.has_value("o365audit.SenderIp")
                    && event
                        .get_str("o365audit.SenderIp")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "related.ip",
                    event
                        .get("o365audit.SenderIp")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.SenderIP")
                    && event
                        .get_str("o365audit.SenderIP")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "related.ip",
                    event
                        .get("o365audit.SenderIP")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Subject")
                    && event
                        .get_str("o365audit.Subject")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.subject",
                    event
                        .get("o365audit.Subject")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Item.Subject")
                    && event
                        .get_str("o365audit.Item.Subject")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.subject",
                    event
                        .get("o365audit.Item.Subject")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Item.Attachments")
                    && event
                        .get_str("o365audit.Item.Attachments")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.email = ctx.email ?: [:];\nctx.email.attachments = [];\n\ndef attachmentList = ctx.o365audit.Item.Attachments.splitOnToken(';');\n\nfor (def attachment : attachmentList) {\n  def att = attachment.trim();\n  if (att.isEmpty() || !att.endsWith(')')) continue;\n\n  // Find the size marker at the end: \" (<digits>b)\"\n  int lastSep = att.lastIndexOf(' (');\n  if (lastSep < 0) continue;\n\n  def maybeSize = att.substring(lastSep + 2, att.length() - 1);\n  if (!maybeSize.endsWith('b')) continue;\n\n  def sizeStr = maybeSize.substring(0, maybeSize.length() - 1);\n  long sizeVal;\n  try { sizeVal = Long.parseLong(sizeStr); } catch (Exception e) { continue; }\n\n  def filename = att.substring(0, lastSep).trim();\n  if (filename.isEmpty()) continue;\n\n  int dotIdx = filename.lastIndexOf('.');\n  if (dotIdx < 0) continue;\n\n  def attachmentObj = [:];\n  attachmentObj.file = [:];\n  attachmentObj.file.extension = filename.substring(dotIdx + 1);\n  attachmentObj.file.name = filename;\n  attachmentObj.file.size = sizeVal;\n  ctx.email.attachments.add(attachmentObj);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"ctx.email = ctx.email ?: [:];\nctx.email.attachments = [];\n\ndef attachmentList = ctx.o365audit.Item.Attachments.splitOnToken(';');\n\nfor (def attachment : attachmentList) {\n  def att = attachment.trim();\n  if (att.isEmpty() || !att.endsWith(')')) continue;\n\n  // Find the size marker at the end: \" (<digits>b)\"\n  int lastSep = att.lastIndexOf(' (');\n  if (lastSep < 0) continue;\n\n  def maybeSize = att.substring(lastSep + 2, att.length() - 1);\n  if (!maybeSize.endsWith('b')) continue;\n\n  def sizeStr = maybeSize.substring(0, maybeSize.length() - 1);\n  long sizeVal;\n  try { sizeVal = Long.parseLong(sizeStr); } catch (Exception e) { continue; }\n\n  def filename = att.substring(0, lastSep).trim();\n  if (filename.isEmpty()) continue;\n\n  int dotIdx = filename.lastIndexOf('.');\n  if (dotIdx < 0) continue;\n\n  def attachmentObj = [:];\n  attachmentObj.file = [:];\n  attachmentObj.file.extension = filename.substring(dotIdx + 1);\n  attachmentObj.file.name = filename;\n  attachmentObj.file.size = sizeVal;\n  ctx.email.attachments.add(attachmentObj);\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_item_attachments",
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
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("o365audit.EndTimeUtc") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.EndTimeUtc") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None)
                    {
                        event.set("o365audit.EndTimeUtc", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.LastUpdateTimeUtc") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.LastUpdateTimeUtc") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.LastUpdateTimeUtc", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.StartTimeUtc") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.StartTimeUtc") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.StartTimeUtc", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.StartTime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.StartTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.StartTime", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.FilteringDate") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.FilteringDate") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.FilteringDate", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.RescanResult.Timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.RescanResult.Timestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.RescanResult.Timestamp", parsed)?;
                    }
                }
            }

            // SKIPPED: condition not transpiled: ctx.o365audit?.containsKey('Data') == true && ctx.o365audit?.RecordType == '64'
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(s) = event.get_string("o365audit.Data") {
                    let re = cached_regex!(
                        ",\\\"QueryTime\\\":\\\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\\\"|\\\"QueryTime\\\":\\\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\\\","
                    );
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("o365audit.Data", replaced)?;
                }
            }

            // SKIPPED: condition not transpiled: ctx.o365audit?.containsKey('Data') == true
            #[allow(unreachable_code, unused_variables)]
            if false {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("o365audit.Data") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "o365audit.Data".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("o365audit.Data", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_o365audit_Data_73eae5fb",
                    )?;
                    event.remove("o365audit.Data");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
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
                // Source: for (def key : params.knownKeys) {\n  if (ctx.o365audit.Data.flattened.containsKey(key)) {\n    ctx.o365audit.Data[key] = ctx.o365audit.Data.flattened[key];\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"for (def key : params.knownKeys) {\n  if (ctx.o365audit.Data.flattened.containsKey(key)) {\n    ctx.o365audit.Data[key] = ctx.o365audit.Data.flattened[key];\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"knownKeys\":[\"ad\",\"af\",\"aii\",\"ail\",\"alk\",\"als\",\"an\",\"at\",\"cid\",\"cpid\",\"dm\",\"dpn\",\"eid\",\"etps\",\"etype\",\"f3u\",\"fvs\",\"imsgid\",\"lon\",\"mat\",\"md\",\"ms\",\"od\",\"op\",\"ot\",\"plk\",\"pud\",\"reid\",\"rid\",\"sev\",\"sict\",\"sid\",\"sip\",\"sitmi\",\"srt\",\"ssic\",\"suid\",\"tdc\",\"te\",\"thn\",\"tht\",\"tid\",\"tpid\",\"tpt\",\"trc\",\"ts\",\"tsd\",\"ttdt\",\"ttr\",\"upfc\",\"upfv\",\"ut\",\"von\",\"wl\",\"zfh\",\"zfn\",\"zmfh\",\"zmfn\",\"zu\"]}"
                    ),
                )?;
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.sip")
                    .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_o365audit_Data_sip_1df0ff2b",
                    )?;
                    event.remove("o365audit.Data.sip");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.at") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.Data.at", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.md") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.md") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.Data.md", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.te") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.te") {
                    if let Some(parsed) = parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd HH:mm:ss'Z'"],
                        None,
                        None,
                    ) {
                        event.set("o365audit.Data.te", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.ts") {
                    if let Some(parsed) = parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd HH:mm:ss'Z'"],
                        None,
                        None,
                    ) {
                        event.set("o365audit.Data.ts", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.ttdt") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.ttdt") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("o365audit.Data.ttdt", parsed)?;
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
                !event.has_value("user.email")
                    && event
                        .get_str("o365audit.Data.f3u")
                        .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                let v = event
                    .get("o365audit.Data.f3u")
                    .cloned()
                    .unwrap_or(Value::Null);
                if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
                }
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
                event.has_value("o365audit.Data.tsd")
                    && event
                        .get_str("o365audit.Data.tsd")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.sender.address",
                    event
                        .get("o365audit.Data.tsd")
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
                event.has_value("o365audit.Data.trc")
                    && event
                        .get_str("o365audit.Data.trc")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.to.address",
                    event
                        .get("o365audit.Data.trc")
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

            let _cond = {
                event.has_value("o365audit.Data.aii")
                    && event
                        .get_str("o365audit.Data.aii")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.local_id",
                    event
                        .get("o365audit.Data.aii")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Data.imsgid")
                    && event
                        .get_str("o365audit.Data.imsgid")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.message_id",
                    event
                        .get("o365audit.Data.imsgid")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Data.ms")
                    && event
                        .get_str("o365audit.Data.ms")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "email.subject",
                    event
                        .get("o365audit.Data.ms")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event
                    .get("o365audit.Data.flattened.Entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: ctx._tmp = ctx._tmp ?: [:]; ctx._tmp.entities = [:]; for (def entity: ctx.o365audit.Data.flattened.Entities) {\n  if (entity instanceof Map) {\n    for (def key : params.knownEntityKeys) {\n      if (! ctx._tmp.entities.containsKey(key)) {\n        ctx._tmp.entities[key] = [];\n      }\n      if (entity.containsKey(key)) {\n        ctx._tmp.entities[key].add(entity[key]);\n      }\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"ctx._tmp = ctx._tmp ?: [:]; ctx._tmp.entities = [:]; for (def entity: ctx.o365audit.Data.flattened.Entities) {\n  if (entity instanceof Map) {\n    for (def key : params.knownEntityKeys) {\n      if (! ctx._tmp.entities.containsKey(key)) {\n        ctx._tmp.entities[key] = [];\n      }\n      if (entity.containsKey(key)) {\n        ctx._tmp.entities[key].add(entity[key]);\n      }\n    }\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"knownEntityKeys\":[\"InternetMessageId\",\"NetworkMessageId\",\"OriginalDeliveryLocation\",\"P1Sender\",\"P2Sender\",\"PhishConfidenceLevel\",\"Recipient\",\"SenderIP\",\"Subject\",\"ThreatDetectionMethods\",\"Upn\"]}"
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("_tmp.entities.InternetMessageId")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("_tmp.entities.InternetMessageId").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.message_id",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.InternetMessageId", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("_tmp.entities.NetworkMessageId")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("_tmp.entities.NetworkMessageId").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.local_id",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.NetworkMessageId", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("_tmp.entities.P1Sender")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.entities.P1Sender").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.sender.address",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.P1Sender", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("_tmp.entities.P2Sender")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.entities.P2Sender").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.from.address",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.P2Sender", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("_tmp.entities.Recipient")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.entities.Recipient").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.to.address",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.Recipient", Value::Array(out))?;
                }
            }

            let _cond = {
                !event.has_value("user.email")
                    && event
                        .get("_tmp.entities.Recipient")
                        .is_some_and(|v| v.is_array())
                    && event
                        .get_i64("_tmp.entities.Recipient.length")
                        .is_some_and(|n| n > 0)
            };
            if _cond {
                if let Some(v) = event
                    .get("_tmp.entities.Recipient")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event
                    .get("_tmp.entities.SenderIP")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.entities.SenderIP").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "related.ip",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.SenderIP", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("_tmp.entities.Subject")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.entities.Subject").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "email.subject",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.Subject", Value::Array(out))?;
                }
            }

            let _cond = { event.get("_tmp.entities.Upn").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("_tmp.entities.Upn").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append(
                            "related.user",
                            event.get("_ingest._value").cloned().unwrap_or(Value::Null),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("_tmp.entities.Upn", Value::Array(out))?;
                }
            }

            if event.has("_tmp.entities.OriginalDeliveryLocation") {
                event.rename(
                    "_tmp.entities.OriginalDeliveryLocation",
                    "o365audit.OriginalDeliveryLocation",
                )?;
            }

            if event.has("_tmp.entities.PhishConfidenceLevel") {
                event.rename(
                    "_tmp.entities.PhishConfidenceLevel",
                    "o365audit.PhishConfidenceLevel",
                )?;
            }

            if let Some(v) = event
                .get("o365audit.AppAccessContext.DeviceId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            let _cond = {
                event
                    .get("o365audit.ExtendedProperties.additionalDetails")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) =
                        event.get_string("o365audit.ExtendedProperties.additionalDetails")
                    {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "o365audit.ExtendedProperties.additionalDetails".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("o365audit.ExtendedProperties.additionalDetails", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json-extract-stringly-ExtendedProperties-additionalDetails",
                    )?;
                    event.remove("o365audit.ExtendedProperties.additionalDetails");
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
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("o365audit.ExtendedProperties.additionalDetails.DeviceId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("device.id") {
                    event.set("device.id", v)?;
                }
            }

            let _cond = { !event.has_value("user_agent") };
            if _cond {
                if event.has("o365audit.ExtendedProperties.additionalDetails.User-Agent") {
                    if let Some(ua_str) = event
                        .get_string("o365audit.ExtendedProperties.additionalDetails.User-Agent")
                    {
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
            }

            if let Some(v) = event
                .get("o365audit.AppAccessContext.AADSessionId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("session.id", v)?;
            }

            if let Some(v) = event
                .get("o365audit.AppAccessContext.UniqueTokenId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("token.id", v)?;
            }

            if let Some(v) = event
                .get("o365audit.ApplicationDisplayName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("application.name", v)?;
            }

            let _cond = { event.get_str("o365audit.RecordType") == Some("50") };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = { event.get_str("o365audit.RecordType") == Some("50") };
            if _cond {
                event.append("event.category", json!("email"))?;
            }

            let _cond = {
                event.has_value("o365audit.Messages")
                    && event.get_str("o365audit.RecordType") == Some("50")
            };
            if _cond {
                event.rename("o365audit.Messages", "o365audit.ExchangeAggregatedMessages")?;
            }

            let _cond = {
                event.has_value("o365audit.Folders")
                    && event.get_str("o365audit.RecordType") == Some("50")
            };
            if _cond {
                event.rename("o365audit.Folders", "o365audit.ExchangeAggregatedFolders")?;
            }

            let _cond = {
                event
                    .get("_tmp.entities.ThreatDetectionMethods")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def methods = ctx._tmp.entities.ThreatDetectionMethods; def result = []; for (def method: methods){\n  if (method instanceof List) {\n    for (def m: method) {\n      result.add(m);\n    }\n  } else if (method instanceof String) {\n    result.add(method);\n  }\n} ctx.o365audit.ThreatDetectionMethods = result;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def methods = ctx._tmp.entities.ThreatDetectionMethods; def result = []; for (def method: methods){\n  if (method instanceof List) {\n    for (def m: method) {\n      result.add(m);\n    }\n  } else if (method instanceof String) {\n    result.add(method);\n  }\n} ctx.o365audit.ThreatDetectionMethods = result;\n"#
                    ),
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

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("user.id").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("user.target.id").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("user.target.email") };
            if _cond {
                event.append(
                    "related.user",
                    event
                        .get("user.target.email")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("user.email").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append(
                    "related.hosts",
                    event.get("host.name").cloned().unwrap_or(Value::Null),
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.host?.hostname != null && ctx.host.hostname != ctx.host?.name
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.append(
                    "related.hosts",
                    event.get("host.hostname").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event.get("user.domain").cloned().unwrap_or(Value::Null),
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.user?.target?.domain != null && ctx.user.target.domain != ctx.user?.domain
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.append(
                    "related.hosts",
                    event
                        .get("user.target.domain")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event.get("source.domain").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event
                        .get("destination.domain")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event.get("url.domain").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("server.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event.get("server.domain").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("client.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    event.get("client.domain").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append(
                    "related.hash",
                    event.get("file.hash.md5").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append(
                    "related.hash",
                    event.get("file.hash.sha1").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append(
                    "related.hash",
                    event
                        .get("file.hash.sha256")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha512") };
            if _cond {
                event.append(
                    "related.hash",
                    event
                        .get("file.hash.sha512")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            event.remove("_conf");
            event.remove("_tmp");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}with tag '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
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
        Ok(TransformResult::Continue)
    }
}
