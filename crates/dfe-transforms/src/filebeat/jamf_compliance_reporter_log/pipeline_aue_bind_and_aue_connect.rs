// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_aue_bind_and_aue_connect` pipeline.
pub struct PipelineAueBindAndAueConnect;

impl Transform for PipelineAueBindAndAueConnect {
    fn name(&self) -> &str {
        "pipeline_aue_bind_and_aue_connect"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.socket_inet.addr") {
                if let Some(val) = event.get("json.socket_inet.addr") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.socket_inet.addr".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.socket.inet.addr", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.arguments.fd") {
                if let Some(val) = event.get("json.arguments.fd") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.fd".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.fd", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.socket_inet.family") {
                if let Some(val) = event.get("json.socket_inet.family") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.socket_inet.family".into(),
                            message,
                        })?;
                    event.set("json.inet_family", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.socket_inet.id") {
                if let Some(val) = event.get("json.socket_inet.id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.socket_inet.id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.socket.inet.id", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.socket_inet.ip_address") {
                if let Some(val) = event.get("json.socket_inet.ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.socket_inet.ip_address".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("json.socket_inet.ip_address");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("server.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.socket_inet.port") {
                if let Some(val) = event.get("json.socket_inet.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.socket_inet.port".into(),
                            message,
                        })?;
                    event.set("server.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.inet_family") };
            if _cond {
                // Painless script
                // Source: Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.inet.family = map.get(ctx.json.inet_family);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.inet.family = map.get(ctx.json.inet_family);\n"#))?;
            }

                // Begin nested pipeline: "pipeline_identity_object"
                if event.has_value("json.identity.cd_hash") {
                event.rename("json.identity.cd_hash", "jamf_compliance_reporter.log.identity.cd_hash")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("jamf_compliance_reporter.log.identity.cd_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.identity.signer_id") {
                event.rename("json.identity.signer_id", "jamf_compliance_reporter.log.identity.signer.id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.signer_id_truncated") {
                if let Some(val) = event.get("json.identity.signer_id_truncated") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.signer_id_truncated".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.signer.id_truncated", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.signer_type") {
                if let Some(val) = event.get("json.identity.signer_type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.signer_type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.signer.type", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.identity.team_id") {
                event.rename("json.identity.team_id", "jamf_compliance_reporter.log.identity.team.id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.team_id_truncated") {
                if let Some(val) = event.get("json.identity.team_id_truncated") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.team_id_truncated".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.team.id_truncated", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_identity_object"

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
