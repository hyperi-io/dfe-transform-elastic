// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_knowledge_base` pipeline.
pub struct PipelineKnowledgeBase;

impl Transform for PipelineKnowledgeBase {
    fn name(&self) -> &str {
        "pipeline_knowledge_base"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.KNOWLEDGE_BASE") {
                    event.rename("json.KNOWLEDGE_BASE", "qualys_vmdr.asset_host_detection.knowledge_base")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.ID_RANGE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.ID_RANGE", "qualys_vmdr.asset_host_detection.knowledge_base.id_range")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.ID") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.ID", "qualys_vmdr.asset_host_detection.knowledge_base.ids")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE", "qualys_vmdr.asset_host_detection.knowledge_base.consequence.value")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE_COMMENT") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE_COMMENT", "qualys_vmdr.asset_host_detection.knowledge_base.consequence.comment")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DETECTION_INFO") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.DETECTION_INFO", "qualys_vmdr.asset_host_detection.knowledge_base.detection_info")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.TYPE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.TYPE", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list.type")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.SECTION") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.SECTION", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list.section")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.DESCRIPTION") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.DESCRIPTION", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list.description")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE", |event| {
                    if event.has_value("_ingest._value.TYPE") {
                    event.rename("_ingest._value.TYPE", "_ingest._value.type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE", |event| {
                    if event.has_value("_ingest._value.SECTION") {
                    event.rename("_ingest._value.SECTION", "_ingest._value.section")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE", |event| {
                    if event.has_value("_ingest._value.DESCRIPTION") {
                    event.rename("_ingest._value.DESCRIPTION", "_ingest._value.description")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list")?;
                }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CATEGORY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CATEGORY", "qualys_vmdr.asset_host_detection.knowledge_base.category")?;
                }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.category") };
            if _cond {
                event.append_unique("vulnerability.category", json!(event.get("qualys_vmdr.asset_host_detection.knowledge_base.category").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS", "qualys_vmdr.asset_host_detection.knowledge_base.diagnosis.value")?;
                }

            if let Some(v) = event.get("qualys_vmdr.asset_host_detection.knowledge_base.diagnosis.value").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.description", v)?;
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS_COMMENT") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS_COMMENT", "qualys_vmdr.asset_host_detection.knowledge_base.diagnosis.comment")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.PCI_REASONS.PCI_REASON") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.PCI_REASONS.PCI_REASON", "qualys_vmdr.asset_host_detection.knowledge_base.pci_reasons.value")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.QID") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.QID", "qualys_vmdr.asset_host_detection.knowledge_base.qid")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION", "qualys_vmdr.asset_host_detection.knowledge_base.solution.value")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION_COMMENT") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION_COMMENT", "qualys_vmdr.asset_host_detection.knowledge_base.solution.comment")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SUPPORTED_MODULES") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SUPPORTED_MODULES", "qualys_vmdr.asset_host_detection.knowledge_base.supported_modules")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.TITLE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.TITLE", "qualys_vmdr.asset_host_detection.knowledge_base.title")?;
                }

            if let Some(v) = event.get("qualys_vmdr.asset_host_detection.knowledge_base.title").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.title", v)?;
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.ID") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.ID", "qualys_vmdr.asset_host_detection.knowledge_base.bugtraq_list.id")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.URL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.URL", "qualys_vmdr.asset_host_detection.knowledge_base.bugtraq_list.url")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ", |event| {
                    if event.has_value("_ingest._value.ID") {
                    event.rename("_ingest._value.ID", "_ingest._value.id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ", |event| {
                    if event.has_value("_ingest._value.URL") {
                    event.rename("_ingest._value.URL", "_ingest._value.url")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ", "qualys_vmdr.asset_host_detection.knowledge_base.bugtraq_list")?;
                }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VULN_TYPE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.VULN_TYPE", "qualys_vmdr.asset_host_detection.knowledge_base.vuln_type")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVE_LIST") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVE_LIST", "qualys_vmdr.asset_host_detection.knowledge_base.cve_list")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.base")?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.base_obj")?;
                }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.TEMPORAL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.TEMPORAL", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.temporal")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.VECTOR_STRING") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.VECTOR_STRING", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.vector_string")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.VECTOR") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.VECTOR", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.access.vector")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.COMPLEXITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.COMPLEXITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.access.complexity")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.CONFIDENTIALITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.CONFIDENTIALITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.impact.confidentiality")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.INTEGRITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.INTEGRITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.impact.integrity")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.AVAILABILITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.AVAILABILITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.impact.availability")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.AUTHENTICATION") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.AUTHENTICATION", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.authentication")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.EXPLOITABILITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.EXPLOITABILITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.exploitability")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REMEDIATION_LEVEL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REMEDIATION_LEVEL", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.remediation_level")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REPORT_CONFIDENCE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REPORT_CONFIDENCE", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.report_confidence")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.BASE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.BASE", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.base")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.TEMPORAL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.TEMPORAL", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.temporal")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.VECTOR_STRING") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.VECTOR_STRING", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.vector_string")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.CVSS3_VERSION") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.CVSS3_VERSION", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.version")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.VECTOR") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.VECTOR", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.attack.vector")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.COMPLEXITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.COMPLEXITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.attack.complexity")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.CONFIDENTIALITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.CONFIDENTIALITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.impact.confidentiality")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.INTEGRITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.INTEGRITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.impact.integrity")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.AVAILABILITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.AVAILABILITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.impact.availability")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.PRIVILEGES_REQUIRED") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.PRIVILEGES_REQUIRED", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.privileges_required")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.USER_INTERACTION") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.USER_INTERACTION", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.user_interaction")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.SCOPE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.SCOPE", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.scope")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.EXPLOIT_CODE_MATURITY") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.EXPLOIT_CODE_MATURITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.exploit_code_maturity")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REMEDIATION_LEVEL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REMEDIATION_LEVEL", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.remediation_level")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REPORT_CONFIDENCE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REPORT_CONFIDENCE", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.report_confidence")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.AUTOMATIC_PCI_FAIL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.AUTOMATIC_PCI_FAIL", "qualys_vmdr.asset_host_detection.knowledge_base.automatic_pci_fail")?;
                }

            if let Some(v) = event.get("qualys_vmdr.asset_host_detection.knowledge_base.cve_list").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.id", v)?;
            }

            let _cond = { event.has_value("vulnerability.id") && !(event.get("vulnerability.id").is_some_and(|v| v.is_array())) };
            if _cond {
            let v = json!(format!("https://cve.mitre.org/cgi-bin/cvename.cgi?name={}", event.get("vulnerability.id").map_or_else(String::new, template_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("vulnerability.reference", v)?;
            }
            }

            let _cond = { event.get("vulnerability.id").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vulnerability.id", |event| {
                    event.append_unique("vulnerability.reference", json!(format!("https://cve.mitre.org/cgi-bin/cvename.cgi?name={}", event.get("_ingest._value").map_or_else(String::new, template_to_string))))?;
                    Ok(())
                })?;
            }

            let v = json!("CVE");
            if !painless_is_empty_value(&v) {
                    event.set("vulnerability.enumeration", v)?;
            }

            let _cond = { event.has_value("vulnerability.reference") && !(event.get("vulnerability.reference").is_some_and(|v| v.is_array())) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if !uri_parts(event, "vulnerability.reference", "url", true, false)?
                    && event.get_str("vulnerability.reference").is_some_and(|value| !value.is_empty())
                {
                    return Err(TransformError::ParseError {
                        path: "vulnerability.reference".into(),
                        message: "uri_parts: not a parseable URI".into(),
                    });
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.DESC") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.DESC", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.list.explt.desc")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.LINK") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.LINK", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.list.explt.link")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.REF") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.REF", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.list.explt.ref")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.SRC_NAME") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.SRC_NAME", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.name")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC", |event| {
                    if event.has_value("_ingest._value.SRC_NAME") {
                    event.rename("_ingest._value.SRC_NAME", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    foreach_array(event, "_ingest._value.EXPLT_LIST.EXPLT", |event| {
                    if event.has_value("_ingest._value.DESC") {
                    event.rename("_ingest._value.DESC", "_ingest._value.desc")?;
                    }
                    Ok(())
                    })?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    foreach_array(event, "_ingest._value.EXPLT_LIST.EXPLT", |event| {
                    if event.has_value("_ingest._value.LINK") {
                    event.rename("_ingest._value.LINK", "_ingest._value.link")?;
                    }
                    Ok(())
                    })?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    foreach_array(event, "_ingest._value.EXPLT_LIST.EXPLT", |event| {
                    if event.has_value("_ingest._value.REF") {
                    event.rename("_ingest._value.REF", "_ingest._value.ref")?;
                    }
                    Ok(())
                    })?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC", |event| {
                    if event.has_value("_ingest._value.EXPLT_LIST.EXPLT") {
                    event.rename("_ingest._value.EXPLT_LIST.EXPLT", "_ingest._value.list.explt")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src")?;
                }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ID") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ID", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.id")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.SRC_NAME") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.SRC_NAME", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.name")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_TYPE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_TYPE", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.type")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_PLATFORM") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_PLATFORM", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.platform")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ALIAS") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ALIAS", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.alias")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_RATING") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_RATING", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.rating")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_LINK") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_LINK", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.link")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.ADDITIONAL_INFO") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.ADDITIONAL_INFO", "qualys_vmdr.asset_host_detection.knowledge_base.discovery.additional_info")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.AUTH_TYPE_LIST.AUTH_TYPE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.AUTH_TYPE_LIST.AUTH_TYPE", "qualys_vmdr.asset_host_detection.knowledge_base.discovery.auth_type_list.value")?;
                }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE") {
                if let Some(val) = event.get("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE".into(),
                            message,
                        })?;
                    event.set("qualys_vmdr.asset_host_detection.knowledge_base.discovery.remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_DISCOVERY_REMOTE_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.PRODUCT") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.PRODUCT", "qualys_vmdr.asset_host_detection.knowledge_base.software_list.product")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.VENDOR") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.VENDOR", "qualys_vmdr.asset_host_detection.knowledge_base.software_list.vendor")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE", |event| {
                    if event.has_value("_ingest._value.PRODUCT") {
                    event.rename("_ingest._value.PRODUCT", "_ingest._value.product")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE", |event| {
                    if event.has_value("_ingest._value.VENDOR") {
                    event.rename("_ingest._value.VENDOR", "_ingest._value.vendor")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE", "qualys_vmdr.asset_host_detection.knowledge_base.software_list")?;
                }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.ID") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.ID", "qualys_vmdr.asset_host_detection.knowledge_base.vendor_reference_list.id")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.URL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.URL", "qualys_vmdr.asset_host_detection.knowledge_base.vendor_reference_list.url")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE", |event| {
                    if event.has_value("_ingest._value.ID") {
                    event.rename("_ingest._value.ID", "_ingest._value.id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE", |event| {
                    if event.has_value("_ingest._value.URL") {
                    event.rename("_ingest._value.URL", "_ingest._value.url")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE", "qualys_vmdr.asset_host_detection.knowledge_base.vendor_reference_list")?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.last.service_modification_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_LAST_SERVICE_MODIFICATION_DATETIME")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.last.customization.datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_LAST_CUSTOMIZATION_DATETIME")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.USER_LOGIN") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.USER_LOGIN", "qualys_vmdr.asset_host_detection.knowledge_base.last.customization.user_login")?;
                }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.published_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_PUBLISHED_DATETIME")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.patch_published_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_PATCH_PUBLISHED_DATE")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CHANGE_LOG_LIST.CHANGE_LOG_INFO") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CHANGE_LOG_LIST.CHANGE_LOG_INFO", "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info")?;
                }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.COMMENTS") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.COMMENTS", "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.comments")?;
                }

            let _cond = { (!(event.get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.change_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_knowledge_base_changelog_list_info_CHANGE_DATE_1")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info", |event| {
                    if event.has_value("_ingest._value.COMMENTS") {
                    event.rename("_ingest._value.COMMENTS", "_ingest._value.comments")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            if let Some(date_str) = event.get_as_string("_ingest._value.CHANGE_DATE") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_ingest._value.change_date", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.CHANGE_DATE".into(),
                            message: format!("unable to parse date [{date_str}]"),
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
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.THREAT_INTELLIGENCE.THREAT_INTEL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.THREAT_INTELLIGENCE.THREAT_INTEL", "qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel", |event| {
                    if event.has_value("_ingest._value.#text") {
                    event.rename("_ingest._value.#text", "_ingest._value.text")?;
                    }
                    Ok(())
                })?;
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel.#text") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel.#text", "qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel.text")?;
                }

            let _cond = { event.get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info", |event| {
                    event.remove("_ingest._value.CHANGE_DATE");
                    Ok(())
                })?;
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED") == Some("1") };
            if _cond {
            event.set("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED", json!(true))?;
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED") == Some("0") };
            if _cond {
            event.set("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED", json!(false))?;
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED") {
                if let Some(val) = event.get("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED".into(),
                            message,
                        })?;
                    event.set("qualys_vmdr.asset_host_detection.knowledge_base.is_disabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_IS_DISABLED_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE") == Some("1") };
            if _cond {
            event.set("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE", json!(true))?;
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE") == Some("0") };
            if _cond {
            event.set("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE", json!(false))?;
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE") {
                if let Some(val) = event.get("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE".into(),
                            message,
                        })?;
                    event.set("qualys_vmdr.asset_host_detection.knowledge_base.patchable", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_PATCHABLE_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG") == Some("1") };
            if _cond {
            event.set("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG", json!(true))?;
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG") == Some("0") };
            if _cond {
            event.set("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG", json!(false))?;
            }

            let _cond = { event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG") {
                if let Some(val) = event.get("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG".into(),
                            message,
                        })?;
                    event.set("qualys_vmdr.asset_host_detection.knowledge_base.pci_flag", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_PCI_FLAG_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (!(ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.SEVERITY_LEVEL instanceof String)) {\n  return;\n} def vuln_type = ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.vuln_type; if (!(vuln_type instanceof String)) {\n  return;\n} String level = ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL; if (params.vuln_types.contains(vuln_type)) {\n  ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL = params.vuln_level.getOrDefault(level, params.vuln_level[\"0\"]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (!(ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.SEVERITY_LEVEL instanceof String)) {\n  return;\n} def vuln_type = ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.vuln_type; if (!(vuln_type instanceof String)) {\n  return;\n} String level = ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL; if (params.vuln_types.contains(vuln_type)) {\n  ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL = params.vuln_level.getOrDefault(level, params.vuln_level[\"0\"]);\n}"#), cached_params!("{\"vuln_level\":{\"0\":\"None\",\"1\":\"Minimal\",\"2\":\"Medium\",\"3\":\"Serious\",\"4\":\"Critical\",\"5\":\"Urgent\"},\"vuln_types\":[\"Potential Vulnerability\",\"Vulnerability\",\"Vulnerability or Potential Vulnerability\",\"Information Gathered\"]}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_set_SEVERITY_LEVEL")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL") {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL", "qualys_vmdr.asset_host_detection.knowledge_base.severity_level")?;
                }

                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.CODE_MODIFIED_DATETIME");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE");
                event.remove("ID_RANGE");
                event.remove("ID");

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.category");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.cve_list");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
                drop_empty(event, &DropPolicy { nulls: true, empty_strings: true, empty_collections: true, prune_lists: true, ..DropPolicy::none() }, None);
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_to_remove_null_values")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

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
                    event.append_unique("tags", json!("preserve_original_event"))?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
