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
                if event.has_value("message") {
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"for (def field : params.fields) {\n  def value = ctx.o365audit[field];\n  if (value instanceof Number) {\n    ctx.o365audit[field] = ((Number)value).longValue().toString();\n  } else if (value instanceof String && (value.indexOf('e') >= 0 || value.indexOf('E') >= 0)) {\n    ctx.o365audit[field] = new BigDecimal(value).toBigIntegerExact().toString();\n  }\n}"#
                    ),
                    cached_params!(
                        "{\"fields\":[\"RecordType\",\"ActorYammerUserId\",\"TargetYammerUserId\",\"YammerNetworkId\",\"Version\",\"InternalLogonType\",\"LogonType\",\"RunningTime\",\"FileSize\"]}"
                    ),
                )?;
            }

            let _cond = { event.get("o365audit.Sender").is_some_and(|v| v.is_object()) };
            if _cond {
                if event.has_value("o365audit.Sender") {
                    event.rename("o365audit.Sender", "o365audit.SenderEntity")?;
                }
            }

            let _cond = {
                event
                    .get("o365audit.Message")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                if event.has_value("o365audit.Message") {
                    event.rename("o365audit.Message", "o365audit.MessageObject")?;
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("o365audit.Id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
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
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx._tmp = [:];\ndef actions = [];\nctx._tmp.action_strings = [];\nif (!(ctx.o365audit.Actions instanceof List)) {\n  ctx.o365audit.Actions = [ctx.o365audit.Actions];\n}\n\n// Actions contains both a human readable `QueryTime` using AM/PM and an ISO8601 format `QueryTime`\n// We remove the AM/PM containing `QueryTime` to avoid duplicate field errors on flattening.\ndef queryTimePattern = /,\"QueryTime\":\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\"|\"QueryTime\":\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\",/;\nfor (def e: ctx.o365audit.Actions) {\n  if (e instanceof Map) {\n    actions.add(e);\n  } else if (e instanceof String) {\n    ctx._tmp.action_strings.add(queryTimePattern.matcher(e).replaceAll(''));\n  }\n}\nif (actions.length == ctx.o365audit.Actions.length) {\n  ctx._tmp.remove(\"action_strings\");\n  return\n}\nctx.o365audit.Actions = actions;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_tmp.action_strings") };
            if _cond {
                foreach_array(event, "_tmp.action_strings", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(event, "_ingest._value", "_ingest._value")?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("_tmp.action_strings") };
            if _cond {
                // Painless script
                // Source: // To reach here, ctx._tmp.action_strings must be non-null\n// and for this to be true, script_select_string_actions\n// must have run, requiring that ctx.o365audit.Actions is\n// non-null, so we do not need to check again.\nctx.o365audit.Actions.addAll(ctx._tmp.action_strings);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// To reach here, ctx._tmp.action_strings must be non-null\n// and for this to be true, script_select_string_actions\n// must have run, requiring that ctx.o365audit.Actions is\n// non-null, so we do not need to check again.\nctx.o365audit.Actions.addAll(ctx._tmp.action_strings);"#
                    ),
                )?;
            }

            let _cond = { event.has_value("o365audit.CreationTime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.CreationTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.CreationTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("o365audit.Id") {
                event.rename("o365audit.Id", "event.id")?;
            }

            if event.has_value("o365audit.ClientIPAddress") {
                event.rename("o365audit.ClientIPAddress", "client._temp")?;
            }

            let _cond = { !event.has_value("client._temp") };
            if _cond {
                if event.has_value("o365audit.ClientIP") {
                    event.rename("o365audit.ClientIP", "client._temp")?;
                }
            }

            let _cond = { !event.has_value("client._temp") };
            if _cond {
                if event.has_value("o365audit.ActorIpAddress") {
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

            if event.has_value("o365audit.Workload") {
                event.rename("o365audit.Workload", "event.provider")?;
            }

            if event.has_value("o365audit.Operation") {
                event.rename("o365audit.Operation", "event.action")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("o365audit.OrganizationId") {
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
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
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
                    parse_json_field(
                        event,
                        "o365audit.AdditionalInfo",
                        "o365audit.AdditionalInfo",
                    )?;
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
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                    parse_json_field(
                        event,
                        "o365audit.OperationProperties",
                        "o365audit.OperationProperties",
                    )?;
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
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            if event.has_value("o365audit.UserAgent") {
                event.rename("o365audit.UserAgent", "user_agent.original")?;
            }

            let _cond = { event.has_value("o365audit.RecordType") };
            if _cond {
                // Painless script
                // Source: def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def schemaId = ctx.o365audit.RecordType.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.code = schema;\n}\n"#
                    ),
                    cached_params!(
                        "{\"1\":\"ExchangeAdmin\",\"2\":\"ExchangeItem\",\"3\":\"ExchangeItemGroup\",\"4\":\"SharePoint\",\"6\":\"SharePointFileOperation\",\"7\":\"OneDrive\",\"8\":\"AzureActiveDirectory\",\"9\":\"AzureActiveDirectoryAccountLogon\",\"10\":\"DataCenterSecurityCmdlet\",\"11\":\"ComplianceDLPSharePoint\",\"12\":\"Sway\",\"13\":\"ComplianceDLPExchange\",\"14\":\"SharePointSharingOperation\",\"15\":\"AzureActiveDirectoryStsLogon\",\"16\":\"SkypeForBusinessPSTNUsage\",\"17\":\"SkypeForBusinessUsersBlocked\",\"18\":\"SecurityComplianceCenterEOPCmdlet\",\"19\":\"ExchangeAggregatedOperation\",\"20\":\"PowerBIAudit\",\"21\":\"CRM\",\"22\":\"Yammer\",\"23\":\"SkypeForBusinessCmdlets\",\"24\":\"Discovery\",\"25\":\"MicrosoftTeams\",\"28\":\"ThreatIntelligence\",\"29\":\"MailSubmission\",\"30\":\"MicrosoftFlow\",\"31\":\"AeD\",\"32\":\"MicrosoftStream\",\"33\":\"ComplianceDLPSharePointClassification\",\"34\":\"ThreatFinder\",\"35\":\"Project\",\"36\":\"SharePointListOperation\",\"37\":\"SharePointCommentOperation\",\"38\":\"DataGovernance\",\"39\":\"Kaizala\",\"40\":\"SecurityComplianceAlerts\",\"41\":\"ThreatIntelligenceUrl\",\"42\":\"SecurityComplianceInsights\",\"43\":\"MIPLabel\",\"44\":\"WorkplaceAnalytics\",\"45\":\"PowerAppsApp\",\"46\":\"PowerAppsPlan\",\"47\":\"ThreatIntelligenceAtpContent\",\"48\":\"LabelContentExplorer\",\"49\":\"TeamsHealthcare\",\"50\":\"ExchangeItemAggregated\",\"51\":\"HygieneEvent\",\"52\":\"DataInsightsRestApiAudit\",\"53\":\"InformationBarrierPolicyApplication\",\"54\":\"SharePointListItemOperation\",\"55\":\"SharePointContentTypeOperation\",\"56\":\"SharePointFieldOperation\",\"57\":\"MicrosoftTeamsAdmin\",\"58\":\"HRSignal\",\"59\":\"MicrosoftTeamsDevice\",\"60\":\"MicrosoftTeamsAnalytics\",\"61\":\"InformationWorkerProtection\",\"62\":\"Campaign\",\"63\":\"DLPEndpoint\",\"64\":\"AirInvestigation\",\"65\":\"Quarantine\",\"66\":\"MicrosoftForms\",\"67\":\"ApplicationAudit\",\"68\":\"ComplianceSupervisionExchange\",\"69\":\"CustomerKeyServiceEncryption\",\"70\":\"OfficeNative\",\"71\":\"MipAutoLabelSharePointItem\",\"72\":\"MipAutoLabelSharePointPolicyLocation\",\"73\":\"MicrosoftTeamsShifts\",\"75\":\"MipAutoLabelExchangeItem\",\"76\":\"CortanaBriefing\",\"78\":\"WDATPAlerts\",\"82\":\"SensitivityLabelPolicyMatch\",\"83\":\"SensitivityLabelAction\",\"84\":\"SensitivityLabeledFileAction\",\"85\":\"AttackSim\",\"86\":\"AirManualInvestigation\",\"87\":\"SecurityComplianceRBAC\",\"88\":\"UserTraining\",\"89\":\"AirAdminActionInvestigation\",\"90\":\"MSTIC\",\"91\":\"PhysicalBadgingSignal\",\"93\":\"AipDiscover\",\"94\":\"AipSensitivityLabelAction\",\"95\":\"AipProtectionAction\",\"96\":\"AipFileDeleted\",\"97\":\"AipHeartBeat\",\"98\":\"MCASAlerts\",\"99\":\"OnPremisesFileShareScannerDlp\",\"100\":\"OnPremisesSharePointScannerDlp\",\"101\":\"ExchangeSearch\",\"102\":\"SharePointSearch\",\"103\":\"PrivacyInsights\",\"105\":\"MyAnalyticsSettings\",\"106\":\"SecurityComplianceUserChange\",\"107\":\"ComplianceDLPExchangeClassification\",\"109\":\"MipExactDataMatch\",\"113\":\"MS365DCustomDetection\",\"147\":\"CoreReportingSettings\",\"148\":\"ComplianceConnector\",\"174\":\"DataShareOperation\",\"181\":\"EduDataLakeDownloadOperation\"}"
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                    || event.get_str("o365audit.NetworkMessageId") == Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("o365audit.Parameters._raw") {
                        if let Some(input) = event.get_string("o365audit.Parameters._raw") {
                            // Grok pattern: ^-?Identity\\s\"?%{DATA:o365audit.NetworkMessageId}\"?$
                            let _ = cached_grok!(
                                "^-?Identity\\s\"?%{DATA:o365audit.NetworkMessageId}\"?$"
                            )
                            .extract_into(&input, event)?;
                        }
                    }
                    Ok(())
                })();
            }

            // Painless script
            // Source: void splitTrimAdd(Set acc, String str) {\n    if (str != null && str != '') {\n        String[] parts = str.splitOnToken(';');\n        for (int i = 0; i < parts.length; i++) {\n            acc.add(parts[i].trim());\n        }\n    }\n}\ndef addressSet = new HashSet(ctx.email?.to?.address ?: []);\nsplitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardAsAttachmentTo); splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardTo); splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.RedirectTo);\nif (!addressSet.isEmpty()) {\n  ctx.email = ctx.email ?: [:];\n  ctx.email.to = ctx.email.to ?: [:];\n  ctx.email.to.address = addressSet.asList();\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
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
                painless_exec_plan_params(
                    event,
                    cached_painless!(
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
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nString lcPlatform = ctx.o365audit.Platform.toLowerCase();\nif (lcPlatform.contains('windows')) {\n    ctx.host.os.type = 'windows';\n} else if (lcPlatform.contains('linux')) {\n    ctx.host.os.type = 'linux';\n} else if (lcPlatform.contains('mac')) {\n    ctx.host.os.type = 'macos';\n} else if (lcPlatform.contains('unix')) {\n    ctx.host.os.type = 'unix';\n} else if (lcPlatform.contains('ios')) {\n    ctx.host.os.type = 'ios';\n} else if (lcPlatform.contains('android')) {\n    ctx.host.os.type = 'android';\n}\n"#
                    ),
                )?;
            }

            if event.has_value("host.os.type") {
                map_strings(event, "host.os.type", "host.os.type", str::to_lowercase)?;
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                    if event.has_value("o365audit.OrganizationName") {
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
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                if event.has_value("o365audit.OriginatingServer") {
                    event.rename("o365audit.OriginatingServer", "server._temp")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                event.append("event.category", json!("email"))?;
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has_value("o365audit.MailboxOwnerUPN") {
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
                if event.has_value("o365audit.LogonUserDisplayName") {
                    event.rename("o365audit.LogonUserDisplayName", "user.full_name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("o365audit.OrganizationName") {
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
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                if event.has_value("o365audit.OriginatingServer") {
                    event.rename("o365audit.OriginatingServer", "server._temp")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has_value("o365audit.ClientIPAddress") {
                    event.rename("o365audit.ClientIPAddress", "client._temp")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ExchangeItem") };
            if _cond {
                if event.has_value("o365audit.ClientProcessName") {
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
                if event.has_value("o365audit.ObjectId") {
                    event.rename("o365audit.ObjectId", "url.original")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SourceRelativeUrl") {
                    event.rename("o365audit.SourceRelativeUrl", "file.directory")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SourceFileName") {
                    event.rename("o365audit.SourceFileName", "file.name")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["SharePointFileOperation", "SharePointSharingOperation"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SourceFileExtension") {
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
                if event.has_value("o365audit.Name") {
                    event.rename("o365audit.Name", "rule.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has_value("o365audit.PolicyId") {
                    event.rename("o365audit.PolicyId", "rule.id")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has_value("o365audit.Category") {
                    event.rename("o365audit.Category", "rule.category")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has_value("o365audit.EntityType") {
                    event.rename("o365audit.EntityType", "rule.ruleset")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has_value("o365audit.AlertEntityId") {
                    event.rename("o365audit.AlertEntityId", "rule.description")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                if event.has_value("o365audit.AlertLinks") {
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
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                event.append_unique("event.category", json!("web"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SecurityComplianceAlerts") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
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
                if event.has_value("o365audit.AlertEntityId") {
                    event.rename("o365audit.AlertEntityId", "user.email")?;
                }
            }

            let _cond = {
                event.get_str("event.code") == Some("SecurityComplianceAlerts")
                    && event.get_str("rule.ruleset") == Some("MalwareFamily")
            };
            if _cond {
                if event.has_value("o365audit.AlertEntityId") {
                    event.rename("o365audit.AlertEntityId", "threat.technique.id")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("ComplianceDLPExchange") };
            if _cond {
                // Painless script
                // Source: def operation = ctx.event?.action ?: ''; def user = ctx.user?.id ?: ''; def subject = ctx.o365audit?.ExchangeMetaData?.Subject ?: ctx.email?.subject ?: '';\nif (operation.isEmpty() && user.isEmpty() && subject.isEmpty()) {\n  ctx.message = \"Office365 Alert\";\n} else {\n  ctx.message = \"Office365 Alert: \" + operation + \" detected in email sent by \" + user + \" with subject '\" + subject + \"'\";\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                if event.has_value("o365audit.SharePointMetaData.From") {
                    event.rename("o365audit.SharePointMetaData.From", "user.id")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SharePointMetaData.FileName") {
                    event.rename("o365audit.SharePointMetaData.FileName", "file.name")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SharePointMetaData.FilePathUrl") {
                    event.rename("o365audit.SharePointMetaData.FilePathUrl", "url.original")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SharePointMetaData.UniqueId") {
                    event.rename("o365audit.SharePointMetaData.UniqueId", "file.inode")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SharePointMetaData.UniqueID") {
                    event.rename("o365audit.SharePointMetaData.UniqueID", "file.inode")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.SharePointMetaData.FileOwner") {
                    event.rename("o365audit.SharePointMetaData.FileOwner", "file.owner")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.ExchangeMetaData.From") {
                    event.rename("o365audit.ExchangeMetaData.From", "source.user.email")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.PolicyId") {
                    event.rename("o365audit.PolicyId", "rule.id")?;
                }
            }

            let _cond = {
                event.has_value("event.code")
                    && ["ComplianceDLPSharePoint", "ComplianceDLPExchange"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if event.has_value("o365audit.PolicyName") {
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
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("file.mtime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.SharePointMetaData.LastModifiedTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                if event.has_value("o365audit.ExceptionInfo") {
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
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"int severityToCode(def x) { \n  if (x.toLowerCase() == \"informational\") {\n    return 1;\n  }\n  if (x.toLowerCase() == \"low\") {\n    return 2;\n  }\n  if (x.toLowerCase() == \"medium\") {\n    return 3;\n  }\n  if (x.toLowerCase() == \"high\") {\n    return 4;\n  }\n  return 0;\n} def policies = ctx.o365audit.PolicyDetails; if (policies == null) {\n  return;\n} if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n} if (ctx.rule.id == null) {\n  ctx.rule.id = new ArrayList();\n} if (ctx.rule.name == null) {\n  ctx.rule.name = new ArrayList();\n} def maxSeverity = 0; def allowed = true; for (int i = 0; i < policies.length && policies instanceof List; ++i) {\n  def rules = policies[i].Rules;\n  if (rules == null) {\n    continue;\n  }\n  for (int j = 0; j < rules.length; ++j) {\n    def rule = rules[j];\n    def id = rule.RuleId;\n    def name = rule.RuleName;\n    def sev = severityToCode(rule.Severity);\n    if (id != null && name != null) {\n      ctx.rule.id.add(id);\n      ctx.rule.name.add(name);\n    }\n    if (sev > maxSeverity) {\n      maxSeverity = sev;\n    }\n    if (allowed) {\n      if (rule.Actions != null && rule.Actions.contains(\"BlockAccess\")) {\n        allowed = false;\n      }\n    }\n  }\n} if (maxSeverity > -1) {\n  ctx.event.severity = maxSeverity;\n} if (allowed) {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpRuleUndo\") {\n  ctx.event.outcome = \"success\";\n  return;\n} if (ctx.event?.action == \"DlpInfo\") {\n  ctx.event.outcome = \"failure\";\n  return;\n} if (ctx.o365audit?.ExceptionInfo != null && !ctx.o365audit?.ExceptionInfo.isEmpty()) {\n  ctx.event.outcome = \"success\";\n  return;\n} ctx.event.outcome = \"failure\";\n"#
                    ),
                )?;
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has_value("o365audit.ActorUserId") {
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
                if event.has_value("o365audit.FileId") {
                    event.rename("o365audit.FileId", "file.inode")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has_value("o365audit.FileName") {
                    event.rename("o365audit.FileName", "file.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has_value("o365audit.GroupName") {
                    event.rename("o365audit.GroupName", "group.name")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has_value("o365audit.TargetUserId") {
                    event.rename("o365audit.TargetUserId", "destination.user.email")?;
                }
            }

            let _cond = { event.get_str("event.code") == Some("Yammer") };
            if _cond {
                if event.has_value("o365audit.TargetYammerUserId") {
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
                if event.has_value("o365audit.TeamName") {
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
                if event.has_value("o365audit.ObjectId") {
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
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def members = ctx.o365audit?.Members; if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} for (int i = 0; i < members.length; ++i) {\n  if (members[i] instanceof Map && members[i].containsKey(\"UPN\") && !members[i][\"UPN\"].isEmpty()) {\n    ctx.related.user.add(members[i][\"UPN\"]);\n  }\n}\n"#
                    ),
                )?;
            }

            if event.has_value("client._temp") {
                gsub_field(
                    event,
                    "client._temp",
                    "client._temp",
                    cached_regex!(
                        "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](:[0-9]+)?)?$"
                    ),
                    "$1$2",
                )?;
            }

            let _cond = {
                event.has_value("client._temp")
                    && event.get("client._temp").is_some_and(|v| !match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                if let Some(input) = event.get_string("client._temp") {
                    // Grok pattern: (?:^\\[%{IP:client.address}\\]:%{POSINT:client._port})
                    // Grok pattern: ^%{IP:client.address}$
                    // Grok pattern: ^\\[%{IP:client.address}\\]$
                    // Grok pattern: (?:^%{IP:client.address}:%{POSINT:client._port})
                    // Grok pattern: ^%{NOTSPACE:client.domain}$
                    // Grok pattern: (?:^\\[%{NOTSPACE:client.domain}\\]:%{POSINT:client._port})
                    // Grok pattern: (?:^%{NOTSPACE:client.domain}:%{POSINT:client._port})
                    // Grok pattern: ^\\[(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))\\]$
                    // Grok pattern: ^(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))$
                    // Grok pattern: %{GREEDYDATA:client.address}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("(?:^\\[%{IP:client.address}\\]:%{POSINT:client._port})"),
                            cached_grok!("^%{IP:client.address}$"),
                            cached_grok!("^\\[%{IP:client.address}\\]$"),
                            cached_grok!("(?:^%{IP:client.address}:%{POSINT:client._port})"),
                            cached_grok!("^%{NOTSPACE:client.domain}$"),
                            cached_grok!(
                                "(?:^\\[%{NOTSPACE:client.domain}\\]:%{POSINT:client._port})"
                            ),
                            cached_grok!("(?:^%{NOTSPACE:client.domain}:%{POSINT:client._port})"),
                            cached_grok_mapped!(
                                "^\\[(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))\\]$",
                                [("client_address", "client.address")]
                            ),
                            cached_grok_mapped!(
                                "^(?:%{NOTSPACE:client.domain} \\((?P<client_address>(?:[^)]*))\\))$",
                                [("client_address", "client.address")]
                            ),
                            cached_grok!("%{GREEDYDATA:client.address}"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            if event.has_value("server._temp") {
                gsub_field(
                    event,
                    "server._temp",
                    "server._temp",
                    cached_regex!("[\n\r]"),
                    "",
                )?;
            }

            let _cond = {
                event.has_value("server._temp")
                    && event.get("server._temp").is_some_and(|v| !match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("server._temp") {
                        // Grok pattern: ^\\[(?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))\\]$
                        // Grok pattern: (?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))
                        // Grok pattern: %{GREEDYDATA:server.address}
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^\\[(?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))\\]$",
                                    [("server_address", "server.address")]
                                ),
                                cached_grok_mapped!(
                                    "(?:%{NOTSPACE:server.domain} \\((?P<server_address>(?:[^)]*))\\))",
                                    [("server_address", "server.address")]
                                ),
                                cached_grok!("%{GREEDYDATA:server.address}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("client.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "client.address".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
                Ok(())
            })();

            if event.has_value("client._port") {
                if let Some(val) = event.get("client._port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "client._port".into(),
                            message,
                        }
                    })?;
                    event.set("client.port", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("server.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "server.address".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("file.owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("o365audit.Parameters.User") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("o365audit.Parameters.User")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("o365audit.ExtendedProperties.UserAgent") };
            if _cond {
                if event.has_value("o365audit.ExtendedProperties.UserAgent") {
                    event.rename(
                        "o365audit.ExtendedProperties.UserAgent",
                        "user_agent.original",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("organization.id") {
                    map_strings(
                        event,
                        "organization.id",
                        "organization.id",
                        str::to_lowercase,
                    )?;
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
                painless_exec_plan(
                    event,
                    cached_painless!(
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
                    && event.get_str("o365audit.UserType") != Some("")
            };
            if _cond {
                let v = json!(
                    event
                        .get("o365audit.UserType")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("event.provider", v)?;
                }
            }

            let _cond = {
                event.has_value("o365audit.InternetMessageId")
                    && event.get_str("o365audit.InternetMessageId") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.message_id",
                    json!(
                        event
                            .get("o365audit.InternetMessageId")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Item.InternetMessageId")
                    && event.get_str("o365audit.Item.InternetMessageId") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.message_id",
                    json!(
                        event
                            .get("o365audit.Item.InternetMessageId")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.NetworkMessageId")
                    && event.get_str("o365audit.NetworkMessageId") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.local_id",
                    json!(
                        event
                            .get("o365audit.NetworkMessageId")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.P1Sender")
                    && event.get_str("o365audit.P1Sender") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.sender.address",
                    json!(
                        event
                            .get("o365audit.P1Sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event.get_str("source.user.email") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.sender.address",
                    json!(
                        event
                            .get("source.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get("o365audit.Recipients").is_some_and(|v| v.is_array()) && event.get("o365audit.Recipients").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                foreach_array(event, "o365audit.Recipients", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("destination.user.email")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "destination.user.email", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("o365audit.SenderIp")
                    && event.get_str("o365audit.SenderIp") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("o365audit.SenderIp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.SenderIP")
                    && event.get_str("o365audit.SenderIP") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("o365audit.SenderIP")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Subject")
                    && event.get_str("o365audit.Subject") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.subject",
                    json!(
                        event
                            .get("o365audit.Subject")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Item.Subject")
                    && event.get_str("o365audit.Item.Subject") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.subject",
                    json!(
                        event
                            .get("o365audit.Item.Subject")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Item.Attachments")
                    && event.get_str("o365audit.Item.Attachments") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.email = ctx.email ?: [:];\nctx.email.attachments = [];\n\ndef attachmentList = ctx.o365audit.Item.Attachments.splitOnToken(';');\n\nfor (def attachment : attachmentList) {\n  def att = attachment.trim();\n  if (att.isEmpty() || !att.endsWith(')')) continue;\n\n  // Find the size marker at the end: \" (<digits>b)\"\n  int lastSep = att.lastIndexOf(' (');\n  if (lastSep < 0) continue;\n\n  def maybeSize = att.substring(lastSep + 2, att.length() - 1);\n  if (!maybeSize.endsWith('b')) continue;\n\n  def sizeStr = maybeSize.substring(0, maybeSize.length() - 1);\n  long sizeVal;\n  try { sizeVal = Long.parseLong(sizeStr); } catch (Exception e) { continue; }\n\n  def filename = att.substring(0, lastSep).trim();\n  if (filename.isEmpty()) continue;\n\n  int dotIdx = filename.lastIndexOf('.');\n  if (dotIdx < 0) continue;\n\n  def attachmentObj = [:];\n  attachmentObj.file = [:];\n  attachmentObj.file.extension = filename.substring(dotIdx + 1);\n  attachmentObj.file.name = filename;\n  attachmentObj.file.size = sizeVal;\n  ctx.email.attachments.add(attachmentObj);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
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
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("o365audit.EndTimeUtc", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.EndTimeUtc".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.LastUpdateTimeUtc") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.LastUpdateTimeUtc") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.LastUpdateTimeUtc", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.LastUpdateTimeUtc".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.StartTimeUtc") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.StartTimeUtc") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.StartTimeUtc", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.StartTimeUtc".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.StartTime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.StartTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.StartTime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.StartTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.FilteringDate") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.FilteringDate") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.FilteringDate", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.FilteringDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.RescanResult.Timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.RescanResult.Timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.RescanResult.Timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.RescanResult.Timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has("o365audit.Data") && event.get_str("o365audit.RecordType") == Some("64")
            };
            if _cond {
                gsub_field(
                    event,
                    "o365audit.Data",
                    "o365audit.Data",
                    cached_regex!(
                        ",\\\"QueryTime\\\":\\\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\\\"|\\\"QueryTime\\\":\\\"[0-9\\/]+\\s[0-9]+:[0-9]+:[0-9]+\\s[AP]M\\\","
                    ),
                    "",
                )?;
            }

            let _cond = { event.has("o365audit.Data") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "o365audit.Data", "o365audit.Data")?;
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

            if event.has_value("o365audit.Data") {
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
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"for (def key : params.knownKeys) {\n  if (ctx.o365audit.Data.flattened.containsKey(key)) {\n    ctx.o365audit.Data[key] = ctx.o365audit.Data.flattened[key];\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"knownKeys\":[\"ad\",\"af\",\"aii\",\"ail\",\"alk\",\"als\",\"an\",\"at\",\"cid\",\"cpid\",\"dm\",\"dpn\",\"eid\",\"etps\",\"etype\",\"f3u\",\"fvs\",\"imsgid\",\"lon\",\"mat\",\"md\",\"ms\",\"od\",\"op\",\"ot\",\"plk\",\"pud\",\"reid\",\"rid\",\"sev\",\"sict\",\"sid\",\"sip\",\"sitmi\",\"srt\",\"ssic\",\"suid\",\"tdc\",\"te\",\"thn\",\"tht\",\"tid\",\"tpid\",\"tpt\",\"trc\",\"ts\",\"tsd\",\"ttdt\",\"ttr\",\"upfc\",\"upfv\",\"ut\",\"von\",\"wl\",\"zfh\",\"zfn\",\"zmfh\",\"zmfn\",\"zu\"]}"
                    ),
                )?;
            }

            let _cond = { event.get_str("o365audit.Data.sip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("o365audit.Data.sip") {
                        if let Some(val) = event.get("o365audit.Data.sip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "o365audit.Data.sip".into(),
                                    message,
                                }
                            })?;
                            event.set("o365audit.Data.sip", converted)?;
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
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.Data.at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.Data.at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.md") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.md") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.Data.md", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.Data.md".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.te") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.te") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd HH:mm:ss'Z'"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("o365audit.Data.te", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.Data.te".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.ts") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601", "yyyy-MM-dd HH:mm:ss'Z'"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("o365audit.Data.ts", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.Data.ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("o365audit.Data.ttdt") };
            if _cond {
                if let Some(date_str) = event.get_as_string("o365audit.Data.ttdt") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("o365audit.Data.ttdt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "o365audit.Data.ttdt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.f3u")
                    .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("o365audit.Data.f3u")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("user.email")
                    && event
                        .get_str("o365audit.Data.f3u")
                        .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                let v = json!(
                    event
                        .get("o365audit.Data.f3u")
                        .map_or_else(String::new, template_to_string)
                );
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
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("o365audit.Data.suid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Data.tsd")
                    && event.get_str("o365audit.Data.tsd") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.sender.address",
                    json!(
                        event
                            .get("o365audit.Data.tsd")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.tsd")
                    .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("o365audit.Data.tsd")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Data.trc")
                    && event.get_str("o365audit.Data.trc") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("o365audit.Data.trc")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get_str("o365audit.Data.trc")
                    .is_some_and(|s| s.split('@').count() == 2)
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("o365audit.Data.trc")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Data.aii")
                    && event.get_str("o365audit.Data.aii") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.local_id",
                    json!(
                        event
                            .get("o365audit.Data.aii")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Data.imsgid")
                    && event.get_str("o365audit.Data.imsgid") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.message_id",
                    json!(
                        event
                            .get("o365audit.Data.imsgid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("o365audit.Data.ms")
                    && event.get_str("o365audit.Data.ms") != Some("")
            };
            if _cond {
                event.append_unique(
                    "email.subject",
                    json!(
                        event
                            .get("o365audit.Data.ms")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                painless_exec_plan_params(
                    event,
                    cached_painless!(
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
                foreach_array(event, "_tmp.entities.InternetMessageId", |event| {
                    event.append_unique(
                        "email.message_id",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("_tmp.entities.NetworkMessageId")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "_tmp.entities.NetworkMessageId", |event| {
                    event.append_unique(
                        "email.local_id",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("_tmp.entities.P1Sender")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "_tmp.entities.P1Sender", |event| {
                    event.append_unique(
                        "email.sender.address",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("_tmp.entities.P2Sender")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "_tmp.entities.P2Sender", |event| {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("_tmp.entities.Recipient")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "_tmp.entities.Recipient", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                !event.has_value("user.email") && event.get("_tmp.entities.Recipient").is_some_and(|v| v.is_array()) && event.get("_tmp.entities.Recipient").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
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
                foreach_array(event, "_tmp.entities.SenderIP", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("_tmp.entities.Subject")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "_tmp.entities.Subject", |event| {
                    event.append_unique(
                        "email.subject",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("_tmp.entities.Upn").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "_tmp.entities.Upn", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("_tmp.entities.OriginalDeliveryLocation") {
                event.rename(
                    "_tmp.entities.OriginalDeliveryLocation",
                    "o365audit.OriginalDeliveryLocation",
                )?;
            }

            if event.has_value("_tmp.entities.PhishConfidenceLevel") {
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
                    parse_json_field(
                        event,
                        "o365audit.ExtendedProperties.additionalDetails",
                        "o365audit.ExtendedProperties.additionalDetails",
                    )?;
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
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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
                if event.has_value("o365audit.ExtendedProperties.additionalDetails.User-Agent") {
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
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def methods = ctx._tmp.entities.ThreatDetectionMethods; def result = []; for (def method: methods){\n  if (method instanceof List) {\n    for (def m: method) {\n      result.add(m);\n    }\n  } else if (method instanceof String) {\n    result.add(method);\n  }\n} ctx.o365audit.ThreatDetectionMethods = result;\n"#
                    ),
                )?;
            }

            if event.has_value("o365audit") {
                event.rename("o365audit", "o365.audit")?;
            }

            if event.has_value("user_agent.original") {
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

            if event.has_value("source.ip") {
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

            if event.has_value("source.ip") {
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
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

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.email")
                            .map_or_else(String::new, template_to_string)
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
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("host.hostname")
                    && event.get("host.hostname").filter(|v| !v.is_null())
                        != event.get("host.name").filter(|v| !v.is_null())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.target.domain")
                    && event.get("user.target.domain").filter(|v| !v.is_null())
                        != event.get("user.domain").filter(|v| !v.is_null())
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("user.target.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("server.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("client.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha512") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha512")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("_conf");
            event.remove("_tmp");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
