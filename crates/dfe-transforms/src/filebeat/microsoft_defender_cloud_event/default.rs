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
            event.set("ecs.version", json!("8.17.0"))?;

            if event.has_value("azure") {
                event.rename("azure", "azure-eventhub")?;
            }

            event.set("event.kind", json!("alert"))?;

            event.set(
                "event.category",
                Value::Array(vec![json!("intrusion_detection")]),
            )?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_message")?;
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

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: void handleMap(Map map) {\n for (def x: map.values()) {\n  if (x instanceof Map) {\n   handleMap(x);\n  } else if (x instanceof List) {\n   handleList(x);\n  }\n }\n def keySet = map.keySet().toArray();\n for (def key: keySet) {\n  def lc = key.toLowerCase();\n  map[lc] = map[key];\n  if (key != lc) {\n   map.remove(key)\n  }\n }\n}\nvoid handleList(List list) {\n for (def x: list) {\n  if (x instanceof Map) {\n   handleMap(x);\n  } else if (x instanceof List) {\n   handleList(x);\n  }\n }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n for (def x: map.values()) {\n  if (x instanceof Map) {\n   handleMap(x);\n  } else if (x instanceof List) {\n   handleList(x);\n  }\n }\n def keySet = map.keySet().toArray();\n for (def key: keySet) {\n  def lc = key.toLowerCase();\n  map[lc] = map[key];\n  if (key != lc) {\n   map.remove(key)\n  }\n }\n}\nvoid handleList(List list) {\n for (def x: list) {\n  if (x instanceof Map) {\n   handleMap(x);\n  } else if (x instanceof List) {\n   handleList(x);\n  }\n }\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.alerttype")
                    && event.get_str("json.alerttype").is_some_and(|s| {
                        [
                            "arm_anomalousserviceoperation.credentialaccess",
                            "arm_anomalousserviceoperation.collection",
                            "arm_anomalousserviceoperation.defenseevasion",
                            "arm_anomalousserviceoperation.execution",
                            "arm_anomalousserviceoperation.impact",
                            "arm_anomalousserviceoperation.initialaccess",
                            "arm_anomalousserviceoperation.lateralmovement",
                            "arm_anomalousserviceoperation.persistence",
                            "arm_anomalousserviceoperation.privilegeescalation",
                            "arm_unusedaccountpersistence",
                            "arm_unusedapppowershellpersistence",
                            "arm_unusedappibizapersistence",
                            "arm_privilegedroledefinitioncreation",
                            "arm_anomalousrbacroleassignment",
                            "arm_anomalousoperation.credentialaccess",
                            "arm_anomalousoperation.collection",
                            "arm_anomalousoperation.defenseevasion",
                            "arm_anomalousoperation.execution",
                            "arm_anomalousoperation.impact",
                            "arm_anomalousoperation.initialaccess",
                            "arm_anomalousoperation.lateralmovement",
                            "arm_anomalousoperation.persistence",
                            "arm_anomalousoperation.privilegeescalation",
                            "arm_microburst.runcodeonbehalf",
                            "arm_netspi.maintainpersistence",
                            "arm_powerzure.runcodeonbehalf",
                            "arm_powerzure.maintainpersistence",
                            "arm_anomalousclassicroleassignment",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("indicator")]))?;
            }

            let _cond = {
                event.has_value("json.alerttype")
                    && event.get_str("json.alerttype").is_some_and(|s| {
                        [
                            "api_populationspikeinapitraffic",
                            "api_spikeinapitraffic",
                            "api_spikeinpayload",
                            "api_spikeinlatency",
                            "api_sprayinrequests",
                            "api_parameterenumeration",
                            "api_distributedparameterenumeration",
                            "api_unseenparamtype",
                            "api_unseenparam",
                            "api_accessfromtorexitnode",
                            "api_accessfromsuspiciousip",
                            "api_accessfromsuspicioususeragent",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("api")]))?;
            }

            let _cond = {
                event.has_value("json.alerttype")
                    && event.get_str("json.alerttype").is_some_and(|s| {
                        [
                            "vm_loginbruteforcesuccess",
                            "vm_vmaccessunusualpasswordreset",
                            "vm_sshkeyaddition",
                            "vm_vmaccessunusualpasswordreset",
                            "vm_vmaccessunusualsshreset",
                            "sql.db_geoanomaly",
                            "sql.vm_geoanomaly",
                            "sql.dw_geoanomaly",
                            "sql.mi_geoanomaly",
                            "sql.db_principalanomaly",
                            "sql.vm_principalanomaly",
                            "sql.dw_principalanomaly",
                            "sql.mi_principalanomaly",
                            "sql.db_domainanomaly",
                            "sql.vm_domainanomaly",
                            "sql.dw_domainanomaly",
                            "sql.mi_domainanomaly",
                            "sql.db_bruteforce",
                            "sql.vm_bruteforce",
                            "sql.dw_bruteforce",
                            "sql.mi_bruteforce",
                            "sql.postgresql_bruteforce",
                            "sql.mariadb_bruteforce",
                            "sql.mysql_bruteforce",
                            "sql.postgresql_principalanomaly",
                            "sql.mariadb_principalanomaly",
                            "sql.mysql_principalanomaly",
                            "sql.mariadb_domainanomaly",
                            "sql.postgresql_domainanomaly",
                            "sql.mysql_domainanomaly",
                            "sql.postgresql_datacenteranomaly",
                            "sql.mariadb_datacenteranomaly",
                            "sql.mysql_datacenteranomaly",
                            "sql.postgresql_cloudprovideranomaly",
                            "sql.mariadb_cloudprovideranomaly",
                            "sql.mysql_cloudprovideranomaly",
                            "sql.mariadb_geoanomaly",
                            "sql.postgresql_geoanomaly",
                            "sql.mysql_geoanomaly",
                            "storage.blob_suspiciousapp",
                            "storage.blob_suspiciousip",
                            "storage.files_suspiciousip",
                            "storage.blob_openacl",
                            "storage.blob_toranomaly",
                            "storage.files_toranomaly",
                            "storage.blob_geoanomaly",
                            "storage.files_geoanomaly",
                            "storage.blob_anonymousaccessanomaly",
                            "storage.blob_opencontainersscanning",
                            "storage.blob_accessinspectionanomaly",
                            "storage.files_accessinspectionanomaly",
                            "cosmosdb_toranomaly",
                            "cosmosdb_suspiciousip",
                            "cosmosdb_geoanomaly",
                            "kv_suspiciousipaccess",
                            "kv_toraccess",
                            "kv_accountvolumeaccessdeniedanomaly",
                            "kv_useraccessdeniedanomaly",
                            "kv_appanomaly",
                            "kv_operationpatternanomaly",
                            "kv_useranomaly",
                            "kv_userappanomaly",
                            "kv_accountvolumeanomaly",
                            "kv_suspiciousipaccessdenied",
                            "kv_unusualaccesssuspiciousip",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set(
                    "event.category",
                    Value::Array(vec![json!("authentication")]),
                )?;
            }

            let _cond = {
                event.has_value("json.alerttype")
                    && event.get_str("json.alerttype").is_some_and(|s| {
                        [
                            "k8s_exposedpostgrestrustauth",
                            "k8s_exposedpostgresbroadiprange",
                            "arm_azurite",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("configuration")]))?;
            }

            let _cond = {
                event.has_value("json.alerttype")
                    && event.get_str("json.alerttype").is_some_and(|s| {
                        [
                            "vm_ammalwarecampaignrelatedexclusion",
                            "vm_filelessattacktoolkit",
                            "vm_runbypsexec",
                            "vm_svchostruninrareservicegroup",
                            "vm_suspiciousactivity",
                            "vm_loginbruteforcevaliduserfailed",
                            "vm_customscriptextensionsuspiciousfailure",
                            "vm_taskkillburst",
                            "vm_vmaccessunusualsshreset",
                            "vm_ambroadfilesexclusion",
                            "vm_amdisablementandcodeexecution",
                            "vm_amdisablement",
                            "vm_amfileexclusionandcodeexecution",
                            "vm_amtempfileexclusionandcodeexecution",
                            "vm_amtempfileexclusion",
                            "vm_amrealtimeprotectiondisabled",
                            "vm_amtemprealtimeprotectiondisablement",
                            "vm_amrealtimeprotectiondisablementandcodeexec",
                            "vm_amtemporarilydisablement",
                            "vm_unusualamfileexclusion",
                            "vm_sshbruteforcefailed",
                            "vm_filelessattackbehavior",
                            "vm_filelessattacktechnique",
                            "vm_mailserverexploitation",
                            "vm_sshbruteforcesuccess",
                            "vm_kubernetesdashboard",
                            "vm_vmaccessunusualconfigreset",
                            "vm_customscriptextensionunusualdeletion",
                            "vm_customscriptextensionunusualexecution",
                            "vm_harmfulapplication",
                            "vm_suspiciousipanomaly",
                            "appservices_base64encodedexecutableincommandlineparams",
                            "appservices_suspectdownload",
                            "appservices_eicar",
                            "appservices_nmap",
                            "appservices_phpinuploadfolder",
                            "k8s_anomalouspoddeployment",
                            "k8s_anomaloussecretaccess",
                            "k8s_exposeddashboard",
                            "k8s_exposedservice",
                            "k8s_exposedredis",
                            "sql.db_harmfulapplication",
                            "sql.vm_harmfulapplication",
                            "sql.mi_harmfulapplication",
                            "sql.dw_harmfulapplication",
                            "sql.db_suspiciousipanomaly",
                            "sql.vm_suspiciousipanomaly",
                            "sql.dw_suspiciousipanomaly",
                            "sql.mi_suspiciousipanomaly",
                            "sql.postgresql_suspiciousipanomaly",
                            "sql.mariadb_suspiciousipanomaly",
                            "sql.mysql_suspiciousipanomaly",
                            "arm_operationfromsuspiciousip",
                            "arm_operationfromsuspiciousproxyip",
                            "arm_suspiciouscomputecreation",
                            "arm_suspicious_vault_recovering",
                            "arm_unusedaccountpersistence",
                            "storage.files_widespreadeam",
                            "storage.blob_malwarehashreputation",
                            "storage.files_malwarehashreputation",
                            "storage.blob_dataexfiltration.amountofdataanomaly",
                            "storage.blob_dataexfiltration.numberofblobsanomaly",
                            "storage.files_dataexfiltration.amountofdataanomaly",
                            "storage.files_dataexfiltration.numberoffilesanomaly",
                            "storage.blob_applicationanomaly",
                            "storage.files_applicationanomaly",
                            "storage.blob_dataexplorationanomaly",
                            "storage.files_dataexplorationanomaly",
                            "network_resourceipindicatedasmalicious",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("malware")]))?;
            }

            let _cond = {
                event.has_value("json.alerttype")
                    && event.get_str("json.alerttype").is_some_and(|s| {
                        [
                            "vm_filelessattackbehavior.windows",
                            "vm_filelessattacktechnique.windows",
                            "azuredns_threatintelsuspectdomain",
                            "azuredns_protocolanomaly",
                            "azuredns_darkweb",
                            "azuredns_darkwebproxy",
                            "azuredns_sinkholeddomain",
                            "azuredns_phishingdomain",
                            "azuredns_domaingenerationalgorithm",
                            "azuredns_randomizeddomain",
                            "azuredns_currencymining",
                            "azuredns_suspiciousdomain",
                            "azuredns_datainfiltration",
                            "azuredns_dataexfiltration",
                            "azuredns_dataobfuscation",
                            "appservices_danglingdomain",
                            "appservices_phishingcontent",
                            "appservices_potentialdanglingdomain",
                            "k8s_exposedkubeflow",
                            "network_communicationwithc2",
                            "network_ddos_detected",
                            "network_ddos_mitigated",
                            "sql_incoming_bf_onetoone",
                            "ddos",
                            "rdp_incoming_bf_manytoone",
                            "rdp_incoming_bf_onetoone",
                            "rdp_outgoing_bf_onetomany",
                            "rdp_outgoing_bf_onetoone",
                            "ssh_incoming_bf_manytoone",
                            "ssh_incoming_bf_onetoone",
                            "ssh_outgoing_bf_onetomany",
                            "ssh_outgoing_bf_onetoone",
                            "portscanning",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("network")]))?;
            }

            let _cond = {
                event.has_value("json.alerttype")
                    && event.get_str("json.alerttype").is_some_and(|s| {
                        [
                            "arm_anomalousserviceoperation.credentialaccess",
                            "arm_anomalousserviceoperation.collection",
                            "arm_anomalousserviceoperation.defenseevasion",
                            "arm_anomalousserviceoperation.execution",
                            "arm_anomalousserviceoperation.impact",
                            "arm_anomalousserviceoperation.initialaccess",
                            "arm_anomalousserviceoperation.lateralmovement",
                            "arm_anomalousserviceoperation.persistence",
                            "arm_anomalousserviceoperation.privilegeescalation",
                            "arm_unusedapppowershellpersistence",
                            "arm_unusedappibizapersistence",
                            "arm_privilegedroledefinitioncreation",
                            "arm_anomalousrbacroleassignment",
                            "arm_anomalousoperation.credentialaccess",
                            "arm_anomalousoperation.collection",
                            "arm_anomalousoperation.defenseevasion",
                            "arm_anomalousoperation.execution",
                            "arm_anomalousoperation.impact",
                            "arm_anomalousoperation.initialaccess",
                            "arm_anomalousoperation.lateralmovement",
                            "arm_anomalousoperation.persistence",
                            "arm_anomalousoperation.privilegeescalation",
                            "arm_microburst.runcodeonbehalf",
                            "arm_netspi.maintainpersistence",
                            "arm_powerzure.runcodeonbehalf",
                            "arm_powerzure.maintainpersistence",
                            "arm_anomalousclassicroleassignment",
                        ]
                        .contains(&s.to_lowercase().as_str())
                    })
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("threat")]))?;
            }

            if event.has_value("json.$type") {
                event.rename("json.$type", "microsoft_defender_cloud.event.event_type")?;
            }

            if event.has_value("json.agentid") {
                event.rename("json.agentid", "microsoft_defender_cloud.event.agent_id")?;
            }

            if event.has_value("json.alertdisplayname") {
                event.rename(
                    "json.alertdisplayname",
                    "microsoft_defender_cloud.event.display_name",
                )?;
            }

            if event.has_value("json.alerttype") {
                event.rename(
                    "json.alerttype",
                    "microsoft_defender_cloud.event.alert_type",
                )?;
            }

            if event.has_value("json.alerturi") {
                event.rename("json.alerturi", "microsoft_defender_cloud.event.uri")?;
            }

            if let Some(v) = event
                .get("microsoft_defender_cloud.event.uri")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            if event.has_value("json.assessmenteventdataenrichment.action") {
                event.rename(
                    "json.assessmenteventdataenrichment.action",
                    "microsoft_defender_cloud.event.assessment_event_data_enrichment.action",
                )?;
            }

            if event.has_value("json.assessmenteventdataenrichment.apiversion") {
                event.rename(
                    "json.assessmenteventdataenrichment.apiversion",
                    "microsoft_defender_cloud.event.assessment_event_data_enrichment.api_version",
                )?;
            }

            let _cond =
                { event.get_str("json.assessmenteventdataenrichment.issnapshot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.assessmenteventdataenrichment.issnapshot") {
                        if let Some(val) =
                            event.get("json.assessmenteventdataenrichment.issnapshot")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.assessmenteventdataenrichment.issnapshot".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_defender_cloud.event.assessment_event_data_enrichment.is_snapshot", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_assessment_event_data_enrichment_is_snapshot_to_boolean",
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

            if event.has_value("json.azureresourceid") {
                event.rename(
                    "json.azureresourceid",
                    "microsoft_defender_cloud.event.azure_resource_id",
                )?;
            }

            if event.has_value("json.compromisedentity") {
                event.rename(
                    "json.compromisedentity",
                    "microsoft_defender_cloud.event.compromised_entity",
                )?;
            }

            if event.has_value("json.confidencelevel") {
                event.rename(
                    "json.confidencelevel",
                    "microsoft_defender_cloud.event.confidence.level",
                )?;
            }

            if event.has_value("json.confidencereasons") {
                event.rename(
                    "json.confidencereasons",
                    "microsoft_defender_cloud.event.confidence.reasons",
                )?;
            }

            if event.has_value("json.confidencescore") {
                event.rename(
                    "json.confidencescore",
                    "microsoft_defender_cloud.event.confidence.score",
                )?;
            }

            if event.has_value("json.correlationkey") {
                event.rename(
                    "json.correlationkey",
                    "microsoft_defender_cloud.event.correlation_key",
                )?;
            }

            if event.has_value("json.description") {
                event.rename(
                    "json.description",
                    "microsoft_defender_cloud.event.description",
                )?;
            }

            let _cond = {
                event.has_value("json.endtimeutc") && event.get_str("json.endtimeutc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.endtimeutc") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("microsoft_defender_cloud.event.end_time_utc", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.endtimeutc".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_end_time_utc")?;
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

            if let Some(v) = event
                .get("microsoft_defender_cloud.event.end_time_utc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = { event.has_value("json.entities") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n      if (key=='location') {\n        updatedJson['location_value'] = value;\n        updatedJson.remove('location');\n      }\n    }\n  }\n  return updatedJson;\n}\ndef entities_obj = new ArrayList();\nfor(entity in ctx.json.entities){\n  entities_obj.add(renameKeys(entity, params));\n}\nctx.entities_obj=entities_obj;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n      if (key=='location') {\n        updatedJson['location_value'] = value;\n        updatedJson.remove('location');\n      }\n    }\n  }\n  return updatedJson;\n}\ndef entities_obj = new ArrayList();\nfor(entity in ctx.json.entities){\n  entities_obj.add(renameKeys(entity, params));\n}\nctx.entities_obj=entities_obj;\n"#
                    ),
                    cached_params!(
                        "{\"$id\":\"id\",\"aadtenantid\":\"aad_tenant_id\",\"aaduserid\":\"aad_user_id\",\"$ref\":\"ref\",\"amazonresourceid\":\"amazon_resource_id\",\"azureid\":\"azure_id\",\"files\":\"files\",\"blobcontainer\":\"blob_container\",\"cloudresource\":\"cloud_resource\",\"commandline\":\"command_line\",\"containerid\":\"container_id\",\"creationtimeutc\":\"creation_time_utc\",\"dnsdomain\":\"dns_domain\",\"domainname\":\"domain_name\",\"elevationtoken\":\"elevation_token\",\"endtimeutc\":\"end_time_utc\",\"filehashes\":\"file_hashes\",\"hostipaddress\":\"host_ip_address\",\"hostname\":\"host_name\",\"imagefile\":\"image_file\",\"imageid\":\"image_id\",\"ipaddresses\":\"ip_addresses\",\"countrycode\":\"country_code\",\"countryname\":\"country_name\",\"isdomainjoined\":\"is_domain_joined\",\"isvalid\":\"is_valid\",\"cloudprovider\":\"cloud_provider\",\"organizationtype\":\"organization_type\",\"systemservice\":\"system_service\",\"logonid\":\"logon_id\",\"netbiosname\":\"net_bios_name\",\"ntdomain\":\"nt_domain\",\"objectguid\":\"object_guid\",\"omsagentid\":\"oms_agent_id\",\"osfamily\":\"os_family\",\"osversion\":\"os_version\",\"parentprocess\":\"parent_process\",\"processid\":\"process_id\",\"projectid\":\"project_id\",\"relatedazureresourceids\":\"related_azure_resource_ids\",\"resourceid\":\"resource_id\",\"resourcename\":\"resource_name\",\"resourcetype\":\"resource_type\",\"sessionid\":\"session_id\",\"sourceaddress\":\"source_address\",\"starttimeutc\":\"start_time_utc\",\"storageresource\":\"storage_resource\",\"threatintelligence\":\"threat_intelligence\",\"providername\":\"provider_name\",\"reportlink\":\"report_link\",\"threatdescription\":\"description\",\"threatname\":\"name\",\"locationtype\":\"location_type\",\"threattype\":\"type\",\"upnsuffix\":\"upn_suffix\"}"
                    ),
                )?;
            }

            if event.has_value("entities_obj") {
                event.rename("entities_obj", "microsoft_defender_cloud.event.entities")?;
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "cloud.provider",
                            json!(
                                event
                                    .get("_ingest._value.location.cloud_provider")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("microsoft_defender_cloud.event.entities")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.creation_time_utc")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event
                                                .set("_ingest._value.creation_time_utc", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.creation_time_utc".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "date_entities_creation_time_utc",
                                    )?;
                                    event.remove("_ingest._value.creation_time_utc");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "microsoft_defender_cloud.event.entities",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("microsoft_defender_cloud.event.entities")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.end_time_utc")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.end_time_utc", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.end_time_utc".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "date_entities_end_time_utc",
                                    )?;
                                    event.remove("_ingest._value.end_time_utc");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "microsoft_defender_cloud.event.entities",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("microsoft_defender_cloud.event.entities")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.start_time_utc")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event
                                                .set("_ingest._value.start_time_utc", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.start_time_utc".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "date_entities_start_time_utc",
                                    )?;
                                    event.remove("_ingest._value.start_time_utc");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "microsoft_defender_cloud.event.entities",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "container.id",
                            json!(
                                event
                                    .get("_ingest._value.container_id")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "host.domain",
                            json!(
                                event
                                    .get("_ingest._value.domain_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.domain_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "host.geo.city_name",
                            json!(
                                event
                                    .get("_ingest._value.location.city")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "host.geo.country_iso_code",
                            json!(
                                event
                                    .get("_ingest._value.location.country_code")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "host.geo.country_name",
                            json!(
                                event
                                    .get("_ingest._value.location.country_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.location.latitude") {
                                if let Some(val) = event.get("_ingest._value.location.latitude") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.location.latitude".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.location.latitude", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_location_latitude_to_double",
                            )?;
                            event.remove("_ingest._value.location.latitude");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.location.longitude") {
                                if let Some(val) = event.get("_ingest._value.location.longitude") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.location.longitude".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.location.longitude", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_location_longitude_to_double",
                            )?;
                            event.remove("_ingest._value.location.longitude");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "host.hostname",
                            json!(
                                event
                                    .get("_ingest._value.host_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.host_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "host.os.family",
                            json!(
                                event
                                    .get("_ingest._value.os_family")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        if event.has_value("_ingest._value.protocol") {
                            map_strings(
                                event,
                                "_ingest._value.protocol",
                                "_ingest._value.protocol",
                                str::to_lowercase,
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "network.transport",
                            json!(
                                event
                                    .get("_ingest._value.protocol")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "process.entity_id",
                            json!(
                                event
                                    .get("_ingest._value.process_id")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.address") {
                                if let Some(val) = event.get("_ingest._value.address") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.address".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.address", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event
                                .set("_ingest.on_failure_processor_tag", "convert_address_to_ip")?;
                            event.remove("_ingest._value.address");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.ip_addresses") {
                                foreach_array(event, "_ingest._value.ip_addresses", |event| {
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value.address") {
                                            if let Some(val) = event.get("_ingest._value.address") {
                                                let converted = convert_value(val, "ip").map_err(
                                                    |message| TransformError::ParseError {
                                                        path: "_ingest._value.address".into(),
                                                        message,
                                                    },
                                                )?;
                                                event.set("_ingest._value.address", converted)?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        event.set(
                                            "_ingest.on_failure_processor_tag",
                                            "convert_ip_addresses_address_to_ip",
                                        )?;
                                        event.remove("_ingest._value.address");
                                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.ip_addresses") {
                                foreach_array(event, "_ingest._value.ip_addresses", |event| {
                                    event.append_unique(
                                        "related.ip",
                                        json!(
                                            event
                                                .get("_ingest._value.address")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.asset") {
                                if let Some(val) = event.get("_ingest._value.asset") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.asset".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.asset", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_asset_to_boolean",
                            )?;
                            event.remove("_ingest._value.asset");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.ip_addresses") {
                                foreach_array(event, "_ingest._value.ip_addresses", |event| {
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value.asset") {
                                            if let Some(val) = event.get("_ingest._value.asset") {
                                                let converted = convert_value(val, "boolean")
                                                    .map_err(|message| {
                                                        TransformError::ParseError {
                                                            path: "_ingest._value.asset".into(),
                                                            message,
                                                        }
                                                    })?;
                                                event.set("_ingest._value.asset", converted)?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        event.set(
                                            "_ingest.on_failure_processor_tag",
                                            "convert_ip_addresses_asset_to_boolean",
                                        )?;
                                        event.remove("_ingest._value.asset");
                                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.ip_addresses") {
                                foreach_array(event, "_ingest._value.ip_addresses", |event| {
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value.location.asn") {
                                            if let Some(val) =
                                                event.get("_ingest._value.location.asn")
                                            {
                                                let converted = convert_value(val, "long")
                                                    .map_err(|message| {
                                                        TransformError::ParseError {
                                                            path: "_ingest._value.location.asn"
                                                                .into(),
                                                            message,
                                                        }
                                                    })?;
                                                event.set(
                                                    "_ingest._value.location.asn",
                                                    converted,
                                                )?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        event.set(
                                            "_ingest.on_failure_processor_tag",
                                            "convert_ip_addresses_location_asn_to_long",
                                        )?;
                                        event.remove("_ingest._value.location.asn");
                                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_domain_joined") {
                                if let Some(val) = event.get("_ingest._value.is_domain_joined") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_domain_joined".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_domain_joined", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_is_domain_joined_to_boolean",
                            )?;
                            event.remove("_ingest._value.is_domain_joined");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.is_valid") {
                                if let Some(val) = event.get("_ingest._value.is_valid") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.is_valid".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.is_valid", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_is_valid_to_boolean",
                            )?;
                            event.remove("_ingest._value.is_valid");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.location.asn") {
                                if let Some(val) = event.get("_ingest._value.location.asn") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.location.asn".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.location.asn", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_location_asn_to_long",
                            )?;
                            event.remove("_ingest._value.location.asn");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            uri_parts(event, "_ingest._value.url", "url", true, false)?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.threat_intelligence") {
                                foreach_array(
                                    event,
                                    "_ingest._value.threat_intelligence",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.confidence") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.confidence")
                                                {
                                                    let converted = convert_value(val, "double")
                                                        .map_err(|message| {
                                                            TransformError::ParseError {
                                                                path: "_ingest._value.confidence"
                                                                    .into(),
                                                                message,
                                                            }
                                                        })?;
                                                    event.set(
                                                        "_ingest._value.confidence",
                                                        converted,
                                                    )?;
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_type",
                                                "convert",
                                            )?;
                                            event.set(
                                                "_ingest.on_failure_processor_tag",
                                                "convert_threat_intelligence_confidence_to_double",
                                            )?;
                                            event.remove("_ingest._value.confidence");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
                                                event.remove("_ingest");
                                            }
                                        }
                                        Ok(())
                                    },
                                )?;
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.resourceidentifiers") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\ndef resource_identifier_obj = new ArrayList();\nfor(entity in ctx.json.resourceidentifiers){\n  resource_identifier_obj.add(renameKeys(entity, params));\n}\nctx.resource_identifier_obj=resource_identifier_obj;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\ndef resource_identifier_obj = new ArrayList();\nfor(entity in ctx.json.resourceidentifiers){\n  resource_identifier_obj.add(renameKeys(entity, params));\n}\nctx.resource_identifier_obj=resource_identifier_obj;\n"#
                    ),
                    cached_params!(
                        "{\"$id\":\"id\",\"aadtenantid\":\"aad_tenant_id\",\"agentid\":\"agent_id\",\"azureresourceid\":\"azure_id\",\"azureresourcetenantid\":\"azure_tenant_id\",\"workspaceid\":\"workspace_id\",\"workspaceresourcegroup\":\"workspace_resource_group\",\"workspacesubscriptionid\":\"workspace_subscription_id\"}"
                    ),
                )?;
            }

            if event.has_value("resource_identifier_obj") {
                event.rename(
                    "resource_identifier_obj",
                    "microsoft_defender_cloud.event.resource_identifiers",
                )?;
            }

            if event.has_value("json.extendedlinks") {
                event.rename(
                    "json.extendedlinks",
                    "microsoft_defender_cloud.event.extended_links",
                )?;
            }

            if event.has_value("json.extendedproperties") {
                event.rename(
                    "json.extendedproperties",
                    "microsoft_defender_cloud.event.extended_properties",
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "microsoft_defender_cloud.event.id")?;
            }

            if event.has_value("json.intent") {
                event.rename("json.intent", "microsoft_defender_cloud.event.intent")?;
            }

            let _cond = { event.get_str("json.isincident") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.isincident") {
                        if let Some(val) = event.get("json.isincident") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.isincident".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_defender_cloud.event.is_incident", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_incident_to_boolean",
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

            if event.has_value("json.kind") {
                event.rename("json.kind", "microsoft_defender_cloud.event.kind")?;
            }

            if event.has_value("json.location") {
                event.rename("json.location", "microsoft_defender_cloud.event.location")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "microsoft_defender_cloud.event.name")?;
            }

            let _cond = {
                event.has_value("json.processingendtime")
                    && event.get_str("json.processingendtime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.processingendtime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_defender_cloud.event.processing_end_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.processingendtime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_processing_end_time",
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

            if event.has_value("json.productname") {
                event.rename(
                    "json.productname",
                    "microsoft_defender_cloud.event.product.name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_cloud.event.product.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.properties.$type") {
                event.rename(
                    "json.properties.$type",
                    "microsoft_defender_cloud.event.properties.type",
                )?;
            }

            if event.has_value("json.properties.assessmentdetailslink") {
                event.rename(
                    "json.properties.assessmentdetailslink",
                    "microsoft_defender_cloud.event.properties.assessment.details_link",
                )?;
            }

            if event.has_value("json.properties.assessmenttype") {
                event.rename(
                    "json.properties.assessmenttype",
                    "microsoft_defender_cloud.event.properties.assessment.type",
                )?;
            }

            if event.has_value("json.properties.category") {
                event.rename(
                    "json.properties.category",
                    "microsoft_defender_cloud.event.properties.category",
                )?;
            }

            if event.has_value("json.properties.definition.id") {
                event.rename(
                    "json.properties.definition.id",
                    "microsoft_defender_cloud.event.properties.definition.id",
                )?;
            }

            if event.has_value("json.properties.definition.name") {
                event.rename(
                    "json.properties.definition.name",
                    "microsoft_defender_cloud.event.properties.definition.name",
                )?;
            }

            let _cond = {
                event.get_str("json.properties.definition.properties.assessmentdefinitions")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("json.properties.definition.properties.assessmentdefinitions")
                    {
                        if let Some(val) =
                            event.get("json.properties.definition.properties.assessmentdefinitions")
                        {
                            let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.properties.definition.properties.assessmentdefinitions".into(),
                            message,
                        })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.assessment.definitions",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_definition_properties_assessment_definitions_to_string",
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

            if event.has_value("json.properties.definition.properties.displayname") {
                event.rename(
                    "json.properties.definition.properties.displayname",
                    "microsoft_defender_cloud.event.properties.definition.display_name",
                )?;
            }

            let _cond =
                { event.get_str("json.properties.definition.properties.maxscore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.definition.properties.maxscore") {
                        if let Some(val) =
                            event.get("json.properties.definition.properties.maxscore")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.definition.properties.maxscore".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.definition.max_score",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_definition_properties_max_score_to_long",
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

            if event.has_value("json.properties.definition.properties.source.sourcetype") {
                event.rename(
                    "json.properties.definition.properties.source.sourcetype",
                    "microsoft_defender_cloud.event.properties.definition.source_type",
                )?;
            }

            if event.has_value("json.properties.definition.type") {
                event.rename(
                    "json.properties.definition.type",
                    "microsoft_defender_cloud.event.properties.definition.type",
                )?;
            }

            if event.has_value("json.properties.description") {
                event.rename(
                    "json.properties.description",
                    "microsoft_defender_cloud.event.properties.description",
                )?;
            }

            if event.has_value("json.properties.displayname") {
                event.rename(
                    "json.properties.displayname",
                    "microsoft_defender_cloud.event.properties.display_name",
                )?;
            }

            if event.has_value("json.properties.environment") {
                event.rename(
                    "json.properties.environment",
                    "microsoft_defender_cloud.event.properties.environment",
                )?;
            }

            let _cond = { event.get_str("json.properties.failedresources") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.failedresources") {
                        if let Some(val) = event.get("json.properties.failedresources") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.failedresources".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.failed_resources",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_failed_resources_to_long",
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

            let _cond = { event.get_str("json.properties.healthyresourcecount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.healthyresourcecount") {
                        if let Some(val) = event.get("json.properties.healthyresourcecount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.healthyresourcecount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.healthy_resource_count",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_healthy_resource_count_to_long",
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

            if event.has_value("json.properties.id") {
                event.rename(
                    "json.properties.id",
                    "microsoft_defender_cloud.event.properties.id",
                )?;
            }

            if event.has_value("json.properties.impact") {
                event.rename(
                    "json.properties.impact",
                    "microsoft_defender_cloud.event.properties.impact",
                )?;
            }

            if event.has_value("json.properties.links.azureportal") {
                event.rename(
                    "json.properties.links.azureportal",
                    "microsoft_defender_cloud.event.properties.links.azure_portal",
                )?;
            }

            if event.has_value("json.properties.metadata.assessmenttype") {
                event.rename(
                    "json.properties.metadata.assessmenttype",
                    "microsoft_defender_cloud.event.properties.metadata.assessment_type",
                )?;
            }

            if event.has_value("json.properties.metadata.categories") {
                event.rename(
                    "json.properties.metadata.categories",
                    "microsoft_defender_cloud.event.properties.metadata.categories",
                )?;
            }

            if event.has_value("json.properties.metadata.description") {
                event.rename(
                    "json.properties.metadata.description",
                    "microsoft_defender_cloud.event.properties.metadata.description",
                )?;
            }

            if event.has_value("json.properties.metadata.displayname") {
                event.rename(
                    "json.properties.metadata.displayname",
                    "microsoft_defender_cloud.event.properties.metadata.display_name",
                )?;
            }

            if event.has_value("json.properties.metadata.implementationeffort") {
                event.rename(
                    "json.properties.metadata.implementationeffort",
                    "microsoft_defender_cloud.event.properties.metadata.implementation_effort",
                )?;
            }

            if event.has_value("json.properties.metadata.policydefinitionid") {
                event.rename(
                    "json.properties.metadata.policydefinitionid",
                    "microsoft_defender_cloud.event.properties.metadata.policy_definition_id",
                )?;
            }

            let _cond = { event.get_str("json.properties.metadata.preview") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.metadata.preview") {
                        if let Some(val) = event.get("json.properties.metadata.preview") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.metadata.preview".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.metadata.preview",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_metadata_preview_to_boolean",
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

            if event.has_value("json.properties.metadata.remediationdescription") {
                event.rename(
                    "json.properties.metadata.remediationdescription",
                    "microsoft_defender_cloud.event.properties.metadata.remediation_description",
                )?;
            }

            if event.has_value("json.properties.metadata.severity") {
                event.rename(
                    "json.properties.metadata.severity",
                    "microsoft_defender_cloud.event.properties.metadata.severity",
                )?;
            }

            if event.has_value("json.properties.metadata.threats") {
                event.rename(
                    "json.properties.metadata.threats",
                    "microsoft_defender_cloud.event.properties.metadata.threats",
                )?;
            }

            if event.has_value("json.properties.metadata.userimpact") {
                event.rename(
                    "json.properties.metadata.userimpact",
                    "microsoft_defender_cloud.event.properties.metadata.user_impact",
                )?;
            }

            let _cond = { event.get_str("json.properties.notapplicableresourcecount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.notapplicableresourcecount") {
                        if let Some(val) = event.get("json.properties.notapplicableresourcecount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.notapplicableresourcecount".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_defender_cloud.event.properties.not_applicable_resource_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_not_applicable_resource_count_to_long",
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

            let _cond = { event.get_str("json.properties.passedresources") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.passedresources") {
                        if let Some(val) = event.get("json.properties.passedresources") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.passedresources".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.passed_resources",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_passed_resources_to_long",
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

            if event.has_value("json.properties.remediation") {
                event.rename(
                    "json.properties.remediation",
                    "microsoft_defender_cloud.event.properties.remediation",
                )?;
            }

            if event.has_value("json.properties.resourcedetails.$type") {
                event.rename(
                    "json.properties.resourcedetails.$type",
                    "microsoft_defender_cloud.event.properties.resource_details.type",
                )?;
            }

            if event.has_value("json.properties.resourcedetails.id") {
                event.rename(
                    "json.properties.resourcedetails.id",
                    "microsoft_defender_cloud.event.properties.resource_details.id",
                )?;
            }

            if event.has_value("json.properties.resourcedetails.machinename") {
                event.rename(
                    "json.properties.resourcedetails.machinename",
                    "microsoft_defender_cloud.event.properties.resource_details.machine_name",
                )?;
            }

            if event.has_value("json.properties.resourcedetails.source") {
                event.rename(
                    "json.properties.resourcedetails.source",
                    "microsoft_defender_cloud.event.properties.resource_details.source",
                )?;
            }

            if event.has_value("json.properties.resourcedetails.sourcecomputerid") {
                event.rename(
                    "json.properties.resourcedetails.sourcecomputerid",
                    "microsoft_defender_cloud.event.properties.resource_details.source_computer_id",
                )?;
            }

            if event.has_value("json.properties.resourcedetails.vmuuid") {
                event.rename(
                    "json.properties.resourcedetails.vmuuid",
                    "microsoft_defender_cloud.event.properties.resource_details.vm_uuid",
                )?;
            }

            if event.has_value("json.properties.resourcedetails.workspaceid") {
                event.rename(
                    "json.properties.resourcedetails.workspaceid",
                    "microsoft_defender_cloud.event.properties.resource_details.workspace_id",
                )?;
            }

            let _cond = { event.get_str("json.properties.score.current") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.score.current") {
                        if let Some(val) = event.get("json.properties.score.current") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.score.current".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.score.current",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_score_current_to_double",
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

            let _cond = { event.get_str("json.properties.score.max") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.score.max") {
                        if let Some(val) = event.get("json.properties.score.max") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.score.max".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.score.max",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_score_max_to_long",
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

            let _cond = { event.get_str("json.properties.score.percentage") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.score.percentage") {
                        if let Some(val) = event.get("json.properties.score.percentage") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.score.percentage".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.score.percentage",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_score_percentage_to_double",
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

            let _cond = { event.get_str("json.properties.skippedresources") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.skippedresources") {
                        if let Some(val) = event.get("json.properties.skippedresources") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.skippedresources".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.skipped_resources",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_skipped_resources_to_long",
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

            if event.has_value("json.properties.state") {
                event.rename(
                    "json.properties.state",
                    "microsoft_defender_cloud.event.properties.state",
                )?;
            }

            if event.has_value("json.properties.status.$type") {
                event.rename(
                    "json.properties.status.$type",
                    "microsoft_defender_cloud.event.properties.status.type",
                )?;
            }

            if event.has_value("json.properties.additionaldata") {
                event.rename(
                    "json.properties.additionaldata",
                    "microsoft_defender_cloud.event.properties.additional_data",
                )?;
            }

            if event.has_value("json.properties.status.cause") {
                event.rename(
                    "json.properties.status.cause",
                    "microsoft_defender_cloud.event.properties.status.cause",
                )?;
            }

            if event.has_value("json.properties.status.code") {
                event.rename(
                    "json.properties.status.code",
                    "microsoft_defender_cloud.event.properties.status.code",
                )?;
            }

            if event.has_value("json.properties.status.description") {
                event.rename(
                    "json.properties.status.description",
                    "microsoft_defender_cloud.event.properties.status.description",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.status.firstevaluationdate")
                    && event.get_str("json.properties.status.firstevaluationdate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.properties.status.firstevaluationdate")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.event.properties.status.first_evaluation_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.properties.status.firstevaluationdate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_properties_status_first_evaluation_date",
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

            if event.has_value("json.properties.status.severity") {
                event.rename(
                    "json.properties.status.severity",
                    "microsoft_defender_cloud.event.properties.status.severity",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.status.statuschangedate")
                    && event.get_str("json.properties.status.statuschangedate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.properties.status.statuschangedate")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("microsoft_defender_cloud.event.properties.status.status_change_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.properties.status.statuschangedate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_properties_status_status_change_date",
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

            let _cond = {
                event.has_value("json.properties.timegenerated")
                    && event.get_str("json.properties.timegenerated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.timegenerated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_defender_cloud.event.properties.time_generated",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.timegenerated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_properties_time_generated",
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

            let _cond = { event.get_str("json.properties.unhealthyresourcecount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.unhealthyresourcecount") {
                        if let Some(val) = event.get("json.properties.unhealthyresourcecount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.unhealthyresourcecount".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_defender_cloud.event.properties.unhealthy_resource_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_unhealthy_resource_count_to_long",
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

            let _cond = { event.get_str("json.properties.weight") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.weight") {
                        if let Some(val) = event.get("json.properties.weight") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.weight".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_defender_cloud.event.properties.weight",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_weight_to_long",
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

            if event.has_value("json.provideralertstatus") {
                event.rename(
                    "json.provideralertstatus",
                    "microsoft_defender_cloud.event.provider_alert_status",
                )?;
            }

            if event.has_value("json.remediationsteps") {
                event.rename(
                    "json.remediationsteps",
                    "microsoft_defender_cloud.event.remediation_steps",
                )?;
            }

            if event.has_value("json.securityeventdataenrichment.$type") {
                event.rename(
                    "json.securityeventdataenrichment.$type",
                    "microsoft_defender_cloud.event.security_event_data_enrichment.type",
                )?;
            }

            if event.has_value("json.securityeventdataenrichment.action") {
                event.rename(
                    "json.securityeventdataenrichment.action",
                    "microsoft_defender_cloud.event.security_event_data_enrichment.action",
                )?;
            }

            if event.has_value("json.securityeventdataenrichment.apiversion") {
                event.rename(
                    "json.securityeventdataenrichment.apiversion",
                    "microsoft_defender_cloud.event.security_event_data_enrichment.api_version",
                )?;
            }

            if event.has_value("json.securityeventdataenrichment.interval") {
                event.rename(
                    "json.securityeventdataenrichment.interval",
                    "microsoft_defender_cloud.event.security_event_data_enrichment.interval",
                )?;
            }

            let _cond =
                { event.get_str("json.securityeventdataenrichment.issnapshot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.securityeventdataenrichment.issnapshot") {
                        if let Some(val) = event.get("json.securityeventdataenrichment.issnapshot")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.securityeventdataenrichment.issnapshot".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_defender_cloud.event.security_event_data_enrichment.is_snapshot", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_security_event_data_enrichment_is_snapshot_to_boolean",
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

            if event.has_value("json.severity") {
                event.rename("json.severity", "microsoft_defender_cloud.event.severity")?;
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.severity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.microsoft_defender_cloud.event.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.microsoft_defender_cloud.event.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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

            let _cond = {
                event.has_value("json.starttimeutc")
                    && event.get_str("json.starttimeutc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.starttimeutc") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("microsoft_defender_cloud.event.start_time_utc", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.starttimeutc".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_start_time_utc'")?;
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

            if let Some(v) = event
                .get("microsoft_defender_cloud.event.start_time_utc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if event.has_value("json.status") {
                event.rename("json.status", "microsoft_defender_cloud.event.status")?;
            }

            if event.has_value("json.subassessmenteventdataenrichment.$type") {
                event.rename(
                    "json.subassessmenteventdataenrichment.$type",
                    "microsoft_defender_cloud.event.sub_assessment_event.data_enrichment.type",
                )?;
            }

            if event.has_value("json.subassessmenteventdataenrichment.action") {
                event.rename(
                    "json.subassessmenteventdataenrichment.action",
                    "microsoft_defender_cloud.event.sub_assessment_event.data_enrichment.action",
                )?;
            }

            if event.has_value("json.subassessmenteventdataenrichment.apiversion") {
                event.rename("json.subassessmenteventdataenrichment.apiversion", "microsoft_defender_cloud.event.sub_assessment_event.data_enrichment.api_version")?;
            }

            let _cond =
                { event.get_str("json.subassessmenteventdataenrichment.issnapshot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.subassessmenteventdataenrichment.issnapshot") {
                        if let Some(val) =
                            event.get("json.subassessmenteventdataenrichment.issnapshot")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subassessmenteventdataenrichment.issnapshot".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_defender_cloud.event.sub_assessment_event.data_enrichment.is_snapshot", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sub_assessment_event_data_enrichmen_is_snapshot_to_boolean",
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

            if event.has_value("json.systemalertid") {
                event.rename(
                    "json.systemalertid",
                    "microsoft_defender_cloud.event.system.alert_id",
                )?;
            }

            if event.has_value("json.tags") {
                event.rename("json.tags", "microsoft_defender_cloud.event.tags")?;
            }

            if event.has_value("json.tenantid") {
                event.rename("json.tenantid", "microsoft_defender_cloud.event.tenant_id")?;
            }

            let _cond = {
                event.has_value("json.timegenerated")
                    && event.get_str("json.timegenerated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timegenerated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("microsoft_defender_cloud.event.time_generated", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timegenerated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_time_generated_custom",
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

            let _cond = {
                event.has_value("json.timegenerated")
                    && event.get_str("json.timegenerated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timegenerated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timegenerated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time_generated")?;
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

            if event.has_value("json.type") {
                event.rename("json.type", "microsoft_defender_cloud.event.type")?;
            }

            if event.has_value("json.vendorname") {
                event.rename(
                    "json.vendorname",
                    "microsoft_defender_cloud.event.vendor_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_defender_cloud.event.vendor_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.vendor", v)?;
            }

            if event.has_value("json.workspaceid") {
                event.rename(
                    "json.workspaceid",
                    "microsoft_defender_cloud.event.workspace.id",
                )?;
            }

            if event.has_value("json.workspaceresourcegroup") {
                event.rename(
                    "json.workspaceresourcegroup",
                    "microsoft_defender_cloud.event.workspace.resource_group",
                )?;
            }

            if event.has_value("json.workspacesubscriptionid") {
                event.rename(
                    "json.workspacesubscriptionid",
                    "microsoft_defender_cloud.event.workspace.subscription_id",
                )?;
            }

            let _cond = {
                event
                    .get("microsoft_defender_cloud.event.entities")
                    .is_some_and(|v| v.is_array())
                    && (!event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        })))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "microsoft_defender_cloud.event.entities", |event| {
                        event.remove("_ingest._value.location.cloud_provider");
                        event.remove("_ingest._value.container_id");
                        event.remove("_ingest._value.domain_name");
                        event.remove("_ingest._value.location.city");
                        event.remove("_ingest._value.location.country_code");
                        event.remove("_ingest._value.location.country_name");
                        event.remove("_ingest._value.host_name");
                        event.remove("_ingest._value.os_family");
                        event.remove("_ingest._value.protocol");
                        event.remove("_ingest._value.process_id");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            event.remove("json");

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
                event.remove("microsoft_defender_cloud.event.uri");
                event.remove("microsoft_defender_cloud.event.end_time_utc");
                event.remove("microsoft_defender_cloud.event.product.name");
                event.remove("microsoft_defender_cloud.event.start_time_utc");
                event.remove("microsoft_defender_cloud.event.time_generated");
                event.remove("microsoft_defender_cloud.event.vendor_name");
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
