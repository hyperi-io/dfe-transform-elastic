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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "a3d0726d34bed0ebb2771c2c574b301ea25a2523f32b64874dcd574d126d7cee",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.category", json!("web"))?;

            let _cond = { event.has_value("o365audit") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "8309192897f1cb0ca96bdc0a1ac7a74a13878d404257d6edba67911a75022bad",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "83df451c436faa86f3e81e65d8747283deeb176f3248466d52afe8c84a9bcdd3",
                        r#"{"params":null,"known":[{"AppendEach":{"source":"o365audit.Actions","target":"o365audit.Actions","map_keys":[],"separator":",","deduped":false}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "21f888c15fa6caee916ddb1fb385f2953250b8fed5b64d5faf1b07c47b9d48dc",
                        r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"Literal":{"AppendAll":{"path":"o365audit.Actions","source":"_tmp.action_strings"}}}],"whole":true}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "0c680041a96a99b423a9dbc2fc39f06e194799d6fff42b0e686743796194662f",
                        r#"{"params":{"KeyedRowAssign":{"source":"o365audit.RecordType","target":"event.code"}},"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "ba72ebdc9b2b595d01ae76e6a6cb0fd34e59a563f7edac1da6c99467102c1ca3",
                        r#"{"params":null,"known":[]}"#
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
                            if !cached_grok!(
                                "^-?Identity\\s\"?%{DATA:o365audit.NetworkMessageId}\"?$"
                            )
                            .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "58d84a7a6731d833ac1909d91bcddce5c9bea82ee182271a87dd5bcee4ac2008",
                    r#"{"params":null,"known":["SplitTrimCollect"]}"#,
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "89947a9278e2440c1b0b631d0c340156d6efc56f8e544113d66f9e73a2fca6ca",
                        r#"{"params":null,"known":[]}"#
                    ),
                    cached_params!(
                        "{\"0\":\"Unknown\",\"1\":\"Windows\",\"2\":\"MacOS\",\"3\":\"iOS\",\"4\":\"Android\",\"5\":\"Web Browser\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("o365audit.Platform") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "56ea5b28b39437a63c8654e30562afd09edfb70893bbf60b60116d80e929a5b6",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "ef1e518d2fcc787043b799cc234ef0401f67840b318755354448c1aed7fd95e9",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "6a9b4ffd86f6e865c1c765cbc69f46fa2cead31ee96092f8046cef3f89f8c264",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "35346429fd988d261bdd7937d6295c2446e224fcf0290550a777e6eaf1c2ff50",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "e9a7a4c39ba77bfb0c639b0bb1b0c527223ccc0ef80aae31df25a69368449f28",
                        r#"{"params":null,"known":["ConcatMessage"]}"#,
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "336c9043f1efcc54e2a67ea3bee403a11e07f5128c1eefc618ba924c279c33dd",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "79f2762a57048664cf94400eddbd91b345df6c04e0f70639489bd18290816fc0",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "c8ded392dbd97084acf49673d8a17df7e612a4e287c3668bcd13fc715b8357ee",
                        r#"{"params":null,"known":[]}"#
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
                    if !extract_first_match(
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
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
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
                        if !extract_first_match(
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
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "a03f8845759da11e022062e55f51f5d4382a183cad75178dbf89944619b4e3ca",
                        r#"{"params":null,"known":[{"EmailSplit":{"source":"user.id","names":["user.name"],"domains":["user.domain"],"emails":["user.email"],"appends":[]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "99407f841fb9b1bb7dd092c9051600f237118d9b940d6f5e0e21d31b1d7d530a",
                        r#"{"params":null,"known":[{"EmailSplit":{"source":"user.target.id","names":["user.target.name"],"domains":["user.target.domain"],"emails":["user.target.email"],"appends":[]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "8e8331674cb8e17243d9a9c8c23f30afa8b6c3bbe95ed15eaf05508e54df536e",
                        r#"{"params":null,"known":[{"EmailSplit":{"source":"source.user.id","names":["source.user.name"],"domains":["source.user.domain"],"emails":["source.user.email"],"appends":[]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "13ca8ab8c5a130a70070ac3970036bf84439edbf20bb0e537301410b608cfc79",
                        r#"{"params":null,"known":[{"EmailSplit":{"source":"destination.user.id","names":["destination.user.name"],"domains":["destination.user.domain"],"emails":["destination.user.email"],"appends":[]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "6adc86085d80b1f8ec825dd07cf0f26805b215003114d30591ffcf039d991aa0",
                        r#"{"params":null,"known":[{"CtxTableLookup":{"table":"_conf.tenants","key":"organization.id","target":"organization.name"}}]}"#
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
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "f292a6b2377f95e21c1aff26236dcdee1a2a075afb03b0845d05d53e653de003",
                            r#"{"params":null,"known":[{"AppendEach":{"source":"o365audit.Item.Attachments","target":"email.attachments","map_keys":[],"separator":";","deduped":false}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "dcc60a769c431bb2d48e3c3d995b1b5a36553b3dcc02649754a27c5a7789c48b",
                        r#"{"params":null,"known":[]}"#
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
                    && event
                        .get_as_string("o365audit.Data.f3u")
                        .is_some_and(|s| s.len() >= 3)
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
                    && event
                        .get_as_string("o365audit.Data.f3u")
                        .is_some_and(|s| s.len() >= 3)
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
                    && event
                        .get_as_string("o365audit.Data.suid")
                        .is_some_and(|s| s.len() >= 3)
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
                    && event
                        .get_as_string("o365audit.Data.tsd")
                        .is_some_and(|s| s.len() >= 3)
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
                    && event
                        .get_as_string("o365audit.Data.trc")
                        .is_some_and(|s| s.len() >= 3)
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "d87fe80dc9d6017d6e188cec968b41449d5568636658da8dd16cb6269f4337ca",
                        r#"{"params":null,"known":[]}"#
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
                            event.remove("user_agent");
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "d76ad891040eb33802c207260c7858d539274beaf36c2d8ab8bb27299c1817bb",
                        r#"{"params":null,"known":[]}"#
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
                        event.remove("user_agent");
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
                    && !condition_eq(event.get("host.hostname"), event.get("host.name"))
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
                    && !condition_eq(event.get("user.target.domain"), event.get("user.domain"))
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
