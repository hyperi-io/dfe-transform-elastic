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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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

            event.set("event.kind", json!("alert"))?;

            event.set("event.category", Value::Array(vec![json!("vulnerability")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { event.has_value("message") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "message", "json")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_message")?;
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.CVE_LIST") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.LAST_SERVICE_MODIFICATION_DATETIME") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.QID") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("ID_RANGE") {
                event.rename("ID_RANGE", "qualys_vmdr.knowledge_base.id_range")?;
            }

            if event.has_value("ID") {
                event.rename("ID", "qualys_vmdr.knowledge_base.ids")?;
            }

            if event.has_value("json.CONSEQUENCE") {
                event.rename(
                    "json.CONSEQUENCE",
                    "qualys_vmdr.knowledge_base.consequence.value",
                )?;
            }

            if event.has_value("json.CONSEQUENCE_COMMENT") {
                event.rename(
                    "json.CONSEQUENCE_COMMENT",
                    "qualys_vmdr.knowledge_base.consequence.comment",
                )?;
            }

            if event.has_value("json.DETECTION_INFO") {
                event.rename(
                    "json.DETECTION_INFO",
                    "qualys_vmdr.knowledge_base.detection_info",
                )?;
            }

            if event.has_value("json.COMPLIANCE_LIST.COMPLIANCE.TYPE") {
                event.rename(
                    "json.COMPLIANCE_LIST.COMPLIANCE.TYPE",
                    "qualys_vmdr.knowledge_base.compliance_list.type",
                )?;
            }

            if event.has_value("json.COMPLIANCE_LIST.COMPLIANCE.SECTION") {
                event.rename(
                    "json.COMPLIANCE_LIST.COMPLIANCE.SECTION",
                    "qualys_vmdr.knowledge_base.compliance_list.section",
                )?;
            }

            if event.has_value("json.COMPLIANCE_LIST.COMPLIANCE.DESCRIPTION") {
                event.rename(
                    "json.COMPLIANCE_LIST.COMPLIANCE.DESCRIPTION",
                    "qualys_vmdr.knowledge_base.compliance_list.description",
                )?;
            }

            let _cond = {
                event
                    .get("json.COMPLIANCE_LIST.COMPLIANCE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.COMPLIANCE_LIST.COMPLIANCE", |event| {
                    if event.has_value("_ingest._value.TYPE") {
                        event.rename("_ingest._value.TYPE", "_ingest._value.type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.COMPLIANCE_LIST.COMPLIANCE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.COMPLIANCE_LIST.COMPLIANCE", |event| {
                    if event.has_value("_ingest._value.SECTION") {
                        event.rename("_ingest._value.SECTION", "_ingest._value.section")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.COMPLIANCE_LIST.COMPLIANCE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.COMPLIANCE_LIST.COMPLIANCE", |event| {
                    if event.has_value("_ingest._value.DESCRIPTION") {
                        event.rename("_ingest._value.DESCRIPTION", "_ingest._value.description")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.COMPLIANCE_LIST.COMPLIANCE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.COMPLIANCE_LIST.COMPLIANCE") {
                    event.rename(
                        "json.COMPLIANCE_LIST.COMPLIANCE",
                        "qualys_vmdr.knowledge_base.compliance_list",
                    )?;
                }
            }

            if event.has_value("json.CATEGORY") {
                event.rename("json.CATEGORY", "qualys_vmdr.knowledge_base.category")?;
            }

            let _cond = { event.has_value("qualys_vmdr.knowledge_base.category") };
            if _cond {
                event.append_unique(
                    "vulnerability.category",
                    json!(
                        event
                            .get("qualys_vmdr.knowledge_base.category")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.DIAGNOSIS") {
                event.rename(
                    "json.DIAGNOSIS",
                    "qualys_vmdr.knowledge_base.diagnosis.value",
                )?;
            }

            if event.has_value("json.DIAGNOSIS_COMMENT") {
                event.rename(
                    "json.DIAGNOSIS_COMMENT",
                    "qualys_vmdr.knowledge_base.diagnosis.comment",
                )?;
            }

            if event.has_value("json.PCI_REASONS.PCI_REASON") {
                event.rename(
                    "json.PCI_REASONS.PCI_REASON",
                    "qualys_vmdr.knowledge_base.pci_reasons.value",
                )?;
            }

            if event.has_value("json.QID") {
                event.rename("json.QID", "qualys_vmdr.knowledge_base.qid")?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.knowledge_base.qid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.SOLUTION") {
                event.rename("json.SOLUTION", "qualys_vmdr.knowledge_base.solution.value")?;
            }

            if event.has_value("json.SOLUTION_COMMENT") {
                event.rename(
                    "json.SOLUTION_COMMENT",
                    "qualys_vmdr.knowledge_base.solution.comment",
                )?;
            }

            if event.has_value("json.SUPPORTED_MODULES") {
                event.rename(
                    "json.SUPPORTED_MODULES",
                    "qualys_vmdr.knowledge_base.supported_modules",
                )?;
            }

            if event.has_value("json.TITLE") {
                event.rename("json.TITLE", "qualys_vmdr.knowledge_base.title")?;
            }

            if event.has_value("json.BUGTRAQ_LIST.BUGTRAQ.ID") {
                event.rename(
                    "json.BUGTRAQ_LIST.BUGTRAQ.ID",
                    "qualys_vmdr.knowledge_base.bugtraq_list.id",
                )?;
            }

            if event.has_value("json.BUGTRAQ_LIST.BUGTRAQ.URL") {
                event.rename(
                    "json.BUGTRAQ_LIST.BUGTRAQ.URL",
                    "qualys_vmdr.knowledge_base.bugtraq_list.url",
                )?;
            }

            let _cond = {
                event
                    .get("json.BUGTRAQ_LIST.BUGTRAQ")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.BUGTRAQ_LIST.BUGTRAQ", |event| {
                    if event.has_value("_ingest._value.ID") {
                        event.rename("_ingest._value.ID", "_ingest._value.id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.BUGTRAQ_LIST.BUGTRAQ")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.BUGTRAQ_LIST.BUGTRAQ", |event| {
                    if event.has_value("_ingest._value.URL") {
                        event.rename("_ingest._value.URL", "_ingest._value.url")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.BUGTRAQ_LIST.BUGTRAQ")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.BUGTRAQ_LIST.BUGTRAQ") {
                    event.rename(
                        "json.BUGTRAQ_LIST.BUGTRAQ",
                        "qualys_vmdr.knowledge_base.bugtraq_list",
                    )?;
                }
            }

            if event.has_value("json.VULN_TYPE") {
                event.rename("json.VULN_TYPE", "qualys_vmdr.knowledge_base.vuln_type")?;
            }

            if event.has_value("json.CVE_LIST") {
                event.rename("json.CVE_LIST", "qualys_vmdr.knowledge_base.cve_list")?;
            }

            let _cond = { event.get("json.CVSS.BASE").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("json.CVSS.BASE") {
                    event.rename("json.CVSS.BASE", "qualys_vmdr.knowledge_base.cvss.base")?;
                }
            }

            // SKIPPED: condition not transpiled: ctx.json?.CVSS?.BASE instanceof Object
            #[allow(unreachable_code, unused_variables)]
            if false {
                if event.has_value("json.CVSS.BASE") {
                    event.rename("json.CVSS.BASE", "qualys_vmdr.knowledge_base.cvss.base_obj")?;
                }
            }

            if event.has_value("json.CVSS.TEMPORAL") {
                event.rename(
                    "json.CVSS.TEMPORAL",
                    "qualys_vmdr.knowledge_base.cvss.temporal",
                )?;
            }

            if event.has_value("json.CVSS.VECTOR_STRING") {
                event.rename(
                    "json.CVSS.VECTOR_STRING",
                    "qualys_vmdr.knowledge_base.cvss.vector_string",
                )?;
            }

            if event.has_value("json.CVSS.ACCESS.VECTOR") {
                event.rename(
                    "json.CVSS.ACCESS.VECTOR",
                    "qualys_vmdr.knowledge_base.cvss.access.vector",
                )?;
            }

            if event.has_value("json.CVSS.ACCESS.COMPLEXITY") {
                event.rename(
                    "json.CVSS.ACCESS.COMPLEXITY",
                    "qualys_vmdr.knowledge_base.cvss.access.complexity",
                )?;
            }

            if event.has_value("json.CVSS.IMPACT.CONFIDENTIALITY") {
                event.rename(
                    "json.CVSS.IMPACT.CONFIDENTIALITY",
                    "qualys_vmdr.knowledge_base.cvss.impact.confidentiality",
                )?;
            }

            if event.has_value("json.CVSS.IMPACT.INTEGRITY") {
                event.rename(
                    "json.CVSS.IMPACT.INTEGRITY",
                    "qualys_vmdr.knowledge_base.cvss.impact.integrity",
                )?;
            }

            if event.has_value("json.CVSS.IMPACT.AVAILABILITY") {
                event.rename(
                    "json.CVSS.IMPACT.AVAILABILITY",
                    "qualys_vmdr.knowledge_base.cvss.impact.availability",
                )?;
            }

            if event.has_value("json.CVSS.AUTHENTICATION") {
                event.rename(
                    "json.CVSS.AUTHENTICATION",
                    "qualys_vmdr.knowledge_base.cvss.authentication",
                )?;
            }

            if event.has_value("json.CVSS.EXPLOITABILITY") {
                event.rename(
                    "json.CVSS.EXPLOITABILITY",
                    "qualys_vmdr.knowledge_base.cvss.exploitability",
                )?;
            }

            if event.has_value("json.CVSS.REMEDIATION_LEVEL") {
                event.rename(
                    "json.CVSS.REMEDIATION_LEVEL",
                    "qualys_vmdr.knowledge_base.cvss.remediation_level",
                )?;
            }

            if event.has_value("json.CVSS.REPORT_CONFIDENCE") {
                event.rename(
                    "json.CVSS.REPORT_CONFIDENCE",
                    "qualys_vmdr.knowledge_base.cvss.report_confidence",
                )?;
            }

            if event.has_value("json.CVSS_V3.BASE") {
                event.rename(
                    "json.CVSS_V3.BASE",
                    "qualys_vmdr.knowledge_base.cvss_v3.base",
                )?;
            }

            if event.has_value("json.CVSS_V3.TEMPORAL") {
                event.rename(
                    "json.CVSS_V3.TEMPORAL",
                    "qualys_vmdr.knowledge_base.cvss_v3.temporal",
                )?;
            }

            if event.has_value("json.CVSS_V3.VECTOR_STRING") {
                event.rename(
                    "json.CVSS_V3.VECTOR_STRING",
                    "qualys_vmdr.knowledge_base.cvss_v3.vector_string",
                )?;
            }

            if event.has_value("json.CVSS_V3.CVSS3_VERSION") {
                event.rename(
                    "json.CVSS_V3.CVSS3_VERSION",
                    "qualys_vmdr.knowledge_base.cvss_v3.version",
                )?;
            }

            if event.has_value("json.CVSS_V3.ATTACK.VECTOR") {
                event.rename(
                    "json.CVSS_V3.ATTACK.VECTOR",
                    "qualys_vmdr.knowledge_base.cvss_v3.attack.vector",
                )?;
            }

            if event.has_value("json.CVSS_V3.ATTACK.COMPLEXITY") {
                event.rename(
                    "json.CVSS_V3.ATTACK.COMPLEXITY",
                    "qualys_vmdr.knowledge_base.cvss_v3.attack.complexity",
                )?;
            }

            if event.has_value("json.CVSS_V3.IMPACT.CONFIDENTIALITY") {
                event.rename(
                    "json.CVSS_V3.IMPACT.CONFIDENTIALITY",
                    "qualys_vmdr.knowledge_base.cvss_v3.impact.confidentiality",
                )?;
            }

            if event.has_value("json.CVSS_V3.IMPACT.INTEGRITY") {
                event.rename(
                    "json.CVSS_V3.IMPACT.INTEGRITY",
                    "qualys_vmdr.knowledge_base.cvss_v3.impact.integrity",
                )?;
            }

            if event.has_value("json.CVSS_V3.IMPACT.AVAILABILITY") {
                event.rename(
                    "json.CVSS_V3.IMPACT.AVAILABILITY",
                    "qualys_vmdr.knowledge_base.cvss_v3.impact.availability",
                )?;
            }

            if event.has_value("json.CVSS_V3.PRIVILEGES_REQUIRED") {
                event.rename(
                    "json.CVSS_V3.PRIVILEGES_REQUIRED",
                    "qualys_vmdr.knowledge_base.cvss_v3.privileges_required",
                )?;
            }

            if event.has_value("json.CVSS_V3.USER_INTERACTION") {
                event.rename(
                    "json.CVSS_V3.USER_INTERACTION",
                    "qualys_vmdr.knowledge_base.cvss_v3.user_interaction",
                )?;
            }

            if event.has_value("json.CVSS_V3.SCOPE") {
                event.rename(
                    "json.CVSS_V3.SCOPE",
                    "qualys_vmdr.knowledge_base.cvss_v3.scope",
                )?;
            }

            if event.has_value("json.CVSS_V3.EXPLOIT_CODE_MATURITY") {
                event.rename(
                    "json.CVSS_V3.EXPLOIT_CODE_MATURITY",
                    "qualys_vmdr.knowledge_base.cvss_v3.exploit_code_maturity",
                )?;
            }

            if event.has_value("json.CVSS_V3.REMEDIATION_LEVEL") {
                event.rename(
                    "json.CVSS_V3.REMEDIATION_LEVEL",
                    "qualys_vmdr.knowledge_base.cvss_v3.remediation_level",
                )?;
            }

            if event.has_value("json.CVSS_V3.REPORT_CONFIDENCE") {
                event.rename(
                    "json.CVSS_V3.REPORT_CONFIDENCE",
                    "qualys_vmdr.knowledge_base.cvss_v3.report_confidence",
                )?;
            }

            if event.has_value("json.AUTOMATIC_PCI_FAIL") {
                event.rename(
                    "json.AUTOMATIC_PCI_FAIL",
                    "qualys_vmdr.knowledge_base.automatic_pci_fail",
                )?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.knowledge_base.cve_list")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            let _cond = { event.has_value("vulnerability.reference") };
            if _cond {
                uri_parts(event, "vulnerability.reference", "url", true, false)?;
            }

            if event.has_value("json.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.DESC") {
                event.rename(
                    "json.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.DESC",
                    "qualys_vmdr.knowledge_base.correlation.exploits.explt_src.list.explt.desc",
                )?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.knowledge_base.correlation.exploits.explt_src.list.explt.desc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.description", v)?;
            }

            if event.has_value("json.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.LINK") {
                event.rename(
                    "json.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.LINK",
                    "qualys_vmdr.knowledge_base.correlation.exploits.explt_src.list.explt.link",
                )?;
            }

            if event.has_value("json.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.REF") {
                event.rename(
                    "json.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.REF",
                    "qualys_vmdr.knowledge_base.correlation.exploits.explt_src.list.explt.ref",
                )?;
            }

            if event.has_value("json.CORRELATION.EXPLOITS.EXPLT_SRC.SRC_NAME") {
                event.rename(
                    "json.CORRELATION.EXPLOITS.EXPLT_SRC.SRC_NAME",
                    "qualys_vmdr.knowledge_base.correlation.exploits.explt_src.name",
                )?;
            }

            if event.has_value("json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ID") {
                event.rename(
                    "json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ID",
                    "qualys_vmdr.knowledge_base.correlation.malware.src.list.info.id",
                )?;
            }

            if event.has_value("json.CORRELATION.MALWARE.MW_SRC.SRC_NAME") {
                event.rename(
                    "json.CORRELATION.MALWARE.MW_SRC.SRC_NAME",
                    "qualys_vmdr.knowledge_base.correlation.malware.src.name",
                )?;
            }

            if event.has_value("json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_TYPE") {
                event.rename(
                    "json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_TYPE",
                    "qualys_vmdr.knowledge_base.correlation.malware.src.list.info.type",
                )?;
            }

            if event.has_value("json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_PLATFORM") {
                event.rename(
                    "json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_PLATFORM",
                    "qualys_vmdr.knowledge_base.correlation.malware.src.list.info.platform",
                )?;
            }

            if event.has_value("json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ALIAS") {
                event.rename(
                    "json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ALIAS",
                    "qualys_vmdr.knowledge_base.correlation.malware.src.list.info.alias",
                )?;
            }

            if event.has_value("json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_RATING") {
                event.rename(
                    "json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_RATING",
                    "qualys_vmdr.knowledge_base.correlation.malware.src.list.info.rating",
                )?;
            }

            if event.has_value("json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_LINK") {
                event.rename(
                    "json.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_LINK",
                    "qualys_vmdr.knowledge_base.correlation.malware.src.list.info.link",
                )?;
            }

            if event.has_value("json.DISCOVERY.ADDITIONAL_INFO") {
                event.rename(
                    "json.DISCOVERY.ADDITIONAL_INFO",
                    "qualys_vmdr.knowledge_base.discovery.additional_info",
                )?;
            }

            if event.has_value("json.DISCOVERY.AUTH_TYPE_LIST.AUTH_TYPE") {
                event.rename(
                    "json.DISCOVERY.AUTH_TYPE_LIST.AUTH_TYPE",
                    "qualys_vmdr.knowledge_base.discovery.auth_type_list.value",
                )?;
            }

            let _cond = { event.get_str("json.DISCOVERY.REMOTE") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.DISCOVERY.REMOTE") {
                        if let Some(val) = event.get("json.DISCOVERY.REMOTE") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.DISCOVERY.REMOTE".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_vmdr.knowledge_base.discovery.remote", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_DISCOVERY_REMOTE_to_long",
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

            if event.has_value("json.SOFTWARE_LIST.SOFTWARE.PRODUCT") {
                event.rename(
                    "json.SOFTWARE_LIST.SOFTWARE.PRODUCT",
                    "qualys_vmdr.knowledge_base.software_list.product",
                )?;
            }

            if event.has_value("json.SOFTWARE_LIST.SOFTWARE.VENDOR") {
                event.rename(
                    "json.SOFTWARE_LIST.SOFTWARE.VENDOR",
                    "qualys_vmdr.knowledge_base.software_list.vendor",
                )?;
            }

            let _cond = {
                event
                    .get("json.SOFTWARE_LIST.SOFTWARE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.SOFTWARE_LIST.SOFTWARE", |event| {
                    if event.has_value("_ingest._value.PRODUCT") {
                        event.rename("_ingest._value.PRODUCT", "_ingest._value.product")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.SOFTWARE_LIST.SOFTWARE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.SOFTWARE_LIST.SOFTWARE", |event| {
                    if event.has_value("_ingest._value.VENDOR") {
                        event.rename("_ingest._value.VENDOR", "_ingest._value.vendor")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.SOFTWARE_LIST.SOFTWARE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.SOFTWARE_LIST.SOFTWARE") {
                    event.rename(
                        "json.SOFTWARE_LIST.SOFTWARE",
                        "qualys_vmdr.knowledge_base.software_list",
                    )?;
                }
            }

            if event.has_value("json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.ID") {
                event.rename(
                    "json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.ID",
                    "qualys_vmdr.knowledge_base.vendor_reference_list.id",
                )?;
            }

            if event.has_value("json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.URL") {
                event.rename(
                    "json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.URL",
                    "qualys_vmdr.knowledge_base.vendor_reference_list.url",
                )?;
            }

            let _cond = {
                event
                    .get("json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE",
                    |event| {
                        if event.has_value("_ingest._value.ID") {
                            event.rename("_ingest._value.ID", "_ingest._value.id")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE",
                    |event| {
                        if event.has_value("_ingest._value.URL") {
                            event.rename("_ingest._value.URL", "_ingest._value.url")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE") {
                    event.rename(
                        "json.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE",
                        "qualys_vmdr.knowledge_base.vendor_reference_list",
                    )?;
                }
            }

            let _cond = {
                event.has_value("json.LAST_SERVICE_MODIFICATION_DATETIME")
                    && event.get_str("json.LAST_SERVICE_MODIFICATION_DATETIME") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.LAST_SERVICE_MODIFICATION_DATETIME")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.knowledge_base.last.service_modification_datetime",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LAST_SERVICE_MODIFICATION_DATETIME".into(),
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
                        "date_LAST_SERVICE_MODIFICATION_DATETIME",
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

            let _cond = {
                event.has_value("json.LAST_SERVICE_MODIFICATION_DATETIME")
                    && event.get_str("json.LAST_SERVICE_MODIFICATION_DATETIME") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.LAST_SERVICE_MODIFICATION_DATETIME")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LAST_SERVICE_MODIFICATION_DATETIME".into(),
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
                        "date_LAST_SERVICE_MODIFICATION_DATETIME_to_timestamp",
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

            let _cond = {
                event.has_value("json.LAST_CUSTOMIZATION.DATETIME")
                    && event.get_str("json.LAST_CUSTOMIZATION.DATETIME") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LAST_CUSTOMIZATION.DATETIME")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.knowledge_base.last.customization.datetime",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LAST_CUSTOMIZATION.DATETIME".into(),
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
                        "date_LAST_CUSTOMIZATION_DATETIME",
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

            if event.has_value("json.LAST_CUSTOMIZATION.USER_LOGIN") {
                event.rename(
                    "json.LAST_CUSTOMIZATION.USER_LOGIN",
                    "qualys_vmdr.knowledge_base.last.customization.user_login",
                )?;
            }

            let _cond = {
                event.has_value("json.PUBLISHED_DATETIME")
                    && event.get_str("json.PUBLISHED_DATETIME") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.PUBLISHED_DATETIME") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_vmdr.knowledge_base.published_datetime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.PUBLISHED_DATETIME".into(),
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
                        "date_PUBLISHED_DATETIME",
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

            let _cond = {
                event.has_value("json.PATCH_PUBLISHED_DATE")
                    && event.get_str("json.PATCH_PUBLISHED_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.PATCH_PUBLISHED_DATE") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_vmdr.knowledge_base.patch_published_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.PATCH_PUBLISHED_DATE".into(),
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
                        "date_PATCH_PUBLISHED_DATE",
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

            if event.has_value("json.CHANGE_LOG_LIST.CHANGE_LOG_INFO") {
                event.rename(
                    "json.CHANGE_LOG_LIST.CHANGE_LOG_INFO",
                    "qualys_vmdr.knowledge_base.changelog_list.info",
                )?;
            }

            if event.has_value("qualys_vmdr.knowledge_base.changelog_list.info.COMMENTS") {
                event.rename(
                    "qualys_vmdr.knowledge_base.changelog_list.info.COMMENTS",
                    "qualys_vmdr.knowledge_base.changelog_list.info.comments",
                )?;
            }

            let _cond = {
                (!(event
                    .get("qualys_vmdr.knowledge_base.changelog_list.info")
                    .is_some_and(|v| v.is_array())))
                    && event.has_value("qualys_vmdr.knowledge_base.changelog_list.info.CHANGE_DATE")
                    && event.get_str("qualys_vmdr.knowledge_base.changelog_list.info.CHANGE_DATE")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("qualys_vmdr.knowledge_base.changelog_list.info.CHANGE_DATE")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.knowledge_base.changelog_list.info.change_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "qualys_vmdr.knowledge_base.changelog_list.info.CHANGE_DATE"
                                            .into(),
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
                        "date_qualys_vmdr_knowledge_base_changelog_list_info_CHANGE_DATE_1",
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

            let _cond = {
                event
                    .get("qualys_vmdr.knowledge_base.changelog_list.info")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.knowledge_base.changelog_list.info",
                    |event| {
                        if event.has_value("_ingest._value.COMMENTS") {
                            event.rename("_ingest._value.COMMENTS", "_ingest._value.comments")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.knowledge_base.changelog_list.info")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.knowledge_base.changelog_list.info")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.CHANGE_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.change_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.CHANGE_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
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
                            "qualys_vmdr.knowledge_base.changelog_list.info",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.THREAT_INTELLIGENCE.THREAT_INTEL") {
                event.rename(
                    "json.THREAT_INTELLIGENCE.THREAT_INTEL",
                    "qualys_vmdr.knowledge_base.threat_intelligence.intel",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.knowledge_base.threat_intelligence.intel")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.knowledge_base.threat_intelligence.intel",
                    |event| {
                        if event.has_value("_ingest._value.#text") {
                            event.rename("_ingest._value.#text", "_ingest._value.text")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.knowledge_base.threat_intelligence.intel.#text") {
                event.rename(
                    "qualys_vmdr.knowledge_base.threat_intelligence.intel.#text",
                    "qualys_vmdr.knowledge_base.threat_intelligence.intel.text",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.knowledge_base.changelog_list.info")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.knowledge_base.changelog_list.info",
                    |event| {
                        event.remove("_ingest._value.CHANGE_DATE");
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.get_str("json.IS_DISABLED") == Some("1") };
            if _cond {
                event.set("json.IS_DISABLED", json!(true))?;
            }

            let _cond = { event.get_str("json.IS_DISABLED") == Some("0") };
            if _cond {
                event.set("json.IS_DISABLED", json!(false))?;
            }

            let _cond = { event.get_str("json.IS_DISABLED") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IS_DISABLED") {
                        if let Some(val) = event.get("json.IS_DISABLED") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IS_DISABLED".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_vmdr.knowledge_base.is_disabled", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_IS_DISABLED_to_boolean",
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

            let _cond = { event.get_str("json.PATCHABLE") == Some("1") };
            if _cond {
                event.set("json.PATCHABLE", json!(true))?;
            }

            let _cond = { event.get_str("json.PATCHABLE") == Some("0") };
            if _cond {
                event.set("json.PATCHABLE", json!(false))?;
            }

            let _cond = { event.get_str("json.PATCHABLE") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PATCHABLE") {
                        if let Some(val) = event.get("json.PATCHABLE") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PATCHABLE".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_vmdr.knowledge_base.patchable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_PATCHABLE_to_boolean",
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

            let _cond = { event.get_str("json.PCI_FLAG") == Some("1") };
            if _cond {
                event.set("json.PCI_FLAG", json!(true))?;
            }

            let _cond = { event.get_str("json.PCI_FLAG") == Some("0") };
            if _cond {
                event.set("json.PCI_FLAG", json!(false))?;
            }

            let _cond = { event.get_str("json.PCI_FLAG") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PCI_FLAG") {
                        if let Some(val) = event.get("json.PCI_FLAG") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PCI_FLAG".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_vmdr.knowledge_base.pci_flag", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_PCI_FLAG_to_boolean",
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

            if let Some(v) = event
                .get("json.SEVERITY_LEVEL")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("qualys_vmdr.knowledge_base.severity_level", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (!(ctx.json?.SEVERITY_LEVEL instanceof String)) {\n  return;\n} def vuln_type = ctx.qualys_vmdr?.knowledge_base?.vuln_type; if (!(vuln_type instanceof String)) {\n  return;\n} def level = Long.parseLong(ctx.json.SEVERITY_LEVEL); if (['Potential Vulnerability', 'Vulnerability', 'Vulnerability or Potential Vulnerability'].contains(vuln_type)) {\n  if (level == 1){\n      ctx.json.SEVERITY_LEVEL = \"Minimal\";\n  } else if (level == 2) {\n      ctx.json.SEVERITY_LEVEL = \"Medium\";\n  } else if (level == 3) {\n      ctx.json.SEVERITY_LEVEL = \"Serious\";\n  } else if (level == 4) {\n      ctx.json.SEVERITY_LEVEL = \"Critical\";\n  } else if (level == 5) {\n      ctx.json.SEVERITY_LEVEL = \"Urgent\";\n  }\n} else if (vuln_type == \"Information Gathered\") {\n  if (level == 1) {\n      ctx.json.SEVERITY_LEVEL = \"Minimal\";\n  } else if (level == 2) {\n      ctx.json.SEVERITY_LEVEL = \"Medium\";\n  } else if (level == 3) {\n      ctx.json.SEVERITY_LEVEL = \"Serious\";\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (!(ctx.json?.SEVERITY_LEVEL instanceof String)) {\n  return;\n} def vuln_type = ctx.qualys_vmdr?.knowledge_base?.vuln_type; if (!(vuln_type instanceof String)) {\n  return;\n} def level = Long.parseLong(ctx.json.SEVERITY_LEVEL); if (['Potential Vulnerability', 'Vulnerability', 'Vulnerability or Potential Vulnerability'].contains(vuln_type)) {\n  if (level == 1){\n      ctx.json.SEVERITY_LEVEL = \"Minimal\";\n  } else if (level == 2) {\n      ctx.json.SEVERITY_LEVEL = \"Medium\";\n  } else if (level == 3) {\n      ctx.json.SEVERITY_LEVEL = \"Serious\";\n  } else if (level == 4) {\n      ctx.json.SEVERITY_LEVEL = \"Critical\";\n  } else if (level == 5) {\n      ctx.json.SEVERITY_LEVEL = \"Urgent\";\n  }\n} else if (vuln_type == \"Information Gathered\") {\n  if (level == 1) {\n      ctx.json.SEVERITY_LEVEL = \"Minimal\";\n  } else if (level == 2) {\n      ctx.json.SEVERITY_LEVEL = \"Medium\";\n  } else if (level == 3) {\n      ctx.json.SEVERITY_LEVEL = \"Serious\";\n  }\n}"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_set_SEVERITY_LEVEL",
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

            if let Some(v) = event
                .get("json.SEVERITY_LEVEL")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.severity", v)?;
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        event.rename("message", "event.original")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            event.remove("json");
            event.remove("message");
            event.remove("qualys_vmdr.knowledge_base.changelog_list.info.CHANGE_DATE");
            event.remove("ID_RANGE");
            event.remove("ID");

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
                event.remove("qualys_vmdr.knowledge_base.last.service_modification_datetime");
                event.remove("qualys_vmdr.knowledge_base.qid");
                event.remove("qualys_vmdr.knowledge_base.category");
                event.remove("qualys_vmdr.knowledge_base.cve_list");
                event.remove(
                    "qualys_vmdr.knowledge_base.correlation.exploits.explt_src.list.explt.desc",
                );
                event.remove("qualys_vmdr.knowledge_base.severity_level");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_remove_null_values",
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

            // Painless script
            // Source: def filterMassive(def src) {\n  if (src instanceof Map) {\n    for (def entry: src.entrySet()) {\n      entry.setValue(filterMassive(entry.getValue()));\n    }\n    return src;\n  } else if (src instanceof List) {\n    for (int i = 0; i < src.length; i++) {\n      src[i] = filterMassive(src[i]);\n    }\n    return src;\n  } else if (src instanceof String && src.length() > 32766) {\n    return src.substring(0, 32700)+' (truncated)';\n  }\n  return src;\n}\nfilterMassive(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def filterMassive(def src) {\n  if (src instanceof Map) {\n    for (def entry: src.entrySet()) {\n      entry.setValue(filterMassive(entry.getValue()));\n    }\n    return src;\n  } else if (src instanceof List) {\n    for (int i = 0; i < src.length; i++) {\n      src[i] = filterMassive(src[i]);\n    }\n    return src;\n  } else if (src instanceof String && src.length() > 32766) {\n    return src.substring(0, 32700)+' (truncated)';\n  }\n  return src;\n}\nfilterMassive(ctx);"#
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
                            .get("_ingest.pipeline")
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
